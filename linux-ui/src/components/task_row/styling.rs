use chrono::{DateTime, Local, Utc};
use kromodo_core::{due_bucket, DueBucket, Priority, Task};
use relm4::gtk::glib;

use super::TaskRow;

pub(super) fn format_due_display(due: Option<DateTime<Utc>>) -> Option<String> {
    let dt = due?;
    Some(match due_bucket(dt, Utc::now()) {
        DueBucket::Today => "Today".to_string(),
        DueBucket::Tomorrow => "Tomorrow".to_string(),
        DueBucket::Yesterday => "Yesterday".to_string(),
        DueBucket::Other => dt
            .with_timezone(&Local)
            .date_naive()
            .format("%a, %-d %b")
            .to_string(),
    })
}

pub(super) fn priority_class(priority: Priority) -> &'static str {
    match priority {
        Priority::Low => "priority-low",
        Priority::Medium => "priority-medium",
        Priority::High => "priority-high",
        Priority::Urgent => "priority-urgent",
        Priority::None => "priority-none",
    }
}

pub(super) fn compact_row_classes(task: &Task) -> Vec<&'static str> {
    let mut classes = vec!["task-row", priority_class(task.priority)];
    if task.is_done {
        classes.push("task-done");
    }
    classes
}

impl TaskRow {
    pub(super) fn formatted_title(&self) -> String {
        if self.task.is_done {
            format!("<s>{}</s>", glib::markup_escape_text(&self.task.title))
        } else {
            glib::markup_escape_text(&self.task.title).to_string()
        }
    }

    pub(super) fn card_classes(&self) -> &'static [&'static str] {
        if self.expanded {
            &["task-edit-card"]
        } else {
            &[]
        }
    }

    pub(super) fn priority_dot_classes(&self, level: Priority) -> Vec<&'static str> {
        let mut classes = vec!["priority-dot"];
        classes.push(match level {
            Priority::Low => "priority-dot-low",
            Priority::Medium => "priority-dot-medium",
            Priority::High => "priority-dot-high",
            Priority::Urgent => "priority-dot-urgent",
            Priority::None => "",
        });
        if self.task.priority == level {
            classes.push("priority-dot-active");
        }
        classes
    }

    pub(super) fn due_label_classes(&self) -> &'static [&'static str] {
        if self.task.is_overdue(Utc::now()) {
            &["caption", "task-due-label", "task-due-overdue"]
        } else {
            &["caption", "dim-label", "task-due-label"]
        }
    }
}
