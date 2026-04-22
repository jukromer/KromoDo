use std::sync::{mpsc, Mutex};

use crate::models::task::Task;

#[derive(Debug, Clone)]
pub enum CoreEvent {
    TaskCreated(Task),
    TaskUpdated(Task),
    TaskDeleted(i64),
}

pub(crate) struct EventBus {
    subscribers: Mutex<Vec<mpsc::Sender<CoreEvent>>>,
}

impl EventBus {
    pub(crate) fn new() -> Self {
        Self {
            subscribers: Mutex::new(Vec::new()),
        }
    }

    pub(crate) fn subscribe(&self) -> mpsc::Receiver<CoreEvent> {
        let (tx, rx) = mpsc::channel();
        self.subscribers.lock().unwrap().push(tx);
        rx
    }

    pub(crate) fn publish(&self, event: CoreEvent) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.retain(|sub| sub.send(event.clone()).is_ok());
    }
}
