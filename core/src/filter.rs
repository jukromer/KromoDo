use chrono::{DateTime, Local, TimeZone, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskFilter {
    Inbox,
    Today,
    Upcoming,
    Completed,
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
