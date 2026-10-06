use super::{PersistentCacheError, PersistentDocumentCache, key::CacheKey};
use crate::multi_format::{BinaryDocumentSource, ViewerSourceIdentity};
use std::fs;
type TestResult = Result<(), Box<dyn std::error::Error>>;
fn cache(
    max_bytes: u64,
) -> Result<(tempfile::TempDir, PersistentDocumentCache), PersistentCacheError> {
    let root = tempfile::tempdir().map_err(PersistentCacheError::Io)?;
    let cache =
        PersistentDocumentCache::new(root.path().join("cache"), max_bytes, "env-v1".into())?;
    Ok((root, cache))
}
fn source(uri: &str, revision: &str, mime: &str, bytes: &[u8]) -> BinaryDocumentSource {
    BinaryDocumentSource::new(
        ViewerSourceIdentity::new(uri, revision),
        mime,
        bytes.to_vec(),
    )
}
#[test]
fn new_rejects_zero_capacity_and_empty_environment() -> TestResult {
    let root = tempfile::tempdir()?;
    assert!(matches!(
        PersistentDocumentCache::new(root.path().join("zero"), 0, "env".into()),
        Err(PersistentCacheError::Capacity)
    ));
    assert!(matches!(
        PersistentDocumentCache::new(root.path().join("empty"), 1, String::new()),
        Err(PersistentCacheError::InvalidEnvironmentRevision)
    ));
    Ok(())
}
#[test]
fn store_round_trip_missing_clear_and_atomic_cleanup() -> TestResult {
    let (_root, cache) = cache(1024)?;
    assert_eq!(None, cache.load("missing")?);
    cache.save("artifact", b"payload")?;
    assert_eq!(Some(b"payload".to_vec()), cache.load("artifact")?);
    assert_eq!(
        fs::metadata(cache.directory().join("artifact"))?.len(),
        cache.used_bytes()?
    );
    assert!(fs::read_dir(cache.directory())?.all(|entry| {
        entry
            .map(|entry| entry.file_name() == ".lock" || entry.file_name() == "artifact")
            .unwrap_or(false)
    }));
    cache.clear()?;
    assert_eq!(0, cache.used_bytes()?);
    assert_eq!(None, cache.load("artifact")?);
    Ok(())
}
#[test]
fn corrupt_magic_and_checksum_are_rejected() -> TestResult {
    let (_root, cache) = cache(1024)?;
    cache.save("artifact", b"payload")?;
    let path = cache.directory().join("artifact");
    let mut bytes = fs::read(&path)?;
    bytes[0] ^= 1;
    fs::write(&path, &bytes)?;
    assert!(matches!(
        cache.load("artifact"),
        Err(PersistentCacheError::Corrupt)
    ));
    cache.clear()?;
    cache.save("artifact", b"payload")?;
    let mut bytes = fs::read(&path)?;
    *bytes.last_mut().ok_or("payload byte missing")? ^= 1;
    fs::write(path, bytes)?;
    assert!(matches!(
        cache.load("artifact"),
        Err(PersistentCacheError::Corrupt)
    ));
    Ok(())
}
#[test]
fn artifact_checksum_is_bound_to_cache_key() -> TestResult {
    let (_root, cache) = cache(1024)?;
    cache.save("first", b"payload")?;
    fs::copy(
        cache.directory().join("first"),
        cache.directory().join("second"),
    )?;
    assert!(matches!(
        cache.load("second"),
        Err(PersistentCacheError::Corrupt)
    ));
    Ok(())
}
#[test]
fn capacity_counts_header_and_existing_entries() -> TestResult {
    let (_root, cache) = cache(73)?;
    cache.save("first", b"x")?;
    assert_eq!(73, cache.used_bytes()?);
    assert!(matches!(
        cache.save("second", b"x"),
        Err(PersistentCacheError::Capacity)
    ));
    cache.save("first", b"a larger replacement is ignored")?;
    assert_eq!(Some(b"x".to_vec()), cache.load("first")?);
    Ok(())
}
#[test]
fn malformed_and_oversized_entries_report_distinct_errors() -> TestResult {
    let (_root, cache) = cache(128)?;
    fs::write(cache.directory().join("short"), [1_u8])?;
    assert!(matches!(
        cache.load("short"),
        Err(PersistentCacheError::Corrupt)
    ));
    fs::write(cache.directory().join("large"), vec![0_u8; 129])?;
    assert!(matches!(
        cache.load("large"),
        Err(PersistentCacheError::Capacity)
    ));
    Ok(())
}
#[test]
fn keys_include_document_environment_and_render_inputs() -> TestResult {
    let (_root, cache) = cache(1024)?;
    let base = source("file:///a.pdf", "r1", "application/pdf", b"pdf");
    let changed_mime = source("file:///a.pdf", "r1", "application/octet-stream", b"pdf");
    let changed_bytes = source("file:///a.pdf", "r1", "application/pdf", b"pdf2");
    assert_ne!(
        CacheKey::pdf(&cache, &base),
        CacheKey::pdf(&cache, &changed_mime)
    );
    let changed_key = CacheKey::pdf(&cache, &changed_bytes);
    assert_ne!(CacheKey::pdf(&cache, &base), changed_key);
    let page = CacheKey::page("document", 0, 1.0, "strict");
    assert_ne!(page, CacheKey::page("document", 1, 1.0, "strict"));
    assert_ne!(page, CacheKey::page("document", 0, 1.1, "strict"));
    assert_ne!(page, CacheKey::page("document", 0, 1.0, "relaxed"));
    let other_env =
        PersistentDocumentCache::new(cache.directory().to_path_buf(), 1024, "env-v2".into())?;
    assert_ne!(
        CacheKey::pdf(&cache, &base),
        CacheKey::pdf(&other_env, &base)
    );
    Ok(())
}
#[test]
fn office_key_includes_worker_bytes_and_configuration() -> TestResult {
    let (_root, cache) = cache(1024)?;
    let executable = tempfile::NamedTempFile::new()?;
    fs::write(executable.path(), b"worker-v1")?;
    let source = super::super::OfficeDocumentSource::new(
        ViewerSourceIdentity::new("file:///a.docx", "r1"),
        super::super::OfficeDocumentFormat::Docx,
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        b"docx".to_vec(),
    );
    let config = super::super::OfficeWorkerConfig::new(executable.path().to_path_buf());
    let first = CacheKey::office(&cache, &source, &config)?;
    let mut changed = config.clone();
    changed.max_output_bytes -= 1;
    assert_ne!(first, CacheKey::office(&cache, &source, &changed)?);
    fs::write(executable.path(), b"worker-v2")?;
    assert_ne!(first, CacheKey::office(&cache, &source, &config)?);
    Ok(())
}
#[cfg(unix)]
#[path = "persistent_cache_platform_tests.rs"]
mod platform;

#[test]
fn metadata_io_failure_is_reported() -> TestResult {
    let (_root, cache) = cache(1024)?;
    let invalid_file_name = "a".repeat(1024);
    assert!(matches!(
        cache.load(&invalid_file_name),
        Err(PersistentCacheError::Io(_))
    ));
    assert!(matches!(
        cache.save(&invalid_file_name, b"payload"),
        Err(PersistentCacheError::Io(_))
    ));
    Ok(())
}
