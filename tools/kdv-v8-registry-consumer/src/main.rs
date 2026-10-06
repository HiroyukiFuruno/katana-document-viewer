use katana_document_viewer::{
    BinaryDocumentSource, KdvThemeSnapshot, KrrMathRenderEngine, PdfPageRenderRequest,
    PdfViewerLimits, PersistentDocumentCache, PersistentOfficeViewerSession,
    PersistentPdfViewerSession, ViewerSourceIdentity,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = KrrMathRenderEngine::render_display_svg("", &KdvThemeSnapshot::katana_light());
    let _: Option<PersistentOfficeViewerSession> = None;
    let expected_hit = std::env::args().nth(1).as_deref() == Some("restart");
    let source = BinaryDocumentSource::new(
        ViewerSourceIdentity::new("fixture:registry-consumer.pdf", "one"),
        "application/pdf",
        include_bytes!("../fixtures/sample.pdf").to_vec(),
    );
    let cache = PersistentDocumentCache::new(
        std::env::current_dir()?.join("cache"),
        128 * 1024 * 1024,
        "registry-consumer-v1".into(),
    )?;
    let mut session = PersistentPdfViewerSession::open(source, PdfViewerLimits::strict(), cache)?;
    let (page, hit) = session.render_page(PdfPageRenderRequest::new(0, 0.5))?;
    if hit != expected_hit || page.surface.rgba.is_empty() {
        return Err("registry consumer persistent artifact contract failed".into());
    }
    Ok(())
}
