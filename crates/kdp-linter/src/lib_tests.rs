use super::*;
use crate::rules::test_helpers::FixtureWorkspace;

#[test]
fn lint_workspace_returns_rule_violations_for_valid_root() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let violations = KdpLinter::lint_workspace(&fixture.root)?;

    assert!(
        violations
            .iter()
            .any(|violation| violation.rule == "egui-library-boundary")
    );
    Ok(())
}

#[test]
fn lint_workspace_returns_error_for_missing_root() -> Result<(), KdpLintError> {
    let missing = std::env::temp_dir().join("kdp-linter-missing-root-fixture");
    let result = KdpLinter::lint_workspace(&missing);
    assert!(result.is_err());
    Ok(())
}
