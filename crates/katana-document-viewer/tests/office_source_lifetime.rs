#[path = "support/office_source_lifetime_allocator.rs"]
mod observed_system;

use katana_document_viewer::{
    DocumentResourceSnapshot, DocumentSession, OfficeDocumentFormat, OfficeDocumentSource,
    OfficeStaticViewerSession, OfficeWorkerConfig, PdfPageRenderRequest, ViewerSourceIdentity,
};
use std::path::{Path, PathBuf};
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;
const CHILD_ENV: &str = "KDV_OFFICE_SOURCE_LIFETIME_TEST_CHILD";
const TEST_NAME: &str = "original_source_is_released_before_pdf_decode_finishes";
const PDF_DECODE_MARKER: &str = "stage=office.pdf_decode";

#[test]
fn original_source_is_released_before_pdf_decode_finishes() -> TestResult {
    if std::env::var_os(CHILD_ENV).as_deref() == Some(std::ffi::OsStr::new("1")) {
        return open_actual_office_fixture();
    }
    let output = Command::new(std::env::current_exe()?)
        .args(["--exact", TEST_NAME, "--nocapture"])
        .env(CHILD_ENV, "1")
        .env("DEBUG", "true")
        .output()?;
    let stderr = String::from_utf8(output.stderr)?;
    assert!(
        output.status.success(),
        "actual Office child failed: {stderr}"
    );
    assert!(
        released_before_decode(&stderr),
        "original bytes overlap PDF decode: {stderr}"
    );
    Ok(())
}

fn open_actual_office_fixture() -> TestResult {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.pptx");
    let bytes = std::fs::read(path)?;
    observed_system::watch(&bytes);
    let identity = ViewerSourceIdentity::new("fixture:pptx", "immutable");
    let source = OfficeDocumentSource::new(
        identity.clone(),
        OfficeDocumentFormat::Pptx,
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        bytes,
    );
    let config = OfficeWorkerConfig::new(PathBuf::from(env!("CARGO_BIN_EXE_kdv-office-worker")));
    let mut session = OfficeStaticViewerSession::open(source, config)?;
    assert_eq!(identity, session.artifact().identity);
    assert_eq!(
        session.artifact().items.len(),
        session.artifact().item_count
    );
    let frame = session.render_item(PdfPageRenderRequest::new(0, 1.0))?;
    assert!(frame.surface.width > 0 && frame.surface.height > 0);
    drop(frame);
    drop(session);
    assert!(!observed_system::marker_write_failed());
    assert_eq!(
        DocumentResourceSnapshot::default(),
        DocumentSession::resource_snapshot()
    );
    Ok(())
}

fn released_before_decode(stderr: &str) -> bool {
    let release = stderr.find(observed_system::RELEASE_MARKER);
    let decode = stderr.find(PDF_DECODE_MARKER);
    matches!((release, decode), (Some(release), Some(decode)) if release < decode)
}

#[test]
fn missing_or_late_deallocation_does_not_pass_the_lifetime_contract() {
    assert!(!released_before_decode(PDF_DECODE_MARKER));
    assert!(!released_before_decode(observed_system::RELEASE_MARKER));
    assert!(!released_before_decode(&format!(
        "{PDF_DECODE_MARKER}\n{}",
        observed_system::RELEASE_MARKER
    )));
    assert!(released_before_decode(&format!(
        "{}\n{PDF_DECODE_MARKER}",
        observed_system::RELEASE_MARKER
    )));
}
