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

#[cfg(target_os = "linux")]
#[test]
fn replaced_running_engine_keeps_its_original_fingerprint() -> TestResult {
    let root = tempfile::tempdir()?;
    let image = root.path().join("running-engine");
    std::fs::hard_link(std::env::current_exe()?, &image)?;
    let output = std::process::Command::new(&image)
        .args([
            "--exact",
            "multi_format::persistent_cache::engine_tests::replaced_engine_process_step",
            "--ignored",
            "--nocapture",
        ])
        .env("KDV_REPLACED_ENGINE", &image)
        .output()?;
    assert!(output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stdout).contains("1 passed"));
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
#[ignore = "real child-process entrypoint invoked by the replacement regression"]
fn replaced_engine_process_step() -> TestResult {
    let image = std::path::PathBuf::from(std::env::var_os("KDV_REPLACED_ENGINE").ok_or("image")?);
    let original = CacheKey::executable_digest(&image)?;
    let replacement = image.with_extension("replacement");
    std::fs::write(&replacement, b"new-package-image")?;
    std::fs::rename(&replacement, &image)?;
    assert!(std::fs::File::open(std::env::current_exe()?).is_err());
    assert_ne!(original, CacheKey::executable_digest(&image)?);
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(root.path().join("cache"), 256, "env".into())?;
    assert_eq!(original, cache.engine_revision);
    assert_eq!(original, CacheKey::engine_revision()?);
    Ok(())
}
