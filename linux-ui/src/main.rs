use kromodo_core::{AppState, Task};
use relm4::prelude::*;
use relm4::gtk::prelude::*;
use relm4::gtk;
use std::sync::Arc;

fn db_path() -> String {
    let dir = gtk::glib::user_data_dir().join("kromodo");
    std::fs::create_dir_all(&dir).expect("Could not create data directory");
    dir.join("tasks.db")
        .to_string_lossy()
        .into_owned()
}

struct App {
    state: Arc<AppState>,
    tasks: Vec<Task>,
    entry_buffer: gtk::EntryBuffer,
}

#[derive(Debug)]
enum Msg {
    AddTask,
    ToggleTask(i64),
    DeleteTask(i64),
    Refresh,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = Arc<AppState>;
    type Input = Msg;
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

                    gtk::Box {
                        set_spacing: 8,
                        
                        
                        gtk::Entry {
                            set_hexpand: true,
                            set_placeholder_text: Some("New Task..."),
                            set_buffer: &model.entry_buffer,
                            connect_activate => Msg::AddTask,
                        },

                        gtk::Button {
                            set_label: "Add",
                            set_css_classes: &["suggested-action"],
                            connect_clicked => Msg::AddTask,
                        },
                    },

                    gtk::ScrolledWindow {
                        set_vexpand: true,

                        gtk::ListBox {
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
        let tasks = state.list_tasks().unwrap_or_default();

        let model = App {
            state,
            tasks,
            entry_buffer: gtk::EntryBuffer::new(None::<&str>),
        };

        let widgets = view_output!();

        rebuild_task_list(&widgets, &model, &sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Msg, sender: ComponentSender<Self>) {
        match msg {
            Msg::AddTask => {
                let text = self.entry_buffer.text();
                if !text.trim().is_empty() {
                    let _ = self.state.add_task(text.trim());
                    self.entry_buffer.set_text("");
                    sender.input(Msg::Refresh);
                }
            }
            Msg::ToggleTask(id) => {
                let _ = self.state.toggle_tasks(id);
                sender.input(Msg::Refresh);
            }
            Msg::DeleteTask(id) => {
                let _ = self.state.delete_task(id);
                sender.input(Msg::Refresh)
            }
            Msg::Refresh => {
                self.tasks = self.state.list_tasks().unwrap_or_default();
            }
        }
    }
}

fn rebuild_task_list(
    _widgets: &AppWidgets,
    _model: &App,
    _sender: &ComponentSender<App>,
) {
    //coming soon
}

fn main() {
    let state = Arc::new(
        AppState::new(&db_path()).expect("Could not open Database"),
    );

    let app = RelmApp::new("dev.kromodo.app");
    app.run::<App>(state);
}