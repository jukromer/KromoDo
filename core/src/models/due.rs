use chrono::{DateTime, Local, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DueBucket {
    Yesterday,
    Today,
    Tomorrow,
    Other,
}

pub fn due_bucket(due: DateTime<Utc>, now: DateTime<Utc>) -> DueBucket {
    let due_local = due.with_timezone(&Local).date_naive();
    let today = now.with_timezone(&Local).date_naive();

    if due_local == today {
        DueBucket::Today
    } else if Some(due_local) == today.succ_opt() {
        DueBucket::Tomorrow
    } else if Some(due_local) == today.pred_opt() {
        DueBucket::Yesterday
    } else {
        DueBucket::Other
    }
}
