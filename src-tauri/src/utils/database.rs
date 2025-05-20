use rusqlite::{Connection, Result as SqlResult};
use std::env;
use std::path::{PathBuf};
use std::fs;
use crate::commands::path::GLOBAL_PATHS;

fn get_path() -> SqlResult<PathBuf, String> {    
    let global_paths = GLOBAL_PATHS.lock().unwrap();
    
    if !global_paths.config_dir.is_empty() {
        let mut path = PathBuf::from(&global_paths.config_dir);
        fs::create_dir_all(&path).map_err(|_| "Failed to create app data dir".to_string())?;
        path.push("database.sqlite");

        Ok(path)
    } else {
        Err("App data path is not set".to_string())
    }
}

pub fn get_connection() -> SqlResult<Connection> {
    let db_path = get_path().map_err(|e| rusqlite::Error::InvalidPath(e.parse().unwrap()))?;    
    Connection::open(db_path.as_path())
}

pub fn init_db() -> SqlResult<()> {
    let conn = get_connection().map_err(|e| {
        eprintln!("Failed to get DB connection: {:?}", e);
        e
    })?;

    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS teachers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR NOT NULL UNIQUE,
            grades VARCHAR,
            total_lessions INTEGER NOT NULL DEFAULT 0,
            present INTEGER NOT NULL DEFAULT 0,
            absent INTEGER NOT NULL DEFAULT 0,
            late INTEGER NOT NULL DEFAULT 0
        )
        "#,
        [],
    ).map_err(|e| {
        eprintln!("Failed to execute CREATE TABLE statement: {:?}", e);
        e
    })?;
    Ok(())
}
