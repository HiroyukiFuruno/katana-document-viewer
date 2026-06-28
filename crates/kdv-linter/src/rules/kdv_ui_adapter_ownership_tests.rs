use super::KdvUiAdapterOwnershipRule;
use crate::diagnostics::KdvLintError;
use crate::rules::test_helpers::FixtureWorkspace;

#[test]
fn flags_kdv_owned_kuc_adapter_crate() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    fixture.write_rust_file(
        "crates/katana-document-viewer-kuc/src/lib.rs",
        "pub struct KdvOwnedAdapter;",
    )?;

    let violations = KdvUiAdapterOwnershipRule::check(&fixture.workspace()?)?;

    assert!(has_rule(&violations));
    Ok(())
}

#[test]
fn flags_storybook_kuc_renderer_and_hit_wrappers() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new();
    fixture.write_rust_file(
        "tools/kdv-storybook/src/mouse_host_action.rs",
        r#"
use katana_document_viewer_kuc::KucMediaControlAction;
use crate::frame_kuc_renderer::kuc_tree_host_action_hits_at;
use katana_ui_core_storybook::{
    UiTreeCanvasRenderer, UiTreeInteractionSurface, UiTreeStorybookHost,
};

fn route() {
    let _ = UiTreeStorybookHost::new(theme);
    let _ = UiTreeHostActionHitQuery::default();
    let _ = host_action_hit_rects();
    let _ = kuc_tree_host_action_hits_at(root, area, x, y, dark);
}
"#,
    )?;

    let violations = KdvUiAdapterOwnershipRule::check(&fixture.workspace()?)?;

    assert_storybook_source_violations(&violations, 10);
    Ok(())
}

#[test]
fn allows_storybook_host_usage_in_test_only_files() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new();
    fixture.write_rust_file(
        "tools/kdv-storybook/src/frame_score_preview_crop_tests.rs",
        r#"
use katana_ui_core_storybook::UiTreeStorybookHost;

fn render_reference() {
    let _ = UiTreeStorybookHost::new(theme);
}
"#,
    )?;

    let violations = KdvUiAdapterOwnershipRule::check(&fixture.workspace()?)?;

    assert!(violations.is_empty());
    Ok(())
}

#[test]
fn flags_storybook_owned_kuc_bridge_module() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new();
    fixture.write_rust_file(
        "tools/kdv-storybook/src/kuc_bridge/mod.rs",
        "pub struct StorybookOwnedBridge;",
    )?;

    let violations = KdvUiAdapterOwnershipRule::check(&fixture.workspace()?)?;

    assert!(violations.iter().any(|violation| {
        violation.rule == "no_kdv_ui_adapter_ownership"
            && violation
                .file
                .to_string_lossy()
                .contains("tools/kdv-storybook/src/kuc_bridge/mod.rs")
    }));
    Ok(())
}

#[test]
fn allows_plain_kdv_engine_model() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new();
    fixture.write_rust_file(
        "crates/katana-document-viewer/src/viewer/document.rs",
        "pub struct ViewerDocument;",
    )?;

    let violations = KdvUiAdapterOwnershipRule::check(&fixture.workspace()?)?;

    assert!(violations.is_empty());
    Ok(())
}

fn has_rule(violations: &[crate::diagnostics::Violation]) -> bool {
    violations
        .iter()
        .any(|violation| violation.rule == "no_kdv_ui_adapter_ownership")
}

fn assert_storybook_source_violations(
    violations: &[crate::diagnostics::Violation],
    expected: usize,
) {
    assert_eq!(expected, violations.len());
    assert!(violations.iter().all(|violation| {
        violation.rule == "no_kdv_ui_adapter_ownership"
            && violation
                .file
                .to_string_lossy()
                .contains("tools/kdv-storybook/src")
    }));
}
