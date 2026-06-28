use super::super::manifest_boundary::ManifestBoundaryRule;
use super::super::test_helpers::FixtureWorkspace;
use super::*;

const VIEWER_WITH_EGUI_DEP: &str = r#"
[package]
name = "katana-document-viewer"
version = "0.1.0"
edition = "2021"

[dependencies]
egui = { version = "0.29" }
"#;

#[test]
fn architecture_rule_collects_preview_boundary() -> Result<(), KdvLintError> {
    let fixture = architecture_fixture(Some(VIEWER_WITH_EGUI_DEP))?;
    let architecture = ArchitectureRule::check(&fixture.workspace()?)?;

    assert!(has_violation(&architecture, "preview-boundary"));
    Ok(())
}

#[test]
fn architecture_rule_passes_default_manifests() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let workspace = fixture.workspace()?;
    let violations = ArchitectureRule::check(&workspace)?;

    assert!(violations.is_empty());
    Ok(())
}

#[test]
fn architecture_rule_calls_manifest_boundaries() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    fixture.write_manifest(
        "crates/katana-document-viewer/Cargo.toml",
        VIEWER_WITH_EGUI_DEP,
    )?;
    let workspace = fixture.workspace()?;

    let architecture = ArchitectureRule::check(&workspace)?;
    let manifest = ManifestBoundaryRule::check(workspace.root())?;

    assert!(
        manifest
            .iter()
            .any(|violation| violation.rule == "preview-boundary")
    );
    assert!(
        architecture
            .iter()
            .any(|violation| violation.rule == "preview-boundary")
    );
    Ok(())
}

fn architecture_fixture(viewer_manifest: Option<&str>) -> Result<FixtureWorkspace, KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    if let Some(manifest) = viewer_manifest {
        fixture.write_manifest("crates/katana-document-viewer/Cargo.toml", manifest)?;
    }
    Ok(fixture)
}

fn has_violation(violations: &[Violation], rule: &str) -> bool {
    violations.iter().any(|violation| violation.rule == rule)
}
