use chrono::{Duration, Local, NaiveDate, TimeZone, Utc};
use kromodo_core::{Priority, Task};
use relm4::gtk;
use relm4::gtk::glib;
use relm4::gtk::prelude::*;
use relm4::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

mod context_menu;
mod date_picker;
mod styling;

use context_menu::show_context_popover;
use date_picker::attach_date_picker;
use styling::{compact_row_classes, format_due_display};

pub struct TaskRow {
    task: Task,
    expanded: bool,
    revealed: bool,
    show_description_field: bool,
    // Mirrors task.is_done so connect_toggled can distinguish user clicks from programmatic set_active calls
    is_done_mirror: Rc<Cell<bool>>,
    title_buffer: gtk::EntryBuffer,
    description_buffer: gtk::TextBuffer,
}

#[derive(Debug)]
pub enum TaskRowInput {
    Toggle,
    ToggleExpand,
    SaveAndCollapse,
    SetPriority(Priority),
    SetDueToday,
    SetDueTomorrow,
    SetDueDate { year: i32, month: u32, day: u32 },
    ClearDueDate,
    Duplicate,
    Delete,
    ReplaceTask(Task),
    SetRevealed(bool),
    ShowDescriptionField,
}

#[derive(Debug)]
pub enum TaskRowOutput {
    Toggled(i64),
    Updated(Task),
    Duplicated(i64),
    Deleted(i64),
}

impl TaskRow {
    pub fn task_id(&self) -> i64 {
        self.task.id
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
        gtk::Revealer {
            #[watch]
            set_reveal_child: self.revealed,
            set_transition_type: gtk::RevealerTransitionType::SlideDown,
            set_transition_duration: 220,

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                #[watch]
                set_css_classes: self.card_classes(),

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,
                    #[watch]
                    set_css_classes: &compact_row_classes(&self.task),

                    #[name = "check_btn"]
                    gtk::CheckButton {
                        set_valign: gtk::Align::Center,
                        #[watch]
                        set_active: self.task.is_done,
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

                gtk::Label {
                    #[watch]
                    set_visible: !self.expanded && self.task.due_date.is_some(),
                    #[watch]
                    set_label: &format_due_display(self.task.due_date).unwrap_or_default(),
                    #[watch]
                    set_css_classes: self.due_label_classes(),
                    set_valign: gtk::Align::Center,
                    set_margin_end: 4,
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
                        #[watch]
                        set_visible: self.show_description_field,
                        set_buffer: Some(&self.description_buffer),
                        set_wrap_mode: gtk::WrapMode::WordChar,
                        set_accepts_tab: false,
                        set_top_margin: 6,
                        set_bottom_margin: 6,
                        set_left_margin: 8,
                        set_right_margin: 8,
                        add_css_class: "task-edit-description",
                    },

                    gtk::Button {
                        #[watch]
                        set_visible: !self.show_description_field,
                        set_css_classes: &["flat", "task-add-description-btn"],
                        set_halign: gtk::Align::Start,
                        connect_clicked => TaskRowInput::ShowDescriptionField,

                        #[wrap(Some)]
                        set_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,
                            gtk::Image {
                                set_icon_name: Some("list-add-symbolic"),
                                set_pixel_size: 12,
                            },
                            gtk::Label {
                                set_label: "Add description",
                                add_css_class: "caption",
                                add_css_class: "dim-label",
                            },
                        },
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 6,

                        #[name = "date_chip"]
                        gtk::MenuButton {
                            set_css_classes: &["flat", "task-edit-chip"],

                            #[wrap(Some)]
                            set_child = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 4,
                                gtk::Image { set_icon_name: Some("x-office-calendar-symbolic") },
                                gtk::Label {
                                    #[watch]
                                    set_label: &format_due_display(self.task.due_date)
                                        .unwrap_or_else(|| "Date".to_string()),
                                    add_css_class: "caption",
                                },
                            },
                        },

                        gtk::Box { set_hexpand: true },

                        gtk::Button {
                            set_label: "!",
                            #[watch]
                            set_css_classes: &self.priority_dot_classes(Priority::Low),
                            set_tooltip_text: Some("Low"),
                            connect_clicked => TaskRowInput::SetPriority(Priority::Low),
                        },
                        gtk::Button {
                            set_label: "!!",
                            #[watch]
                            set_css_classes: &self.priority_dot_classes(Priority::Medium),
                            set_tooltip_text: Some("Medium"),
                            connect_clicked => TaskRowInput::SetPriority(Priority::Medium),
                        },
                        gtk::Button {
                            set_label: "!!!",
                            #[watch]
                            set_css_classes: &self.priority_dot_classes(Priority::High),
                            set_tooltip_text: Some("High"),
                            connect_clicked => TaskRowInput::SetPriority(Priority::High),
                        },
                        gtk::Button {
                            set_label: "!!!!",
                            #[watch]
                            set_css_classes: &self.priority_dot_classes(Priority::Urgent),
                            set_tooltip_text: Some("Urgent"),
                            connect_clicked => TaskRowInput::SetPriority(Priority::Urgent),
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
        let is_done_mirror = Rc::new(Cell::new(task.is_done));
        let show_description_field = !task.description.is_empty();

        Self {
            task,
            expanded: false,
            revealed: false,
            show_description_field,
            is_done_mirror,
            title_buffer,
            description_buffer,
        }
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();

        // connect_toggled with guard: only fires when user actually clicks, not on programmatic set_active
        let mirror = self.is_done_mirror.clone();
        let s = sender.clone();
        widgets.check_btn.connect_toggled(move |cb| {
            if cb.is_active() != mirror.get() {
                s.input(TaskRowInput::Toggle);
            }
        });

        let s = sender.clone();
        let root_widget = root.clone().upcast::<gtk::Widget>();
        let gesture = gtk::GestureClick::new();
        gesture.set_button(3);
        gesture.connect_released(move |_gesture, _n, x, y| {
            show_context_popover(&root_widget, x as i32, y as i32, &s);
        });
        root.add_controller(gesture);

        attach_date_picker(&widgets.date_chip, self.task.due_date, &sender);

        // Trigger reveal animation once the widget is mounted
        let s = sender.clone();
        glib::idle_add_local_once(move || {
            s.input(TaskRowInput::SetRevealed(true));
        });

        widgets
    }

    fn update(&mut self, msg: TaskRowInput, sender: FactorySender<Self>) {
        match msg {
            TaskRowInput::Toggle => {
                self.task.is_done = !self.task.is_done;
                self.is_done_mirror.set(self.task.is_done);
                sender.output(TaskRowOutput::Toggled(self.task.id)).ok();
            }
            TaskRowInput::ToggleExpand => {
                if self.expanded {
                    sender.input(TaskRowInput::SaveAndCollapse);
                } else {
                    self.sync_buffers_from_task();
                    self.show_description_field = !self.task.description.is_empty();
                    self.expanded = true;
                }
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
                    self.task.priority = Priority::None;
                } else {
                    self.task.priority = level;
                }
                sender
                    .output(TaskRowOutput::Updated(self.task.clone()))
                    .ok();
            }
            TaskRowInput::SetDueToday => {
                self.task.due_date = Some(Utc::now());
                self.task.has_due_time = false;
                sender
                    .output(TaskRowOutput::Updated(self.task.clone()))
                    .ok();
            }
            TaskRowInput::SetDueTomorrow => {
                self.task.due_date = Some(Utc::now() + Duration::days(1));
                self.task.has_due_time = false;
                sender
                    .output(TaskRowOutput::Updated(self.task.clone()))
                    .ok();
            }
            TaskRowInput::SetDueDate { year, month, day } => {
                let Some(naive) = NaiveDate::from_ymd_opt(year, month, day) else {
                    return;
                };
                let Some(local) = Local
                    .from_local_datetime(&naive.and_hms_opt(0, 0, 0).unwrap())
                    .single()
                else {
                    return;
                };
                self.task.due_date = Some(local.with_timezone(&Utc));
                self.task.has_due_time = false;
                sender
                    .output(TaskRowOutput::Updated(self.task.clone()))
                    .ok();
            }
            TaskRowInput::ClearDueDate => {
                self.task.due_date = None;
                self.task.has_due_time = false;
                sender
                    .output(TaskRowOutput::Updated(self.task.clone()))
                    .ok();
            }
            TaskRowInput::Duplicate => {
                sender.output(TaskRowOutput::Duplicated(self.task.id)).ok();
            }
            TaskRowInput::Delete => {
                sender.output(TaskRowOutput::Deleted(self.task.id)).ok();
            }
            TaskRowInput::ReplaceTask(task) => {
                self.task = task;
                self.is_done_mirror.set(self.task.is_done);
                self.show_description_field = !self.task.description.is_empty();
            }
            TaskRowInput::SetRevealed(value) => {
                self.revealed = value;
            }
            TaskRowInput::ShowDescriptionField => {
                self.show_description_field = true;
            }
        }
    }
}

