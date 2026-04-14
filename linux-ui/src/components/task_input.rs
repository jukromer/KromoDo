use relm4::prelude::*;
use relm4::gtk::prelude::*;
use relm4::gtk;

pub struct TaskInput {
    buffer: gtk::EntryBuffer,
}

#[derive(Debug)]
pub enum TaskInputMsg {
    Submit,
}

#[derive(Debug)]
pub enum TaskInputOutput {
    AddTask(String),
}

#[relm4::component(pub)]
impl SimpleComponent for TaskInput {
    type Init = ();
    type Input = TaskInputMsg;
    type Output = TaskInputOutput;

    view! {
        gtk::Box {
            set_spacing: 8,

            gtk::Entry {
                set_hexpand: true,
                set_placeholder_text: Some("New Task..."),
                set_buffer: &model.buffer,
                connect_activate => TaskInputMsg::Submit,
            },

            gtk::Button {
                set_label: "Add",
                set_css_classes: &["suggested-action"],
                connect_clicked => TaskInputMsg::Submit,
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = TaskInput {
            buffer: gtk::EntryBuffer::new(None::<&str>),
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: TaskInputMsg, sender: ComponentSender<Self>) {
        match msg {
            TaskInputMsg::Submit => {
                let text = self.buffer.text();
                if !text.trim().is_empty() {
                    sender.output(TaskInputOutput::AddTask(
                        text.trim().to_string()
                    )).ok();
                    self.buffer.set_text("");
                }
            }
        }
    }
}