use rusqlite::{params, Connection, Result};
// use std::fs;
// use std::io;

#[derive(Debug)]
struct Project {
    id: i32,
    name: String,
    path: String,
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
        "INSERT INTO projects (id, name, path)
           VALUES (?, ?, ?)",
    );

    match conn.execute(&sql, params![project.id, project.name, project.path]) {
        Ok(updated) => println!("Added {} row to the project db", updated),
        Err(err) => println!(
            "ERROR: failed to update project database: {}",
            err.to_string()
        ),
    }
}

fn main() -> Result<()> {
    let path = "./projects.db3";

    let conn = Connection::open(path)?;

    create_db(&conn);

    let fred = Project {
        id: 0,
        name: "Steven".to_string(),
        path: "/home/steven/steven".to_string(),
    };
    save_to_db(&conn, &fred);

    /*
       conn.execute(
           "INSERT INTO person (name, data) VALUES (?1, ?2)",
           (&me.name, &me.data),
       )?;

       let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
       let person_iter = stmt.query_map([], |row| {
           Ok(Person {
               id: row.get(0)?,
               name: row.get(1)?,
               data: row.get(2)?,
           })
       })?;

       for person in person_iter {
           println!("Found person {:?}", person.unwrap());
       }
    */
    Ok(())
}
