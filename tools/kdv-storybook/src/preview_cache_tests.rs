use super::PreviewBuilder;
use crate::catalog::StorybookFixture;
use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use katana_document_viewer::{
    DiagramViewportState, ViewerInteractionConfig, ViewerMode, ViewerSearchState,
    ViewerTypographyConfig, ViewerVector, ViewerViewport,
};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[test]
fn parsed_output_cache_keeps_multiple_fixture_outputs() -> Result<(), Box<dyn std::error::Error>> {
    let builder = PreviewBuilder::default();
    let first = fixture("direct/sample.md");
    let second = fixture("direct/html-alignment.html");

    builder.build_scene(lazy_request(&first))?;
    builder.build_scene(lazy_request(&second))?;
    builder.build_scene(lazy_request(&first))?;

    let stats = builder.builder_cache_stats()?;
    assert_eq!(2, stats.parsed_misses);
    assert_eq!(1, stats.lazy_scene_hits);
    assert_eq!(2, stats.lazy_scene_misses);
    assert_eq!(0, stats.parsed_hits);
    Ok(())
}

#[test]
fn lazy_scene_cache_keeps_state_specific_scenes() -> Result<(), Box<dyn std::error::Error>> {
    let builder = PreviewBuilder::default();
    let fixture = fixture("direct/sample.md");
    let mut first = lazy_request(&fixture);
    first.interaction.hover_highlight_enabled = true;
    builder.build_scene(first)?;

    let mut changed = lazy_request(&fixture);
    changed.interaction.hover_highlight_enabled = false;
    builder.build_scene(changed)?;

    let mut restored = lazy_request(&fixture);
    restored.interaction.hover_highlight_enabled = true;
    builder.build_scene(restored)?;

    let stats = builder.builder_cache_stats()?;
    assert_eq!(1, stats.lazy_scene_hits);
    assert_eq!(2, stats.lazy_scene_misses);
    Ok(())
}

#[test]
fn lazy_scene_cache_keeps_code_control_specific_scenes() -> Result<(), Box<dyn std::error::Error>> {
    let builder = PreviewBuilder::default();
    let fixture = fixture("direct/sample.md");
    let mut first = lazy_request(&fixture);
    first.interaction.code_controls_enabled = true;
    builder.build_scene(first)?;

    let mut changed = lazy_request(&fixture);
    changed.interaction.code_controls_enabled = false;
    builder.build_scene(changed)?;

    let mut restored = lazy_request(&fixture);
    restored.interaction.code_controls_enabled = true;
    builder.build_scene(restored)?;

    let stats = builder.builder_cache_stats()?;
    assert_eq!(1, stats.lazy_scene_hits);
    assert_eq!(2, stats.lazy_scene_misses);
    Ok(())
}

#[test]
fn lazy_scene_cache_keeps_typography_specific_scenes() -> Result<(), Box<dyn std::error::Error>> {
    let builder = PreviewBuilder::default();
    let fixture = fixture("direct/sample.md");
    builder.build_scene(lazy_request(&fixture))?;

    let mut changed = lazy_request(&fixture);
    changed.typography = ViewerTypographyConfig {
        preview_font_size: 22,
    };
    let scene = builder.build_scene(changed)?;

    assert_eq!(22, scene.typography.preview_font_size);
    assert_eq!(
        Some(22.0),
        scene.theme.font("document-body").map(|font| font.size)
    );
    assert_eq!(
        Some(20.0),
        scene.theme.font("document-code").map(|font| font.size)
    );
    assert_eq!(Some(20.0), scene.theme.font("code").map(|font| font.size));
    let stats = builder.builder_cache_stats()?;
    assert_eq!(2, stats.lazy_scene_misses);
    assert_eq!(0, stats.lazy_scene_hits);
    Ok(())
}

#[test]
fn artifact_cache_keeps_multiple_fixture_outputs_after_round_trip_switch()
-> Result<(), Box<dyn std::error::Error>> {
    let builder = PreviewBuilder::default();
    let first = fixture("direct/kdv-icon.png");
    let second = fixture("direct/kdv-icon.jpg");

    let first_scene = builder.build_scene(request(&first, BTreeMap::new()))?;
    assert!(first_scene.loaded_asset_count > 0);
    builder.build_scene(request(&second, BTreeMap::new()))?;
    let restored = builder.build_scene(request(&first, BTreeMap::new()))?;

    let stats = builder.builder_cache_stats()?;
    assert!(stats.artifact_hits >= 1);
    assert_eq!(2, stats.artifact_misses);
    assert!(restored.loaded_asset_count > 0);
    assert!(restored.image_surface_count > 0);
    Ok(())
}

#[test]
fn loaded_build_reuses_artifacts_when_only_diagram_viewport_state_changes()
-> Result<(), Box<dyn std::error::Error>> {
    let builder = PreviewBuilder::default();
    let fixture = fixture("direct/sample.drawio");
    let first = builder.build_scene(request(&fixture, BTreeMap::new()))?;
    assert!(first.loaded_asset_count > 0);
    assert!(first.image_surface_count > 0);

    let mut diagram_viewports = BTreeMap::new();
    diagram_viewports.insert(
        "diagram-1".to_string(),
        DiagramViewportState {
            zoom: 1.5,
            pan: ViewerVector { x: 32.0, y: 16.0 },
            fullscreen_open: false,
            help_requested: false,
        },
    );

    let second = builder.build_scene(request(&fixture, diagram_viewports))?;
    let stats = builder.builder_cache_stats()?;
    assert_eq!(1, stats.artifact_hits);
    assert_eq!(1, stats.artifact_misses);
    assert!(second.loaded_asset_count > 0);
    assert!(second.image_surface_count > 0);
    Ok(())
}

#[test]
fn loaded_build_reuses_artifacts_across_repeated_document_builds()
-> Result<(), Box<dyn std::error::Error>> {
    let builder = PreviewBuilder::default();
    let fixture = fixture("katana/sample_diagrams.md");
    let first = builder.build_scene(request(&fixture, BTreeMap::new()))?;
    assert!(first.loaded_asset_count > 0);
    assert!(first.image_surface_count > 0);
    let first_stats = builder.builder_cache_stats()?;
    assert!(first_stats.cached_artifact_count > 0);

    let repeated = builder.build_scene(request(&fixture, BTreeMap::new()))?;
    assert!(repeated.image_surface_count > 0);
    assert_eq!(0, repeated.tree.root().props().scroll_area.offset_y);
    let repeated_stats = builder.builder_cache_stats()?;
    assert_eq!(1, repeated_stats.artifact_hits);
    assert_eq!(1, repeated_stats.artifact_misses);
    assert!(repeated_stats.cached_artifact_count >= first_stats.cached_artifact_count);

    builder.build_scene(request(&fixture, BTreeMap::new()))?;
    let stable_stats = builder.builder_cache_stats()?;

    assert_eq!(
        repeated_stats.cached_artifact_count,
        stable_stats.cached_artifact_count
    );
    assert_eq!(2, stable_stats.artifact_hits);
    assert_eq!(1, stable_stats.artifact_misses);
    Ok(())
}

#[test]
fn lazy_scene_asset_scope_is_stable_without_scroll_input() -> Result<(), Box<dyn std::error::Error>>
{
    let builder = PreviewBuilder::default();
    let fixture = fixture("katana/sample_diagrams.md");
    let first = builder.build_scene(lazy_request(&fixture))?;
    let repeated = builder.build_scene(lazy_request(&fixture))?;

    assert_eq!(first.asset_request_count, repeated.asset_request_count);
    assert_eq!(first.asset_request_key, repeated.asset_request_key);
    assert_eq!(0, repeated.tree.root().props().scroll_area.offset_y);
    let stats = builder.builder_cache_stats()?;
    assert_eq!(1, stats.lazy_scene_hits);
    assert_eq!(1, stats.lazy_scene_misses);
    Ok(())
}

#[test]
fn no_surface_asset_mode_keeps_kuc_image_surface_without_export_surface()
-> Result<(), Box<dyn std::error::Error>> {
    let builder = PreviewBuilder::default();
    let fixture = fixture("direct/sample.drawio");
    let mut request = request(&fixture, BTreeMap::new());
    request.attach_surface = false;

    let scene = builder.build_scene(request)?;

    assert!(scene.image_surface_count > 0);
    assert!(scene.surface.is_none());
    Ok(())
}

fn request(
    fixture: &StorybookFixture,
    diagram_viewports: BTreeMap<String, DiagramViewportState>,
) -> PreviewBuildRequest<'_> {
    PreviewBuildRequest {
        fixture,
        viewport: ViewerViewport {
            width: 900.0,
            height: 600.0,
        },
        dark: true,
        theme: None,
        interaction: ViewerInteractionConfig::default(),
        mode: ViewerMode::Document,
        typography: Default::default(),
        search: ViewerSearchState::default(),
        diagram_viewports,
        image_viewports: BTreeMap::new(),
        task_state_overrides: BTreeMap::new(),
        accordion_open_overrides: BTreeMap::new(),
        copied_code_node_ids: Default::default(),
        asset_mode: PreviewBuildAssetMode::VisibleAndNearViewport,
        attach_surface: true,
        export_surface: false,
    }
}

fn lazy_request(fixture: &StorybookFixture) -> PreviewBuildRequest<'_> {
    PreviewBuildRequest {
        fixture,
        viewport: ViewerViewport {
            width: 900.0,
            height: 600.0,
        },
        dark: true,
        theme: None,
        interaction: ViewerInteractionConfig::default(),
        mode: ViewerMode::Document,
        typography: Default::default(),
        search: ViewerSearchState::default(),
        diagram_viewports: BTreeMap::new(),
        image_viewports: BTreeMap::new(),
        task_state_overrides: BTreeMap::new(),
        accordion_open_overrides: BTreeMap::new(),
        copied_code_node_ids: Default::default(),
        asset_mode: PreviewBuildAssetMode::Lazy,
        attach_surface: false,
        export_surface: false,
    }
}

fn fixture(path: &str) -> StorybookFixture {
    StorybookFixture {
        label: path.to_string(),
        path: fixture_path(path),
    }
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../assets/fixtures/{path}"))
}
