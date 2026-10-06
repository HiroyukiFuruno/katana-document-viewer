use super::PersistentPdfViewerSession;
use crate::ViewerSourceIdentity;
use crate::multi_format::{BinaryDocumentSource, PdfViewerLimits, PersistentDocumentCache};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn persistent_pdf_accessors_expose_the_decoded_artifact_and_outline() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        128 * 1024 * 1024,
        "accessor-tests".to_owned(),
    )?;
    let identity = ViewerSourceIdentity::new("fixture:pdf-accessors", "sha256:accessors");
    let source = BinaryDocumentSource::new(
        identity.clone(),
        "application/pdf",
        include_bytes!("../../../../assets/reference/katana/pdf/sample.pdf").to_vec(),
    );
    let persistent = PersistentPdfViewerSession::open(source, PdfViewerLimits::strict(), cache)?;

    assert_eq!(identity, persistent.artifact().identity);
    assert_eq!(13, persistent.artifact().page_count);
    assert_eq!(persistent.session.outline(), persistent.outline());
    Ok(())
}
