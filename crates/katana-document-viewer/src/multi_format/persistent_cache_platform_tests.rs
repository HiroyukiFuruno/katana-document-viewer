use super::{PersistentCacheError, PersistentDocumentCache, TestResult, cache};
use std::fs;

#[test]
fn unsafe_root_lock_and_artifact_symlinks_are_rejected() -> TestResult {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir()?;
    let target = tempfile::tempdir()?;
    symlink(target.path(), root.path().join("root-link"))?;
    assert!(matches!(
        PersistentDocumentCache::new(root.path().join("root-link"), 1024, "env".into()),
        Err(PersistentCacheError::UnsafeDirectory)
    ));
    let cache = PersistentDocumentCache::new(root.path().join("cache"), 1024, "env".into())?;
    symlink(
        target.path().join("missing"),
        cache.directory().join(".lock"),
    )?;
    assert!(matches!(
        cache.load("missing"),
        Err(PersistentCacheError::UnsafeDirectory)
    ));
    fs::remove_file(cache.directory().join(".lock"))?;
    symlink(target.path(), cache.directory().join("artifact"))?;
    assert!(matches!(
        cache.load("artifact"),
        Err(PersistentCacheError::UnsafeDirectory)
    ));
    assert!(matches!(
        cache.used_bytes(),
        Err(PersistentCacheError::UnsafeDirectory)
    ));
    Ok(())
}

#[test]
fn prepare_makes_cache_directory_private() -> TestResult {
    use std::os::unix::fs::PermissionsExt;
    let (_root, cache) = cache(1024)?;
    assert_eq!(
        0o700,
        fs::metadata(cache.directory())?.permissions().mode() & 0o777
    );
    Ok(())
}
