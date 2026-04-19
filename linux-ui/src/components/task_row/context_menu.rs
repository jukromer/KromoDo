use relm4::gtk;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

use super::{TaskRow, TaskRowInput};

fn context_menu_btn(label: &str) -> gtk::Button {
    let btn = gtk::Button::with_label(label);
    btn.set_has_frame(false);
    btn.add_css_class("context-menu-item");
    btn
}

fn context_menu_sep() -> gtk::Separator {
    let sep = gtk::Separator::new(gtk::Orientation::Horizontal);
    sep.add_css_class("context-menu-sep");
    sep
}

pub(super) fn show_context_popover(
    widget: &gtk::Widget,
    x: i32,
    y: i32,
    sender: &FactorySender<TaskRow>,
) {
    let popover = gtk::Popover::new();
    popover.set_parent(widget);
    popover.set_has_arrow(false);
    popover.add_css_class("task-context-menu");
    popover.set_pointing_to(Some(&gtk::gdk::Rectangle::new(x, y, 1, 1)));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let btn_complete = context_menu_btn("Mark Complete");
    let btn_edit = context_menu_btn("Edit");
    let btn_dup = context_menu_btn("Duplicate");
    let btn_today = context_menu_btn("Due Today");
    let btn_tomorrow = context_menu_btn("Due Tomorrow");
    let btn_delete = context_menu_btn("Delete Task");
    btn_delete.add_css_class("context-menu-item-destructive");

    vbox.append(&btn_complete);
    vbox.append(&btn_edit);
    vbox.append(&btn_dup);
    vbox.append(&context_menu_sep());
    vbox.append(&btn_today);
    vbox.append(&btn_tomorrow);
    vbox.append(&context_menu_sep());
    vbox.append(&btn_delete);

    popover.set_child(Some(&vbox));

    let p = popover.clone(); let s = sender.clone();
    btn_complete.connect_clicked(move |_| { p.popdown(); s.input(TaskRowInput::Toggle); });
    let p = popover.clone(); let s = sender.clone();
    btn_edit.connect_clicked(move |_| { p.popdown(); s.input(TaskRowInput::ToggleExpand); });
    let p = popover.clone(); let s = sender.clone();
    btn_dup.connect_clicked(move |_| { p.popdown(); s.input(TaskRowInput::Duplicate); });
    let p = popover.clone(); let s = sender.clone();
    btn_today.connect_clicked(move |_| { p.popdown(); s.input(TaskRowInput::SetDueToday); });
    let p = popover.clone(); let s = sender.clone();
    btn_tomorrow.connect_clicked(move |_| { p.popdown(); s.input(TaskRowInput::SetDueTomorrow); });
    let p = popover.clone(); let s = sender.clone();
    btn_delete.connect_clicked(move |_| { p.popdown(); s.input(TaskRowInput::Delete); });

    popover.popup();
    popover.connect_closed(|p| p.unparent());
}
