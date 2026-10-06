use super::PersistentDocumentCache;

type TestResult = Result<(), Box<dyn std::error::Error>>;

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
