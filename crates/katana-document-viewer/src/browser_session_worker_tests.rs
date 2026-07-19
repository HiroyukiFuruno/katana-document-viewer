use super::{BrowserSessionWorker, dispatch, publish_updates, start_session};
use crate::browser_session::{
    BrowserSessionAdapter, BrowserSessionUpdate,
    browser_session_state::BrowserSessionState,
    browser_session_types::{BrowserSessionCommand, BrowserSessionRequest},
};
use katana_render_runtime::{
    HtmlBrowserInput, HtmlBrowserNavigation, HtmlBrowserSession, HtmlBrowserSource,
    HtmlBrowserViewport,
};
use std::{
    sync::{Arc, mpsc},
    thread,
    time::Duration,
};

const UPDATE_TIMEOUT: Duration = Duration::from_secs(1);
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dispatch_handles_every_browser_command() -> TestResult {
    let mut session = start_session(&request("index")?)?;

    dispatch(
        &mut session,
        BrowserSessionCommand::Input(HtmlBrowserInput::Focus { focused: true }),
    )?;
    dispatch(
        &mut session,
        BrowserSessionCommand::Navigate(HtmlBrowserNavigation::new(source("next")?)?),
    )?;
    dispatch(&mut session, BrowserSessionCommand::Resize(viewport()?))?;
    dispatch(&mut session, BrowserSessionCommand::Refresh)?;
    dispatch(&mut session, BrowserSessionCommand::Close)?;
    session.close()?;
    Ok(())
}

#[test]
fn dispatch_propagates_errors_after_the_runtime_session_is_closed() -> TestResult {
    let mut session = start_session(&request("index")?)?;
    session.close()?;

    assert!(
        dispatch(
            &mut session,
            BrowserSessionCommand::Input(HtmlBrowserInput::Focus { focused: true }),
        )
        .is_err()
    );
    assert!(dispatch(&mut session, BrowserSessionCommand::Resize(viewport()?),).is_err());
    assert!(
        dispatch(
            &mut session,
            BrowserSessionCommand::Navigate(HtmlBrowserNavigation::new(source("next")?)?),
        )
        .is_err()
    );
    assert!(dispatch(&mut session, BrowserSessionCommand::Refresh).is_err());
    Ok(())
}

#[test]
fn worker_publishes_startup_errors() -> TestResult {
    let (_sender, receiver) = mpsc::sync_channel(1);
    let state = Arc::new(BrowserSessionState::default());
    let request = BrowserSessionRequest::new(
        source("index")?,
        HtmlBrowserViewport {
            width: 0,
            height: 1,
            device_scale_factor: 1.0,
        },
    );

    BrowserSessionWorker::run(request, receiver, Arc::clone(&state));

    assert!(matches!(
        state.take_update(),
        Some(BrowserSessionUpdate::Error(_))
    ));
    Ok(())
}

#[test]
fn publishing_without_pending_browser_updates_is_a_noop() -> TestResult {
    let mut session = HtmlBrowserSession::new(source("index")?, viewport()?)?;
    let state = BrowserSessionState::default();
    let _ = session.take_frame_update();

    publish_updates(&mut session, &state);

    assert!(state.take_update().is_none());
    Ok(())
}

#[test]
fn worker_publishes_initial_frame_and_closes_after_sender_drop() -> TestResult {
    let (sender, receiver) = mpsc::sync_channel(1);
    let state = Arc::new(BrowserSessionState::default());
    let worker_state = Arc::clone(&state);
    let request = request("index")?;
    let worker = thread::spawn(move || {
        BrowserSessionWorker::run(request, receiver, worker_state);
    });

    assert_frame(state.wait_for_update(UPDATE_TIMEOUT))?;
    drop(sender);
    worker
        .join()
        .map_err(|_| std::io::Error::other("worker panicked"))?;
    Ok(())
}

#[test]
fn worker_publishes_invalid_resize_errors() -> TestResult {
    let mut adapter = BrowserSessionAdapter::start(request("index")?);
    assert_frame(adapter.wait_for_update(UPDATE_TIMEOUT))?;
    adapter.resize(HtmlBrowserViewport {
        width: 0,
        height: 1,
        device_scale_factor: 1.0,
    })?;
    assert!(matches!(
        adapter.wait_for_update(UPDATE_TIMEOUT),
        Some(BrowserSessionUpdate::Error(_))
    ));
    adapter.close()?;
    Ok(())
}

fn request(page: &str) -> Result<BrowserSessionRequest, katana_render_runtime::HtmlBrowserError> {
    Ok(BrowserSessionRequest::new(source(page)?, viewport()?))
}

fn source(page: &str) -> Result<HtmlBrowserSource, katana_render_runtime::HtmlBrowserError> {
    HtmlBrowserSource::new(
        "<button>Run</button>",
        format!("https://example.test/{page}.html"),
    )
}

fn viewport() -> Result<HtmlBrowserViewport, katana_render_runtime::HtmlBrowserError> {
    HtmlBrowserViewport::new(320, 240, 1.0)
}

fn assert_frame(update: Option<BrowserSessionUpdate>) -> TestResult {
    match update {
        Some(BrowserSessionUpdate::Frame(frame)) if !frame.pixels.is_empty() => Ok(()),
        _ => Err("expected initial browser frame".into()),
    }
}
