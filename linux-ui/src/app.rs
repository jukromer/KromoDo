use adw::prelude::*;
use kromodo_core::AppState;
use relm4::gtk::gdk;
use relm4::prelude::*;
use relm4::{adw, gtk};
use std::sync::Arc;

use crate::components::quick_add::{QuickAdd, QuickAddInput, QuickAddOutput};
use crate::components::sidebar::{Sidebar, SidebarOutput, SidebarSelection};
use crate::components::task_row::{TaskRow, TaskRowOutput};

pub struct App {
    state: Arc<AppState>,
    tasks: FactoryVecDeque<TaskRow>,
    quick_add: Controller<QuickAdd>,
    sidebar: Controller<Sidebar>,
    selection: SidebarSelection,
    show_sidebar: bool,
}

#[derive(Debug)]
pub enum AppMsg {
    OpenQuickAdd,
    AddTask { title: String, description: String },
    ToggleTask(i64),
    Refresh,
    SelectView(SidebarSelection),
    ToggleSidebar,
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

                            gtk::Button {
                                set_css_classes: &["flat", "task-add-link"],
                                set_halign: gtk::Align::Start,
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
            });

        match state.list_tasks() {
            Ok(initial_tasks) => {
                let mut guard = tasks.guard();
                for task in initial_tasks {
                    guard.push_back(task);
                }
            }
            Err(err) => eprintln!("kromodo: failed to load tasks: {err}"),
        }

        let model = App {
            state,
            tasks,
            quick_add,
            sidebar,
            selection: SidebarSelection::Inbox,
            show_sidebar: true,
        };

        let task_list_box = model.tasks.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: AppMsg, sender: ComponentSender<Self>) {
        match msg {
            AppMsg::OpenQuickAdd => {
                self.quick_add.sender().send(QuickAddInput::Present).ok();
            }
            AppMsg::AddTask { title, description } => {
                if let Err(err) = self.state.add_task(&title, &description, 0, None, false) {
                    eprintln!("kromodo: add_task failed: {err}");
                }
                sender.input(AppMsg::Refresh);
            }
            AppMsg::ToggleTask(id) => {
                if let Err(err) = self.state.toggle_task(id) {
                    eprintln!("kromodo: toggle_task failed: {err}");
                }
                sender.input(AppMsg::Refresh);
            }
            AppMsg::Refresh => match self.state.list_tasks() {
                Ok(updated_tasks) => {
                    let mut guard = self.tasks.guard();
                    guard.clear();
                    for task in updated_tasks {
                        guard.push_back(task);
                    }
                }
                Err(err) => eprintln!("kromodo: list_tasks failed: {err}"),
            },
            AppMsg::SelectView(selection) => {
                self.selection = selection;
            }
            AppMsg::ToggleSidebar => {
                self.show_sidebar = !self.show_sidebar;
            }
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
