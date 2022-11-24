use std::collections::HashMap;
// use std::fs::read;

use config::Config;
use rusqlite::{params, Connection, Result};
// use std::fs;
// use std::io;

#[derive(Debug)]
struct Project {
    name: String,
    path: String,
}

#[derive(Debug)]
#[allow(unused)]
struct Settings {
    db_file: String,
    project_dirs: [String],
}

fn get_config() -> Config {
    // TODO: Handle case for configuration file not found
    match Config::builder()
        // what's an error case for set_default()??
        .set_default("db_file", "./projects.db3")
        .unwrap()
        .set_default("project_dirs", "/home/lloyd/Devel")
        .unwrap()
        .add_source(config::File::with_name("./projects.yaml"))
        .build()
    {
        Err(err) => panic!("Error reading config file: {}", err),
        Ok(settings) => settings,
    }
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

fn main() {
    let settings = get_config();

    let conn = match Connection::open(settings.get::<String>("db_file").unwrap()) {
        Err(err) => panic!("Configuration retrieval failure: {}", err),
        Ok(connection) => connection,
    };

    create_db(&conn);

    let fred = Project {
        name: "Steven".to_string(),
        path: "/home/steven/steven".to_string(),
    };
    save_to_db(&conn, &fred);

    let projects = match read_from_db(&conn) {
        Ok(projects) => projects,
        Err(e) => panic!("failure reading from database: {}", e),
    };

    for (name, path) in projects {
        println!("project: {}: {}", name, path);
    }
}
