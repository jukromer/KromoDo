use relm4::prelude::*;

use super::{App, AppMsg};
use crate::components::sidebar::SidebarSelection;
use crate::components::task_row::TaskRowInput;
use kromodo_core::{CoreEvent, Task};

impl App {
    pub(super) fn handle_core_event(
        &mut self,
        event: CoreEvent,
        sender: &ComponentSender<Self>,
    ) {
        match event {
            CoreEvent::TaskCreated(task) => {
                let in_view = self
                    .selection
                    .task_filter()
                    .map_or(false, |f| f.matches(&task));
                if in_view {
                    self.tasks.guard().push_front(task);
                }
            }
            CoreEvent::TaskUpdated(task) => self.handle_task_updated(task, sender),
            CoreEvent::TaskDeleted(id) => {
                self.pending_finalize.remove(&id);
                let mut guard = self.tasks.guard();
                let index = (0..guard.len())
                    .find(|&i| guard.get(i).map(|r| r.task_id() == id).unwrap_or(false));
                if let Some(i) = index {
                    guard.remove(i);
                }
                drop(guard);

                let mut done_guard = self.completed_tasks.guard();
                let done_index = (0..done_guard.len())
                    .find(|&i| done_guard.get(i).map(|r| r.task_id() == id).unwrap_or(false));
                if let Some(i) = done_index {
                    done_guard.remove(i);
                }
            }
        }
    }

    fn handle_task_updated(&mut self, task: Task, sender: &ComponentSender<Self>) {
        let had_pending = self.pending_finalize.remove(&task.id).is_some();

        let still_matches = self
            .selection
            .task_filter()
            .map_or(false, |f| f.matches(&task));
        let in_inbox = matches!(self.selection, SidebarSelection::Inbox);

        let open_guard = self.tasks.guard();
        let open_index = (0..open_guard.len())
            .find(|&i| open_guard.get(i).map(|r| r.task_id() == task.id).unwrap_or(false));
        drop(open_guard);

        let done_guard = self.completed_tasks.guard();
        let done_index = (0..done_guard.len())
            .find(|&i| done_guard.get(i).map(|r| r.task_id() == task.id).unwrap_or(false));
        drop(done_guard);

        if had_pending {
            if let Some(i) = open_index {
                self.tasks.send(i, TaskRowInput::SetRevealed(true));
            }
            if let Some(i) = done_index {
                self.completed_tasks.send(i, TaskRowInput::SetRevealed(true));
            }
        }

        if !still_matches {
            if let Some(i) = open_index {
                self.tasks.send(i, TaskRowInput::SetRevealed(false));
            }
            if let Some(i) = done_index {
                self.completed_tasks.send(i, TaskRowInput::SetRevealed(false));
            }
            if open_index.is_some() || done_index.is_some() {
                let id = task.id;
                self.schedule_finalize(id, sender, move |token| {
                    AppMsg::FinalizeRemove { id, token }
                });
            }
        } else if in_inbox {
            if let Some(i) = open_index {
                if task.is_done {
                    self.tasks.send(i, TaskRowInput::SetRevealed(false));
                    let id = task.id;
                    let t = task.clone();
                    self.schedule_finalize(id, sender, move |token| AppMsg::FinalizeMove {
                        task: t,
                        to_done: true,
                        token,
                    });
                } else {
                    self.tasks.send(i, TaskRowInput::ReplaceTask(task));
                }
            } else if let Some(i) = done_index {
                if !task.is_done {
                    self.completed_tasks
                        .send(i, TaskRowInput::SetRevealed(false));
                    let id = task.id;
                    let t = task.clone();
                    self.schedule_finalize(id, sender, move |token| AppMsg::FinalizeMove {
                        task: t,
                        to_done: false,
                        token,
                    });
                } else {
                    self.completed_tasks.send(i, TaskRowInput::ReplaceTask(task));
                }
            } else if task.is_done {
                self.completed_tasks.guard().push_front(task);
            } else {
                self.tasks.guard().push_front(task);
            }
        } else if let Some(i) = open_index {
            self.tasks.send(i, TaskRowInput::ReplaceTask(task));
        } else {
            self.tasks.guard().push_front(task);
        }
    }

    pub(super) fn handle_finalize_remove(&mut self, id: i64, token: u64) {
        if !self.claim_finalize(id, token) {
            return;
        }
        let open_guard = self.tasks.guard();
        let idx = (0..open_guard.len())
            .find(|&i| open_guard.get(i).map(|r| r.task_id() == id).unwrap_or(false));
        drop(open_guard);
        if let Some(i) = idx {
            self.tasks.guard().remove(i);
        }

        let done_guard = self.completed_tasks.guard();
        let idx = (0..done_guard.len())
            .find(|&i| done_guard.get(i).map(|r| r.task_id() == id).unwrap_or(false));
        drop(done_guard);
        if let Some(i) = idx {
            self.completed_tasks.guard().remove(i);
        }
    }

    pub(super) fn handle_finalize_move(&mut self, task: Task, to_done: bool, token: u64) {
        if !self.claim_finalize(task.id, token) {
            return;
        }
        if to_done {
            let open_guard = self.tasks.guard();
            let idx = (0..open_guard.len())
                .find(|&i| open_guard.get(i).map(|r| r.task_id() == task.id).unwrap_or(false));
            drop(open_guard);
            if let Some(i) = idx {
                self.tasks.guard().remove(i);
                self.completed_tasks.guard().push_front(task);
            }
        } else {
            let done_guard = self.completed_tasks.guard();
            let idx = (0..done_guard.len())
                .find(|&i| done_guard.get(i).map(|r| r.task_id() == task.id).unwrap_or(false));
            drop(done_guard);
            if let Some(i) = idx {
                self.completed_tasks.guard().remove(i);
                self.tasks.guard().push_front(task);
            }
        }
    }
}
