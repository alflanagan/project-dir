use rusqlite::{Connection, Result};
// use std::fs;
// use std::io;

#[derive(Debug)]
struct Project {
    id: i32,
    name: String,
    path: String,
}

fn create_db(conn: Connection) {
    match conn.execute(
        "CREATE TABLE projects (
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

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    create_db(conn);

    let fred = Project {
        id: 0,
        name: "Steven".to_string(),
        path: "/home/steven/steven".to_string(),
    };
    println!("{}", fred.id);
    println!("{:?}", fred);
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
