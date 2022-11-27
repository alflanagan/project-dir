pub use rusqlite;
pub use rusqlite::{params, Connection};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub path: String,
}

pub fn create_db(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            path  TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )
    .and_then(|updated| {
        println!("Create table projects, got {} result.", updated);
        Ok(())
    })
}

pub fn save_to_db(conn: &Connection, project: &Project) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT INTO projects (name, path) VALUES (?, ?)",
        params![project.name, project.path],
    )
}

pub fn read_from_db(conn: &Connection) -> rusqlite::Result<HashMap<String, String>> {
    let mut sql = conn.prepare("SELECT name, path FROM projects;")?;
    let result: HashMap<String, String> = HashMap::new();
    let rows = sql.query_map([], |row| row.get::<usize, String>(0))?;
    for row in rows {
        println!("{}", row.unwrap());
    }
    Ok(result)
}

pub fn clear_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM projects", ())
        .and_then(|_updated| {
            Ok(())
        })
}
