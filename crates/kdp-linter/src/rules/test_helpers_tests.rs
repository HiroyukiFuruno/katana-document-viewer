use super::*;
use std::path::Path;

const NESTED_DIR: &str = "nested";

#[test]
fn unique_fixture_root_is_unique() {
    let first = unique_fixture_root();
    let second = unique_fixture_root();

    assert_ne!(first, second);
}

#[test]
fn write_file_reports_read_error_for_directory_target() -> Result<(), KdpLintError> {
    let root = unique_fixture_root();
    let target = root.join(NESTED_DIR);
    fs::create_dir_all(&target).map_err(|source| KdpLintError::Read {
        path: target.clone(),
        source,
    })?;

    let error = write_file(&target, "content");
    let path = match error {
        Ok(_) => return Err(KdpLintError::WorkspaceRoot { path: target }),
        Err(error) => match error {
            KdpLintError::Read { path, .. } => path,
            _ => return Err(KdpLintError::WorkspaceRoot { path: target }),
        },
    };

    assert_eq!(path, target);
    let _ = fs::remove_dir_all(&root);
    Ok(())
}

#[test]
fn write_file_handles_relative_parentless_path() {
    let target = Path::new("kdp-linter-parentless-write.txt");
    let result = write_file(target, "content");
    let _ = fs::remove_file(target);

    assert!(result.is_ok());
}

#[test]
fn write_file_handles_empty_path_without_parent() {
    let target = Path::new("");
    let error = write_file(target, "content");

    let path = match error {
        Ok(_) => return,
        Err(error) => match error {
            KdpLintError::Read { path, .. } => path,
            _ => return,
        },
    };

    assert_eq!(path, Path::new("").to_path_buf());
}

#[test]
fn ensure_directory_reports_error_if_parent_is_file() -> Result<(), KdpLintError> {
    let root = unique_fixture_root();
    fs::create_dir_all(&root).map_err(|source| KdpLintError::Read {
        path: root.clone(),
        source,
    })?;
    let file = root.join("blocking_file");
    fs::write(&file, "").map_err(|source| KdpLintError::Read {
        path: file.clone(),
        source,
    })?;

    let nested = file.join(NESTED_DIR);
    let error = ensure_directory(&nested);
    let path = match error {
        Ok(_) => return Err(KdpLintError::WorkspaceRoot { path: nested }),
        Err(error) => match error {
            KdpLintError::Read { path, .. } => path,
            _ => return Err(KdpLintError::WorkspaceRoot { path: nested }),
        },
    };

    assert_eq!(path, nested);
    let _ = fs::remove_dir_all(&root);
    Ok(())
}
