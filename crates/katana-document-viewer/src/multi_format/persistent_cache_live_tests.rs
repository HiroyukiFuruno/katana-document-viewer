use super::{CachedPage, PersistentPdfViewerSession};
use crate::multi_format::persistent_cache::key::CacheKey;
use crate::multi_format::{
    BinaryDocumentSource, OfficeDocumentFormat, OfficeDocumentSource, OfficeStaticViewerSession,
    OfficeWorkerConfig, PdfPageRenderRequest, PdfViewerLimits, PersistentDocumentCache,
    ViewerSourceIdentity,
};
use std::path::PathBuf;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn pdf_memory_cache_survives_disk_clear() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        128 * 1024 * 1024,
        "live-v1".into(),
    )?;
    let source = pdf_source();
    let request = PdfPageRenderRequest::new(0, 1.0);
    let mut session =
        PersistentPdfViewerSession::open(source, PdfViewerLimits::strict(), cache.clone())?;
    let (page, disk_hit) = session.render_page(request)?;
    assert!(!disk_hit);
    cache.clear()?;
    assert_eq!((page, true), session.render_page(request)?);
    assert!(
        cache
            .load(&CacheKey::page(
                &session.key,
                0,
                1.0,
                &format!("{:?}", PdfViewerLimits::strict())
            ))?
            .is_none()
    );

    Ok(())
}

#[test]
fn pdf_zero_capacity_bypasses_memory_and_repopulates_disk() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        128 * 1024 * 1024,
        "live-v1".into(),
    )?;
    let mut limits = PdfViewerLimits::strict();
    limits.max_cached_pages = 0;
    let request = PdfPageRenderRequest::new(0, 1.0);
    let mut session = PersistentPdfViewerSession::open(pdf_source(), limits, cache.clone())?;
    cache.clear()?;
    assert!(!session.render_page(request)?.1);
    assert!(session.render_page(request)?.1);
    Ok(())
}

#[test]
fn disk_restored_pdf_page_is_promoted_to_live_cache() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        128 * 1024 * 1024,
        "live-v1".into(),
    )?;
    let request = PdfPageRenderRequest::new(0, 1.0);
    let mut first =
        PersistentPdfViewerSession::open(pdf_source(), PdfViewerLimits::strict(), cache.clone())?;
    first.render_page(request)?;
    let mut restored =
        PersistentPdfViewerSession::open(pdf_source(), PdfViewerLimits::strict(), cache.clone())?;
    assert!(restored.render_page(request)?.1);
    cache.clear()?;
    assert!(restored.render_page(request)?.1);
    Ok(())
}

#[test]
fn office_disk_page_hit_survives_disk_clear_without_worker() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        128 * 1024 * 1024,
        "live-v1".into(),
    )?;
    let request = PdfPageRenderRequest::new(0, 1.0);
    let source = office_source();
    let config = OfficeWorkerConfig::new(PathBuf::from("worker"));
    let mut session = office_session(&cache, &source, &config)?;
    let page = office_page(source, config, request)?;
    let key = CacheKey::page(
        "office-live",
        0,
        1.0,
        &format!("{:?}", PdfViewerLimits::strict()),
    );
    CachedPage::save(&cache, &key, &page)?;
    assert!(session.render_item(request)?.1);
    cache.clear()?;
    assert!(session.render_item(request)?.1);
    Ok(())
}

fn office_session(
    cache: &PersistentDocumentCache,
    source: &OfficeDocumentSource,
    config: &OfficeWorkerConfig,
) -> Result<super::PersistentOfficeViewerSession, Box<dyn std::error::Error>> {
    let conversion_key =
        crate::multi_format::office_conversion_key::OfficeConversionKey::new(source, config);
    let office_session = OfficeStaticViewerSession::from_output(
        source.clone(),
        config.clone(),
        crate::multi_format::office_worker_parent::OfficeWorkerOutput {
            pdf: sample_pdf(),
            warnings: Vec::new(),
            preflight_diagnostics: Vec::new(),
        },
        conversion_key,
        None,
    )?;
    Ok(super::PersistentOfficeViewerSession {
        session: office_session,
        cache: cache.clone(),
        key: "office-live".into(),
        conversion_cache_hit: true,
    })
}

fn office_page(
    source: OfficeDocumentSource,
    config: OfficeWorkerConfig,
    request: PdfPageRenderRequest,
) -> Result<crate::multi_format::PdfRenderedPage, Box<dyn std::error::Error>> {
    let conversion_key =
        crate::multi_format::office_conversion_key::OfficeConversionKey::new(&source, &config);
    let mut preview = OfficeStaticViewerSession::from_output(
        source,
        config,
        crate::multi_format::office_worker_parent::OfficeWorkerOutput {
            pdf: sample_pdf(),
            warnings: Vec::new(),
            preflight_diagnostics: Vec::new(),
        },
        conversion_key,
        None,
    )?;
    Ok(preview.render_item(request)?)
}

fn pdf_source() -> BinaryDocumentSource {
    BinaryDocumentSource::new(
        ViewerSourceIdentity::new("fixture:live.pdf", "sha256:live"),
        "application/pdf",
        sample_pdf(),
    )
}

fn office_source() -> OfficeDocumentSource {
    OfficeDocumentSource::new(
        ViewerSourceIdentity::new("fixture:live.docx", "sha256:live"),
        OfficeDocumentFormat::Docx,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        include_bytes!("../../../../assets/fixtures/multi-format/representative.docx").to_vec(),
    )
}

fn sample_pdf() -> Vec<u8> {
    include_bytes!("../../../../assets/reference/katana/pdf/sample.pdf").to_vec()
}
