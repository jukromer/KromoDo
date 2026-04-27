use relm4::prelude::*;

use super::{App, AppMsg};
use crate::components::sidebar::SidebarSelection;
use crate::components::task_row::{TaskRow, TaskRowInput};
use kromodo_core::{CoreEvent, Task};

fn task_index(factory: &FactoryVecDeque<TaskRow>, id: i64) -> Option<usize> {
    factory.iter().position(|r| r.task_id() == id)
}

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
                if let Some(i) = task_index(&self.tasks, id) {
                    self.tasks.guard().remove(i);
                }
                if let Some(i) = task_index(&self.completed_tasks, id) {
                    self.completed_tasks.guard().remove(i);
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

        let open_index = task_index(&self.tasks, task.id);
        let done_index = task_index(&self.completed_tasks, task.id);

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
        if let Some(i) = task_index(&self.tasks, id) {
            self.tasks.guard().remove(i);
        }
        if let Some(i) = task_index(&self.completed_tasks, id) {
            self.completed_tasks.guard().remove(i);
        }
    }

    pub(super) fn handle_finalize_move(&mut self, task: Task, to_done: bool, token: u64) {
        if !self.claim_finalize(task.id, token) {
            return;
        }
        if to_done {
            if let Some(i) = task_index(&self.tasks, task.id) {
                self.tasks.guard().remove(i);
                self.completed_tasks.guard().push_front(task);
            }
        } else if let Some(i) = task_index(&self.completed_tasks, task.id) {
            self.completed_tasks.guard().remove(i);
            self.tasks.guard().push_front(task);
        }
    }
}
