use std::ffi::OsString;
use std::path::Path;

pub(super) fn worker_environment(workspace: &Path) -> Vec<(OsString, OsString)> {
    // AppContainerの起動環境を製品と揃え、profile由来の変数も欠落させない。
    let parent_environment: Vec<_> = std::env::vars_os().collect();
    build_worker_environment(&parent_environment, workspace)
}

fn build_worker_environment(
    parent_environment: &[(OsString, OsString)],
    workspace: &Path,
) -> Vec<(OsString, OsString)> {
    let mut environment: Vec<_> = parent_environment
        .iter()
        .filter(|(name, _)| !is_worker_temp_variable(name))
        .cloned()
        .collect();
    let workspace = workspace.as_os_str().to_owned();
    environment.push((OsString::from("TEMP"), workspace.clone()));
    environment.push((OsString::from("TMP"), workspace));
    environment.sort_by(|left, right| {
        left.0
            .to_string_lossy()
            .to_ascii_lowercase()
            .cmp(&right.0.to_string_lossy().to_ascii_lowercase())
    });
    environment
}

fn is_worker_temp_variable(name: &std::ffi::OsStr) -> bool {
    name.eq_ignore_ascii_case("TEMP") || name.eq_ignore_ascii_case("TMP")
}

#[test]
fn worker_environment_preserves_essentials_and_workspace_temp() -> super::TestResult {
    let workspace = Path::new(r"C:\wrapped title test");
    let environment = worker_environment(workspace);
    let expected_count = std::env::vars_os()
        .filter(|(name, _)| !is_worker_temp_variable(name))
        .count()
        + 2;
    for key in ["SystemRoot", "windir", "ComSpec", "PATHEXT", "PATH"] {
        #[cfg(windows)]
        assert!(
            std::env::var_os(key).is_some(),
            "missing Windows key: {key}"
        );
        if let Some(expected) = std::env::var_os(key) {
            assert!(
                environment
                    .iter()
                    .any(|(name, value)| { name.eq_ignore_ascii_case(key) && value == &expected })
            );
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

#[test]
fn builder_preserves_parent_variables_and_replaces_mixed_case_temp_names() {
    let workspace = Path::new(r"C:\wrapped title test");
    let parent_environment = [
        (
            OsString::from("ArbitraryParent"),
            OsString::from("parent-value"),
        ),
        (
            OsString::from("ProfileSpecific"),
            OsString::from("profile-value"),
        ),
        (OsString::from("tEmP"), OsString::from("old-temp")),
        (OsString::from("tMp"), OsString::from("old-tmp")),
    ];

    let environment = build_worker_environment(&parent_environment, workspace);
    assert_eq!(environment.len(), 4);

    assert!(environment.contains(&(
        OsString::from("ArbitraryParent"),
        OsString::from("parent-value")
    )));
    assert!(environment.contains(&(
        OsString::from("ProfileSpecific"),
        OsString::from("profile-value")
    )));
    for key in ["TEMP", "TMP"] {
        assert_eq!(
            environment
                .iter()
                .filter(|(name, _)| name.eq_ignore_ascii_case(key))
                .count(),
            1
        );
        assert!(environment.contains(&(OsString::from(key), workspace.as_os_str().to_owned())));
    }
}
