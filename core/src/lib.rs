mod db;
mod error;
mod migration;
mod models;

pub use chrono::{DateTime, Utc};
pub use error::{CoreError, CoreResult};
pub use models::task::Task;

use db::Database;
use std::sync::Mutex;

pub struct AppState {
    db: Mutex<Database>,
}

impl AppState {
    pub fn new(db_path: &str) -> CoreResult<Self> {
        let db = Database::open(db_path)?;
        Ok(Self { db: Mutex::new(db) })
    }

    pub fn add_task(
        &self,
        title: &str,
        description: &str,
        priority: i8,
        due_date: Option<DateTime<Utc>>,
        has_due_time: bool,
    ) -> CoreResult<Task> {
        self.db.lock().unwrap().add_task(
            title,
            description,
            priority,
            due_date,
            has_due_time,
        )
    }

    pub fn list_tasks(&self) -> CoreResult<Vec<Task>> {
        self.db.lock().unwrap().list_tasks()
    }

    pub fn list_tasks_due_between(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> CoreResult<Vec<Task>> {
        self.db.lock().unwrap().list_tasks_due_between(from, to)
    }

    pub fn list_completed_tasks(&self) -> CoreResult<Vec<Task>> {
        self.db.lock().unwrap().list_completed_tasks()
    }

    pub fn update_task(&self, task: &mut Task) -> CoreResult<bool> {
        self.db.lock().unwrap().update_task(task)
    }

    pub fn toggle_task(&self, id: i64) -> CoreResult<bool> {
        self.db.lock().unwrap().toggle_task(id)
    }

    pub fn delete_task(&self, id: i64) -> CoreResult<bool> {
        self.db.lock().unwrap().delete_task(id)
    }
}
