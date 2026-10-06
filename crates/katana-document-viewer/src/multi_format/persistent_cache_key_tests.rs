use super::CacheKey;
use std::path::Path;

#[test]
fn explicit_executable_paths_bind_to_the_current_directory()
-> Result<(), Box<dyn std::error::Error>> {
    let current = std::env::current_exe()?;
    assert_eq!(current, CacheKey::resolve_executable(&current)?);
    let relative = Path::new("./Cargo.toml");
    let resolved = CacheKey::resolve_executable(relative)?;
    assert_eq!(std::env::current_dir()?.join(relative), resolved);
    assert_eq!(
        CacheKey::executable_digest(relative)?,
        CacheKey::executable_digest(&resolved)?
    );
    Ok(())
}

#[test]
fn bare_executable_names_resolve_from_path() -> Result<(), Box<dyn std::error::Error>> {
    let resolved = CacheKey::resolve_executable(Path::new("rustc"))?;
    assert!(resolved.is_absolute());
    assert!(resolved.is_file());
    let output = std::process::Command::new(&resolved)
        .arg("--version")
        .output()?;
    assert!(output.status.success());
    assert!(!CacheKey::executable_digest(&resolved)?.is_empty());
    Ok(())
}

#[test]
fn unavailable_path_worker_is_an_explicit_not_found_error() {
    let error = CacheKey::resolve_executable(Path::new("kdv-nonexistent-worker-7f016568"));
    assert!(matches!(error, Err(error) if error.kind() == std::io::ErrorKind::NotFound));
}
