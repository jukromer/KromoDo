use chrono::{DateTime, Utc};

use crate::db::Database;
use crate::error::{CoreError, CoreResult};
use crate::models::task::Task;

const SELECT_COLUMNS: &str = "id, title, description, is_done, priority, \
     due_date, has_due_time, created_at, updated_at, completed_at";

impl Database {
    pub fn add_task(
        &self,
        title: &str,
        description: &str,
        priority: i8,
        due_date: Option<DateTime<Utc>>,
        has_due_time: bool,
    ) -> CoreResult<Task> {
        let title = title.trim();
        if title.is_empty() {
            return Err(CoreError::Validation("Title can't be empty.".into()));
        }
        let description = description.trim();
        let now = Utc::now();

        self.conn.execute(
            "INSERT INTO tasks
                 (title, description, is_done, priority,
                  due_date, has_due_time, created_at, updated_at)
             VALUES (?1, ?2, 0, ?3, ?4, ?5, ?6, ?6)",
            rusqlite::params![
                title,
                description,
                priority,
                due_date,
                has_due_time,
                now,
            ],
        )?;

        Ok(Task {
            id: self.conn.last_insert_rowid(),
            title: title.to_string(),
            description: description.to_string(),
            is_done: false,
            priority,
            due_date,
            has_due_time,
            created_at: now,
            updated_at: now,
            completed_at: None,
        })
    }

    pub fn list_tasks(&self) -> CoreResult<Vec<Task>> {
        let sql = format!("SELECT {SELECT_COLUMNS} FROM tasks ORDER BY id DESC");
        let mut stmt = self.conn.prepare(&sql)?;
        let tasks = stmt
            .query_map([], map_task_row)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(tasks)
    }

    pub fn list_tasks_due_between(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> CoreResult<Vec<Task>> {
        let sql = format!(
            "SELECT {SELECT_COLUMNS} FROM tasks
             WHERE due_date IS NOT NULL
               AND due_date >= ?1
               AND due_date <  ?2
             ORDER BY is_done ASC, due_date ASC, id DESC"
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let tasks = stmt
            .query_map(rusqlite::params![from, to], map_task_row)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(tasks)
    }

    pub fn list_completed_tasks(&self) -> CoreResult<Vec<Task>> {
        let sql = format!(
            "SELECT {SELECT_COLUMNS} FROM tasks
             WHERE is_done = 1
             ORDER BY completed_at DESC, id DESC"
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let tasks = stmt
            .query_map([], map_task_row)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(tasks)
    }

    pub fn toggle_task(&self, id: i64) -> CoreResult<bool> {
        let now = Utc::now();
        let affected = self.conn.execute(
            "UPDATE tasks
                SET is_done      = NOT is_done,
                    completed_at = CASE WHEN is_done = 0 THEN ?1 ELSE NULL END,
                    updated_at   = ?1
              WHERE id = ?2",
            rusqlite::params![now, id],
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

fn map_task_row(row: &rusqlite::Row) -> rusqlite::Result<Task> {
    Ok(Task {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        is_done: row.get(3)?,
        priority: row.get(4)?,
        due_date: row.get(5)?,
        has_due_time: row.get(6)?,
        created_at: row.get(7)?,
        updated_at: row.get(8)?,
        completed_at: row.get(9)?,
    })
}
