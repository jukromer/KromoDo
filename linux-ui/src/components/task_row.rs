use kromodo_core::Task;
use relm4::gtk;
use relm4::gtk::glib;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

pub struct TaskRow {
    task: Task,
    expanded: bool,
    title_buffer: gtk::EntryBuffer,
    description_buffer: gtk::TextBuffer,
}

#[derive(Debug)]
pub enum TaskRowInput {
    Toggle,
    ToggleExpand,
    SaveAndCollapse,
    SetPriority(i8),
    Delete,
}

#[derive(Debug)]
pub enum TaskRowOutput {
    Toggled(i64),
    Updated(Task),
    Deleted(i64),
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

fn compact_row_classes(task: &Task) -> Vec<&'static str> {
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

    fn card_classes(&self) -> &'static [&'static str] {
        if self.expanded {
            &["task-edit-card"]
        } else {
            &[]
        }
    }

    fn priority_dot_classes(&self, level: i8) -> Vec<&'static str> {
        let mut classes = vec!["priority-dot"];
        classes.push(match level {
            1 => "priority-dot-low",
            2 => "priority-dot-medium",
            3 => "priority-dot-high",
            4 => "priority-dot-urgent",
            _ => "",
        });
        if self.task.priority == level {
            classes.push("priority-dot-active");
        }
        classes
    }

    fn sync_buffers_from_task(&self) {
        self.title_buffer.set_text(&self.task.title);
        self.description_buffer.set_text(&self.task.description);
    }

    fn read_buffers_into_task(&mut self) {
        self.task.title = self.title_buffer.text().trim().to_string();
        let (start, end) = self.description_buffer.bounds();
        self.task.description = self
            .description_buffer
            .text(&start, &end, false)
            .trim()
            .to_string();
    }

    fn has_changes(&self) -> bool {
        let buf_title = self.title_buffer.text().trim().to_string();
        let (start, end) = self.description_buffer.bounds();
        let buf_desc = self
            .description_buffer
            .text(&start, &end, false)
            .trim()
            .to_string();
        buf_title != self.task.title || buf_desc != self.task.description
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
            set_orientation: gtk::Orientation::Vertical,
            #[watch]
            set_css_classes: self.card_classes(),

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                #[watch]
                set_css_classes: &compact_row_classes(&self.task),

                gtk::CheckButton {
                    set_valign: gtk::Align::Center,
                    #[watch]
                    set_active: self.task.is_done,
                    connect_toggled => TaskRowInput::Toggle,
                },

                gtk::Button {
                    #[watch]
                    set_visible: !self.expanded,
                    set_css_classes: &["flat", "task-title-btn"],
                    set_hexpand: true,
                    connect_clicked => TaskRowInput::ToggleExpand,

                    #[wrap(Some)]
                    set_child = &gtk::Label {
                        set_use_markup: true,
                        #[watch]
                        set_label: &self.formatted_title(),
                        set_halign: gtk::Align::Start,
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                        add_css_class: "task-title",
                    },
                },

                gtk::Entry {
                    #[watch]
                    set_visible: self.expanded,
                    set_buffer: &self.title_buffer,
                    set_hexpand: true,
                    set_has_frame: false,
                    add_css_class: "task-edit-title",
                    connect_activate => TaskRowInput::SaveAndCollapse,
                },

                gtk::Button {
                    #[watch]
                    set_visible: self.expanded,
                    set_icon_name: "window-close-symbolic",
                    set_css_classes: &["flat", "circular", "task-close-btn"],
                    set_valign: gtk::Align::Center,
                    connect_clicked => TaskRowInput::SaveAndCollapse,
                },
            },

            gtk::Revealer {
                #[watch]
                set_reveal_child: self.expanded,
                set_transition_type: gtk::RevealerTransitionType::SlideDown,

                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 6,
                    set_margin_start: 38,
                    set_margin_end: 4,
                    set_margin_bottom: 8,

                    gtk::TextView {
                        set_buffer: Some(&self.description_buffer),
                        set_wrap_mode: gtk::WrapMode::WordChar,
                        set_accepts_tab: false,
                        set_top_margin: 6,
                        set_bottom_margin: 6,
                        set_left_margin: 8,
                        set_right_margin: 8,
                        add_css_class: "task-edit-description",
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 6,

                        gtk::Button {
                            set_css_classes: &["flat", "task-edit-chip"],
                            set_sensitive: false,

                            #[wrap(Some)]
                            set_child = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 4,
                                gtk::Image { set_icon_name: Some("x-office-calendar-symbolic") },
                                gtk::Label {
                                    set_label: "Date",
                                    add_css_class: "caption",
                                },
                            },
                        },

                        gtk::Box { set_hexpand: true },

                        gtk::Button {
                            set_label: "!",
                            #[watch]
                            set_css_classes: &self.priority_dot_classes(1),
                            set_tooltip_text: Some("Low"),
                            connect_clicked => TaskRowInput::SetPriority(1),
                        },
                        gtk::Button {
                            set_label: "!!",
                            #[watch]
                            set_css_classes: &self.priority_dot_classes(2),
                            set_tooltip_text: Some("Medium"),
                            connect_clicked => TaskRowInput::SetPriority(2),
                        },
                        gtk::Button {
                            set_label: "!!!",
                            #[watch]
                            set_css_classes: &self.priority_dot_classes(3),
                            set_tooltip_text: Some("High"),
                            connect_clicked => TaskRowInput::SetPriority(3),
                        },
                        gtk::Button {
                            set_label: "!!!!",
                            #[watch]
                            set_css_classes: &self.priority_dot_classes(4),
                            set_tooltip_text: Some("Urgent"),
                            connect_clicked => TaskRowInput::SetPriority(4),
                        },
                    },

                    gtk::Separator {
                        set_orientation: gtk::Orientation::Horizontal,
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 6,

                        gtk::Image {
                            set_icon_name: Some("mail-inbox-symbolic"),
                            set_pixel_size: 14,
                            add_css_class: "dim-label",
                        },
                        gtk::Label {
                            set_label: "Inbox",
                            add_css_class: "caption",
                            add_css_class: "dim-label",
                        },

                        gtk::Box { set_hexpand: true },

                        gtk::Button {
                            set_icon_name: "user-trash-symbolic",
                            set_css_classes: &["flat", "circular", "task-delete-btn"],
                            set_tooltip_text: Some("Delete"),
                            connect_clicked => TaskRowInput::Delete,
                        },
                    },
                },
            },
        }
    }

    fn init_model(
        task: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        let title_buffer = gtk::EntryBuffer::new(Some(&task.title));
        let description_buffer = gtk::TextBuffer::new(None);
        description_buffer.set_text(&task.description);

        Self {
            task,
            expanded: false,
            title_buffer,
            description_buffer,
        }
    }

    fn update(&mut self, msg: TaskRowInput, sender: FactorySender<Self>) {
        match msg {
            TaskRowInput::Toggle => {
                self.task.is_done = !self.task.is_done;
                sender.output(TaskRowOutput::Toggled(self.task.id)).ok();
            }
            TaskRowInput::ToggleExpand => {
                self.sync_buffers_from_task();
                self.expanded = true;
            }
            TaskRowInput::SaveAndCollapse => {
                if self.has_changes() {
                    self.read_buffers_into_task();
                    if !self.task.title.is_empty() {
                        sender
                            .output(TaskRowOutput::Updated(self.task.clone()))
                            .ok();
                    }
                }
                self.expanded = false;
            }
            TaskRowInput::SetPriority(level) => {
                if self.task.priority == level {
                    self.task.priority = 0;
                } else {
                    self.task.priority = level;
                }
            }
            TaskRowInput::Delete => {
                sender.output(TaskRowOutput::Deleted(self.task.id)).ok();
            }
        }
    }
}
