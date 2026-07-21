//! Worker-backed adapter for the KRR browser session.

#[path = "browser_session_command_queue.rs"]
mod browser_session_command_queue;
#[path = "browser_session_runtime.rs"]
mod browser_session_runtime;
#[path = "browser_session_state.rs"]
mod browser_session_state;
#[path = "browser_session_types.rs"]
mod browser_session_types;
#[path = "browser_session_worker.rs"]
mod browser_session_worker;

use browser_session_state::BrowserSessionState;
use browser_session_types::BrowserSessionCommand;
pub use browser_session_types::{
    BrowserSessionAdapterError, BrowserSessionOperation, BrowserSessionRequest,
    BrowserSessionUpdate,
};
pub use katana_render_runtime::{
    HtmlBrowserInput, HtmlBrowserNavigation, HtmlBrowserSource, HtmlBrowserViewport,
};
use std::{
    sync::{Arc, mpsc},
    thread::{self, JoinHandle},
    time::Duration,
};

const COMMAND_QUEUE_CAPACITY: usize = 64;

/// Non-blocking handle for one KRR-owned persistent browser page.
#[derive(Debug)]
pub struct BrowserSessionAdapter {
    commands: mpsc::SyncSender<BrowserSessionCommand>,
    state: Arc<BrowserSessionState>,
    worker: Option<JoinHandle<()>>,
}

impl BrowserSessionAdapter {
    pub fn start(request: BrowserSessionRequest) -> Self {
        let (commands, receiver) = mpsc::sync_channel(COMMAND_QUEUE_CAPACITY);
        let state = Arc::new(BrowserSessionState::default());
        let worker_state = Arc::clone(&state);
        let worker = thread::spawn(move || {
            browser_session_worker::BrowserSessionWorker::run(request, receiver, worker_state);
        });
        Self {
            commands,
            state,
            worker: Some(worker),
        }
    }

    pub fn dispatch_input(
        &self,
        input: katana_render_runtime::HtmlBrowserInput,
    ) -> Result<(), BrowserSessionAdapterError> {
        self.enqueue(BrowserSessionCommand::Input(input))
    }

    pub fn resize(
        &self,
        viewport: katana_render_runtime::HtmlBrowserViewport,
    ) -> Result<(), BrowserSessionAdapterError> {
        self.enqueue(BrowserSessionCommand::Resize(viewport))
    }

    pub fn navigate(
        &self,
        navigation: katana_render_runtime::HtmlBrowserNavigation,
    ) -> Result<(), BrowserSessionAdapterError> {
        self.enqueue(BrowserSessionCommand::Navigate(navigation))
    }

    pub fn refresh_frame(&self) -> Result<(), BrowserSessionAdapterError> {
        self.enqueue(BrowserSessionCommand::Refresh)
    }

    pub fn take_update(&self) -> Option<BrowserSessionUpdate> {
        self.state.take_update()
    }

    pub fn wait_for_update(&self, timeout: Duration) -> Option<BrowserSessionUpdate> {
        self.state.wait_for_update(timeout)
    }

    pub fn close(&mut self) -> Result<(), BrowserSessionAdapterError> {
        let Some(worker) = self.worker.take() else {
            return Ok(());
        };
        self.commands
            .send(BrowserSessionCommand::Close)
            .map_err(|_| BrowserSessionAdapterError::WorkerStopped)?;
        worker
            .join()
            .map_err(|_| BrowserSessionAdapterError::WorkerPanicked)
    }

    fn enqueue(&self, command: BrowserSessionCommand) -> Result<(), BrowserSessionAdapterError> {
        self.commands.try_send(command).map_err(command_send_error)
    }
}

impl Drop for BrowserSessionAdapter {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

fn command_send_error(
    error: mpsc::TrySendError<BrowserSessionCommand>,
) -> BrowserSessionAdapterError {
    match error {
        mpsc::TrySendError::Full(_) => BrowserSessionAdapterError::CommandQueueFull,
        mpsc::TrySendError::Disconnected(_) => BrowserSessionAdapterError::WorkerStopped,
    }
}

#[cfg(test)]
#[path = "browser_session_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "browser_session_adapter_tests.rs"]
mod adapter_tests;
