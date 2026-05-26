use super::*;
use crate::rules::test_helpers::FixtureWorkspace;
use std::path::Path;

#[test]
fn file_length_rule_skips_short_files() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    fixture.write_rust_file(
        "crates/katana-document-viewer/src/short.rs",
        "fn short() {}\n",
    )?;
    fixture.write_rust_file(
        "crates/katana-document-viewer/src/short2.rs",
        &"fn x() {}\n".repeat(199),
    )?;

    let workspace = fixture.workspace()?;
    let violations = FileLengthRule::check(&workspace)?;
    let has_file_violation = violations
        .iter()
        .any(|violation| violation.rule == "file-length");

    assert!(!has_file_violation);
    Ok(())
}

#[test]
fn file_length_rule_reports_long_file() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    fixture.write_rust_file(
        "crates/katana-document-viewer/src/too_long.rs",
        &"fn x() {}\n".repeat(201),
    )?;

    let workspace = fixture.workspace()?;
    let violations = FileLengthRule::check(&workspace)?;
    let long_target =
        Path::new(&fixture.root).join("crates/katana-document-viewer/src/too_long.rs");
    let long_violation = violations
        .iter()
        .find(|violation| violation.file == long_target);
    let violation = long_violation.ok_or_else(|| KdpLintError::WorkspaceRoot {
        path: fixture.root.clone(),
    })?;
    assert_eq!(violation.rule, "file-length");
    Ok(())
}

#[test]
fn violation_line_column_are_reported_from_file_root() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    fixture.write_rust_file(
        "crates/katana-document-viewer/src/too_long.rs",
        &"fn x() {}\n".repeat(201),
    )?;

    let workspace = fixture.workspace()?;
    let violations = FileLengthRule::check(&workspace)?;
    let long_violation = violations
        .iter()
        .find(|violation| violation.file.ends_with(Path::new("too_long.rs")));
    let first = long_violation.ok_or(KdpLintError::WorkspaceRoot {
        path: fixture.root.clone(),
    })?;

    assert_eq!(first.line, 1);
    assert_eq!(first.column, 1);
    Ok(())
}

#[test]
fn file_length_rule_tolerates_exact_limit() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    fixture.write_rust_file(
        "crates/katana-document-viewer/src/exact_limit.rs",
        &"fn x() {}\n".repeat(MAX_FILE_LINES),
    )?;

    let workspace = fixture.workspace()?;
    let violations = FileLengthRule::check(&workspace)?;
    assert!(
        !violations
            .iter()
            .any(|violation| violation.file.ends_with(Path::new("exact_limit.rs")))
    );
    Ok(())
}
