use super::PersistentDocumentCache;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn relative_cache_root_remains_bound_when_child_process_changes_directory() -> TestResult {
    let output = std::process::Command::new(std::env::current_exe()?)
        .args([
            "--exact",
            "multi_format::persistent_cache::root_tests::relative_root_process_step",
            "--ignored",
            "--nocapture",
        ])
        .output()?;
    assert!(output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stdout).contains("1 passed"));
    Ok(())
}

#[test]
#[ignore = "cwd mutation is isolated in the real child process"]
fn relative_root_process_step() -> TestResult {
    let initial = std::env::current_dir()?;
    let original = tempfile::tempdir()?;
    let other = tempfile::tempdir()?;
    let unrelated = other.path().join("cache");
    std::fs::create_dir(&unrelated)?;
    std::fs::write(unrelated.join("keep"), b"unrelated")?;
    std::env::set_current_dir(original.path())?;
    let cache = PersistentDocumentCache::new("cache".into(), 256, "env".into())?;
    let expected = std::fs::canonicalize(original.path().join("cache"))?;
    assert!(cache.directory().is_absolute());
    assert_eq!(expected, std::fs::canonicalize(cache.directory())?);
    cache.save("owned", b"original")?;
    std::env::set_current_dir(other.path())?;
    assert_eq!(Some(b"original".to_vec()), cache.load("owned")?);
    cache.save("second", b"original-second")?;
    cache.clear()?;
    assert_eq!(None, cache.load("owned")?);
    assert!(!unrelated.join("second").exists());
    assert_eq!(
        b"unrelated",
        std::fs::read(unrelated.join("keep"))?.as_slice()
    );
    std::env::set_current_dir(initial)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn cache_root_stays_bound_after_ancestor_symlink_is_retargeted() -> TestResult {
    use std::os::unix::fs::symlink;

    let original = tempfile::tempdir()?;
    let unrelated = tempfile::tempdir()?;
    let (link, original_root, redirected_root, second_parent) =
        symlinked_cache_roots(original.path(), unrelated.path())?;

    let cache = PersistentDocumentCache::new(link.join("cache"), 256, "env".into())?;
    cache.save("before", b"original")?;
    std::fs::remove_file(&link)?;
    symlink(&second_parent, &link)?;

    assert_eq!(Some(b"original".to_vec()), cache.load("before")?);
    cache.save("after", b"still-original")?;
    assert!(original_root.join("after").is_file());
    assert!(!redirected_root.join("after").exists());
    cache.clear()?;
    assert_eq!(None, cache.load("before")?);
    assert_eq!(None, cache.load("after")?);
    assert_eq!(
        b"unrelated",
        std::fs::read(redirected_root.join("keep"))?.as_slice()
    );
    assert!(redirected_root.join("keep").is_file());
    Ok(())
}

#[cfg(unix)]
fn symlinked_cache_roots(
    original: &std::path::Path,
    unrelated: &std::path::Path,
) -> TestResult<(
    std::path::PathBuf,
    std::path::PathBuf,
    std::path::PathBuf,
    std::path::PathBuf,
)> {
    use std::os::unix::fs::symlink;

    let link = original.join("cache-parent");
    let first_parent = original.join("first");
    let second_parent = unrelated.join("second");
    std::fs::create_dir(&first_parent)?;
    std::fs::create_dir(&second_parent)?;
    let original_root = first_parent.join("cache");
    let redirected_root = second_parent.join("cache");
    std::fs::create_dir(&redirected_root)?;
    std::fs::write(redirected_root.join("keep"), b"unrelated")?;
    symlink(&first_parent, &link)?;
    Ok((link, original_root, redirected_root, second_parent))
}
