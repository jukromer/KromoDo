use kromodo_core::Task;
use relm4::prelude::*;
use relm4::gtk::prelude::*;
use relm4::gtk;
use relm4::gtk::glib;

pub struct TaskRow {
    task: Task,
}

#[derive(Debug)]
pub enum TaskRowInput {
    Toggle,
    Delete,
}

#[derive(Debug)]
pub enum TaskRowOutput {
    Toggled(i64),
    Deleted(i64),
}

fn priority_css_class(priority: i8) -> &'static str {
    match priority {
        1 => "priority-low",
        2 => "priority-medium",
        3 => "priority-high",
        4 => "priority-urgent",
        _ => "priority-none",
    }
}

impl TaskRow {
    fn formatted_title(&self) -> String {
        if self.task.is_done {
            format!("<s>{}</s>", glib::markup_escape_text(&self.task.title))
        } else {
            glib::markup_escape_text(&self.task.title).to_string()
        }
    }
}

#[relm4::factory(pub)]
impl FactoryComponent for TaskRow {
    type Init = Task;
    type Input = TaskRowInput;
    type Output = TaskRowOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 8,
            set_margin_all: 8,
            add_css_class: "task-row",
            #[watch]
            set_css_classes: if self.task.is_done {
                &["task-row", "task-done"]
            } else {
                &["task-row"]
            },

            gtk::Box {
                add_css_class: "priority-indicator",
                #[watch]
                add_css_class: priority_css_class(self.task.priority),
            },

            gtk::CheckButton {
                set_active: self.task.is_done,
                connect_toggled => TaskRowInput::Toggle,
            },

            gtk::Label {
                set_use_markup: true,
                #[watch]
                set_label: &self.formatted_title(),
                set_hexpand: true,
                set_halign: gtk::Align::Start,
                add_css_class: "task-title",
            },

            gtk::Button {
                set_icon_name: "edit-delete-symbolic",
                set_css_classes: &["flat"],
                connect_clicked => TaskRowInput::Delete,
            },
        }
    }

    fn init_model(task: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { task }
    }

    fn update(&mut self, msg: TaskRowInput, sender: FactorySender<Self>) {
        match msg {
            TaskRowInput::Toggle => {
                sender.output(TaskRowOutput::Toggled(self.task.id)).ok();
            }
            TaskRowInput::Delete => {
                sender.output(TaskRowOutput::Deleted(self.task.id)).ok();
            }
        }
    }
}