use chrono::{DateTime, Local, TimeZone, Utc};

use crate::models::task::Task;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskFilter {
    Inbox,
    Today,
    Upcoming,
    Completed,
}

impl TaskFilter {
    pub fn matches(&self, task: &Task) -> bool {
        match self {
            Self::Inbox => true,
            Self::Today => {
                !task.is_done
                    && task.due_date.map_or(false, |d| d < today_range().1)
            }
            Self::Upcoming => {
                !task.is_done
                    && task.due_date.map_or(false, |d| d >= today_range().1)
            }
            Self::Completed => task.is_done,
        }
    }
}

pub(crate) fn today_range() -> (DateTime<Utc>, DateTime<Utc>) {
    let today = Local::now().date_naive();
    let tomorrow = today.succ_opt().expect("date overflow");
    let start = Local
        .from_local_datetime(&today.and_hms_opt(0, 0, 0).unwrap())
        .single()
        .expect("ambiguous local time")
        .with_timezone(&Utc);
    let end = Local
        .from_local_datetime(&tomorrow.and_hms_opt(0, 0, 0).unwrap())
        .single()
        .expect("ambiguous local time")
        .with_timezone(&Utc);
    (start, end)
}
