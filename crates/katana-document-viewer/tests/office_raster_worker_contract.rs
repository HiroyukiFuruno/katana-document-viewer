use katana_document_viewer::{
    DocumentResourceSnapshot, DocumentSession, DocumentSessionCommand, DocumentSessionConfig,
    DocumentViewerCommand, DocumentViewport, OfficeDocumentFormat, OfficeDocumentSource,
    OfficeStaticViewerSession, OfficeWorkerConfig, PdfPageRenderRequest, ViewerSource,
    ViewerSourceIdentity,
};
use std::path::{Path, PathBuf};
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;
const CHILD_ENV: &str = "KDV_OFFICE_RASTER_TEST_CHILD";
const TEST_NAME: &str = "unified_office_frames_match_direct_raster_and_reuse_the_parent_cache";

#[test]
fn unified_office_frames_match_direct_raster_and_reuse_the_parent_cache() -> TestResult {
    if std::env::var_os(CHILD_ENV).as_deref() == Some(std::ffi::OsStr::new("1")) {
        return actual_office_frames();
    }
    let output = Command::new(std::env::current_exe()?)
        .args(["--exact", TEST_NAME, "--nocapture"])
        .env(CHILD_ENV, "1")
        .env("DEBUG", "true")
        .output()?;
    let stderr = String::from_utf8(output.stderr)?;
    assert!(
        output.status.success(),
        "actual Office raster child failed: {stderr}"
    );
    assert_eq!(
        4,
        stderr.matches("stage=pdf_raster.worker_total ").count(),
        "two uncached frames per format must launch workers, repeated scale must reuse cache: {stderr}"
    );
    Ok(())
}

fn actual_office_frames() -> TestResult {
    for (name, format, mime) in [
        (
            "representative.docx",
            OfficeDocumentFormat::Docx,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        ),
        (
            "representative.pptx",
            OfficeDocumentFormat::Pptx,
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        ),
    ] {
        let bytes = std::fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../assets/fixtures/multi-format")
                .join(name),
        )?;
        let source = OfficeDocumentSource::new(
            ViewerSourceIdentity::new(format!("fixture:{name}"), "immutable"),
            format,
            mime,
            bytes,
        );
        compare_actual_frames(source)?;
        assert_eq!(
            DocumentResourceSnapshot::default(),
            DocumentSession::resource_snapshot()
        );
    }
    Ok(())
}

fn compare_actual_frames(source: OfficeDocumentSource) -> TestResult {
    let mut config =
        OfficeWorkerConfig::new(PathBuf::from(env!("CARGO_BIN_EXE_kdv-office-worker")));
    // 変換PDFの容量上限はRGBA容量上限ではなく、小さなconsumer設定も維持する。
    config.max_output_bytes = 1024 * 1024;
    let mut direct = OfficeStaticViewerSession::open(source.clone(), config.clone())?;
    let session_config =
        DocumentSessionConfig::new(DocumentViewport::new(640, 480)).office_worker(config);
    let mut unified = DocumentSession::open(ViewerSource::Office(source.clone()), session_config)?;
    assert_eq!(source.identity, unified.info().identity);
    assert_eq!(source.mime, unified.info().mime);
    assert_eq!(direct.artifact().diagnostics, unified.info().diagnostics);
    assert_eq!(direct.artifact().capabilities, unified.info().capabilities);
    for scale in [1.0, 1.25, 1.0] {
        let _ = unified.apply(DocumentSessionCommand::Viewer(
            DocumentViewerCommand::SetZoom(scale),
        ))?;
        let frame = unified.frame()?;
        let page = frame
            .surface
            .page()
            .ok_or("expected an actual paged Office frame")?;
        let expected = direct.render_item(PdfPageRenderRequest::new(0, scale))?;
        assert!(expected.surface.rgba.len() as u64 > 1024 * 1024);
        assert_eq!(expected.surface.rgba, page.rgba);
        assert_eq!(expected.surface.width, page.width);
        assert_eq!(expected.surface.height, page.height);
        assert_eq!(expected.surface.fingerprint, page.fingerprint);
        assert_eq!(expected.surface.content_scale, page.content_scale);
        assert_eq!(direct.artifact().item_count, frame.state.item_count);
    }
    unified.close();
    drop(direct);
    Ok(())
}
