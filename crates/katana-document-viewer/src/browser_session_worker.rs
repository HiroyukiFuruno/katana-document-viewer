use super::{
    BrowserSessionUpdate,
    browser_session_command_queue::{coalesce_command, receive_command},
    browser_session_runtime::{dispatch, publish_updates, start_session},
    browser_session_state::BrowserSessionState,
    browser_session_types::{
        BrowserSessionAdapterError, BrowserSessionCommand, BrowserSessionOperation,
        BrowserSessionRequest,
    },
};
use katana_render_runtime::HtmlBrowserSession;
use std::sync::{Arc, mpsc::Receiver};

pub(crate) struct BrowserSessionWorker;

impl BrowserSessionWorker {
    pub(crate) fn run(
        request: BrowserSessionRequest,
        commands: Receiver<BrowserSessionCommand>,
        state: Arc<BrowserSessionState>,
    ) {
        let mut active = ActiveBrowserSession::new(request, state);
        let mut pending = None;
        while let Some(command) = receive_command(&commands, pending.take()) {
            let (command, next) = coalesce_command(command, &commands);
            pending = next;
            if active.handle(command) {
                return;
            }
        }
        active.close();
    }
}

struct ActiveBrowserSession {
    session: Option<HtmlBrowserSession>,
    viewport: katana_render_runtime::HtmlBrowserViewport,
    document_origin: String,
    state: Arc<BrowserSessionState>,
}

impl ActiveBrowserSession {
    fn new(request: BrowserSessionRequest, state: Arc<BrowserSessionState>) -> Self {
        let viewport = request.viewport;
        let document_origin = request.source.origin.as_str().to_owned();
        let session = start_session(&request)
            .map_err(|error| state.publish(BrowserSessionUpdate::Error(error)))
            .ok();
        let mut active = Self {
            session,
            viewport,
            document_origin,
            state,
        };
        active.publish_updates();
        active
    }

    fn handle(&mut self, command: BrowserSessionCommand) -> bool {
        match command {
            BrowserSessionCommand::Close => {
                self.close();
                return true;
            }
            BrowserSessionCommand::Resize(viewport) => self.resize(viewport),
            BrowserSessionCommand::Navigate(navigation) => self.navigate(navigation),
            BrowserSessionCommand::Input(input) => {
                self.dispatch(BrowserSessionCommand::Input(input))
            }
            BrowserSessionCommand::Refresh => self.dispatch(BrowserSessionCommand::Refresh),
        }
        false
    }

    fn resize(&mut self, viewport: katana_render_runtime::HtmlBrowserViewport) {
        let result = self
            .session
            .as_mut()
            .map(|session| dispatch(session, BrowserSessionCommand::Resize(viewport)))
            .unwrap_or_else(|| {
                viewport.validate().map_err(|source| {
                    BrowserSessionAdapterError::browser_operation(
                        BrowserSessionOperation::Resize,
                        &self.document_origin,
                        source,
                    )
                })
            });
        if self.complete(result) {
            self.viewport = viewport;
        }
    }

    fn navigate(&mut self, navigation: katana_render_runtime::HtmlBrowserNavigation) {
        let Some(session) = self.session.as_mut() else {
            self.restart(navigation);
            return;
        };
        let result = dispatch(session, BrowserSessionCommand::Navigate(navigation));
        self.complete(result);
    }

    fn restart(&mut self, navigation: katana_render_runtime::HtmlBrowserNavigation) {
        self.document_origin = navigation.source.origin.as_str().to_owned();
        let request = BrowserSessionRequest::new(navigation.source, self.viewport);
        match start_session(&request) {
            Ok(session) => self.session = Some(session),
            Err(error) => self.state.publish(BrowserSessionUpdate::Error(error)),
        }
        self.publish_updates();
    }

    fn dispatch(&mut self, command: BrowserSessionCommand) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let result = dispatch(session, command);
        self.complete(result);
    }

    fn complete(&mut self, result: Result<(), BrowserSessionAdapterError>) -> bool {
        match result {
            Ok(()) => {
                self.publish_updates();
                true
            }
            Err(error) => {
                self.state.publish(BrowserSessionUpdate::Error(error));
                false
            }
        }
    }

    fn publish_updates(&mut self) {
        if let Some(session) = self.session.as_mut() {
            publish_updates(session, &self.state);
        }
    }

    fn close(&mut self) {
        if let Some(mut session) = self.session.take() {
            let _ = session.close();
        }
    }
}

#[cfg(test)]
#[path = "browser_session_worker_tests.rs"]
mod tests;
