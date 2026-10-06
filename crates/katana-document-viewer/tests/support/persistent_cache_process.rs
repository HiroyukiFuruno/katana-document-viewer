use katana_document_viewer::{
    BinaryDocumentSource, DocumentResourceSnapshot, OfficeDocumentFormat, OfficeDocumentSource,
    OfficeWorkerConfig, PdfPageRenderRequest, PdfViewerLimits, PersistentCacheError,
    PersistentDocumentCache, PersistentOfficeViewerSession, PersistentPdfViewerSession,
    ViewerSourceIdentity,
};
use std::path::{Path, PathBuf};
use std::time::Instant;
type TestResult = Result<(), Box<dyn std::error::Error>>;
const PDF: &[u8] = include_bytes!("../../../../assets/reference/katana/pdf/sample.pdf");
const EXTERNAL_INPUT_ENV: &str = "KDV_CACHE_INPUT";

pub fn run() -> TestResult {
    let root = PathBuf::from(std::env::var("KDV_CACHE_ROOT")?);
    let revision = std::env::var("KDV_CACHE_REVISION")?;
    let expected_hit = std::env::var("KDV_CACHE_EXPECT_HIT")? == "true";
    let format = std::env::var("KDV_CACHE_FORMAT")?;
    let cache_start = Instant::now();
    let cache = PersistentDocumentCache::new(root, 128 * 1024 * 1024, "font-config-v1".into())?;
    println!(
        "KDV_CACHE_INITIALIZE format={format} elapsed_seconds={}",
        cache_start.elapsed().as_secs_f64()
    );
    for (mode, hit) in [("process_open", expected_hit), ("warm_reopen", true)] {
        let start = Instant::now();
        let acquired = Instant::now();
        let source = input(&format, &revision)?;
        let acquire_seconds = acquired.elapsed().as_secs_f64();
        match source {
            Input::Pdf(source) => pdf_step(source, cache.clone(), hit)?,
            Input::Office(source) => office_step(source, cache.clone(), hit)?,
        }
        assert_eq!(
            DocumentResourceSnapshot::default(),
            DocumentResourceSnapshot::capture()
        );
        println!(
            "KDV_CACHE_MEASURE format={format} mode={mode} expected_hit={hit} acquire_seconds={acquire_seconds} first_frame_seconds={}",
            start.elapsed().as_secs_f64()
        );
    }
    Ok(())
}

enum Input {
    Pdf(BinaryDocumentSource),
    Office(OfficeDocumentSource),
}

fn input(format: &str, revision: &str) -> Result<Input, Box<dyn std::error::Error>> {
    let identity = ViewerSourceIdentity::new(format!("fixture:{format}"), revision);
    if format == "pdf" {
        return Ok(Input::Pdf(BinaryDocumentSource::new(
            identity,
            "application/pdf",
            input_bytes(format)?,
        )));
    }
    let office_format = if format == "docx" {
        OfficeDocumentFormat::Docx
    } else {
        OfficeDocumentFormat::Pptx
    };
    let mime = if format == "docx" {
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    } else {
        "application/vnd.openxmlformats-officedocument.presentationml.presentation"
    };
    Ok(Input::Office(OfficeDocumentSource::new(
        identity,
        office_format,
        mime,
        input_bytes(format)?,
    )))
}

fn input_bytes(format: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if let Some(path) = std::env::var_os(EXTERNAL_INPUT_ENV) {
        return std::fs::read(path).map_err(|error| {
            format!("failed to read {EXTERNAL_INPUT_ENV} for {format}: {error}").into()
        });
    }
    let path = if format == "pdf" {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/reference/katana/pdf/sample.pdf")
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../assets/fixtures/multi-format")
            .join(format!("representative.{format}"))
    };
    Ok(std::fs::read(path)?)
}

fn pdf_step(source: BinaryDocumentSource, cache: PersistentDocumentCache, hit: bool) -> TestResult {
    let mut session = PersistentPdfViewerSession::open(source, PdfViewerLimits::strict(), cache)?;
    assert!(session.artifact().page_count > 0);
    let (first, actual_hit) = session.render_page(PdfPageRenderRequest::new(0, 0.5))?;
    assert_eq!(hit, actual_hit);
    assert!(!first.surface.rgba.is_empty());
    let (second, repeated_hit) = session.render_page(PdfPageRenderRequest::new(0, 0.5))?;
    assert!(repeated_hit);
    assert_eq!(first, second);
    Ok(())
}

fn office_step(
    source: OfficeDocumentSource,
    cache: PersistentDocumentCache,
    hit: bool,
) -> TestResult {
    let config = OfficeWorkerConfig::new(PathBuf::from(env!("CARGO_BIN_EXE_kdv-office-worker")));
    let mut session = PersistentOfficeViewerSession::open(source, config, cache)?;
    assert_eq!(hit, session.conversion_cache_hit());
    assert!(session.artifact().item_count > 0);
    let (first, actual_hit) = session.render_item(PdfPageRenderRequest::new(0, 0.5))?;
    assert_eq!(hit, actual_hit);
    assert!(!first.surface.rgba.is_empty());
    let (second, repeated_hit) = session.render_item(PdfPageRenderRequest::new(0, 0.5))?;
    assert!(repeated_hit);
    assert_eq!(first, second);
    Ok(())
}

pub fn storage_contract() -> TestResult {
    let root = tempfile::tempdir()?;
    let source = BinaryDocumentSource::new(
        ViewerSourceIdentity::new("fixture:pdf", "one"),
        "application/pdf",
        PDF.to_vec(),
    );
    let small = PersistentDocumentCache::new(root.path().join("small"), 1, "v1".into())?;
    let mut session =
        PersistentPdfViewerSession::open(source.clone(), PdfViewerLimits::strict(), small)?;
    assert!(matches!(
        session.render_page(PdfPageRenderRequest::new(0, 0.5)),
        Err(PersistentCacheError::Capacity)
    ));
    let cache =
        PersistentDocumentCache::new(root.path().join("normal"), 128 * 1024 * 1024, "v1".into())?;
    let mut session =
        PersistentPdfViewerSession::open(source.clone(), PdfViewerLimits::strict(), cache.clone())?;
    let (_, hit) = session.render_page(PdfPageRenderRequest::new(0, 0.5))?;
    assert!(!hit);
    drop(session);
    corrupt_artifact(&cache)?;
    let mut session =
        PersistentPdfViewerSession::open(source, PdfViewerLimits::strict(), cache.clone())?;
    assert!(matches!(
        session.render_page(PdfPageRenderRequest::new(0, 0.5)),
        Err(PersistentCacheError::Corrupt)
    ));
    cache.clear()?;
    assert_eq!(0, cache.used_bytes()?);
    let (_, hit) = session.render_page(PdfPageRenderRequest::new(0, 0.5))?;
    assert!(!hit);
    invalidation_contract(&cache)?;
    Ok(())
}

fn invalidation_contract(cache: &PersistentDocumentCache) -> TestResult {
    let mut bytes = PDF.to_vec();
    bytes.push(b'\n');
    let source = BinaryDocumentSource::new(
        ViewerSourceIdentity::new("fixture:pdf", "one"),
        "application/pdf",
        bytes,
    );
    let mut changed =
        PersistentPdfViewerSession::open(source.clone(), PdfViewerLimits::strict(), cache.clone())?;
    assert!(!changed.render_page(PdfPageRenderRequest::new(0, 0.5))?.1);
    assert!(!changed.render_page(PdfPageRenderRequest::new(0, 0.6))?.1);
    let environment = PersistentDocumentCache::new(
        cache.directory().to_path_buf(),
        128 * 1024 * 1024,
        "v2".into(),
    )?;
    let mut session =
        PersistentPdfViewerSession::open(source.clone(), PdfViewerLimits::strict(), environment)?;
    assert!(!session.render_page(PdfPageRenderRequest::new(0, 0.5))?.1);
    let mut limits = PdfViewerLimits::strict();
    limits.max_render_dimension -= 1;
    let mut session = PersistentPdfViewerSession::open(source, limits, cache.clone())?;
    assert!(!session.render_page(PdfPageRenderRequest::new(0, 0.5))?.1);
    Ok(())
}

fn corrupt_artifact(cache: &PersistentDocumentCache) -> TestResult {
    let entry = std::fs::read_dir(cache.directory())?
        .filter_map(Result::ok)
        .find(|entry| entry.file_name() != ".lock")
        .ok_or("missing cache artifact")?;
    let mut bytes = std::fs::read(entry.path())?;
    let last = bytes.last_mut().ok_or("empty cache artifact")?;
    *last ^= 1;
    std::fs::write(entry.path(), bytes)?;
    Ok(())
}
