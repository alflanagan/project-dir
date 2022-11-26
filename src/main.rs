mod config;
mod db;

use std::fs;
use std::io;
use std::path::Path;

fn scan_for_projects(conn: &db::Connection, dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                scan_for_projects(conn, &path)?;
            } else {
                if path.ends_with(".git") {
                    println!("{:?}", path);
                    let project = db::Project {
                        path: path.display().to_string(),
                        name: "fred".to_string(),
                    };
                    db::save_to_db(conn, &project);
                }
            }
        }
    }
    Ok(())
}

fn find_projects(conn: &db::Connection, dirs: Vec<String>) {
    for dir in dirs.iter() {
        let path = Path::new(dir);
        scan_for_projects(conn, path)
            .expect(&*format!("Unable to scan directory {}", path.display()));
    }
}

fn main() {
    let settings = config::get_config();

    let conn = match db::Connection::open(settings.get::<String>("db_file").unwrap()) {
        Err(err) => panic!("Configuration retrieval failure: {}", err),
        Ok(connection) => connection,
    };

    db::create_db(&conn);

    // TODO: check for param --refresh, if found do find-projects(), otherwise read from db
    find_projects(&conn, settings.get::<Vec<String>>("project_dirs").unwrap());

    let projects = match db::read_from_db(&conn) {
        Ok(projects) => projects,
        Err(e) => panic!("failure reading from database: {}", e),
    };

    for (name, path) in projects {
        println!("project: {}: {}", name, path);
    }
}
