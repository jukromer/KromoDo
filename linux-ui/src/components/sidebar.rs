use adw::prelude::*;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarSelection {
    Inbox,
}

impl SidebarSelection {
    pub fn icon(self) -> &'static str {
        match self {
            Self::Inbox => "mail-inbox-symbolic",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Inbox => "Inbox",
        }
    }

    fn all() -> &'static [SidebarSelection] {
        &[Self::Inbox]
    }
}

pub struct Sidebar {
    selection: SidebarSelection,
    buttons: Vec<(SidebarSelection, gtk::Button)>,
}

#[derive(Debug)]
pub enum SidebarInput {
    Select(SidebarSelection),
}

#[derive(Debug)]
pub enum SidebarOutput {
    Selected(SidebarSelection),
}

#[relm4::component(pub)]
impl SimpleComponent for Sidebar {
    type Init = ();
    type Input = SidebarInput;
    type Output = SidebarOutput;

    view! {
        #[root]
        adw::ToolbarView {
            add_top_bar = &adw::HeaderBar {
                add_css_class: "flat",
                set_show_end_title_buttons: false,
                set_show_start_title_buttons: false,
                #[wrap(Some)]
                set_title_widget = &gtk::Label {
                    set_label: "KromoDo",
                    add_css_class: "heading",
                },
            },

            #[wrap(Some)]
            set_content = &gtk::ScrolledWindow {
                set_vexpand: true,
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_child: Some(&entries_box),
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let entries_box = gtk::Box::new(gtk::Orientation::Vertical, 2);
        entries_box.set_margin_start(8);
        entries_box.set_margin_end(8);
        entries_box.set_margin_top(8);
        entries_box.set_margin_bottom(8);
        entries_box.add_css_class("sidebar-list");

        let buttons: Vec<_> = SidebarSelection::all()
            .iter()
            .copied()
            .map(|entry| {
                let button = build_entry_button(entry);
                let sender_clone = sender.clone();
                button.connect_clicked(move |_| {
                    sender_clone.input(SidebarInput::Select(entry));
                });
                entries_box.append(&button);
                (entry, button)
            })
            .collect();

        let model = Sidebar {
            selection: SidebarSelection::Inbox,
            buttons,
        };
        apply_active_class(&model.buttons, model.selection);

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: SidebarInput, sender: ComponentSender<Self>) {
        match msg {
            SidebarInput::Select(selection) => {
                if selection != self.selection {
                    self.selection = selection;
                    apply_active_class(&self.buttons, self.selection);
                    let _ = sender.output(SidebarOutput::Selected(selection));
                }
            }
        }
    }
}

fn build_entry_button(entry: SidebarSelection) -> gtk::Button {
    let icon = gtk::Image::from_icon_name(entry.icon());

    let label = gtk::Label::new(Some(entry.title()));
    label.set_hexpand(true);
    label.set_halign(gtk::Align::Start);

    let row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    row.append(&icon);
    row.append(&label);

    let button = gtk::Button::new();
    button.set_child(Some(&row));
    button.add_css_class("flat");
    button.add_css_class("sidebar-button");
    button
}

fn apply_active_class(
    buttons: &[(SidebarSelection, gtk::Button)],
    active: SidebarSelection,
) {
    for (entry, button) in buttons {
        if *entry == active {
            button.add_css_class("sidebar-button-active");
        } else {
            button.remove_css_class("sidebar-button-active");
        }
    }
}
