use super::{
    BrowserSessionUpdate,
    browser_session_state::BrowserSessionState,
    browser_session_types::{
        BrowserSessionAdapterError, BrowserSessionCommand, BrowserSessionRequest,
    },
};
use katana_render_runtime::{HtmlBrowserInput, HtmlBrowserSession, HtmlRuntime};
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
        let mut pending = None;
        while let Some(command) = receive_command(&commands, pending.take()) {
            let (command, next) = coalesce_command(command, &commands);
            pending = next;
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

fn receive_command(
    commands: &Receiver<BrowserSessionCommand>,
    pending: Option<BrowserSessionCommand>,
) -> Option<BrowserSessionCommand> {
    pending.or_else(|| commands.recv().ok())
}

fn coalesce_command(
    mut command: BrowserSessionCommand,
    commands: &Receiver<BrowserSessionCommand>,
) -> (BrowserSessionCommand, Option<BrowserSessionCommand>) {
    while let Ok(next) = commands.try_recv() {
        match merge_command(&mut command, next) {
            Ok(()) => {}
            Err(next) => return (command, Some(next)),
        }
    }
    (command, None)
}

fn merge_command(
    command: &mut BrowserSessionCommand,
    next: BrowserSessionCommand,
) -> Result<(), BrowserSessionCommand> {
    match (command, next) {
        (
            BrowserSessionCommand::Input(HtmlBrowserInput::Scroll { delta_x, delta_y }),
            BrowserSessionCommand::Input(HtmlBrowserInput::Scroll {
                delta_x: next_x,
                delta_y: next_y,
            }),
        ) => {
            *delta_x += next_x;
            *delta_y += next_y;
            Ok(())
        }
        (command @ BrowserSessionCommand::Resize(_), BrowserSessionCommand::Resize(viewport)) => {
            *command = BrowserSessionCommand::Resize(viewport);
            Ok(())
        }
        (_, next) => Err(next),
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

#[cfg(test)]
#[path = "browser_session_worker_coalescing_tests.rs"]
mod coalescing_tests;
