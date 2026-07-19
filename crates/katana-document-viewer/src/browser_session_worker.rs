use super::{
    BrowserSessionUpdate,
    browser_session_state::BrowserSessionState,
    browser_session_types::{
        BrowserSessionAdapterError, BrowserSessionCommand, BrowserSessionRequest,
    },
};
use katana_render_runtime::{HtmlBrowserSession, HtmlRuntime};
use std::sync::{Arc, mpsc::Receiver};

pub(crate) struct BrowserSessionWorker;

impl BrowserSessionWorker {
    pub(crate) fn run(
        request: BrowserSessionRequest,
        commands: Receiver<BrowserSessionCommand>,
        state: Arc<BrowserSessionState>,
    ) {
        let mut session = match start_session(&request) {
            Ok(session) => session,
            Err(error) => {
                state.publish(BrowserSessionUpdate::Error(error));
                return;
            }
        };
        publish_updates(&mut session, &state);
        while let Ok(command) = commands.recv() {
            if matches!(command, BrowserSessionCommand::Close) {
                let _ = session.close();
                return;
            }
            match dispatch(&mut session, command) {
                Ok(()) => publish_updates(&mut session, &state),
                Err(error) => state.publish(BrowserSessionUpdate::Error(error)),
            }
        }
        let _ = session.close();
    }
}

fn start_session(
    request: &BrowserSessionRequest,
) -> Result<HtmlBrowserSession, BrowserSessionAdapterError> {
    HtmlRuntime
        .open(request.source.clone(), request.viewport)
        .map_err(Into::into)
}

fn dispatch(
    session: &mut HtmlBrowserSession,
    command: BrowserSessionCommand,
) -> Result<(), BrowserSessionAdapterError> {
    match command {
        BrowserSessionCommand::Input(input) => session.dispatch_input(input)?,
        BrowserSessionCommand::Resize(viewport) => session.resize(viewport)?,
        BrowserSessionCommand::Navigate(navigation) => session.navigate(navigation)?,
        BrowserSessionCommand::Refresh => session.refresh_frame()?,
        BrowserSessionCommand::Close => return Ok(()),
    }
    Ok(())
}

fn publish_updates(session: &mut HtmlBrowserSession, state: &BrowserSessionState) {
    if let Some(frame) = session.take_frame_update().cloned() {
        state.publish(BrowserSessionUpdate::Frame(frame));
    }
    if let Some(navigation) = session.take_navigation() {
        state.publish(BrowserSessionUpdate::Navigation(navigation));
    }
}

#[cfg(test)]
#[path = "browser_session_worker_tests.rs"]
mod tests;
