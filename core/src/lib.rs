mod db;
mod error;
mod task;

pub use error::{CoreError, CoreResult};
pub use task::Task;

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

    pub fn add_task(&self, title: &str) -> CoreResult<Task> {
        self.db.lock().unwrap().add_task(title)
    }

    pub fn list_tasks(&self) -> CoreResult<Vec<Task>> {
        self.db.lock().unwrap().list_tasks()
    }

    pub fn toggle_tasks(&self, id: i64) -> CoreResult<bool> {
        self.db.lock().unwrap().toggle_task(id)
    }

    pub fn delete_task(&self, id: i64) -> CoreResult<bool> {
        self.db.lock().unwrap().delete_task(id)
    }
}