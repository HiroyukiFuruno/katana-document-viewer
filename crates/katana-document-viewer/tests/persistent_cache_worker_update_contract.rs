#![cfg(unix)]

use katana_document_viewer::{
    OfficeDocumentFormat, OfficeDocumentSource, OfficeWorkerConfig, PdfPageRenderRequest,
    PersistentCacheError, PersistentDocumentCache, PersistentOfficeViewerSession,
    ViewerSourceIdentity,
};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn source() -> OfficeDocumentSource {
    OfficeDocumentSource::new(
        ViewerSourceIdentity::new("fixture:worker-update", "one"),
        OfficeDocumentFormat::Docx,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        include_bytes!("../../../assets/fixtures/multi-format/representative.docx").to_vec(),
    )
}

fn replace_worker(path: &Path, revision: &str, mutate_during_render: bool) -> TestResult {
    let binary = env!("CARGO_BIN_EXE_kdv-office-worker");
    let mutation = if mutate_during_render {
        "if [ \"$2\" = pdf-raster ]; then printf '\\n# changed during raster\\n' >> \"$0\"; fi\n"
    } else {
        ""
    };
    let script = format!("#!/bin/sh\n# {revision}\n{mutation}exec '{binary}' \"$@\"\n");
    let replacement = path.with_extension("replacement");
    std::fs::write(&replacement, script)?;
    std::fs::set_permissions(&replacement, std::fs::Permissions::from_mode(0o700))?;
    std::fs::rename(replacement, path)?;
    Ok(())
}

#[test]
fn raster_cache_is_not_reused_under_rolled_back_worker_identity() -> TestResult {
    let root = tempfile::tempdir()?;
    let worker = root.path().join("worker");
    replace_worker(&worker, "old", false)?;
    let cache = PersistentDocumentCache::new(root.path().join("cache"), 128 << 20, "env".into())?;
    let config = OfficeWorkerConfig::new(worker.clone());
    let mut original =
        PersistentOfficeViewerSession::open(source(), config.clone(), cache.clone())?;
    let request = PdfPageRenderRequest::new(0, 0.5);
    assert!(!original.render_item(request)?.1);
    assert!(original.render_item(request)?.1);
    cache.clear()?;
    replace_worker(&worker, "new", false)?;
    assert!(!original.render_item(request)?.1);
    replace_worker(&worker, "old", false)?;
    assert!(!original.render_item(request)?.1);
    let mut rolled_back =
        PersistentOfficeViewerSession::open(source(), config.clone(), cache.clone())?;
    assert!(rolled_back.render_item(request)?.1);
    let mut restarted = PersistentOfficeViewerSession::open(source(), config, cache)?;
    assert!(restarted.render_item(request)?.1);
    Ok(())
}

#[test]
fn worker_change_during_raster_is_rejected_without_persisting_page() -> TestResult {
    let root = tempfile::tempdir()?;
    let worker = root.path().join("worker");
    replace_worker(&worker, "changing", true)?;
    let cache = PersistentDocumentCache::new(root.path().join("cache"), 128 << 20, "env".into())?;
    let config = OfficeWorkerConfig::new(worker.clone());
    let mut session = PersistentOfficeViewerSession::open(source(), config.clone(), cache.clone())?;
    let request = PdfPageRenderRequest::new(0, 0.5);
    let entries_before = std::fs::read_dir(cache.directory())?.count();
    assert!(matches!(
        session.render_item(request),
        Err(PersistentCacheError::Io(_))
    ));
    assert_eq!(
        entries_before,
        std::fs::read_dir(cache.directory())?.count()
    );
    replace_worker(&worker, "changing", false)?;
    let mut restarted = PersistentOfficeViewerSession::open(source(), config, cache)?;
    assert!(!restarted.render_item(request)?.1);
    Ok(())
}
