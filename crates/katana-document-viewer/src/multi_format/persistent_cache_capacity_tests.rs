use super::{PersistentCacheError, PersistentDocumentCache};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn per_artifact_limit_applies_even_when_total_capacity_is_larger() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        256 * 1024 * 1024,
        "env-v1".into(),
    )?;
    std::fs::File::create(cache.directory().join("oversized"))?.set_len(128 * 1024 * 1024 + 1)?;
    assert!(matches!(
        cache.load("oversized"),
        Err(PersistentCacheError::Capacity)
    ));
    Ok(())
}

#[test]
fn load_rejects_a_cache_whose_total_size_exceeds_the_current_capacity() -> TestResult {
    let root = tempfile::tempdir()?;
    let path = root.path().join("cache");
    let payload = vec![b'x'; 10];
    let cache = PersistentDocumentCache::new(path.clone(), 256, "env-v1".into())?;
    cache.save("first", &payload)?;
    cache.save("second", &payload)?;
    let exact_capacity = cache.used_bytes()?;
    let exact = PersistentDocumentCache::new(path.clone(), exact_capacity, "env-v1".into())?;
    assert_eq!(Some(payload.clone()), exact.load("first")?);
    assert_eq!(Some(payload.clone()), exact.load("second")?);

    let reduced = PersistentDocumentCache::new(path, exact_capacity - 14, "env-v1".into())?;
    assert!(matches!(
        reduced.load("first"),
        Err(PersistentCacheError::Capacity)
    ));
    reduced.clear()?;
    assert_eq!(None, reduced.load("first")?);
    reduced.save("recovered", &payload)?;
    assert_eq!(Some(payload), reduced.load("recovered")?);
    Ok(())
}

#[test]
fn load_reports_capacity_for_a_missing_key_when_the_cache_is_over_capacity() -> TestResult {
    let root = tempfile::tempdir()?;
    let path = root.path().join("cache");
    let payload = vec![b'x'; 10];
    let cache = PersistentDocumentCache::new(path.clone(), 256, "env-v1".into())?;
    cache.save("first", &payload)?;
    cache.save("second", &payload)?;
    let reduced = PersistentDocumentCache::new(path, cache.used_bytes()? - 14, "env-v1".into())?;

    assert!(matches!(
        reduced.load("missing"),
        Err(PersistentCacheError::Capacity)
    ));
    Ok(())
}

#[test]
fn save_reuses_regular_files_but_rejects_existing_directories() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(root.path().join("cache"), 256, "env-v1".into())?;
    std::fs::write(cache.directory().join("regular"), b"existing")?;
    std::fs::create_dir(cache.directory().join("directory"))?;

    cache.save("regular", b"replacement")?;
    assert!(matches!(
        cache.save("directory", b"replacement"),
        Err(PersistentCacheError::UnsafeDirectory)
    ));
    assert_eq!(
        b"existing".as_slice(),
        std::fs::read(cache.directory().join("regular"))?.as_slice()
    );
    assert!(cache.directory().join("directory").is_dir());
    Ok(())
}

#[cfg(unix)]
#[test]
fn save_rejects_an_existing_unix_symlink() -> TestResult {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(root.path().join("cache"), 256, "env-v1".into())?;
    let target = cache.directory().join("target");
    let link = cache.directory().join("link");
    std::fs::write(&target, b"existing")?;
    symlink(&target, &link)?;

    assert!(matches!(
        cache.save("link", b"replacement"),
        Err(PersistentCacheError::UnsafeDirectory)
    ));
    assert_eq!(target, std::fs::read_link(link)?);
    assert_eq!(b"existing".as_slice(), std::fs::read(target)?.as_slice());
    Ok(())
}
