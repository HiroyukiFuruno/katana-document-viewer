use super::{
    BrowserSessionAdapter, BrowserSessionAdapterError, BrowserSessionRequest, BrowserSessionUpdate,
    browser_session_state::BrowserSessionState, browser_session_types::BrowserSessionCommand,
    command_send_error,
};
use katana_render_runtime::{HtmlBrowserSource, HtmlBrowserViewport};
use std::{
    sync::{Arc, mpsc},
    thread,
    time::Duration,
};

const UPDATE_TIMEOUT: Duration = Duration::from_secs(1);
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn take_update_delegates_to_adapter_state() -> TestResult {
    let (commands, _receiver) = mpsc::sync_channel(1);
    let state = Arc::new(BrowserSessionState::default());
    state.publish(BrowserSessionUpdate::Error(
        BrowserSessionAdapterError::WorkerStopped,
    ));
    let adapter = BrowserSessionAdapter {
        commands,
        state,
        worker: None,
    };

    assert!(matches!(
        adapter.take_update(),
        Some(BrowserSessionUpdate::Error(
            BrowserSessionAdapterError::WorkerStopped
        ))
    ));
    Ok(())
}

#[test]
fn start_publishes_an_in_process_runtime_frame() -> TestResult {
    let mut adapter =
        BrowserSessionAdapter::start(BrowserSessionRequest::new(source()?, viewport()?));

    assert!(matches!(
        adapter.wait_for_update(UPDATE_TIMEOUT),
        Some(BrowserSessionUpdate::Frame(frame)) if !frame.pixels.is_empty()
    ));
    adapter.close()?;
    Ok(())
}

#[test]
fn close_is_idempotent_after_the_worker_has_stopped() -> TestResult {
    let mut adapter =
        BrowserSessionAdapter::start(BrowserSessionRequest::new(source()?, viewport()?));

    assert!(matches!(
        adapter.wait_for_update(UPDATE_TIMEOUT),
        Some(BrowserSessionUpdate::Frame(frame)) if !frame.pixels.is_empty()
    ));
    adapter.close()?;
    adapter.close()?;
    Ok(())
}

#[test]
fn command_errors_map_queue_full_and_disconnected() -> TestResult {
    let (sender, receiver) = mpsc::sync_channel(0);
    let full = rejected_command(sender.try_send(BrowserSessionCommand::Refresh))?;
    assert_eq!(
        command_send_error(full),
        BrowserSessionAdapterError::CommandQueueFull
    );
    drop(receiver);
    let disconnected = rejected_command(sender.try_send(BrowserSessionCommand::Refresh))?;
    assert_eq!(
        command_send_error(disconnected),
        BrowserSessionAdapterError::WorkerStopped
    );
    Ok(())
}

#[test]
fn close_reports_worker_panic() -> TestResult {
    let (commands, receiver) = mpsc::sync_channel(1);
    let worker = thread::spawn(move || {
        let _ = receiver.recv();
        std::panic::resume_unwind(Box::new("test worker panic"));
    });
    let mut adapter = BrowserSessionAdapter {
        commands,
        state: Default::default(),
        worker: Some(worker),
    };

    assert_eq!(
        adapter.close(),
        Err(BrowserSessionAdapterError::WorkerPanicked)
    );
    Ok(())
}

#[test]
fn close_reports_a_worker_that_stopped_before_receiving_close() -> TestResult {
    let (commands, receiver) = mpsc::sync_channel(1);
    let (stopped, stopped_receiver) = mpsc::sync_channel(1);
    let worker = thread::spawn(move || {
        drop(receiver);
        let _ = stopped.send(());
    });
    stopped_receiver
        .recv()
        .map_err(|_| std::io::Error::other("worker did not stop"))?;
    let mut adapter = BrowserSessionAdapter {
        commands,
        state: Default::default(),
        worker: Some(worker),
    };

    assert_eq!(
        adapter.close(),
        Err(BrowserSessionAdapterError::WorkerStopped)
    );
    Ok(())
}

fn source() -> Result<HtmlBrowserSource, katana_render_runtime::HtmlBrowserError> {
    HtmlBrowserSource::new("<button>Run</button>", "https://example.test/index.html")
}

fn viewport() -> Result<HtmlBrowserViewport, katana_render_runtime::HtmlBrowserError> {
    HtmlBrowserViewport::new(320, 240, 1.0)
}

fn rejected_command<T>(
    result: Result<(), mpsc::TrySendError<T>>,
) -> Result<mpsc::TrySendError<T>, Box<dyn std::error::Error>> {
    match result {
        Ok(()) => Err("expected command rejection".into()),
        Err(error) => Ok(error),
    }
}
