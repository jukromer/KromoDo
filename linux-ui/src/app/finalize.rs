use relm4::gtk::glib;
use relm4::prelude::*;
use std::time::Duration;

use super::{App, AppMsg};

impl App {
    pub(super) fn schedule_finalize(
        &mut self,
        id: i64,
        sender: &ComponentSender<Self>,
        make_msg: impl FnOnce(u64) -> AppMsg + 'static,
    ) {
        let token = self.next_finalize_token;
        self.next_finalize_token = self.next_finalize_token.wrapping_add(1);
        self.pending_finalize.insert(id, token);
        let msg = make_msg(token);
        let s = sender.clone();
        glib::timeout_add_local_once(Duration::from_millis(240), move || {
            s.input(msg);
        });
    }

    pub(super) fn claim_finalize(&mut self, id: i64, token: u64) -> bool {
        match self.pending_finalize.get(&id) {
            Some(&t) if t == token => {
                self.pending_finalize.remove(&id);
                true
            }
            _ => false,
        }
    }
}
