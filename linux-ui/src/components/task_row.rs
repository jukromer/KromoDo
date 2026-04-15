use kromodo_core::Task;
use relm4::gtk;
use relm4::gtk::glib;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

pub struct TaskRow {
    task: Task,
}

#[derive(Debug)]
pub enum TaskRowInput {
    Toggle,
}

#[derive(Debug)]
pub enum TaskRowOutput {
    Toggled(i64),
}

fn priority_class(priority: i8) -> &'static str {
    match priority {
        1 => "priority-low",
        2 => "priority-medium",
        3 => "priority-high",
        4 => "priority-urgent",
        _ => "priority-none",
    }
}

fn row_classes(task: &Task) -> Vec<&'static str> {
    let mut classes = vec!["task-row", priority_class(task.priority)];
    if task.is_done {
        classes.push("task-done");
    }
    classes
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
            set_spacing: 10,
            #[watch]
            set_css_classes: &row_classes(&self.task),

            gtk::CheckButton {
                set_valign: gtk::Align::Center,
                set_active: self.task.is_done,
                connect_toggled => TaskRowInput::Toggle,
            },

            gtk::Label {
                set_valign: gtk::Align::Center,
                set_use_markup: true,
                #[watch]
                set_label: &self.formatted_title(),
                set_hexpand: true,
                set_halign: gtk::Align::Start,
                set_ellipsize: gtk::pango::EllipsizeMode::End,
                add_css_class: "task-title",
            },
        }
    }

    fn init_model(
        task: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self { task }
    }

    fn update(&mut self, msg: TaskRowInput, sender: FactorySender<Self>) {
        match msg {
            TaskRowInput::Toggle => {
                sender.output(TaskRowOutput::Toggled(self.task.id)).ok();
            }
        }
    }
}
