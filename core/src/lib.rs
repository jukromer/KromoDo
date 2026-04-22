mod db;
mod error;
mod filter;
mod migration;
mod models;

pub use chrono::{DateTime, Utc};
pub use error::{CoreError, CoreResult};
pub use filter::TaskFilter;
pub use models::due::{due_bucket, DueBucket};
pub use models::priority::Priority;
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
        priority: Priority,
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

    pub fn list_tasks_for_filter(&self, filter: TaskFilter) -> CoreResult<Vec<Task>> {
        self.db.lock().unwrap().list_tasks_for_filter(filter)
    }

    pub fn update_task(&self, task: &mut Task) -> CoreResult<bool> {
        self.db.lock().unwrap().update_task(task)
    }

    pub fn toggle_task(&self, id: i64) -> CoreResult<Option<Task>> {
        self.db.lock().unwrap().toggle_task(id)
    }

    pub fn delete_task(&self, id: i64) -> CoreResult<bool> {
        self.db.lock().unwrap().delete_task(id)
    }

    pub fn duplicate_task(&self, id: i64) -> CoreResult<Task> {
        self.db.lock().unwrap().duplicate_task(id)
    }
}
