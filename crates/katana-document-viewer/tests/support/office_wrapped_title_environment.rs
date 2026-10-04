use std::ffi::OsString;
use std::path::Path;

pub(super) fn worker_environment(workspace: &Path) -> Vec<(OsString, OsString)> {
    let temporary_paths = ["TEMP", "TMP"]
        .into_iter()
        .map(|name| (OsString::from(name), workspace.as_os_str().to_owned()))
        .collect();
    // opts.envは自動補完されないため、必須変数の補完を明示しWin32順序へ並べる。
    let mut environment = rappct::launch::merge_parent_env(temporary_paths);
    environment.sort_by(|left, right| {
        left.0
            .to_string_lossy()
            .to_ascii_lowercase()
            .cmp(&right.0.to_string_lossy().to_ascii_lowercase())
    });
    environment
}

#[test]
fn worker_environment_preserves_essentials_and_workspace_temp() -> super::TestResult {
    let workspace = Path::new(r"C:\wrapped title test");
    let environment = worker_environment(workspace);
    let mut expected_count = 2;
    for key in ["SystemRoot", "windir", "ComSpec", "PATHEXT", "PATH"] {
        #[cfg(windows)]
        assert!(
            std::env::var_os(key).is_some(),
            "missing Windows key: {key}"
        );
        if let Some(expected) = std::env::var_os(key) {
            assert!(environment.contains(&(OsString::from(key), expected)));
            expected_count += 1;
        }
    }
    for key in ["TEMP", "TMP"] {
        assert!(environment.contains(&(OsString::from(key), workspace.as_os_str().to_owned())));
    }
    let names: Vec<_> = environment
        .iter()
        .map(|(name, _)| name.to_string_lossy().to_ascii_lowercase())
        .collect();
    assert!(names.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(environment.len(), expected_count);
    Ok(())
}
