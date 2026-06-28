use super::{FrameRenderRequest, StorybookFrameRenderer};
use crate::canvas::Canvas;
use crate::catalog::StorybookFixture;
use crate::frame_pixel_guard::StorybookFramePixelGuard;
use crate::layout::{HEADER_HEIGHT, SIDEBAR_WIDTH, StorybookPreviewArea, preview_content_width};
use crate::media_host_action::StorybookMediaHostAction;
use crate::mouse::StorybookHostActionHits;
use crate::palette::StorybookPalette;
use crate::preview::{PreviewBuilder, PreviewScene};
use katana_document_viewer::{
    ViewerInteractionConfig, ViewerMediaControlKind, ViewerMode, ViewerRect, ViewerSearchEngine,
    ViewerViewport,
};
use katana_ui_core::render_model::{UiHostActionPlan, UiNode, UiNodeId};
use katana_ui_core_storybook::{
    UiTreeHostActionHit, UiTreeNodeHit, UiTreeRenderArea, UiTreeSurfaceHost,
};
use std::{collections::BTreeMap, io, path::PathBuf};

const FRAME_WIDTH: usize = 1280;
const FRAME_HEIGHT: usize = 720;
const PREVIEW_HEIGHT: f32 = 600.0;
const SEARCH_HIGHLIGHT_PIXEL: u32 = 0x4a4620;
const HOVER_BORDER_PIXEL: u32 = 0x569cd6;

thread_local! {
    static SHARED_PREVIEW_BUILDER: PreviewBuilder = PreviewBuilder::default();
}

#[test]
fn search_highlight_reaches_storybook_frame_pixels() -> Result<(), Box<dyn std::error::Error>> {
    let scene = FrameInteractionTestSupport::build_scene_with_search(
        "direct/sample.md",
        ViewerSearchEngine::state("Direct", Vec::new(), None),
    )?;
    let canvas = FrameInteractionTestSupport::render_scene("direct/sample.md", &scene);

    assert!(canvas.pixels().contains(&SEARCH_HIGHLIGHT_PIXEL));
    assert!(FrameInteractionTestSupport::preview_pixel_count(&canvas) > 512);
    Ok(())
}

#[test]
fn slideshow_mode_reaches_storybook_frame_pixels() -> Result<(), Box<dyn std::error::Error>> {
    let scene = FrameInteractionTestSupport::build_scene_with_mode(
        "katana/sample.md",
        ViewerMode::Slideshow,
    )?;
    let canvas = FrameInteractionTestSupport::render_scene("katana/sample.md", &scene);

    assert_eq!(ViewerMode::Slideshow, scene.mode);
    assert!(scene.slideshow_max_page > 0);
    assert!(FrameInteractionTestSupport::preview_pixel_count(&canvas) > 1024);
    Ok(())
}

#[test]
fn media_control_toggle_changes_scene_and_frame_pixels() -> Result<(), Box<dyn std::error::Error>> {
    let enabled = FrameInteractionTestSupport::build_scene_with_interaction(
        "direct/sample.md",
        ViewerInteractionConfig::default(),
    )?;
    let disabled = FrameInteractionTestSupport::build_scene_with_interaction(
        "direct/sample.md",
        ViewerInteractionConfig {
            image_controls_enabled: false,
            diagram_controls_enabled: false,
            ..ViewerInteractionConfig::default()
        },
    )?;
    let enabled_canvas = FrameInteractionTestSupport::render_scene("direct/sample.md", &enabled);
    let disabled_canvas = FrameInteractionTestSupport::render_scene("direct/sample.md", &disabled);

    assert!(action_count(enabled.tree.root(), "fullscreen") > 0);
    assert_eq!(0, action_count(disabled.tree.root(), "fullscreen"));
    assert!(preview_diff_pixel_count(&enabled_canvas, &disabled_canvas) > 128);
    Ok(())
}

#[test]
fn hover_highlight_reaches_storybook_frame_pixels() -> Result<(), Box<dyn std::error::Error>> {
    let scene = FrameInteractionTestSupport::build_scene_with_interaction(
        "katana/sample_basic.md",
        ViewerInteractionConfig::default(),
    )?;
    let target_id = scene
        .targets
        .first()
        .ok_or_else(|| std::io::Error::other("missing visible hover target"))?
        .node_id
        .0
        .clone();
    let normal = FrameInteractionTestSupport::render_scene("katana/sample_basic.md", &scene);
    let hovered = FrameInteractionTestSupport::render_scene_with_hover(
        "katana/sample_basic.md",
        &scene,
        &target_id,
    );

    assert!(preview_diff_pixel_count(&normal, &hovered) > 128);
    Ok(())
}

#[test]
fn media_control_hover_reaches_kuc_interactive_preset_border_pixels()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = FrameInteractionTestSupport::build_scene_with_interaction(
        "direct/sample.md",
        ViewerInteractionConfig::default(),
    )?;
    let hit = diagram_host_action_hit(&scene)?;
    let normal = FrameInteractionTestSupport::render_scene("direct/sample.md", &scene);
    let hovered = FrameInteractionTestSupport::render_scene_with_action_hover(
        "direct/sample.md",
        &scene,
        &hit.action.target,
    );
    let normal_count = preview_color_count(&normal, HOVER_BORDER_PIXEL);
    let hovered_count = preview_color_count(&hovered, HOVER_BORDER_PIXEL);
    let preview_area = StorybookPreviewArea::for_window(FRAME_WIDTH, FRAME_HEIGHT, 0.0);
    let x = preview_area.x + hit.rect.x;
    let y = preview_area.y + hit.rect.y;

    assert!(
        hovered_count > normal_count,
        "host action hover must increase KUC hover border pixels: normal={normal_count} hovered={hovered_count} rect=({}, {}, {}, {})",
        hit.rect.x,
        hit.rect.y,
        hit.rect.width,
        hit.rect.height
    );
    assert_eq!(
        HOVER_BORDER_PIXEL,
        hovered.pixels()[y * hovered.width() + x],
        "hovered KUC node id must paint the same rect used by host action hit-test; hit=({}, {}, {}, {}) hover_bounds={:?}",
        hit.rect.x,
        hit.rect.y,
        hit.rect.width,
        hit.rect.height,
        preview_color_bounds(&hovered, HOVER_BORDER_PIXEL)
    );
    Ok(())
}

#[test]
fn host_action_hits_overlap_rendered_frame_pixels() -> Result<(), Box<dyn std::error::Error>> {
    for path in [
        "katana/sample_basic.md",
        "direct/html-alignment.html",
        "direct/sample.md",
    ] {
        assert_host_action_hits_visible(path)?;
    }
    Ok(())
}

#[test]
fn hover_highlight_covers_scrolled_target_block_row() -> Result<(), Box<dyn std::error::Error>> {
    let scene = FrameInteractionTestSupport::build_scene_with_interaction(
        "katana/sample_basic.md",
        ViewerInteractionConfig::default(),
    )?;
    let target = hovered_target_node_hit(&scene)?;
    let hovered_node_id = hovered_node_id_for_hit(&scene, &target)?;
    let scroll_y = target.rect.y.saturating_sub(64) as f32;
    let normal = FrameInteractionTestSupport::render_scene_with_hover_and_scroll(
        "katana/sample_basic.md",
        &scene,
        "",
        scroll_y,
    );
    let hovered = FrameInteractionTestSupport::render_scene_with_hover_and_scroll(
        "katana/sample_basic.md",
        &scene,
        hovered_node_id.as_str(),
        scroll_y,
    );

    let render_scroll_y = render_scroll_delta(&scene, scroll_y);
    let target_bounds =
        hover_target_preview_bounds(&scene, &hovered_node_id, &target, render_scroll_y, &normal);
    let (inside, outside) =
        preview_hover_diff_by_expected_target_rect(&normal, &hovered, &target_bounds);
    let diff_bounds = preview_diff_bounds(&normal, &hovered);
    assert!(
        inside > 0,
        "hover highlight produced no pixel diff inside target rect"
    );
    assert_eq!(
        0, outside,
        "hover highlight changed pixels outside target block row: {outside} target={target_bounds:?} target_rect=({}, {}, {}, {}) diff={diff_bounds:?}",
        target.rect.x, target.rect.y, target.rect.width, target.rect.height
    );
    Ok(())
}

#[test]
fn hover_highlight_clips_scrolled_target_top_edge() -> Result<(), Box<dyn std::error::Error>> {
    let scene = FrameInteractionTestSupport::build_scene_with_interaction(
        "katana/sample_basic.md",
        ViewerInteractionConfig::default(),
    )?;
    let target = hovered_target_node_hit(&scene)?;
    let hovered_node_id = hovered_node_id_for_hit(&scene, &target)?;
    let scroll_y = target.rect.y as f32 + target.rect.height as f32 / 2.0;
    let normal = FrameInteractionTestSupport::render_scene_with_hover_and_scroll(
        "katana/sample_basic.md",
        &scene,
        "",
        scroll_y,
    );
    let hovered = FrameInteractionTestSupport::render_scene_with_hover_and_scroll(
        "katana/sample_basic.md",
        &scene,
        hovered_node_id.as_str(),
        scroll_y,
    );

    let target_bounds = clipped_hover_target_preview_bounds(
        &scene,
        &hovered_node_id,
        &target,
        render_scroll_delta(&scene, scroll_y),
        &normal,
    );
    let (inside, outside) =
        preview_hover_diff_by_expected_target_rect(&normal, &hovered, &target_bounds);
    let diff_bounds = preview_diff_bounds(&normal, &hovered);
    let resolved_target = scene.target_for_node_id(&hovered_node_id).map(|target| {
        (
            target.rect.x,
            target.rect.y,
            target.rect.width,
            target.rect.height,
        )
    });
    assert!(
        inside > 0,
        "hover highlight produced no pixel diff inside clipped target rect"
    );
    assert_eq!(
        0, outside,
        "hover highlight overpainted clipped target top edge: {outside} target={target_bounds:?} diff={diff_bounds:?} hovered_node_id={hovered_node_id} resolved_target={resolved_target:?}"
    );
    Ok(())
}

struct FrameInteractionTestSupport;

impl FrameInteractionTestSupport {
    fn build_scene_with_mode(
        path: &str,
        mode: ViewerMode,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        Self::builder().build_with_mode(
            &Self::fixture(path),
            Self::viewport(),
            true,
            ViewerInteractionConfig::default(),
            mode,
        )
    }

    fn build_scene_with_search(
        path: &str,
        search: katana_document_viewer::ViewerSearchState,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        Self::builder().build_with_mode_and_search(
            &Self::fixture(path),
            Self::viewport(),
            true,
            ViewerInteractionConfig::default(),
            ViewerMode::Document,
            search,
        )
    }

    fn build_scene_with_interaction(
        path: &str,
        interaction: ViewerInteractionConfig,
    ) -> Result<PreviewScene, Box<dyn std::error::Error>> {
        Self::builder().build(&Self::fixture(path), Self::viewport(), true, interaction)
    }

    fn builder() -> PreviewBuilder {
        SHARED_PREVIEW_BUILDER.with(Clone::clone)
    }

    fn render_scene(path: &str, scene: &PreviewScene) -> Canvas {
        Self::render_scene_with_hover(path, scene, "")
    }

    fn render_scene_with_hover(path: &str, scene: &PreviewScene, hovered_node_id: &str) -> Canvas {
        Self::render_scene_with_hover_and_scroll(path, scene, hovered_node_id, 0.0)
    }

    fn render_scene_with_action_hover(
        path: &str,
        scene: &PreviewScene,
        hovered_action_node_id: &UiNodeId,
    ) -> Canvas {
        Self::render_scene_with_hover_request(path, scene, "", 0.0, Some(hovered_action_node_id))
    }

    fn render_scene_with_hover_and_scroll(
        path: &str,
        scene: &PreviewScene,
        hovered_node_id: &str,
        scroll_y: f32,
    ) -> Canvas {
        Self::render_scene_with_hover_request(path, scene, hovered_node_id, scroll_y, None)
    }

    fn render_scene_with_hover_request(
        path: &str,
        scene: &PreviewScene,
        hovered_node_id: &str,
        scroll_y: f32,
        hovered_action_node_id: Option<&UiNodeId>,
    ) -> Canvas {
        StorybookFrameRenderer::render(FrameRenderRequest {
            width: FRAME_WIDTH,
            height: FRAME_HEIGHT,
            fixtures: &[Self::fixture(path)],
            selected_index: 0,
            scene: Some(scene),
            scroll_y,
            sidebar_scroll: Default::default(),
            file_tree_state: Default::default(),
            settings_state: &Default::default(),
            dark: true,
            interaction: &ViewerInteractionConfig::default(),
            typography: Default::default(),
            last_command_label: "none",
            task_context_menu: None,
            hovered_node_id: (!hovered_node_id.is_empty()).then_some(hovered_node_id),
            hovered_action_node_id,
            animation_phase: 0,
        })
    }

    fn preview_pixel_count(canvas: &Canvas) -> usize {
        StorybookFramePixelGuard::preview_content_pixel_count(canvas, true)
    }

    fn viewport() -> ViewerViewport {
        ViewerViewport {
            width: preview_content_width(FRAME_WIDTH) as f32,
            height: PREVIEW_HEIGHT,
        }
    }

    fn fixture(path: &str) -> StorybookFixture {
        StorybookFixture {
            label: path.to_string(),
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(format!("../../assets/fixtures/{path}")),
        }
    }
}

fn hovered_target_node_hit(scene: &PreviewScene) -> Result<UiTreeNodeHit, std::io::Error> {
    document_node_hits(scene)
        .into_iter()
        .find(|hit| hit.rect.width > 0 && hit.rect.height > 16)
        .ok_or_else(|| std::io::Error::other("missing rendered hover target"))
}

fn hovered_node_id_for_hit(
    scene: &PreviewScene,
    hit: &UiTreeNodeHit,
) -> Result<String, std::io::Error> {
    let x = hit.rect.x as f32 + hit.rect.width as f32 / 2.0;
    let y = hit.rect.y as f32 + hit.rect.height as f32 / 2.0;
    UiTreeSurfaceHost::hovered_node_id_at(&document_node_hits(scene), x, y)
        .map(|node_id| node_id.as_str().to_string())
        .ok_or_else(|| std::io::Error::other("missing resolved hovered node id"))
}

fn document_node_hits(scene: &PreviewScene) -> Vec<UiTreeNodeHit> {
    UiTreeSurfaceHost::new(scene.theme.clone()).document_node_hits(
        scene.tree.root(),
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: preview_content_width(FRAME_WIDTH),
            height: scene.content_height.ceil().max(1.0) as usize,
            scroll_y: 0.0,
        },
    )
}

fn render_scroll_delta(scene: &PreviewScene, requested_scroll_y: f32) -> f32 {
    let tree_offset = scene.tree.root().props().scroll_area.offset_y as f32;
    (requested_scroll_y - tree_offset).max(0.0)
}

fn action_count(node: &UiNode, action: &str) -> usize {
    usize::from(node.props().interaction.value == action)
        + node
            .children()
            .iter()
            .map(|child| action_count(child, action))
            .sum::<usize>()
}

fn diagram_host_action_hit(scene: &PreviewScene) -> Result<UiTreeHostActionHit, io::Error> {
    let hits =
        StorybookHostActionHits::hits_for_preview_width(scene, preview_content_width(FRAME_WIDTH));
    hits.into_iter()
        .find(|hit| is_diagram_host_action(&hit.action))
        .ok_or_else(|| io::Error::other("missing diagram host action hit"))
}

fn is_diagram_host_action(action: &UiHostActionPlan) -> bool {
    StorybookMediaHostAction::from_host_action_plan(action)
        .map(|action| action.into_viewer_action())
        .is_some_and(|action| {
            action.kind == ViewerMediaControlKind::Diagram && action.command == "fullscreen"
        })
}

fn assert_host_action_hits_visible(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scene = FrameInteractionTestSupport::build_scene_with_interaction(
        path,
        ViewerInteractionConfig::default(),
    )?;
    let hits =
        StorybookHostActionHits::hits_for_preview_width(&scene, preview_content_width(FRAME_WIDTH));
    assert!(!hits.is_empty(), "{path} has no host action hit rects");
    let mut canvases = BTreeMap::new();
    for hit in hits {
        let scroll_y = hit_scroll_y(&hit);
        let canvas = canvases.entry(scroll_y).or_insert_with(|| {
            FrameInteractionTestSupport::render_scene_with_hover_and_scroll(
                path,
                &scene,
                "",
                scroll_y as f32,
            )
        });
        assert!(
            non_background_pixels_in_scrolled_hit_rect(canvas, scroll_y, &hit) > 0,
            "{path} host action hit rect has no rendered pixels: action={} payload={} rect=({}, {}, {}, {})",
            hit.action.action_id,
            hit.action.payload,
            hit.rect.x,
            hit.rect.y,
            hit.rect.width,
            hit.rect.height
        );
    }
    Ok(())
}

fn non_background_pixels_in_scrolled_hit_rect(
    canvas: &Canvas,
    scroll_y: usize,
    hit: &UiTreeHostActionHit,
) -> usize {
    let background = StorybookPalette::new(true).preview_background();
    let area = StorybookPreviewArea::for_window(FRAME_WIDTH, FRAME_HEIGHT, scroll_y as f32);
    let start_x = area.x.saturating_add(hit.rect.x);
    let visible_y = hit.rect.y.saturating_sub(scroll_y);
    let start_y = area.y.saturating_add(visible_y);
    let end_x = start_x.saturating_add(hit.rect.width).min(canvas.width());
    let end_y = start_y.saturating_add(hit.rect.height).min(canvas.height());
    let mut count = 0usize;
    for y in start_y..end_y {
        for x in start_x..end_x {
            count += usize::from(canvas.pixels()[y * canvas.width() + x] != background);
        }
    }
    count
}

fn hit_scroll_y(hit: &UiTreeHostActionHit) -> usize {
    hit.rect.y.saturating_sub(96)
}

fn preview_diff_pixel_count(left: &Canvas, right: &Canvas) -> usize {
    left.pixels()
        .iter()
        .zip(right.pixels().iter())
        .enumerate()
        .filter(|(index, (left_pixel, right_pixel))| {
            inside_preview(*index, left.width()) && left_pixel != right_pixel
        })
        .count()
}

fn preview_color_count(canvas: &Canvas, color: u32) -> usize {
    canvas
        .pixels()
        .iter()
        .filter(|pixel| **pixel == color)
        .count()
}

fn preview_color_bounds(canvas: &Canvas, color: u32) -> Option<(usize, usize, usize, usize)> {
    let mut min_x = canvas.width();
    let mut min_y = canvas.height();
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    let mut found = false;
    for (index, pixel) in canvas.pixels().iter().enumerate() {
        if *pixel != color || !inside_preview(index, canvas.width()) {
            continue;
        }
        let x = index % canvas.width();
        let y = index / canvas.width();
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        found = true;
    }
    found.then_some((
        min_x,
        min_y,
        max_x.saturating_sub(min_x) + 1,
        max_y.saturating_sub(min_y) + 1,
    ))
}

fn preview_hover_diff_by_expected_target_rect(
    left: &Canvas,
    right: &Canvas,
    bounds: &(usize, usize, usize, usize),
) -> (usize, usize) {
    let width = left.width();
    let (left_offset, top_offset, width_offset, height_offset) = *bounds;
    let preview_left = SIDEBAR_WIDTH + 16;
    let preview_top = HEADER_HEIGHT + 16;
    let preview_right = width.saturating_sub(16);
    let preview_bottom = left.height().saturating_sub(16);

    let mut inside = 0usize;
    let mut outside = 0usize;
    for (index, (left_pixel, right_pixel)) in
        left.pixels().iter().zip(right.pixels().iter()).enumerate()
    {
        let x = index % width;
        let y = index / width;
        if x < preview_left || x >= preview_right || y < preview_top || y >= preview_bottom {
            continue;
        }
        if left_pixel == right_pixel {
            continue;
        }
        let in_target_x = x >= left_offset && x < left_offset.saturating_add(width_offset);
        let in_target_y = y >= top_offset && y < top_offset.saturating_add(height_offset);
        if in_target_x && in_target_y && x < preview_right && y < preview_bottom {
            inside += 1;
        } else {
            outside += 1;
        }
    }
    (inside, outside)
}

fn preview_diff_bounds(
    left: &Canvas,
    right: &Canvas,
) -> Option<(usize, usize, usize, usize, usize)> {
    let width = left.width();
    let height = left.height();
    let mut min_x = width;
    let mut max_x = 0usize;
    let mut min_y = height;
    let mut max_y = 0usize;
    let mut count = 0usize;

    for (index, (left_pixel, right_pixel)) in
        left.pixels().iter().zip(right.pixels().iter()).enumerate()
    {
        if left_pixel == right_pixel {
            continue;
        }
        let x = index % width;
        let y = index / width;
        count += 1;
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    if count == 0 {
        None
    } else {
        Some((min_x, max_x, min_y, max_y, count))
    }
}

fn node_hit_block_row_preview_bounds(
    target: &UiTreeNodeHit,
    scroll_y: f32,
    canvas: &Canvas,
) -> (usize, usize, usize, usize) {
    let preview_left = SIDEBAR_WIDTH + 16;
    let preview_top = HEADER_HEIGHT + 16;
    let preview_right = canvas.width().saturating_sub(16);
    let preview_bottom = canvas.height().saturating_sub(16);
    let left = preview_left;
    let target_y = target.rect.y as f32;
    let top = preview_top + ((target_y - scroll_y).round().max(0.0) as usize);
    let width = preview_right.saturating_sub(preview_left);
    let height = target.rect.height.max(1);
    (
        left.min(preview_right),
        top.min(preview_bottom),
        width.min(preview_right.saturating_sub(left)),
        height.min(preview_bottom.saturating_sub(top)),
    )
}

fn hover_target_preview_bounds(
    scene: &PreviewScene,
    hovered_node_id: &str,
    hit: &UiTreeNodeHit,
    scroll_y: f32,
    canvas: &Canvas,
) -> (usize, usize, usize, usize) {
    scene
        .target_for_node_id(hovered_node_id)
        .map(|target| viewer_rect_block_row_preview_bounds(target.rect, scroll_y, canvas))
        .unwrap_or_else(|| node_hit_block_row_preview_bounds(hit, scroll_y, canvas))
}

fn clipped_hover_target_preview_bounds(
    scene: &PreviewScene,
    hovered_node_id: &str,
    hit: &UiTreeNodeHit,
    scroll_y: f32,
    canvas: &Canvas,
) -> (usize, usize, usize, usize) {
    scene
        .target_for_node_id(hovered_node_id)
        .map(|target| clipped_viewer_rect_block_row_preview_bounds(target.rect, scroll_y, canvas))
        .unwrap_or_else(|| clipped_node_hit_block_row_preview_bounds(hit, scroll_y, canvas))
}

fn viewer_rect_block_row_preview_bounds(
    rect: ViewerRect,
    scroll_y: f32,
    canvas: &Canvas,
) -> (usize, usize, usize, usize) {
    let preview_left = SIDEBAR_WIDTH + 16;
    let preview_top = HEADER_HEIGHT + 16;
    let preview_right = canvas.width().saturating_sub(16);
    let preview_bottom = canvas.height().saturating_sub(16);
    let left = preview_left;
    let top = preview_top + ((rect.y - scroll_y).round().max(0.0) as usize);
    let width = preview_right.saturating_sub(preview_left);
    let height = rect.height.round().max(1.0) as usize;
    (
        left.min(preview_right),
        top.min(preview_bottom),
        width.min(preview_right.saturating_sub(left)),
        height.min(preview_bottom.saturating_sub(top)),
    )
}

fn clipped_viewer_rect_block_row_preview_bounds(
    rect: ViewerRect,
    scroll_y: f32,
    canvas: &Canvas,
) -> (usize, usize, usize, usize) {
    let (left, top, width, _) = viewer_rect_block_row_preview_bounds(rect, scroll_y, canvas);
    let preview_bottom = canvas.height().saturating_sub(16);
    let clipped_top = (scroll_y - rect.y).max(0.0);
    let visible_height = (rect.height - clipped_top).round().max(1.0) as usize;
    (
        left,
        top,
        width,
        visible_height.min(preview_bottom.saturating_sub(top)),
    )
}

fn clipped_node_hit_block_row_preview_bounds(
    target: &UiTreeNodeHit,
    scroll_y: f32,
    canvas: &Canvas,
) -> (usize, usize, usize, usize) {
    let (left, top, width, _) = node_hit_block_row_preview_bounds(target, scroll_y, canvas);
    let preview_bottom = canvas.height().saturating_sub(16);
    let clipped_top = (scroll_y - target.rect.y as f32).max(0.0);
    let visible_height = (target.rect.height as f32 - clipped_top).round().max(1.0) as usize;
    (
        left,
        top,
        width,
        visible_height.min(preview_bottom.saturating_sub(top)),
    )
}

fn inside_preview(index: usize, width: usize) -> bool {
    let x = index % width;
    let y = index / width;
    x >= SIDEBAR_WIDTH + 16 && y >= HEADER_HEIGHT + 16
}
