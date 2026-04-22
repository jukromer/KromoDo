mod db;
mod error;
mod events;
mod filter;
mod migration;
mod models;

pub use chrono::{DateTime, Utc};
pub use error::{CoreError, CoreResult};
pub use events::CoreEvent;
pub use filter::TaskFilter;
pub use models::due::{due_bucket, DueBucket};
pub use models::priority::Priority;
pub use models::task::Task;

use db::Database;
use events::EventBus;
use std::sync::{mpsc, Mutex};

pub struct AppState {
    db: Mutex<Database>,
    events: EventBus,
}

impl AppState {
    pub fn new(db_path: &str) -> CoreResult<Self> {
        let db = Database::open(db_path)?;
        Ok(Self {
            db: Mutex::new(db),
            events: EventBus::new(),
        })
    }

    pub fn subscribe(&self) -> mpsc::Receiver<CoreEvent> {
        self.events.subscribe()
    }

    pub fn add_task(
        &self,
        title: &str,
        description: &str,
        priority: Priority,
        due_date: Option<DateTime<Utc>>,
        has_due_time: bool,
    ) -> CoreResult<Task> {
        let task = self.db.lock().unwrap().add_task(
            title,
            description,
            priority,
            due_date,
            has_due_time,
        )?;
        self.events.publish(CoreEvent::TaskCreated(task.clone()));
        Ok(task)
    }

    pub fn list_tasks_for_filter(&self, filter: TaskFilter) -> CoreResult<Vec<Task>> {
        self.db.lock().unwrap().list_tasks_for_filter(filter)
    }

    pub fn update_task(&self, task: &mut Task) -> CoreResult<bool> {
        let updated = self.db.lock().unwrap().update_task(task)?;
        if updated {
            self.events.publish(CoreEvent::TaskUpdated(task.clone()));
        }
        Ok(updated)
    }

    pub fn toggle_task(&self, id: i64) -> CoreResult<Option<Task>> {
        let task = self.db.lock().unwrap().toggle_task(id)?;
        if let Some(ref t) = task {
            self.events.publish(CoreEvent::TaskUpdated(t.clone()));
        }
        Ok(task)
    }

    pub fn delete_task(&self, id: i64) -> CoreResult<bool> {
        let deleted = self.db.lock().unwrap().delete_task(id)?;
        if deleted {
            self.events.publish(CoreEvent::TaskDeleted(id));
        }
        Ok(deleted)
    }

    pub fn duplicate_task(&self, id: i64) -> CoreResult<Task> {
        let task = self.db.lock().unwrap().duplicate_task(id)?;
        self.events.publish(CoreEvent::TaskCreated(task.clone()));
        Ok(task)
    }
}
