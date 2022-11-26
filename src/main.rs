mod config;

use std::collections::HashMap;
// use std::fs::read;

use rusqlite::{params, Connection, Result};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
struct Project {
    name: String,
    path: String,
}

fn scan_for_projects(conn: &Connection, dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                scan_for_projects(conn, &path)?;
            } else {
                if path.ends_with(".git") {
                    println!("{:?}", path);
                    let project = Project {
                        path: path.display().to_string(),
                        name: "fred".to_string(),
                    };
                    save_to_db(conn, &project);
                }
            }
        }
    }
    Ok(())
}

fn create_db(conn: &Connection) {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            path  TEXT NOT NULL
        )",
        (), // empty list of parameters.
    ) {
        Ok(updated) => println!("Create table projects, got {} result.", updated),
        Err(err) => panic!("Failed to create table projects: {}.", err),
    };
}

fn save_to_db(conn: &Connection, project: &Project) {
    let sql = String::from(
        "INSERT INTO projects (name, path)
           VALUES (?, ?)",
    );

    match conn.execute(&sql, params![project.name, project.path]) {
        Ok(updated) => println!("Added {} row to the project db", updated),
        Err(err) => println!("ERROR: failed to update project database: {}", err),
    }
}

fn read_from_db(conn: &Connection) -> Result<HashMap<String, String>> {
    let mut sql = conn.prepare("SELECT name, path FROM projects;")?;
    let result: HashMap<String, String> = HashMap::new();
    let rows = sql.query_map([], |row| row.get::<usize, String>(0))?;
    for row in rows {
        println!("{}", row.unwrap());
    }
    Ok(result)
}

fn find_projects(conn: &Connection, dirs: Vec<String>) {
    for dir in dirs.iter() {
        let path = Path::new(dir);
        scan_for_projects(conn, path)
            .expect(&*format!("Unable to scan directory {}", path.display()));
    }
}

fn main() {
    let settings = config::get_config();

    let conn = match Connection::open(settings.get::<String>("db_file").unwrap()) {
        Err(err) => panic!("Configuration retrieval failure: {}", err),
        Ok(connection) => connection,
    };

    create_db(&conn);

    // TODO: check for param --refresh, if found do find-projects(), otherwise read from db
    find_projects(&conn, settings.get("project_dirs").unwrap());

    let projects = match read_from_db(&conn) {
        Ok(projects) => projects,
        Err(e) => panic!("failure reading from database: {}", e),
    };

    for (name, path) in projects {
        println!("project: {}: {}", name, path);
    }
}
