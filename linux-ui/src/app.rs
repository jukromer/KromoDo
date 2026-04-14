use kromodo_core::AppState;
use relm4::prelude::*;
use relm4::gtk::prelude::*;
use relm4::gtk;
use std::sync::Arc;

use crate::components::task_input::{TaskInput, TaskInputOutput};
use crate::components::task_row::{TaskRow, TaskRowOutput};

pub struct App {
    state: Arc<AppState>,
    tasks: FactoryVecDeque<TaskRow>,
    task_input: Controller<TaskInput>,
}

#[derive(Debug)]
pub enum AppMsg {
    AddTask(String),
    ToggleTask(i64),
    DeleteTask(i64),
    Refresh,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = Arc<AppState>;
    type Input = AppMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("KromoDo"),
            set_default_width: 480,
            set_default_height: 640,

            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    set_title_widget: Some(&gtk::Label::new(Some("KromoDo"))),
                },

                #[wrap(Some)]
                set_content = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 12,
                    set_margin_all: 16,

                    model.task_input.widget(),

                    gtk::ScrolledWindow {
                        set_vexpand: true,

                        #[local_ref]
                        task_list_box -> gtk::ListBox {
                            set_css_classes: &["boxed-list"],
                            set_selection_mode: gtk::SelectionMode::None,
                        },
                    },
                },
            },
        }
    }

    fn init(
        state: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let task_input = TaskInput::builder()
            .launch(())
            .forward(sender.input_sender(), |output| match output {
                TaskInputOutput::AddTask(title) => AppMsg::AddTask(title),
            });

        let mut tasks = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |output| match output {
                TaskRowOutput::Toggled(id) => AppMsg::ToggleTask(id),
                TaskRowOutput::Deleted(id) => AppMsg::DeleteTask(id),
            });

        let initial_tasks = state.list_tasks().unwrap_or_default();
        {
            let mut guard = tasks.guard();
            for task in initial_tasks {
                guard.push_back(task);
            }
        }

        let model = App {
            state,
            tasks,
            task_input,
        };

        let task_list_box = model.tasks.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: AppMsg, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::AddTask(title) => {
                let _ = self.state.add_task(&title, "", 0);
                sender.input(AppMsg::Refresh);
            }
            AppMsg::ToggleTask(id) => {
                let _ = self.state.toggle_task(id);
                sender.input(AppMsg::Refresh);
            }
            AppMsg::DeleteTask(id) => {
                let _ = self.state.delete_task(id);
                sender.input(AppMsg::Refresh);
            }
            AppMsg::Refresh => {
                let updated_tasks = self.state.list_tasks().unwrap_or_default();
                let mut guard = self.tasks.guard();
                guard.clear();
                for task in updated_tasks {
                    guard.push_back(task);
                }
            }
        }
    }
}