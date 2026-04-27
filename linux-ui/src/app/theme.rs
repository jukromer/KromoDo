use adw::prelude::*;
use relm4::gtk::gdk;
use relm4::{adw, gtk};

pub(super) fn apply_dark_class(window: &adw::ApplicationWindow, is_dark: bool) {
    if is_dark {
        window.add_css_class("kromodo-dark");
    } else {
        window.remove_css_class("kromodo-dark");
    }
}

pub(super) fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(include_str!("../../data/styles.css"));
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not get default display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
