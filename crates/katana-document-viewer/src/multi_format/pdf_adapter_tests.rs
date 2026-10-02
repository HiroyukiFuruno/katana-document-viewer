use super::{
    BinaryDocumentSource, PdfPageRenderRequest, PdfViewerError, PdfViewerSession, map_load_error,
};
use crate::ViewerSourceIdentity;
use hayro::hayro_syntax::{DecryptionError, LoadPdfError};

#[test]
fn load_errors_map_to_typed_pdf_semantics() {
    assert_eq!(
        PdfViewerError::PasswordProtected,
        map_load_error(LoadPdfError::Decryption(DecryptionError::PasswordProtected))
    );
    assert_eq!(
        PdfViewerError::InvalidDocument,
        map_load_error(LoadPdfError::Invalid)
    );
}

#[test]
fn session_accessors_and_cache_are_covered_in_the_library_target() -> Result<(), PdfViewerError> {
    let bytes = include_bytes!("../../../../assets/reference/katana/pdf/sample.pdf").to_vec();
    let source = BinaryDocumentSource::new(
        ViewerSourceIdentity::new("file:///unit.pdf", "sha256:unit"),
        "application/pdf",
        bytes,
    );
    let mut session = PdfViewerSession::open(source)?;

    assert_eq!(13, session.artifact().page_count);
    assert_eq!(0, session.cached_page_count());
    assert_eq!(0, session.cached_byte_count());
    let page = session.render_page(PdfPageRenderRequest::new(0, 1.0))?;
    assert_eq!(1, session.cached_page_count());
    assert_eq!(page.surface.rgba.len(), session.cached_byte_count());
    Ok(())
}

#[test]
fn worker_raster_reuses_the_bounded_cache_and_preserves_validation_errors()
-> Result<(), Box<dyn std::error::Error>> {
    let bytes = include_bytes!("../../../../assets/reference/katana/pdf/sample.pdf").to_vec();
    let source = BinaryDocumentSource::new(
        ViewerSourceIdentity::new("fixture:cache", "immutable"),
        "application/pdf",
        bytes,
    );
    let mut session = PdfViewerSession::open(source)?;
    let config = super::super::OfficeWorkerConfig::new(std::path::PathBuf::from(
        "/missing/kdv-office-worker",
    ));
    let request = PdfPageRenderRequest::new(0, 1.0);
    let cached = session.render_page(request)?;
    assert_eq!(cached, session.render_page_with_worker(request, &config)?);
    assert_eq!(
        Err(super::super::OfficeWorkerError::Pdf(
            PdfViewerError::InvalidScale
        )),
        session.render_page_with_worker(PdfPageRenderRequest::new(0, f32::NAN), &config)
    );
    assert!(matches!(
        session.render_page_with_worker(PdfPageRenderRequest::new(1, 1.0), &config),
        Err(super::super::OfficeWorkerError::WorkerUnavailable { .. })
    ));
    assert_eq!(1, session.cached_page_count());
    Ok(())
}
