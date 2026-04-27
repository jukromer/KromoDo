use chrono::{DateTime, Datelike, Local, Utc};
use relm4::gtk;
use relm4::gtk::glib;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

use super::{TaskRow, TaskRowInput};

pub(super) fn attach_date_picker(
    menu_button: &gtk::MenuButton,
    initial: Option<DateTime<Utc>>,
    sender: &FactorySender<TaskRow>,
) {
    let calendar = gtk::Calendar::new();
    if let Some(due) = initial {
        let local = due.with_timezone(&Local);
        if let Ok(dt) = glib::DateTime::from_local(
            local.year(),
            local.month() as i32,
            local.day() as i32,
            0,
            0,
            0.0,
        ) {
            calendar.select_day(&dt);
        }
    }

    let clear_btn = gtk::Button::with_label("Clear");
    clear_btn.add_css_class("flat");
    let ok_btn = gtk::Button::with_label("OK");
    ok_btn.add_css_class("suggested-action");

    let btn_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    btn_box.set_halign(gtk::Align::End);
    btn_box.append(&clear_btn);
    btn_box.append(&ok_btn);

    let popover_box = gtk::Box::new(gtk::Orientation::Vertical, 8);
    popover_box.set_margin_start(8);
    popover_box.set_margin_end(8);
    popover_box.set_margin_top(8);
    popover_box.set_margin_bottom(8);
    popover_box.append(&calendar);
    popover_box.append(&btn_box);

    let popover = gtk::Popover::new();
    popover.add_css_class("date-picker-popover");
    popover.set_child(Some(&popover_box));
    menu_button.set_popover(Some(&popover));

    let s = sender.clone();
    let cal_weak = calendar.downgrade();
    let popover_weak = popover.downgrade();
    ok_btn.connect_clicked(move |_| {
        let Some(cal) = cal_weak.upgrade() else {
            return;
        };
        s.input(TaskRowInput::SetDueDate {
            year: cal.year(),
            month: (cal.month() + 1) as u32,
            day: cal.day() as u32,
        });
        if let Some(p) = popover_weak.upgrade() {
            p.popdown();
        }
    });

    let s = sender.clone();
    let popover_weak = popover.downgrade();
    clear_btn.connect_clicked(move |_| {
        s.input(TaskRowInput::ClearDueDate);
        if let Some(p) = popover_weak.upgrade() {
            p.popdown();
        }
    });
}
