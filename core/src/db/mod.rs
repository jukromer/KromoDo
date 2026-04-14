pub mod tasks;

use rusqlite::Connection;
use crate::error::CoreResult;
use crate::migration;

pub struct Database {
    pub(crate) conn: Connection,
}

impl Database {
    pub fn open(path: &str) -> CoreResult<Self> {
        let conn = Connection::open(path)?;

        conn.execute_batch("PRAGMA journal_mode=WAL;")?;

        migration::run_migrations(&conn)?;

        Ok(Self { conn })
    }
}