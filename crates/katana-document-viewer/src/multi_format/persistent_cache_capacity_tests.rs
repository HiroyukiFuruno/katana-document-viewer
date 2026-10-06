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
