// this file is used to define the schema of our logs
use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};
use serde_json::Value;

// The NightOwlLog struct for SQLite (Database table)
#[derive(Serialize, Deserialize, Debug)]
pub struct NightOwlLog {
    pub id: Option<i32>, 
    pub tag: String,
    pub message: String,
    pub data: Value, 
}

// Model to add a log (used when adding a log via API)
#[derive(Serialize, Deserialize)]
pub struct AddNightOwlLog {
    pub tag: String,
    pub message: String,
    pub data: Value,
}

// Model to get a log (used when retrieving logs via API)
#[derive(serde::Serialize,Deserialize)]
pub struct GetNightOwlLog {
    pub tag: String,
}

// Initialize the database and create the table if it doesn't exist
pub fn initialize_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS NightOwlLog (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tag TEXT NOT NULL,
            message TEXT NOT NULL,
            data TEXT NOT NULL
        )",
        (),
    )?;
    Ok(())
}

// Insert a new log into the NightOwlLog table
pub fn insert_log(conn: &Connection, log: &AddNightOwlLog) -> Result<()> {
    let serialized_data = log.data.to_string(); // Serialize the `data` field as a string

    // Insert the log into the database
    conn.execute(
        "INSERT INTO NightOwlLog (tag, message, data) VALUES (?1, ?2, ?3)",
        &[&log.tag, &log.message, &serialized_data],
    )?;
    Ok(())
}

// Retrieve logs from the database by tag
pub fn get_logs_by_tag(conn: &Connection, tag: &str) -> Result<Vec<NightOwlLog>> {
    let mut stmt = conn.prepare("SELECT id, tag, message, data FROM NightOwlLog WHERE tag = ?1")?;
    let log_iter = stmt.query_map([tag], |row| {
        Ok(NightOwlLog {
            id: row.get(0)?,
            tag: row.get(1)?,
            message: row.get(2)?,
            data: serde_json::from_str(&row.get::<_, String>(3)?).unwrap(),
        })
    })?;

    let mut logs = Vec::new();
    for log in log_iter {
        logs.push(log?);
    }
    Ok(logs)
}