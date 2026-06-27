use kdv_linter::{KdvLintError, KdvLinter, ViolationReport};
use std::path::{Path, PathBuf};

fn workspace_root() -> Result<PathBuf, KdvLintError> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let Some(crates_dir) = manifest_dir.parent() else {
        return Err(KdvLintError::WorkspaceRoot {
            path: manifest_dir.to_path_buf(),
        });
    };
    let Some(root) = crates_dir.parent() else {
        return Err(KdvLintError::WorkspaceRoot {
            path: crates_dir.to_path_buf(),
        });
    };
    Ok(root.to_path_buf())
}

#[test]
fn ast_linter_workspace_rules() -> Result<(), KdvLintError> {
    let root = workspace_root()?;
    let violations = KdvLinter::lint_workspace(&root)?;
    assert!(
        violations.is_empty(),
        "{}",
        ViolationReport::format(&violations)
    );
    Ok(())
}
