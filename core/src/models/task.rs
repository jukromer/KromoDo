use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

use super::priority::Priority;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub is_done: bool,
    pub priority: Priority,
    pub due_date: Option<DateTime<Utc>>,
    pub has_due_time: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn is_overdue(&self, now: DateTime<Utc>) -> bool {
        if self.is_done {
            return false;
        }
        match self.due_date {
            Some(due) => {
                due.with_timezone(&Local).date_naive()
                    < now.with_timezone(&Local).date_naive()
            }
            None => false,
        }
    }
}
