use super::PersistentPdfViewerSession;
use crate::multi_format::persistent_cache::key::CacheKey;
use crate::multi_format::{
    BinaryDocumentSource, PdfPageRenderRequest, PdfViewerLimits, PersistentCacheError,
    PersistentDocumentCache, ViewerSourceIdentity,
};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn pdf_failed_persistence_does_not_leave_a_live_page_hit() -> TestResult {
    let root = tempfile::tempdir()?;
    let request = PdfPageRenderRequest::new(0, 1.0);
    let entry_bytes = rendered_pdf_entry_bytes(root.path(), request)?;
    assert!(entry_bytes > 1);
    let cache =
        PersistentDocumentCache::new(root.path().join("cache"), entry_bytes, "live-v1".into())?;
    std::fs::write(cache.directory().join("filler"), b"x")?;
    let mut session =
        PersistentPdfViewerSession::open(pdf_source(), PdfViewerLimits::strict(), cache.clone())?;
    assert_capacity_errors_twice(&mut session, request);
    cache.clear()?;
    assert!(!session.render_page(request)?.1);
    let mut restarted =
        PersistentPdfViewerSession::open(pdf_source(), PdfViewerLimits::strict(), cache)?;
    assert!(restarted.render_page(request)?.1);
    Ok(())
}

fn rendered_pdf_entry_bytes(
    root: &std::path::Path,
    request: PdfPageRenderRequest,
) -> TestResult<u64> {
    let cache = PersistentDocumentCache::new(
        root.join("sizing-cache"),
        16 * 1024 * 1024,
        "live-v1".into(),
    )?;
    let mut session =
        PersistentPdfViewerSession::open(pdf_source(), PdfViewerLimits::strict(), cache.clone())?;
    session.render_page(request)?;
    let key = CacheKey::page(
        &session.key,
        request.page_index,
        request.scale,
        &format!("{:?}", PdfViewerLimits::strict()),
    );
    Ok(std::fs::metadata(cache.directory().join(key))?.len())
}

fn assert_capacity_errors_twice(
    session: &mut PersistentPdfViewerSession,
    request: PdfPageRenderRequest,
) {
    for _ in 0..2 {
        assert!(matches!(
            session.render_page(request),
            Err(PersistentCacheError::Capacity)
        ));
    }
}

fn pdf_source() -> BinaryDocumentSource {
    BinaryDocumentSource::new(
        ViewerSourceIdentity::new("fixture:live.pdf", "sha256:live"),
        "application/pdf",
        include_bytes!("../../../../assets/reference/katana/pdf/sample.pdf").to_vec(),
    )
}
