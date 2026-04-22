use chrono::{DateTime, Utc};
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
