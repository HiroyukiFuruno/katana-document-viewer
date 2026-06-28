use super::StorybookWindow;
use crate::args::StorybookArgs;
use crate::catalog::{FixtureCatalog, StorybookFixture};
use crate::layout::sidebar_content_height;
use crate::preview::PreviewBuilder;
use crate::sidebar::StorybookSidebar;
use crate::sidebar_test_support::StorybookFileTreeItemPointRequest;
use std::path::PathBuf;

#[test]
fn sidebar_hover_reuses_kuc_interaction_surface_inside_same_row()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        catalog_with_many_katana_labels(),
        PreviewBuilder::default(),
    );
    let (x, y) = tree_item_canvas_point(&storybook, "katana/sample-3.md", 900)?;

    assert!(storybook.update_sidebar_tree_hover_for_canvas_point(x, y, 1000, 900));
    let first_misses = storybook.sidebar_interaction_surface_cache_misses;
    assert!(first_misses > 0);

    assert!(!storybook.update_sidebar_tree_hover_for_canvas_point(x + 1.0, y, 1000, 900));

    assert_eq!(
        first_misses,
        storybook.sidebar_interaction_surface_cache_misses
    );
    Ok(())
}

fn tree_item_canvas_point(
    storybook: &StorybookWindow,
    item_id: &str,
    height: usize,
) -> Result<(f32, f32), Box<dyn std::error::Error>> {
    let point =
        StorybookSidebar::fixture_canvas_point_for_item_id(StorybookFileTreeItemPointRequest {
            fixtures: &storybook.catalog.fixtures,
            selected_index: storybook.selected_index,
            state: &storybook.file_tree_state,
            item_id,
            height: sidebar_content_height(height),
            scroll: storybook.sidebar_scroll,
        })
        .ok_or_else(|| format!("tree item hit not found: {item_id}"))?;
    Ok((point.x, point.y))
}

fn catalog_with_many_katana_labels() -> FixtureCatalog {
    let fixtures = (0..24)
        .map(|index| StorybookFixture {
            label: format!("katana/sample-{index}.md"),
            path: PathBuf::from(format!("/tmp/sample-{index}.md")),
        })
        .collect();
    FixtureCatalog { fixtures }
}
