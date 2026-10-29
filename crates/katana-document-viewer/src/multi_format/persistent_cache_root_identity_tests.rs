#[cfg(unix)]
use super::PersistentCacheError;
use super::PersistentDocumentCache;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[cfg(unix)]
#[test]
fn replaced_cache_root_uses_original_directory_for_locked_operations() -> TestResult {
    let (_temp, root, moved, cache) = prepared_cache()?;
    let lock = cache.lock()?;
    std::fs::rename(&root, &moved)?;
    std::fs::create_dir(&root)?;
    std::fs::write(root.join("keep"), b"replacement")?;

    exercise_locked_operations(&cache)?;
    assert_replacement_preserved(&root)?;
    assert_original_cache_cleared(&moved)?;
    drop(lock);
    assert_operations_reject_root(&cache);
    std::fs::remove_file(root.join("keep"))?;
    std::fs::remove_dir(&root)?;
    std::fs::rename(&moved, &root)?;
    cache.save("restored", b"works")?;
    assert_eq!(Some(b"works".to_vec()), cache.load("restored")?);
    Ok(())
}

#[cfg(unix)]
fn exercise_locked_operations(cache: &PersistentDocumentCache) -> TestResult {
    assert_eq!(Some(b"original".to_vec()), cache.load_locked("owned")?);
    cache.save_locked("new", b"original-new")?;
    assert!(cache.used_bytes_locked()? > 0);
    cache.clear_locked()?;
    Ok(())
}

#[cfg(unix)]
fn assert_replacement_preserved(root: &std::path::Path) -> TestResult {
    assert_eq!(b"replacement", std::fs::read(root.join("keep"))?.as_slice());
    assert!(!root.join("owned").exists());
    assert!(!root.join("new").exists());
    Ok(())
}

#[cfg(unix)]
fn assert_original_cache_cleared(moved: &std::path::Path) -> TestResult {
    assert!(!moved.join("owned").exists());
    assert!(!moved.join("new").exists());
    assert!(moved.join(".lock").exists());
    Ok(())
}

#[cfg(unix)]
#[test]
fn symlink_replacement_is_rejected_at_public_entry_points() -> TestResult {
    use std::os::unix::fs::symlink;

    let (_temp, root, moved, cache) = prepared_cache()?;
    let unrelated = tempfile::tempdir()?;
    std::fs::rename(&root, &moved)?;
    std::fs::write(unrelated.path().join("keep"), b"unrelated")?;
    symlink(unrelated.path(), &root)?;

    assert_operations_reject_root(&cache);
    assert_eq!(
        b"unrelated",
        std::fs::read(unrelated.path().join("keep"))?.as_slice()
    );
    assert!(moved.join("owned").exists());
    Ok(())
}

#[cfg(windows)]
#[test]
fn open_cache_directory_prevents_root_rename_and_cache_stays_usable() -> TestResult {
    let (_temp, root, moved, cache) = prepared_cache()?;
    assert!(std::fs::rename(&root, &moved).is_err());
    assert_eq!(Some(b"original".to_vec()), cache.load("owned")?);
    cache.save("new", b"new")?;
    assert!(cache.used_bytes()? > 0);
    cache.clear()?;
    assert_eq!(None, cache.load("owned")?);
    Ok(())
}

fn prepared_cache() -> TestResult<(
    tempfile::TempDir,
    std::path::PathBuf,
    std::path::PathBuf,
    PersistentDocumentCache,
)> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("cache");
    let moved = temp.path().join("original-cache");
    let cache = PersistentDocumentCache::new(root.clone(), 1024, "root-identity-v1".into())?;
    cache.save("owned", b"original")?;
    Ok((temp, root, moved, cache))
}

#[cfg(unix)]
fn assert_operations_reject_root(cache: &PersistentDocumentCache) {
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

#[test]
fn anchored_publication_never_clobbers_existing_entry_and_cleans_temporary() -> TestResult {
    let (_temp, root, _moved, cache) = prepared_cache()?;
    let before = std::fs::read(root.join("owned"))?;
    let count = std::fs::read_dir(&root)?.count();
    let result = super::temporary::publish(
        cache.root_identity.directory(),
        "owned",
        b"replacement",
        b"KDVC0001",
    );
    assert!(matches!(result, Err(super::PersistentCacheError::Io(error))
        if error.kind() == std::io::ErrorKind::AlreadyExists));
    assert_eq!(before, std::fs::read(root.join("owned"))?);
    assert_eq!(count, std::fs::read_dir(&root)?.count());
    Ok(())
}

#[test]
fn unwritable_lock_reports_io_without_mutating_entries() -> TestResult {
    let (_temp, root, _moved, cache) = prepared_cache()?;
    let lock = root.join(".lock");
    let original = std::fs::metadata(&lock)?.permissions();
    let mut readonly = original.clone();
    readonly.set_readonly(true);
    std::fs::set_permissions(&lock, readonly)?;
    let result = cache.load("owned");
    std::fs::set_permissions(&lock, original)?;
    assert!(matches!(result, Err(super::PersistentCacheError::Io(error))
        if error.kind() == std::io::ErrorKind::PermissionDenied));
    assert_eq!(Some(b"original".to_vec()), cache.load("owned")?);
    Ok(())
}

#[cfg(unix)]
#[test]
fn unwritable_directory_rejects_temporary_creation_without_residue() -> TestResult {
    use std::os::unix::fs::PermissionsExt;
    let (_temp, root, _moved, cache) = prepared_cache()?;
    let original = std::fs::metadata(&root)?.permissions();
    let count = std::fs::read_dir(&root)?.count();
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o500))?;
    let result = cache.save("denied", b"new");
    std::fs::set_permissions(&root, original)?;
    assert!(matches!(result, Err(super::PersistentCacheError::Io(error))
        if error.kind() == std::io::ErrorKind::PermissionDenied));
    assert_eq!(count, std::fs::read_dir(&root)?.count());
    assert_eq!(None, cache.load("denied")?);
    Ok(())
}
