use katana_document_viewer::browser_session::{
    BrowserSessionAdapter, BrowserSessionRequest, BrowserSessionUpdate, HtmlBrowserNavigation,
    HtmlBrowserSource, HtmlBrowserViewport,
};
use std::{fs, path::Path, time::Duration};

const UPDATE_TIMEOUT: Duration = Duration::from_secs(1);
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn public_adapter_forwards_in_process_runtime_commands() -> TestResult {
    let mut adapter = BrowserSessionAdapter::start(request("<p>Initial</p>")?);

    assert_frame(adapter.wait_for_update(UPDATE_TIMEOUT))?;
    adapter.resize(HtmlBrowserViewport::new(480, 160, 1.0)?)?;
    assert_frame(adapter.wait_for_update(UPDATE_TIMEOUT))?;
    adapter.refresh_frame()?;
    assert_frame(adapter.wait_for_update(UPDATE_TIMEOUT))?;
    adapter.navigate(HtmlBrowserNavigation::new(HtmlBrowserSource::new(
        "<p>Next</p>",
        "https://example.test/next.html",
    )?)?)?;
    assert_frame(adapter.wait_for_update(UPDATE_TIMEOUT))?;
    adapter.close()?;
    Ok(())
}

#[test]
fn adapter_forwards_document_lifecycle_errors_with_runtime_context() -> TestResult {
    let mut adapter = BrowserSessionAdapter::start(request(
        "<script>document.addEventListener('DOMContentLoaded', () => { throw new Error('lifecycle failed'); });</script>",
    )?);

    let error = match adapter.wait_for_update(UPDATE_TIMEOUT) {
        Some(BrowserSessionUpdate::Error(error)) => error,
        update => return Err(format!("expected lifecycle error, got {update:?}").into()),
    };
    let report = error.to_string();
    for expected in [
        "Layer: KRR runtime",
        "Operation: start",
        "Document: https://example.test/index.html",
        "Cause: in-process HTML runtime failed",
        "JavaScript exception: Error: lifecycle failed",
        "inline-script:1:",
    ] {
        assert!(
            report.contains(expected),
            "missing {expected:?} in {report}"
        );
    }
    adapter.close()?;
    Ok(())
}

#[test]
fn adapter_boundary_does_not_reintroduce_html_semantics_or_an_external_browser() -> TestResult {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for source_name in [
        "src/browser_session.rs",
        "src/browser_session_state.rs",
        "src/browser_session_types.rs",
        "src/browser_session_worker.rs",
    ] {
        let source = fs::read_to_string(crate_root.join(source_name))?;
        for forbidden in [
            "html5ever",
            "markup5ever",
            "cssparser",
            "v8::",
            "HtmlParser",
            "HtmlRenderer",
            "HtmlBrowserProcess",
            "headless_chrome",
            "Chromium",
            "WebView",
        ] {
            assert!(
                !source.contains(forbidden),
                "KDV browser-session adapter must not own {forbidden}: {source_name}"
            );
        }
    }
    Ok(())
}

fn request(html: &str) -> Result<BrowserSessionRequest, Box<dyn std::error::Error>> {
    Ok(BrowserSessionRequest::new(
        HtmlBrowserSource::new(html, "https://example.test/index.html")?,
        HtmlBrowserViewport::new(320, 240, 1.0)?,
    ))
}

fn assert_frame(update: Option<BrowserSessionUpdate>) -> TestResult {
    match update {
        Some(BrowserSessionUpdate::Frame(frame)) if !frame.pixels.is_empty() => Ok(()),
        _ => Err(format!("expected browser frame, got {update:?}").into()),
    }
}
