use super::super::architecture::{EGUI_CRATE, VIEWER_CRATE};
use super::*;
use crate::rules::test_helpers::FixtureWorkspace;
use std::path::Path;
use toml::Value;

#[test]
fn manifest_boundary_check_rejects_ui_dependency() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let viewer_manifest = r#"
[package]
name = "katana-document-viewer"
version = "0.1.0"
edition = "2021"

[dependencies]
egui = "0.29"
katana-document-preview-egui = { path = "../katana-document-preview-egui" }
"#;
    fixture.write_manifest(&format!("{VIEWER_CRATE}/Cargo.toml"), viewer_manifest)?;

    let violations = ManifestBoundaryRule::check(&fixture.root)?;

    assert!(
        violations
            .iter()
            .any(|violation| violation.message.contains("UI crate `egui`"))
    );
    Ok(())
}

#[test]
fn manifest_boundary_check_egui_manifest_requires_preview_dependency() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let egui_manifest = r#"
[package]
name = "katana-document-preview-egui"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = "1"
"#;
    fixture.write_manifest(&format!("{EGUI_CRATE}/Cargo.toml"), egui_manifest)?;
    let violations = ManifestBoundaryRule::check(&fixture.root)?;

    assert!(
        violations
            .iter()
            .any(|violation| violation.rule == "egui-library-boundary")
    );
    Ok(())
}

#[test]
fn manifest_boundary_check_egui_manifest_passes_with_dependency() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let egui_manifest = r#"
[package]
name = "katana-document-preview-egui"
version = "0.1.0"
edition = "2021"

[dependencies]
katana-document-preview = { path = "../katana-document-preview" }
"#;
    fixture.write_manifest(&format!("{EGUI_CRATE}/Cargo.toml"), egui_manifest)?;
    let violations = ManifestBoundaryRule::check(&fixture.root)?;

    assert!(
        !violations
            .iter()
            .any(|violation| violation.rule == "egui-library-boundary")
    );
    Ok(())
}

#[test]
fn manifest_boundary_check_neutral_manifest_collects_package_dependency() -> Result<(), KdpLintError>
{
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let viewer_manifest = r#"
[package]
name = "katana-document-viewer"
version = "0.1.0"
edition = "2021"

[dependencies]
alias-ui = { package = "egui", version = "0.29" }
"#;
    fixture.write_manifest("crates/katana-document-viewer/Cargo.toml", viewer_manifest)?;

    let violations = ManifestBoundaryRule::check(&fixture.root)?;
    let has_preview_boundary = violations
        .iter()
        .any(|violation| violation.rule == "preview-boundary");

    assert!(has_preview_boundary);
    Ok(())
}

#[test]
fn manifest_boundary_check_accepts_non_ui_dependency() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let viewer_manifest = r#"
[package]
name = "katana-document-viewer"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1"
"#;
    fixture.write_manifest("crates/katana-document-viewer/Cargo.toml", viewer_manifest)?;
    let violations = ManifestBoundaryRule::check(&fixture.root)?;
    let has_preview_boundary = violations
        .iter()
        .any(|violation| violation.rule == "preview-boundary");

    assert!(!has_preview_boundary);
    Ok(())
}

#[test]
fn manifest_reader_collects_dependencies_from_all_tables_and_package_names()
-> Result<(), KdpLintError> {
    let fixture = fixture_with_dependency_tables()?;
    let dependencies = manifest_dependencies(&fixture.root)?;

    assert!(dependencies.iter().any(|name| name == "egui"));
    assert!(dependencies.iter().any(|name| name == "floem"));
    assert!(dependencies.iter().any(|name| name == "builder"));
    Ok(())
}

fn fixture_with_dependency_tables() -> Result<FixtureWorkspace, KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let manifest = r#"
[package]
name = "katana-document-viewer"
version = "0.1.0"
edition = "2021"

[dependencies]
egui = "0.29"

[dev-dependencies]
helper = { package = "floem", version = "0.1" }

[build-dependencies]
builder = "1.0"
"#;
    fixture.write_manifest("crates/katana-document-viewer/Cargo.toml", manifest)?;
    Ok(fixture)
}

fn manifest_dependencies(root: &Path) -> Result<Vec<String>, KdpLintError> {
    let data = manifest_reader_fixture(root)?;
    Ok(ManifestReader::dependency_names(&data))
}

fn manifest_reader_fixture(root: &Path) -> Result<Value, KdpLintError> {
    ManifestReader::read(&root.join("crates/katana-document-viewer/Cargo.toml"))
}

#[test]
fn manifest_reader_reports_toml_parse_error() -> Result<(), KdpLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    let invalid_manifest = r#"
[package]
name = "katana-document-viewer"
version = "0.1.0"
edition = "2021"

[dependencies]
egui = ["
"#;
    fixture.write_manifest("crates/katana-document-viewer/Cargo.toml", invalid_manifest)?;

    assert!(ManifestBoundaryRule::check(&fixture.root).is_err());
    Ok(())
}
