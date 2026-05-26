use super::super::egui_duplication::EguiDuplicationRule;
use super::super::manifest_boundary::ManifestBoundaryRule;
use super::super::test_helpers::FixtureWorkspace;
use super::*;

const EGUI_PREVIEW_WITH_PREVIEW_DEP: &str = r#"
[package]
name = "katana-document-preview-egui"
version = "0.1.0"
edition = "2021"

[dependencies]
katana-document-preview = { path = "../katana-document-preview" }
"#;

const VIEWER_WITH_EGUI_DEP: &str = r#"
[package]
name = "katana-document-viewer"
version = "0.1.0"
edition = "2021"

[dependencies]
egui = { version = "0.29" }
"#;

#[test]
fn architecture_rule_collects_preview_boundary_without_egui_boundary() -> Result<(), KdpLintError> {
    let fixture = architecture_fixture(
        Some(VIEWER_WITH_EGUI_DEP),
        Some(EGUI_PREVIEW_WITH_PREVIEW_DEP),
    )?;
    let architecture = ArchitectureRule::check(&fixture.workspace()?)?;

    assert!(has_violation(&architecture, "preview-boundary"));
    assert!(!has_violation(&architecture, "egui-library-boundary"));
    assert!(!has_violation(&architecture, "egui-preview-duplication"));
    Ok(())
}

#[test]
fn architecture_rule_collects_egui_duplication_without_boundaries() -> Result<(), KdpLintError> {
    let fixture = architecture_fixture(None, Some(EGUI_PREVIEW_WITH_PREVIEW_DEP))?;

    fixture.write_rust_file(
        "crates/katana-document-preview-egui/src/lib.rs",
        "pub struct MarkdownPreview {}",
    )?;
    let architecture = ArchitectureRule::check(&fixture.workspace()?)?;

    assert!(has_violation(&architecture, "egui-preview-duplication"));
    assert!(!has_violation(&architecture, "egui-library-boundary"));
    assert!(!has_violation(&architecture, "preview-boundary"));
    Ok(())
}

#[test]
fn architecture_rule_combines_boundary_checks() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let workspace = fixture.workspace()?;
    let violations = ArchitectureRule::check(&workspace)?;

    let has_preview_boundary = violations
        .iter()
        .any(|violation| violation.rule == "preview-boundary");
    let has_library_boundary = violations
        .iter()
        .any(|violation| violation.rule == "egui-library-boundary");
    let has_preview_duplication = violations
        .iter()
        .any(|violation| violation.rule == "egui-preview-duplication");
    assert!(has_preview_boundary || has_library_boundary || has_preview_duplication);
    Ok(())
}

#[test]
fn architecture_rule_collects_egui_violations() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    fixture.write_rust_file(
        "crates/katana-document-preview-egui/src/lib.rs",
        "pub struct MarkdownPreview {}",
    )?;
    let workspace = fixture.workspace()?;
    let violations = EguiDuplicationRule::check(&workspace);
    let architecture = ArchitectureRule::check(&workspace)?;

    let has_duplication = violations
        .iter()
        .any(|violation| violation.rule == "egui-preview-duplication");
    assert!(has_duplication);
    assert!(
        architecture
            .iter()
            .any(|violation| violation.rule == "egui-preview-duplication")
    );
    Ok(())
}

#[test]
fn architecture_rule_calls_manifest_boundaries() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let viewer_manifest = r#"
[package]
name = "katana-document-viewer"
version = "0.1.0"
edition = "2021"

[dependencies]
egui = "0.29"
"#;
    fixture.write_manifest("crates/katana-document-viewer/Cargo.toml", viewer_manifest)?;
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

fn architecture_fixture(
    viewer_manifest: Option<&str>,
    egui_manifest: Option<&str>,
) -> Result<FixtureWorkspace, KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    if let Some(manifest) = viewer_manifest {
        fixture.write_manifest("crates/katana-document-viewer/Cargo.toml", manifest)?;
    }
    if let Some(manifest) = egui_manifest {
        fixture.write_manifest("crates/katana-document-preview-egui/Cargo.toml", manifest)?;
    }
    Ok(fixture)
}

fn has_violation(violations: &[Violation], rule: &str) -> bool {
    violations.iter().any(|violation| violation.rule == rule)
}
