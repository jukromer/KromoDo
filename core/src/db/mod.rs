pub mod tasks;

use rusqlite::Connection;
use crate::error::CoreResult;

pub struct Database {
    pub(crate) conn: Connection,
}

impl Database {
    pub fn open(path: &str) -> CoreResult<Self> {
        let conn = Connection::open(path)?;

        conn.execute_batch("PRAGMA journal_mode=WAL;")?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tasks (
                id          INTEGER PRIMARY KEY,
                title       TEXT    NOT NULL,
                is_done     INTEGER NOT NULL DEFAULT 0,
                created_at  TEXT    NOT NULL
            );",
        
        )?;

        Ok(Self { conn })
    }
}