use super::PreviewBuilder;
use crate::catalog::StorybookFixture;
use crate::preview_interaction_command_support::collect_code_actions;
use katana_document_viewer::{ViewerInteractionConfig, ViewerMode, ViewerViewport};
use std::path::PathBuf;

#[test]
fn preview_build_with_slideshow_mode_reaches_scene_state() -> Result<(), Box<dyn std::error::Error>>
{
    let scene = PreviewBuilder::default().build_with_mode(
        &StorybookFixture {
            label: "katana/sample.md".to_string(),
            path: fixture_path("assets/fixtures/katana/sample.md"),
        },
        ViewerViewport {
            width: 800.0,
            height: 360.0,
        },
        true,
        ViewerInteractionConfig::default(),
        ViewerMode::Slideshow,
    )?;

    assert_eq!(ViewerMode::Slideshow, scene.mode);
    assert!(scene.slideshow_max_page > 0);
    assert_eq!(0, scene.slideshow_current_page);
    assert!(scene.content_height > 360.0);
    Ok(())
}

#[test]
fn slideshow_mode_hides_code_copy_controls_like_katana() -> Result<(), Box<dyn std::error::Error>> {
    let interaction = ViewerInteractionConfig {
        code_controls_enabled: true,
        ..ViewerInteractionConfig::default()
    };
    let fixture = StorybookFixture {
        label: "katana/sample_basic.md".to_string(),
        path: fixture_path("assets/fixtures/katana/sample_basic.md"),
    };
    let viewport = ViewerViewport {
        width: 800.0,
        height: 360.0,
    };

    let document = PreviewBuilder::default().build_with_mode(
        &fixture,
        viewport,
        true,
        interaction.clone(),
        ViewerMode::Document,
    )?;
    let slideshow = PreviewBuilder::default().build_with_mode(
        &fixture,
        viewport,
        true,
        interaction,
        ViewerMode::Slideshow,
    )?;

    assert!(
        collect_code_actions(document.tree.root())
            .iter()
            .any(|value| value == "copy-code"),
        "document mode must keep code copy controls"
    );
    assert!(
        collect_code_actions(slideshow.tree.root()).is_empty(),
        "slideshow mode must hide code copy controls"
    );
    Ok(())
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../{path}"))
}
