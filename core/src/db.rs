use rusqlite::Connection;
use crate::error::{CoreError, CoreResult};
use crate::task::Task;

pub struct Database {
    conn: Connection,
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

    pub fn add_task(&self, title: &str) -> CoreResult<Task> {
        if title.trim().is_empty() {
            return Err(CoreError::Validation(
                "Title can't be empty.".into()
            ));
        }

        let now = chrono::Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO tasks (title, is_done, created_at) VALUES (?1, 0, ?2)",
            rusqlite::params![title.trim(), &now],
        )?;

        let id = self.conn.last_insert_rowid();

        Ok(Task {
            id,
            title: title.trim().to_string(),
            is_done: false,
            created_at: now,
        })
    }

    pub fn list_tasks(&self) -> CoreResult<Vec<Task>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, is_done, created_at FROM tasks ORDER BY id DESC")?;
        
        let tasks = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    is_done: row.get::<_, i32>(2)? != 0,
                    created_at: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tasks)
    }

    pub fn toggle_task(&self, id: i64) -> CoreResult<bool> {
        let affected = self.conn.execute(
            "UPDATE tasks SET is_done = NOT is_done WHERE id = ?1",
            rusqlite::params![id],
        )?;

        Ok(affected > 0)
    }

    pub fn delete_task(&self, id: i64) -> CoreResult<bool> {
        let affected = self.conn.execute(
            "DELETE FROM tasks WHERE id = ?1",
            rusqlite::params![id],
        )?;

        Ok(affected > 0)
    }
}