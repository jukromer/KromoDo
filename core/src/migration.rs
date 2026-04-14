use rusqlite::Connection;
use crate::error::CoreResult;

pub fn run_migrations(conn: &Connection) -> CoreResult<()> {
    let version: i32 = conn.pragma_query_value(
        None, "user_version", |row|row.get(0)
    )?;

    if version < 1 {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tasks (
                id          INTEGER PRIMARY KEY,
                title       TEXT    NOT NULL,
                description TEXT    NOT NULL DEFAULT '',
                is_done     INTEGER NOT NULL DEFAULT 0,
                priority    INTEGER NOT NULL DEFAULT 0,
                created_at  TEXT    NOT NULL
            );
            PRAGMA user_version = 1;",
        )?;
    }

    Ok(())
}