use crate::error::CoreResult;
use rusqlite::Connection;

pub fn run_migrations(conn: &Connection) -> CoreResult<()> {
    let version: i32 =
        conn.pragma_query_value(None, "user_version", |row| row.get(0))?;

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

    if version < 2 {
        conn.execute_batch(
            "ALTER TABLE tasks ADD COLUMN due_date     TEXT;
             ALTER TABLE tasks ADD COLUMN has_due_time INTEGER NOT NULL DEFAULT 0;
             ALTER TABLE tasks ADD COLUMN updated_at   TEXT;
             ALTER TABLE tasks ADD COLUMN completed_at TEXT;

             UPDATE tasks SET updated_at = created_at WHERE updated_at IS NULL;

             CREATE INDEX IF NOT EXISTS idx_tasks_due_date  ON tasks(due_date);
             CREATE INDEX IF NOT EXISTS idx_tasks_is_done   ON tasks(is_done);

             PRAGMA user_version = 2;",
        )?;
    }

    Ok(())
}
