use crate::catalog::StorybookFixture;
use crate::preview::{PreviewBuilder, PreviewScene};
use crate::preview_requirement_matrix_assertions::PreviewRequirementAssertions;
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use katana_ui_core::render_model::UiNodeKind;
use std::path::PathBuf;

#[test]
fn katana_required_features_reach_kuc_storybook_scene() -> Result<(), Box<dyn std::error::Error>> {
    let sample = build_scene("katana/sample_basic.md", ViewerInteractionConfig::default())?;
    let scene = PreviewRequirementAssertions::new(&sample);

    scene.assert_role("code");
    scene.assert_role("table");
    scene.assert_role("alert");
    scene.assert_role("blockquote");
    scene.assert_role("footnote");
    scene.assert_role("list-marker");
    scene.assert_role("list-item");
    scene.assert_indented_list_depths();
    scene.assert_style("kdv-task-empty");
    scene.assert_style("kdv-task-done");
    scene.assert_style("kdv-task-progress");
    scene.assert_style("kdv-task-blocked");
    scene.assert_kind(UiNodeKind::Divider);
    scene.assert_link_span();
    scene.assert_accessibility_label("math");
    scene.assert_syntax_highlighted_code();
    scene.assert_strikethrough_span("Strikethrough");
    Ok(())
}

#[test]
fn direct_html_requirement_features_reach_kuc_storybook_scene()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene(
        "direct/html-alignment.html",
        ViewerInteractionConfig::default(),
    )?;
    let requirements = PreviewRequirementAssertions::new(&scene);

    requirements.assert_role("html-centered-preview");
    requirements.assert_role("html-right-preview");
    requirements.assert_role("html-left-preview");
    requirements.assert_role("table");
    requirements.assert_kind(UiNodeKind::Accordion);
    requirements.assert_link_target("https://example.com/docs");
    Ok(())
}

#[test]
fn diagram_requirement_features_reach_kuc_storybook_scene() -> Result<(), Box<dyn std::error::Error>>
{
    let interaction = ViewerInteractionConfig {
        diagram_controls_enabled: true,
        ..ViewerInteractionConfig::default()
    };
    let scene = build_scene("katana/sample_diagrams.md", interaction)?;
    let requirements = PreviewRequirementAssertions::new(&scene);

    assert!(scene.loaded_asset_count >= 3);
    assert!(scene.image_surface_count >= 3);
    requirements.assert_accessibility_label("diagram:Mermaid");
    requirements.assert_accessibility_label("diagram:PlantUml");
    requirements.assert_accessibility_label("diagram:DrawIo");
    requirements.assert_internal_diagram_action("pan-up");
    requirements.assert_internal_diagram_action("reset-view");
    requirements.assert_action("fullscreen");
    Ok(())
}

fn build_scene(
    path: &str,
    interaction: ViewerInteractionConfig,
) -> Result<PreviewScene, Box<dyn std::error::Error>> {
    PreviewBuilder::default().build_without_preview_surface(
        &StorybookFixture {
            label: path.to_string(),
            path: fixture_path(&format!("assets/fixtures/{path}")),
        },
        ViewerViewport {
            width: 800.0,
            height: 20_000.0,
        },
        true,
        interaction,
    )
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../{path}"))
}
