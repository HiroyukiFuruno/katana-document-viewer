use super::TestResult;
use katana_document_viewer::{
    OfficeDocumentFormat, OfficeDocumentSource, OfficeStaticViewerSession, OfficeWorkerConfig,
    ViewerSourceIdentity,
};
use process_control::{ChildExt, Control};
use std::path::PathBuf;
use std::sync::Barrier;
use std::time::Duration;

const CHILD_MARKER: &str = "KDV_WRAPPED_TITLE_NORMAL_PARENT_CHILD";
const TEST_NAME: &str = "windows_production::normal_office_parent_launches_survive_concurrency";

#[test]
fn normal_office_parent_launches_survive_concurrency() -> TestResult {
    if std::env::var_os(CHILD_MARKER).is_some() {
        assert_eq!(std::env::var("DEBUG")?.as_str(), "false");
        return concurrent_parent_conversions();
    }
    // 他testの環境を変更せず、DEBUG=falseの本番Null経路を実childで固定する。
    let mut child = std::process::Command::new(std::env::current_exe()?)
        .args(["--exact", TEST_NAME, "--nocapture"])
        .env("DEBUG", "false")
        .env(CHILD_MARKER, "1")
        .stdin(std::process::Stdio::null())
        .spawn()?;
    let status = child
        .controlled()
        .time_limit(Duration::from_secs(45))
        .terminate_for_timeout()
        .strict_errors()
        .wait()?
        .ok_or("normal Office parent exceeded its existing deadline")?;
    assert!(
        status.success(),
        "normal Office parent must convert all inputs"
    );
    Ok(())
}

fn concurrent_parent_conversions() -> TestResult {
    let bytes = super::title_pptx(false, "ctr")?;
    let barrier = Barrier::new(3);
    std::thread::scope(|scope| -> TestResult {
        let mut workers = Vec::new();
        for index in 0..3 {
            let bytes = &bytes;
            let barrier = &barrier;
            workers.push(scope.spawn(move || {
                barrier.wait();
                convert_with_normal_parent(bytes.clone(), index).map_err(|error| error.to_string())
            }));
        }
        for worker in workers {
            worker
                .join()
                .map_err(|_| "normal Office parent panicked")??;
        }
        Ok(())
    })
}

fn convert_with_normal_parent(bytes: Vec<u8>, index: usize) -> TestResult {
    let source = OfficeDocumentSource::new(
        ViewerSourceIdentity::new(
            format!("memory:wrapped-title-{index}"),
            "synthetic-no-autofit",
        ),
        OfficeDocumentFormat::Pptx,
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        bytes,
    );
    let config = OfficeWorkerConfig::new(PathBuf::from(env!("CARGO_BIN_EXE_kdv-office-worker")));
    let session = OfficeStaticViewerSession::open(source, config)?;
    assert_eq!(session.artifact().format, OfficeDocumentFormat::Pptx);
    assert!(session.artifact().item_count > 0);
    Ok(())
}
