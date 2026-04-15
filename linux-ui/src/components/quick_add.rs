use adw::prelude::*;
use relm4::gtk::prelude::*;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct QuickAdd {
    title_buffer: gtk::EntryBuffer,
    description_buffer: gtk::TextBuffer,
    dialog: adw::Dialog,
    parent: adw::ApplicationWindow,
}

#[derive(Debug)]
pub enum QuickAddInput {
    Present,
    Submit,
    Close,
}

#[derive(Debug)]
pub enum QuickAddOutput {
    AddTask {
        title: String,
        description: String,
    },
}

#[relm4::component(pub)]
impl SimpleComponent for QuickAdd {
    type Init = adw::ApplicationWindow;
    type Input = QuickAddInput;
    type Output = QuickAddOutput;

    view! {
        #[root]
        adw::Dialog {
            set_content_width: 520,
            set_title: "",

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                add_css_class: "quick-add",

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_start: 12,
                    set_margin_end: 12,
                    set_margin_top: 12,
                    set_margin_bottom: 4,

                    gtk::Button {
                        set_icon_name: "help-about-symbolic",
                        set_css_classes: &["flat", "circular"],
                        set_sensitive: false,
                    },

                    gtk::Box { set_hexpand: true },

                    gtk::Button {
                        set_icon_name: "window-close-symbolic",
                        set_css_classes: &["flat", "circular"],
                        connect_clicked => QuickAddInput::Close,
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_start: 16,
                    set_margin_end: 16,
                    set_margin_top: 4,
                    set_margin_bottom: 8,
                    set_spacing: 4,
                    add_css_class: "quick-add-card",

                    gtk::Entry {
                        set_buffer: &model.title_buffer,
                        set_placeholder_text: Some("To-do name"),
                        set_has_frame: false,
                        add_css_class: "quick-add-title",
                        connect_activate => QuickAddInput::Submit,
                    },

                    gtk::ScrolledWindow {
                        set_height_request: 72,
                        set_hscrollbar_policy: gtk::PolicyType::Never,
                        set_vscrollbar_policy: gtk::PolicyType::Automatic,

                        #[wrap(Some)]
                        set_child = &gtk::TextView {
                            set_buffer: Some(&model.description_buffer),
                            set_wrap_mode: gtk::WrapMode::Word,
                            set_accepts_tab: false,
                            set_top_margin: 4,
                            set_bottom_margin: 4,
                            set_left_margin: 2,
                            set_right_margin: 2,
                            add_css_class: "quick-add-description",
                        },
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 4,
                        set_margin_top: 8,

                        gtk::Button {
                            set_css_classes: &["flat", "quick-add-chip"],
                            set_sensitive: false,

                            #[wrap(Some)]
                            set_child = &gtk::Box {
                                set_orientation: gtk::Orientation::Horizontal,
                                set_spacing: 6,
                                gtk::Image { set_icon_name: Some("x-office-calendar-symbolic") },
                                gtk::Label { set_label: "Date" },
                            },
                        },

                        gtk::Box { set_hexpand: true },

                        gtk::Button {
                            set_icon_name: "tag-symbolic",
                            set_css_classes: &["flat", "circular"],
                            set_sensitive: false,
                        },
                        gtk::Button {
                            set_icon_name: "flag-outline-thick-symbolic",
                            set_css_classes: &["flat", "circular"],
                            set_sensitive: false,
                        },
                        gtk::Button {
                            set_icon_name: "alarm-symbolic",
                            set_css_classes: &["flat", "circular"],
                            set_sensitive: false,
                        },
                        gtk::Button {
                            set_icon_name: "view-pin-symbolic",
                            set_css_classes: &["flat", "circular"],
                            set_sensitive: false,
                        },
                    },
                },

                gtk::Separator {
                    set_orientation: gtk::Orientation::Horizontal,
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    set_margin_start: 16,
                    set_margin_end: 12,
                    set_margin_top: 10,
                    set_margin_bottom: 12,

                    gtk::Image {
                        set_icon_name: Some("mail-inbox-symbolic"),
                        set_pixel_size: 18,
                    },
                    gtk::Label {
                        set_label: "Inbox",
                        add_css_class: "heading",
                    },

                    gtk::Box { set_hexpand: true },

                    gtk::Button {
                        set_icon_name: "document-send-symbolic",
                        set_css_classes: &["suggested-action", "circular", "quick-add-send"],
                        connect_clicked => QuickAddInput::Submit,
                    },
                },
            },
        }
    }

    fn init(
        parent: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = QuickAdd {
            title_buffer: gtk::EntryBuffer::new(None::<&str>),
            description_buffer: gtk::TextBuffer::new(None),
            dialog: root.clone(),
            parent,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: QuickAddInput, sender: ComponentSender<Self>) {
        match msg {
            QuickAddInput::Present => {
                self.dialog.present(Some(&self.parent));
            }
            QuickAddInput::Close => {
                self.clear();
                self.dialog.close();
            }
            QuickAddInput::Submit => {
                let title = self.title_buffer.text().trim().to_string();
                if title.is_empty() {
                    return;
                }
                let description = self.description_text();
                let _ = sender.output(QuickAddOutput::AddTask { title, description });
                self.clear();
                self.dialog.close();
            }
        }
    }
}

impl QuickAdd {
    fn description_text(&self) -> String {
        let (start, end) = self.description_buffer.bounds();
        self.description_buffer.text(&start, &end, false).to_string()
    }

    fn clear(&self) {
        self.title_buffer.set_text("");
        self.description_buffer.set_text("");
    }
}
