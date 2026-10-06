use super::{PersistentDocumentCache, key::CacheKey};
use crate::{ViewerSourceIdentity, multi_format::BinaryDocumentSource};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn rebuilt_engine_image_invalidates_artifacts_with_the_same_package_and_environment() -> TestResult
{
    let root = tempfile::tempdir()?;
    let image = root.path().join("engine");
    std::fs::write(&image, vec![b'a'; 65537])?;
    let mut cache = PersistentDocumentCache::new(root.path().join("cache"), 256, "env".into())?;
    cache.engine_revision = CacheKey::executable_digest(&image)?;
    let source = BinaryDocumentSource::new(
        ViewerSourceIdentity::new("fixture:engine", "source-v1"),
        "application/pdf",
        b"unchanged-input".to_vec(),
    );
    let original = CacheKey::pdf(&cache, &source);
    cache.save(&original, b"old-renderer-output")?;
    std::fs::write(&image, vec![b'b'; 65537])?;
    cache.engine_revision = CacheKey::executable_digest(&image)?;
    let rebuilt = CacheKey::pdf(&cache, &source);
    assert_ne!(original, rebuilt);
    assert_eq!(None, cache.load(&rebuilt)?);
    assert_eq!(
        Some(b"old-renderer-output".to_vec()),
        cache.load(&original)?
    );
    Ok(())
}

#[test]
fn process_engine_revision_is_reused_and_image_io_failures_remain_explicit() -> TestResult {
    let first = CacheKey::engine_revision()?;
    assert_eq!(64, first.len());
    assert_eq!(first, CacheKey::engine_revision()?);
    let root = tempfile::tempdir()?;
    assert!(CacheKey::executable_digest(&root.path().join("missing")).is_err());
    Ok(())
}
