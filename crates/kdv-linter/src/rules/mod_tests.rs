use super::*;
use crate::rules::test_helpers::FixtureWorkspace;

#[test]
fn rule_runner_collects_violations_from_all_rules() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    fixture.write_rust_file(
        "crates/katana-document-viewer/src/sample.rs",
        "pub fn bad() {\n    let _ = Some(1).unwrap();\n}\n",
    )?;
    let workspace = fixture.workspace()?;
    let violations = RuleRunner::check(&workspace)?;

    assert!(
        violations
            .iter()
            .any(|violation| violation.rule == "prohibited-method")
    );
    assert!(
        violations
            .iter()
            .any(|violation| violation.rule == "public-free-function")
    );
    Ok(())
}

#[test]
fn rule_runner_exposes_rule_chain() {
    let rules = RuleRunner::rules();

    assert_eq!(rules.len(), 15);
}
