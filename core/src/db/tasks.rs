use crate::db::Database;
use crate::error::{CoreError, CoreResult};
use crate::models::task::Task;

impl Database {
    pub fn add_task(&self, title: &str, description: &str, priority: i8) -> CoreResult<Task> {
        if title.trim().is_empty() {
            return Err(CoreError::Validation(
                "Title can't be empty.".into()
            ));
        }

        let now = chrono::Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO tasks (title, description, is_done, priority, created_at) VALUES (?1, ?2, 0, ?3, ?4)",
            rusqlite::params![title.trim(), description.trim(), priority, &now],
        )?;

        let id = self.conn.last_insert_rowid();

        Ok(Task {
            id,
            title: title.trim().to_string(),
            description: description.trim().to_string(),
            is_done: false,
            priority: priority,
            created_at: now,
        })
    }

    pub fn list_tasks(&self) -> CoreResult<Vec<Task>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, description, is_done, priority, created_at FROM tasks ORDER BY id DESC")?;
        
        let tasks = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    is_done: row.get::<_, i32>(3)? != 0,
                    priority: row.get::<_, i8>(4)?,
                    created_at: row.get(5)?,
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