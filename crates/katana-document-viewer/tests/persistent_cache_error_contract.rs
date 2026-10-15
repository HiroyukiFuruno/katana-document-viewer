use katana_document_viewer::{
    OfficeDocumentFormat, OfficeDocumentSource, OfficeWorkerConfig, OfficeWorkerError,
    PersistentCacheError, PersistentDocumentCache, PersistentOfficeViewerSession,
    ViewerSourceIdentity,
};
use sha2::{Digest, Sha256};
use std::path::Path;
type TestResult = Result<(), Box<dyn std::error::Error>>;
#[path = "support/persistent_outline.rs"]
mod outline_fixture;

#[test]
fn empty_environment_revision_is_a_configuration_error() -> TestResult {
    let root = tempfile::tempdir()?;
    assert!(matches!(
        PersistentDocumentCache::new(root.path().join("cache"), 1024, String::new()),
        Err(PersistentCacheError::InvalidEnvironmentRevision)
    ));
    Ok(())
}

#[test]
fn persistent_pdf_preserves_source_outline() -> TestResult {
    use katana_document_viewer::{
        BinaryDocumentSource, PdfViewerLimits, PdfViewerSession, PersistentPdfViewerSession,
    };
    let source = BinaryDocumentSource::new(
        ViewerSourceIdentity::new("fixture:outline.pdf", "one"),
        "application/pdf",
        outline_fixture::bytes(),
    );
    let expected = PdfViewerSession::open(source.clone())?;
    assert_eq!(expected.outline().len(), 1);
    assert_eq!(expected.outline()[0].title, "Chapter");
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(root.path().join("cache"), 1024, "one".into())?;
    let session = PersistentPdfViewerSession::open(source, PdfViewerLimits::strict(), cache)?;
    assert_eq!(expected.outline(), session.outline());
    Ok(())
}

fn source(format: OfficeDocumentFormat) -> OfficeDocumentSource {
    OfficeDocumentSource::new(
        ViewerSourceIdentity::new("fixture:cache-error", "one"),
        format,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        include_bytes!("../../../assets/fixtures/multi-format/representative.docx").to_vec(),
    )
}

fn config() -> OfficeWorkerConfig {
    OfficeWorkerConfig::new(env!("CARGO_BIN_EXE_kdv-office-worker").into())
}

#[test]
fn public_persistent_office_rejects_xlsx() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(root.path().join("cache"), 1024, "one".into())?;
    assert!(matches!(
        PersistentOfficeViewerSession::open(source(OfficeDocumentFormat::Xlsx), config(), cache),
        Err(PersistentCacheError::Office(
            OfficeWorkerError::UnsupportedFormat(OfficeDocumentFormat::Xlsx)
        ))
    ));
    Ok(())
}

#[test]
fn restored_conversion_cannot_exceed_worker_output_limit() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache =
        PersistentDocumentCache::new(root.path().join("cache"), 64 * 1024 * 1024, "one".into())?;
    let mut config = config();
    config.max_output_bytes = 4 * 1024 * 1024;
    let session = PersistentOfficeViewerSession::open(
        source(OfficeDocumentFormat::Docx),
        config.clone(),
        cache.clone(),
    )?;
    assert!(!session.conversion_cache_hit());
    drop(session);
    enlarge_conversion(&cache, config.max_output_bytes as usize + 1)?;
    assert!(matches!(
        PersistentOfficeViewerSession::open(source(OfficeDocumentFormat::Docx), config, cache),
        Err(PersistentCacheError::Capacity)
    ));
    Ok(())
}

fn enlarge_conversion(cache: &PersistentDocumentCache, length: usize) -> TestResult {
    let path = conversion_path(cache)?;
    let bytes = std::fs::read(&path)?;
    let mut payload = bytes[72..].to_vec();
    let metadata_len = usize::try_from(u64::from_le_bytes(payload[..8].try_into()?))?;
    let metadata: serde_json::Value = serde_json::from_slice(&payload[8..8 + metadata_len])?;
    assert_eq!(metadata["pdf"], serde_json::json!([]));
    payload.resize(8 + metadata_len + length, 0);
    write_bound_payload(&path, &bytes[..8], &payload)
}

fn conversion_path(
    cache: &PersistentDocumentCache,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let entries = std::fs::read_dir(cache.directory())?.collect::<Result<Vec<_>, _>>()?;
    Ok(entries
        .into_iter()
        .find(|entry| entry.file_name() != ".lock")
        .ok_or("conversion artifact missing")?
        .path())
}

#[test]
fn truncated_conversion_header_is_corrupt() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache =
        PersistentDocumentCache::new(root.path().join("cache"), 64 * 1024 * 1024, "one".into())?;
    let session = PersistentOfficeViewerSession::open(
        source(OfficeDocumentFormat::Docx),
        config(),
        cache.clone(),
    )?;
    drop(session);
    std::fs::write(conversion_path(&cache)?, [1_u8])?;
    assert!(matches!(
        PersistentOfficeViewerSession::open(source(OfficeDocumentFormat::Docx), config(), cache),
        Err(PersistentCacheError::Corrupt)
    ));
    Ok(())
}

fn write_bound_payload(path: &Path, magic: &[u8], payload: &[u8]) -> TestResult {
    // checksum破損とは独立して、復元PDFの出力上限が適用されることを検証する。
    let key = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("artifact key missing")?;
    let mut digest = Sha256::new();
    digest.update(key.as_bytes());
    digest.update(payload);
    let checksum: String = digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    std::fs::write(path, [magic, checksum.as_bytes(), payload].concat())?;
    Ok(())
}
