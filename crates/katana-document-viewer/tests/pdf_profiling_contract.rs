use katana_document_viewer::{
    BinaryDocumentSource, PdfPageRenderRequest, PdfViewerSession, ViewerSourceIdentity,
};
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const CHILD_ENV: &str = "KDV_PDF_PROFILING_CHILD";
const TEST_NAME: &str = "direct_pdf_profiling_emits_decode_raster_and_frame_stages";
const SAMPLE_PDF: &[u8] = include_bytes!("../../../assets/reference/katana/pdf/sample.pdf");

#[test]
fn direct_pdf_profiling_emits_decode_raster_and_frame_stages() -> TestResult {
    if std::env::var_os(CHILD_ENV).as_deref() == Some(std::ffi::OsStr::new("1")) {
        return render_real_fixture();
    }
    let output = Command::new(std::env::current_exe()?)
        .args(["--exact", TEST_NAME, "--nocapture"])
        .env(CHILD_ENV, "1")
        .env("DEBUG", "true")
        .output()?;
    let stderr = String::from_utf8(output.stderr)?;
    assert!(
        output.status.success(),
        "PDF profiling child failed: {stderr}"
    );
    for stage in [
        "pdf.decode",
        "pdf.render",
        "pdf.rasterize",
        "pdf.image_encode",
        "pdf.frame_decode",
    ] {
        assert!(
            stderr.contains(&format!("stage={stage} ")),
            "missing {stage} trace: {stderr}"
        );
    }
    Ok(())
}

fn render_real_fixture() -> TestResult {
    let source = BinaryDocumentSource::new(
        ViewerSourceIdentity::new("fixture:sample.pdf", "sha256:sample"),
        "application/pdf",
        SAMPLE_PDF.to_vec(),
    );
    let mut session = PdfViewerSession::open(source)?;
    let frame = session.render_page(PdfPageRenderRequest::new(0, 1.0))?;
    assert_real_frame(&frame)?;
    Ok(())
}

fn assert_real_frame(frame: &katana_document_viewer::PdfRenderedPage) -> TestResult {
    assert!(frame.surface.width > 0);
    assert!(frame.surface.height > 0);
    assert!(
        frame
            .surface
            .rgba
            .as_chunks::<4>()
            .0
            .iter()
            .all(|pixel| pixel[3] == 255)
    );
    assert!(
        frame
            .surface
            .rgba
            .as_chunks::<4>()
            .0
            .iter()
            .any(|pixel| pixel[..3].iter().any(|channel| *channel < 240))
    );
    assert_eq!(
        usize::try_from(frame.surface.width * frame.surface.height * 4)?,
        frame.surface.rgba.len()
    );
    Ok(())
}
