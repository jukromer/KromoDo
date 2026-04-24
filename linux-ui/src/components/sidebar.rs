use adw::prelude::*;
use kromodo_core::TaskFilter;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarSelection {
    Inbox,
    Today,
    Scheduled,
    Labels,
    Filters,
    Completed,
}

impl SidebarSelection {
    pub fn icon(self) -> &'static str {
        match self {
            Self::Inbox => "mail-inbox-symbolic",
            Self::Today => "starred-symbolic",
            Self::Scheduled => "x-office-calendar-symbolic",
            Self::Labels => "tag-symbolic",
            Self::Filters => "preferences-other-symbolic",
            Self::Completed => "checkbox-checked-symbolic",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Inbox => "Inbox",
            Self::Today => "Today",
            Self::Scheduled => "Scheduled",
            Self::Labels => "Labels",
            Self::Filters => "Filters",
            Self::Completed => "Completed",
        }
    }

    pub fn task_filter(self) -> Option<TaskFilter> {
        match self {
            Self::Inbox => Some(TaskFilter::Inbox),
            Self::Today => Some(TaskFilter::Today),
            Self::Scheduled => Some(TaskFilter::Upcoming),
            Self::Labels => None,
            Self::Filters => None,
            Self::Completed => Some(TaskFilter::Completed),
        }
    }

    fn all() -> &'static [SidebarSelection] {
        &[
            Self::Inbox,
            Self::Today,
            Self::Scheduled,
            Self::Labels,
            Self::Filters,
            Self::Completed,
        ]
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
            add_css_class: "kromodo-sidebar",
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
        let cards_flow = gtk::FlowBox::new();
        cards_flow.set_row_spacing(8);
        cards_flow.set_column_spacing(8);
        cards_flow.set_homogeneous(true);
        cards_flow.set_selection_mode(gtk::SelectionMode::None);
        cards_flow.set_min_children_per_line(2);
        cards_flow.set_max_children_per_line(3);
        cards_flow.set_halign(gtk::Align::Fill);
        cards_flow.set_valign(gtk::Align::Start);
        cards_flow.set_hexpand(true);

        let entries_box = gtk::Box::new(gtk::Orientation::Vertical, 8);
        entries_box.set_margin_start(12);
        entries_box.set_margin_end(12);
        entries_box.set_margin_top(12);
        entries_box.set_margin_bottom(12);
        entries_box.set_valign(gtk::Align::Start);
        entries_box.add_css_class("sidebar-list");
        entries_box.append(&cards_flow);

        let buttons: Vec<_> = SidebarSelection::all()
            .iter()
            .copied()
            .map(|entry| {
                let button = build_entry_button(entry);
                let sender_clone = sender.clone();
                button.connect_clicked(move |_| {
                    sender_clone.input(SidebarInput::Select(entry));
                });
                cards_flow.insert(&button, -1);
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
    icon.set_pixel_size(20);
    icon.set_halign(gtk::Align::Start);
    icon.add_css_class("sidebar-card-icon");

    let label = gtk::Label::new(Some(entry.title()));
    label.set_halign(gtk::Align::Start);
    label.set_valign(gtk::Align::End);
    label.set_vexpand(true);
    label.add_css_class("sidebar-card-label");

    let content = gtk::Box::new(gtk::Orientation::Vertical, 4);
    content.set_halign(gtk::Align::Fill);
    content.set_valign(gtk::Align::Fill);
    content.set_hexpand(true);
    content.set_vexpand(true);
    content.append(&icon);
    content.append(&label);

    let button = gtk::Button::new();
    button.set_child(Some(&content));
    button.set_halign(gtk::Align::Fill);
    button.set_valign(gtk::Align::Start);
    button.set_hexpand(true);
    button.set_size_request(-1, 72);
    button.add_css_class("flat");
    button.add_css_class("sidebar-card");
    button
}

fn apply_active_class(
    buttons: &[(SidebarSelection, gtk::Button)],
    active: SidebarSelection,
) {
    for (entry, button) in buttons {
        if *entry == active {
            button.add_css_class("sidebar-card-active");
        } else {
            button.remove_css_class("sidebar-card-active");
        }
    }
}
