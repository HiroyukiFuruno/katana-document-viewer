use super::PreviewBuilder;
use crate::catalog::StorybookFixture;
use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use katana_document_viewer::{
    ViewerInteractionConfig, ViewerMode, ViewerSearchState, ViewerViewport,
};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[test]
fn repeated_state_only_scene_build_reuses_fixture_source() -> Result<(), Box<dyn std::error::Error>>
{
    let builder = PreviewBuilder::default();
    let fixture = fixture("katana/sample.md");

    builder.build_scene(request(&fixture))?;
    builder.build_scene(request(&fixture))?;

    let stats = builder.source_cache_stats()?;
    assert_eq!(1, stats.misses);
    assert_eq!(1, stats.hits);
    Ok(())
}

fn request(fixture: &StorybookFixture) -> PreviewBuildRequest<'_> {
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

fn fixture(label: &str) -> StorybookFixture {
    StorybookFixture {
        label: label.to_string(),
        path: fixture_path(label),
    }
}

fn fixture_path(label: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../assets/fixtures/{label}"))
}
