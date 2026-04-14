use kromodo_core::Task;
use relm4::prelude::*;
use relm4::gtk::prelude::*;
use relm4::gtk;

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

            gtk::CheckButton {
                set_active: self.task.is_done,
                connect_toggled => TaskRowInput::Toggle,
            },

            gtk::Label {
                #[watch]
                set_label: &self.task.title,
                set_hexpand: true,
                set_halign: gtk::Align::Start,
                #[watch]
                set_css_classes: if self.task.is_done {
                    &["dim-label"]
                } else {
                    &[]
                },
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