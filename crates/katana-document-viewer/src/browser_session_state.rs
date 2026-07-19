use super::BrowserSessionUpdate;
use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex, MutexGuard},
    time::Duration,
};

#[derive(Debug, Default)]
pub(crate) struct BrowserSessionState {
    updates: Mutex<PendingUpdates>,
    changed: Condvar,
}

#[derive(Debug, Default)]
struct PendingUpdates {
    latest_frame: Option<BrowserSessionUpdate>,
    events: VecDeque<BrowserSessionUpdate>,
}

impl BrowserSessionState {
    pub(crate) fn publish(&self, update: BrowserSessionUpdate) {
        let mut updates = self.lock_updates();
        match update {
            BrowserSessionUpdate::Frame(_) => updates.latest_frame = Some(update),
            BrowserSessionUpdate::Navigation(_) | BrowserSessionUpdate::Error(_) => {
                updates.events.push_back(update);
            }
        }
        self.changed.notify_all();
    }

    pub(crate) fn take_update(&self) -> Option<BrowserSessionUpdate> {
        self.take_from(&mut self.lock_updates())
    }

    pub(crate) fn wait_for_update(&self, timeout: Duration) -> Option<BrowserSessionUpdate> {
        let updates = self.lock_updates();
        let mut updates = if updates_available(&updates) {
            updates
        } else {
            self.wait_for_change(updates, timeout)
        };
        self.take_from(&mut updates)
    }

    fn take_from(&self, updates: &mut PendingUpdates) -> Option<BrowserSessionUpdate> {
        updates
            .events
            .pop_front()
            .or_else(|| updates.latest_frame.take())
    }

    fn lock_updates(&self) -> MutexGuard<'_, PendingUpdates> {
        self.updates
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn wait_for_change<'a>(
        &self,
        updates: MutexGuard<'a, PendingUpdates>,
        timeout: Duration,
    ) -> MutexGuard<'a, PendingUpdates> {
        self.changed
            .wait_timeout(updates, timeout)
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .0
    }
}

fn updates_available(updates: &PendingUpdates) -> bool {
    updates.latest_frame.is_some() || !updates.events.is_empty()
}

#[cfg(test)]
#[path = "browser_session_state_tests.rs"]
mod tests;
