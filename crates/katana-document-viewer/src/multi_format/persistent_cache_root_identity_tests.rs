use super::{PersistentCacheError, PersistentDocumentCache};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn replaced_cache_root_rejects_operations_and_recovers_when_restored() -> TestResult {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("cache");
    let moved = temp.path().join("original-cache");
    let cache = PersistentDocumentCache::new(root.clone(), 1024, "root-identity-v1".into())?;
    cache.save("owned", b"original")?;
    let original = std::fs::read(root.join("owned"))?;
    std::fs::rename(&root, &moved)?;
    std::fs::create_dir(&root)?;
    std::fs::write(root.join("keep"), b"unrelated")?;

    assert_operations_reject_replaced_root(&cache);
    assert_eq!(b"unrelated", std::fs::read(root.join("keep"))?.as_slice());
    assert_eq!(original, std::fs::read(moved.join("owned"))?);
    assert!(!moved.join("new").exists());

    std::fs::remove_file(root.join("keep"))?;
    std::fs::remove_dir(&root)?;
    std::fs::rename(&moved, &root)?;
    assert_eq!(Some(b"original".to_vec()), cache.load("owned")?);
    cache.save("after-restore", b"works")?;
    cache.clear()?;
    assert_eq!(None, cache.load("owned")?);
    Ok(())
}

#[cfg(unix)]
#[test]
fn cache_root_replaced_by_symlink_rejects_operations() -> TestResult {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir()?;
    let unrelated = tempfile::tempdir()?;
    let root = temp.path().join("cache");
    let moved = temp.path().join("original-cache");
    let cache = PersistentDocumentCache::new(root.clone(), 1024, "root-identity-v1".into())?;
    cache.save("owned", b"original")?;
    let original = std::fs::read(root.join("owned"))?;
    std::fs::rename(&root, &moved)?;
    std::fs::write(unrelated.path().join("keep"), b"unrelated")?;
    symlink(unrelated.path(), &root)?;

    assert_operations_reject_replaced_root(&cache);
    assert_eq!(
        b"unrelated",
        std::fs::read(unrelated.path().join("keep"))?.as_slice()
    );
    assert_eq!(original, std::fs::read(moved.join("owned"))?);
    assert!(!moved.join("new").exists());
    Ok(())
}

fn assert_operations_reject_replaced_root(cache: &PersistentDocumentCache) {
    assert!(matches!(
        cache.load("owned"),
        Err(PersistentCacheError::UnsafeDirectory)
    ));
    assert!(matches!(
        cache.save("new", b"new"),
        Err(PersistentCacheError::UnsafeDirectory)
    ));
    assert!(matches!(
        cache.used_bytes(),
        Err(PersistentCacheError::UnsafeDirectory)
    ));
    assert!(matches!(
        cache.clear(),
        Err(PersistentCacheError::UnsafeDirectory)
    ));
}
