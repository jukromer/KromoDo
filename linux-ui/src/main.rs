
mod app;
mod components;
use app::App;
use kromodo_core::AppState;
use relm4::gtk;
use relm4::prelude::*;
use std::sync::Arc;

fn db_path() -> String {
    let dir = gtk::glib::user_data_dir().join("kromodo");
    std::fs::create_dir_all(&dir).expect("Could not create data directory");
    dir.join("tasks.db")
        .to_string_lossy()
        .into_owned()
}

fn main() {
    let state = Arc::new(
        AppState::new(&db_path()).expect("Could not open Database"),
    );

    let app = RelmApp::new("dev.kromodo.app");
    gtk::Window::set_default_icon_name("dev.kromodo.app");
    app.run::<App>(state);
}