use super::{
    BrowserSessionUpdate,
    browser_session_state::BrowserSessionState,
    browser_session_types::{
        BrowserSessionAdapterError, BrowserSessionCommand, BrowserSessionOperation,
        BrowserSessionRequest,
    },
};
use katana_render_runtime::{HtmlBrowserSession, HtmlRuntime};

pub(super) fn start_session(
    request: &BrowserSessionRequest,
) -> Result<HtmlBrowserSession, BrowserSessionAdapterError> {
    let document_origin = request.source.origin.as_str();
    HtmlRuntime
        .open(request.source.clone(), request.viewport)
        .map_err(|source| {
            BrowserSessionAdapterError::browser_operation(
                BrowserSessionOperation::Start,
                document_origin,
                source,
            )
        })
}

pub(super) fn dispatch(
    session: &mut HtmlBrowserSession,
    command: BrowserSessionCommand,
) -> Result<(), BrowserSessionAdapterError> {
    let current_origin = session.source().origin.as_str().to_owned();
    match command {
        BrowserSessionCommand::Input(input) => session.dispatch_input(input).map_err(|source| {
            operation_error(BrowserSessionOperation::Input, current_origin, source)
        })?,
        BrowserSessionCommand::Resize(viewport) => session.resize(viewport).map_err(|source| {
            operation_error(BrowserSessionOperation::Resize, current_origin, source)
        })?,
        BrowserSessionCommand::Navigate(navigation) => {
            let target_origin = navigation.source.origin.as_str().to_owned();
            session.navigate(navigation).map_err(|source| {
                operation_error(BrowserSessionOperation::Navigate, target_origin, source)
            })?
        }
        BrowserSessionCommand::Refresh => session.refresh_frame().map_err(|source| {
            operation_error(BrowserSessionOperation::Refresh, current_origin, source)
        })?,
        BrowserSessionCommand::Close => return Ok(()),
    }
    Ok(())
}

fn operation_error(
    operation: BrowserSessionOperation,
    document_origin: String,
    source: katana_render_runtime::HtmlBrowserError,
) -> BrowserSessionAdapterError {
    BrowserSessionAdapterError::browser_operation(operation, document_origin, source)
}

pub(super) fn publish_updates(session: &mut HtmlBrowserSession, state: &BrowserSessionState) {
    if let Some(frame) = session.take_frame_update().cloned() {
        state.publish(BrowserSessionUpdate::Frame(frame));
    }
    if let Some(navigation) = session.take_navigation() {
        state.publish(BrowserSessionUpdate::Navigation(navigation));
    }
}
