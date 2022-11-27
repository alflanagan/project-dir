//! project-dir is a simple utility that reads a configured directory tree (or trees) and finds all
//! the .git source control directories in them. For each directory found, it remembers the parent
//! directory and the project name (the name of the immediate parent of .git).
//!
//! It then creates a shell function the user can invoke with the command `project project-name`.
//! The shell function will change the user's directory to the project directory, and set up the
//! shell environment in useful ways depending on the type of project. This may include, for instance,
//! switching to a specific node version, activating a virtual environment, or starting some
//! background service. (The part where we set up actions is not yet well-specified).

// #![allow(unused_imports, unused_variables, dead_code)]
mod config;
mod db;
mod scripts;

use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::exit;

fn scan_for_projects(conn: &db::Connection, dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // TODO: really need to handle the error conditions here
                let parts = path
                    .components()
                    .map(|x| x.as_os_str().to_str().unwrap())
                    .collect::<Vec<&str>>();
                let tail = parts[parts.len() - 1];
                if tail.eq(".git") {
                    let project_name = parts[parts.len() - 2];
                    let project = db::Project {
                        path: path.display().to_string(),
                        name: project_name.to_owned(),
                    };
                    let count = db::save_to_db(conn, &project).expect(
                        format!("Failure to save project {} to database", project_name).as_str(),
                    );
                    // panic is appropriate here, I don't know how this could happen or what to do
                    if count != 1 {
                        panic!("Expected to update 1 row, but updated {}", count)
                    }
                    // don't recurse here, we don't want sub-modules
                } else {
                    scan_for_projects(conn, &path)?;
                }
            }
        }
    }
    Ok(())
}

fn find_projects(conn: &db::Connection, dirs: Vec<String>) {
    db::clear_table(conn).expect("Error clearing database table");
    for dir in dirs.iter() {
        let path = Path::new(dir);
        scan_for_projects(conn, path)
            .expect(&*format!("Unable to scan directory {}", path.display()));
    }
}

fn main() -> rusqlite::Result<()> {
    let settings = config::get_config();

    for arg in env::args() {
        if env::args().len() != 2
            || arg.eq_ignore_ascii_case("--help")
            || arg.eq_ignore_ascii_case("-h")
        {
            println!("Usage: project-dir _project_name_");
            println!("  Switch to directory for project_name, activate appropriate environments.");
            exit(0);
        }
    }
    let db_file = config::expand_home(settings.get::<String>("db_file").unwrap().as_str());
    let project_dirs = settings
        .get::<Vec<String>>("project_dirs")
        .expect(&*format!(
            "Unable to retrieve config value project_dirs from {:?}",
            settings
        ));

    let conn = match db::Connection::open(db_file) {
        Err(err) => panic!("Configuration retrieval failure: {}", err),
        Ok(connection) => connection,
    };

    db::create_db(&conn)?;

    // TODO: check for param --refresh, if found do find-projects(), otherwise read from db
    find_projects(&conn, project_dirs);

    let projects = match db::read_from_db(&conn) {
        Ok(projects) => projects,
        Err(e) => panic!("failure reading from database: {}", e),
    };

    for (name, path) in projects {
        println!("project: {}: {}", name, path);
    }
    Ok(())
}
