use super::OpenSpecIntegrityRule;
use crate::diagnostics::KdvLintError;
use crate::rules::test_helpers::FixtureWorkspace;

#[test]
fn openspec_integrity_rejects_completed_fake_four_vendor_dod() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    write_required_openspec_files(
        &fixture,
        "- [x] Storybook確認としてadapter planを4実runtime表示として扱う。証跡: `rtk just storybook-check`\n",
        "adapter planだけでは不可\n",
        "adapter planだけでは不可\n",
    )?;

    let violations = OpenSpecIntegrityRule::check(&fixture.root)?;

    assert!(has_rule(&violations, "openspec-fake-vendor-dod"));
    Ok(())
}

#[test]
fn openspec_integrity_accepts_explicit_real_runtime_wording() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    write_required_openspec_files(
        &fixture,
        "- [ ] Storybook確認として4実runtimeで開ける。adapter planだけでは不可\n",
        "release DoDでは4つの実runtimeで開く。adapter plan一致だけでは完了にしない\n",
        "- **THEN** Storybook gateは4実runtime coverageを検証する\n",
    )?;

    let violations = OpenSpecIntegrityRule::check(&fixture.root)?;

    assert!(!has_rule(&violations, "openspec-fake-vendor-dod"));
    assert!(!has_rule(&violations, "openspec-fake-vendor-wording"));
    Ok(())
}

#[test]
fn openspec_integrity_rejects_egui_route_selector_runtime_claim() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    write_required_openspec_files(
        &fixture,
        "- [ ] Storybook確認は未完了\n",
        "egui route selectorを4実runtime表示として扱う\n",
        "- **THEN** route selectorを4実runtimeで開いた扱いにする\n",
    )?;

    let violations = OpenSpecIntegrityRule::check(&fixture.root)?;

    assert!(has_rule(&violations, "openspec-fake-vendor-wording"));
    Ok(())
}

#[test]
fn openspec_integrity_accepts_negative_route_selector_wording() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    write_required_openspec_files(
        &fixture,
        "- [ ] Storybook確認は未完了\n",
        "egui route selectorを4実runtime表示として扱っていた誤りを修正する\n",
        "- **THEN** route selectorだけでは4実runtimeで開いた扱いにしない\n",
    )?;

    let violations = OpenSpecIntegrityRule::check(&fixture.root)?;

    assert!(!has_rule(&violations, "openspec-fake-vendor-wording"));
    Ok(())
}

#[test]
fn openspec_integrity_rejects_high_risk_completed_item_without_evidence() -> Result<(), KdvLintError>
{
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    write_required_openspec_files(
        &fixture,
        "- [x] Storybook確認として実runtimeで検証する\n",
        "adapter planだけでは不可\n",
        "adapter planだけでは不可\n",
    )?;

    let violations = OpenSpecIntegrityRule::check(&fixture.root)?;

    assert!(has_rule(&violations, "openspec-missing-evidence"));
    Ok(())
}

#[test]
fn openspec_integrity_rejects_high_risk_completed_item_with_empty_evidence()
-> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    write_required_openspec_files(
        &fixture,
        "- [x] Storybook確認を検証する。証跡: 作業済み\n",
        "adapter planだけでは不可\n",
        "adapter planだけでは不可\n",
    )?;

    let violations = OpenSpecIntegrityRule::check(&fixture.root)?;

    assert!(has_rule(&violations, "openspec-missing-evidence"));
    Ok(())
}

#[test]
fn openspec_integrity_accepts_high_risk_completed_item_with_command_evidence()
-> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    write_required_openspec_files(
        &fixture,
        "- [x] Storybook確認を検証する。証跡: `rtk just storybook-check`\n",
        "adapter planだけでは不可\n",
        "adapter planだけでは不可\n",
    )?;

    let violations = OpenSpecIntegrityRule::check(&fixture.root)?;

    assert!(!has_rule(&violations, "openspec-missing-evidence"));
    Ok(())
}

#[test]
fn openspec_integrity_accepts_high_risk_completed_item_with_issue_url() -> Result<(), KdvLintError>
{
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    write_required_openspec_files(
        &fixture,
        "- [x] 互換性を検証する。証跡: https://github.com/HiroyukiFuruno/katana-document-viewer/issues/6\n",
        "adapter planだけでは不可\n",
        "adapter planだけでは不可\n",
    )?;

    let violations = OpenSpecIntegrityRule::check(&fixture.root)?;

    assert!(!has_rule(&violations, "openspec-missing-evidence"));
    Ok(())
}

#[test]
fn openspec_integrity_rejects_completed_user_feedback_without_evidence() -> Result<(), KdvLintError>
{
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    write_required_openspec_files(
        &fixture,
        "- [/] Storybookの偽完了を防ぐgateを追加する\n",
        "adapter planだけでは不可\n",
        "adapter planだけでは不可\n",
    )?;

    let violations = OpenSpecIntegrityRule::check(&fixture.root)?;

    assert!(has_rule(&violations, "openspec-missing-evidence"));
    Ok(())
}

#[test]
fn openspec_integrity_ignores_archived_changes() -> Result<(), KdvLintError> {
    let fixture = FixtureWorkspace::new().with_default_manifests()?;
    fixture.write_text_file(
        "openspec/changes/archive/old-change/tasks.md",
        "- [x] Storybook確認として `katana`, `egui`, `gpui`, `floem` の4経路で開ける\n",
    )?;

    let violations = OpenSpecIntegrityRule::check(&fixture.root)?;

    assert!(violations.is_empty());
    Ok(())
}

fn write_required_openspec_files(
    fixture: &FixtureWorkspace,
    tasks: &str,
    design: &str,
    spec: &str,
) -> Result<(), KdvLintError> {
    fixture.write_text_file(
        "openspec/changes/v0-2-0-markdown-viewer-kuc-integration/tasks.md",
        tasks,
    )?;
    fixture.write_text_file(
        "openspec/changes/v0-2-0-markdown-viewer-kuc-integration/design.md",
        design,
    )?;
    fixture.write_text_file(
        "openspec/changes/v0-2-0-markdown-viewer-kuc-integration/specs/markdown-viewer-kuc-integration/spec.md",
        spec,
    )
}

fn has_rule(violations: &[crate::diagnostics::Violation], rule: &str) -> bool {
    violations.iter().any(|violation| violation.rule == rule)
}
