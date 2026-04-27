use adw::prelude::*;
use kromodo_core::AppState;
use relm4::gtk::gdk;
use relm4::gtk::glib;
use relm4::prelude::*;
use relm4::{adw, gtk};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::components::quick_add::{QuickAdd, QuickAddInput, QuickAddOutput};
use crate::components::sidebar::{Sidebar, SidebarOutput, SidebarSelection};
use crate::components::task_row::{TaskRow, TaskRowInput, TaskRowOutput};
use kromodo_core::{CoreEvent, Priority, Task};

pub struct App {
    state: Arc<AppState>,
    tasks: FactoryVecDeque<TaskRow>,
    completed_tasks: FactoryVecDeque<TaskRow>,
    quick_add: Controller<QuickAdd>,
    sidebar: Controller<Sidebar>,
    selection: SidebarSelection,
    show_sidebar: bool,
    pending_finalize: HashMap<i64, u64>,
    next_finalize_token: u64,
}

#[derive(Debug)]
pub enum AppMsg {
    OpenQuickAdd,
    AddTask { title: String, description: String },
    UpdateTask(Task),
    DuplicateTask(i64),
    DeleteTask(i64),
    ToggleTask(i64),
    Refresh,
    SelectView(SidebarSelection),
    ToggleSidebar,
    CoreEvent(CoreEvent),
    FinalizeMove { task: Task, to_done: bool, token: u64 },
    FinalizeRemove { id: i64, token: u64 },
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = Arc<AppState>;
    type Input = AppMsg;
    type Output = ();

    view! {
        #[root]
        adw::ApplicationWindow {
            set_title: Some(""),
            set_default_width: 1000,
            set_default_height: 640,

            #[wrap(Some)]
            set_content = &adw::OverlaySplitView {
                #[watch]
                set_show_sidebar: model.show_sidebar,
                set_min_sidebar_width: 260.0,
                set_max_sidebar_width: 320.0,
                set_sidebar: Some(model.sidebar.widget()),

                #[wrap(Some)]
                set_content = &adw::ToolbarView {
                    add_css_class: "kromodo-content",
                    add_top_bar = &adw::HeaderBar {
                        pack_start = &gtk::ToggleButton {
                            set_icon_name: "sidebar-show-symbolic",
                            set_active: true,
                            set_tooltip_text: Some("Toggle sidebar"),
                            connect_clicked => AppMsg::ToggleSidebar,
                        },
                    },

                    #[wrap(Some)]
                    set_content = &gtk::Overlay {
                        #[wrap(Some)]
                        set_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_margin_start: 24,
                            set_margin_end: 24,
                            set_margin_top: 16,
                            set_margin_bottom: 16,
                            set_spacing: 12,

                            gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 6,
                                set_margin_bottom: 4,

                                gtk::Image {
                                    set_pixel_size: 16,
                                    add_css_class: "view-icon",
                                    #[watch]
                                    set_icon_name: Some(model.selection.icon()),
                                },
                                gtk::Label {
                                    set_css_classes: &["title-2", "view-title"],
                                    set_halign: gtk::Align::Start,
                                    #[watch]
                                    set_label: model.selection.title(),
                                },
                            },

                            gtk::ScrolledWindow {
                                set_vexpand: true,
                                set_hscrollbar_policy: gtk::PolicyType::Never,

                                #[wrap(Some)]
                                set_child = &gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,

                                    #[local_ref]
                                    task_list_box -> gtk::ListBox {
                                        add_css_class: "task-list",
                                        set_selection_mode: gtk::SelectionMode::None,
                                    },

                                    gtk::Button {
                                        set_css_classes: &["flat", "task-add-link"],
                                        set_halign: gtk::Align::Start,
                                        #[watch]
                                        set_visible: matches!(model.selection, SidebarSelection::Inbox),
                                        connect_clicked => AppMsg::OpenQuickAdd,

                                        #[wrap(Some)]
                                        set_child = &gtk::Box {
                                            set_orientation: gtk::Orientation::Horizontal,
                                            set_spacing: 8,
                                            gtk::Image {
                                                set_icon_name: Some("list-add-symbolic"),
                                                set_pixel_size: 14,
                                            },
                                            gtk::Label {
                                                set_label: "Add tasks",
                                            },
                                        },
                                    },

                                    #[local_ref]
                                    completed_task_list_box -> gtk::ListBox {
                                        add_css_class: "task-list",
                                        add_css_class: "task-list-completed",
                                        set_selection_mode: gtk::SelectionMode::None,
                                        set_margin_top: 8,
                                        #[watch]
                                        set_visible: matches!(model.selection, SidebarSelection::Inbox)
                                            && model.completed_tasks.len() > 0,
                                    },
                                },
                            },
                        },

                        add_overlay = &gtk::Button {
                            set_icon_name: "list-add-symbolic",
                            set_css_classes: &["suggested-action", "circular", "fab"],
                            set_halign: gtk::Align::End,
                            set_valign: gtk::Align::End,
                            set_margin_end: 24,
                            set_margin_bottom: 24,
                            set_tooltip_text: Some("Add task"),
                            connect_clicked => AppMsg::OpenQuickAdd,
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
        load_css();

        let style_manager = adw::StyleManager::default();
        apply_dark_class(&root, style_manager.is_dark());
        {
            let root_weak = root.downgrade();
            style_manager.connect_dark_notify(move |mgr| {
                if let Some(window) = root_weak.upgrade() {
                    apply_dark_class(&window, mgr.is_dark());
                }
            });
        }

        let quick_add = QuickAdd::builder().launch(root.clone()).forward(
            sender.input_sender(),
            |output| match output {
                QuickAddOutput::AddTask { title, description } => {
                    AppMsg::AddTask { title, description }
                }
            },
        );

        let sidebar = Sidebar::builder().launch(()).forward(
            sender.input_sender(),
            |output| match output {
                SidebarOutput::Selected(sel) => AppMsg::SelectView(sel),
            },
        );

        let mut tasks = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |output| match output {
                TaskRowOutput::Toggled(id) => AppMsg::ToggleTask(id),
                TaskRowOutput::Updated(task) => AppMsg::UpdateTask(task),
                TaskRowOutput::Duplicated(id) => AppMsg::DuplicateTask(id),
                TaskRowOutput::Deleted(id) => AppMsg::DeleteTask(id),
            });

        let mut completed_tasks = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |output| match output {
                TaskRowOutput::Toggled(id) => AppMsg::ToggleTask(id),
                TaskRowOutput::Updated(task) => AppMsg::UpdateTask(task),
                TaskRowOutput::Duplicated(id) => AppMsg::DuplicateTask(id),
                TaskRowOutput::Deleted(id) => AppMsg::DeleteTask(id),
            });

        let initial_filter = SidebarSelection::Inbox
            .task_filter()
            .expect("Inbox selection must have a filter");
        match state.list_tasks_for_filter(initial_filter) {
            Ok(initial_tasks) => {
                let mut open_guard = tasks.guard();
                let mut done_guard = completed_tasks.guard();
                for task in initial_tasks {
                    if task.is_done {
                        done_guard.push_back(task);
                    } else {
                        open_guard.push_back(task);
                    }
                }
            }
            Err(err) => eprintln!("kromodo: failed to load tasks: {err}"),
        }

        let events = state.subscribe();
        {
            let sender = sender.clone();
            std::thread::spawn(move || {
                while let Ok(event) = events.recv() {
                    sender.input(AppMsg::CoreEvent(event));
                }
            });
        }

        let model = App {
            state,
            tasks,
            completed_tasks,
            quick_add,
            sidebar,
            selection: SidebarSelection::Inbox,
            show_sidebar: true,
            pending_finalize: HashMap::new(),
            next_finalize_token: 0,
        };

        let task_list_box = model.tasks.widget();
        let completed_task_list_box = model.completed_tasks.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: AppMsg, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::OpenQuickAdd => {
                self.quick_add.sender().send(QuickAddInput::Present).ok();
            }
            AppMsg::AddTask { title, description } => {
                if let Err(err) = self
                    .state
                    .add_task(&title, &description, Priority::None, None, false)
                {
                    eprintln!("kromodo: add_task failed: {err}");
                }
            }
            AppMsg::UpdateTask(mut task) => {
                if let Err(err) = self.state.update_task(&mut task) {
                    eprintln!("kromodo: update_task failed: {err}");
                }
            }
            AppMsg::DuplicateTask(id) => {
                if let Err(err) = self.state.duplicate_task(id) {
                    eprintln!("kromodo: duplicate_task failed: {err}");
                }
            }
            AppMsg::DeleteTask(id) => {
                if let Err(err) = self.state.delete_task(id) {
                    eprintln!("kromodo: delete_task failed: {err}");
                }
            }
            AppMsg::ToggleTask(id) => {
                if let Err(err) = self.state.toggle_task(id) {
                    eprintln!("kromodo: toggle_task failed: {err}");
                }
            }
            AppMsg::Refresh => {
                let result = match self.selection.task_filter() {
                    Some(filter) => self.state.list_tasks_for_filter(filter),
                    None => Ok(Vec::new()),
                };
                match result {
                    Ok(updated_tasks) => {
                        let split_done = matches!(self.selection, SidebarSelection::Inbox);
                        let mut open_guard = self.tasks.guard();
                        let mut done_guard = self.completed_tasks.guard();
                        open_guard.clear();
                        done_guard.clear();
                        for task in updated_tasks {
                            if split_done && task.is_done {
                                done_guard.push_back(task);
                            } else {
                                open_guard.push_back(task);
                            }
                        }
                    }
                    Err(err) => eprintln!("kromodo: list_tasks failed: {err}"),
                }
            }
            AppMsg::SelectView(selection) => {
                self.selection = selection;
                sender.input(AppMsg::Refresh);
            }
            AppMsg::ToggleSidebar => {
                self.show_sidebar = !self.show_sidebar;
            }
            AppMsg::CoreEvent(event) => match event {
                CoreEvent::TaskCreated(task) => {
                    let in_view = self
                        .selection
                        .task_filter()
                        .map_or(false, |f| f.matches(&task));
                    if in_view {
                        self.tasks.guard().push_front(task);
                    }
                }
                CoreEvent::TaskUpdated(task) => {
                    let had_pending = self.pending_finalize.remove(&task.id).is_some();

                    let still_matches = self
                        .selection
                        .task_filter()
                        .map_or(false, |f| f.matches(&task));
                    let in_inbox = matches!(self.selection, SidebarSelection::Inbox);

                    let open_guard = self.tasks.guard();
                    let open_index = (0..open_guard.len()).find(|&i| {
                        open_guard.get(i).map(|r| r.task_id() == task.id).unwrap_or(false)
                    });
                    drop(open_guard);

                    let done_guard = self.completed_tasks.guard();
                    let done_index = (0..done_guard.len()).find(|&i| {
                        done_guard.get(i).map(|r| r.task_id() == task.id).unwrap_or(false)
                    });
                    drop(done_guard);

                    if had_pending {
                        if let Some(i) = open_index {
                            self.tasks.send(i, TaskRowInput::SetRevealed(true));
                        }
                        if let Some(i) = done_index {
                            self.completed_tasks.send(i, TaskRowInput::SetRevealed(true));
                        }
                    }

                    if !still_matches {
                        if let Some(i) = open_index {
                            self.tasks.send(i, TaskRowInput::SetRevealed(false));
                        }
                        if let Some(i) = done_index {
                            self.completed_tasks.send(i, TaskRowInput::SetRevealed(false));
                        }
                        if open_index.is_some() || done_index.is_some() {
                            let id = task.id;
                            self.schedule_finalize(id, &sender, move |token| {
                                AppMsg::FinalizeRemove { id, token }
                            });
                        }
                    } else if in_inbox {
                        if let Some(i) = open_index {
                            if task.is_done {
                                self.tasks.send(i, TaskRowInput::SetRevealed(false));
                                let id = task.id;
                                let t = task.clone();
                                self.schedule_finalize(id, &sender, move |token| {
                                    AppMsg::FinalizeMove {
                                        task: t,
                                        to_done: true,
                                        token,
                                    }
                                });
                            } else {
                                self.tasks.send(i, TaskRowInput::ReplaceTask(task));
                            }
                        } else if let Some(i) = done_index {
                            if !task.is_done {
                                self.completed_tasks
                                    .send(i, TaskRowInput::SetRevealed(false));
                                let id = task.id;
                                let t = task.clone();
                                self.schedule_finalize(id, &sender, move |token| {
                                    AppMsg::FinalizeMove {
                                        task: t,
                                        to_done: false,
                                        token,
                                    }
                                });
                            } else {
                                self.completed_tasks.send(i, TaskRowInput::ReplaceTask(task));
                            }
                        } else if task.is_done {
                            self.completed_tasks.guard().push_front(task);
                        } else {
                            self.tasks.guard().push_front(task);
                        }
                    } else if let Some(i) = open_index {
                        self.tasks.send(i, TaskRowInput::ReplaceTask(task));
                    } else {
                        self.tasks.guard().push_front(task);
                    }
                }
                CoreEvent::TaskDeleted(id) => {
                    self.pending_finalize.remove(&id);
                    let mut guard = self.tasks.guard();
                    let index = (0..guard.len())
                        .find(|&i| guard.get(i).map(|r| r.task_id() == id).unwrap_or(false));
                    if let Some(i) = index {
                        guard.remove(i);
                    }
                    drop(guard);

                    let mut done_guard = self.completed_tasks.guard();
                    let done_index = (0..done_guard.len())
                        .find(|&i| done_guard.get(i).map(|r| r.task_id() == id).unwrap_or(false));
                    if let Some(i) = done_index {
                        done_guard.remove(i);
                    }
                }
            },
            AppMsg::FinalizeRemove { id, token } => {
                if !self.claim_finalize(id, token) {
                    return;
                }
                let open_guard = self.tasks.guard();
                let idx = (0..open_guard.len())
                    .find(|&i| open_guard.get(i).map(|r| r.task_id() == id).unwrap_or(false));
                drop(open_guard);
                if let Some(i) = idx {
                    self.tasks.guard().remove(i);
                }

                let done_guard = self.completed_tasks.guard();
                let idx = (0..done_guard.len())
                    .find(|&i| done_guard.get(i).map(|r| r.task_id() == id).unwrap_or(false));
                drop(done_guard);
                if let Some(i) = idx {
                    self.completed_tasks.guard().remove(i);
                }
            }
            AppMsg::FinalizeMove { task, to_done, token } => {
                if !self.claim_finalize(task.id, token) {
                    return;
                }
                if to_done {
                    let open_guard = self.tasks.guard();
                    let idx = (0..open_guard.len()).find(|&i| {
                        open_guard.get(i).map(|r| r.task_id() == task.id).unwrap_or(false)
                    });
                    drop(open_guard);
                    if let Some(i) = idx {
                        self.tasks.guard().remove(i);
                        self.completed_tasks.guard().push_front(task);
                    }
                } else {
                    let done_guard = self.completed_tasks.guard();
                    let idx = (0..done_guard.len()).find(|&i| {
                        done_guard.get(i).map(|r| r.task_id() == task.id).unwrap_or(false)
                    });
                    drop(done_guard);
                    if let Some(i) = idx {
                        self.completed_tasks.guard().remove(i);
                        self.tasks.guard().push_front(task);
                    }
                }
            }
        }
    }
}

impl App {
    fn schedule_finalize(
        &mut self,
        id: i64,
        sender: &ComponentSender<Self>,
        make_msg: impl FnOnce(u64) -> AppMsg + 'static,
    ) {
        let token = self.next_finalize_token;
        self.next_finalize_token = self.next_finalize_token.wrapping_add(1);
        self.pending_finalize.insert(id, token);
        let msg = make_msg(token);
        let s = sender.clone();
        glib::timeout_add_local_once(Duration::from_millis(240), move || {
            s.input(msg);
        });
    }

    fn claim_finalize(&mut self, id: i64, token: u64) -> bool {
        match self.pending_finalize.get(&id) {
            Some(&t) if t == token => {
                self.pending_finalize.remove(&id);
                true
            }
            _ => false,
        }
    }
}

fn apply_dark_class(window: &adw::ApplicationWindow, is_dark: bool) {
    if is_dark {
        window.add_css_class("kromodo-dark");
    } else {
        window.remove_css_class("kromodo-dark");
    }
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(include_str!("../data/styles.css"));
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not get default display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
