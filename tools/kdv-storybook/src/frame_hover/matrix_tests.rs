use super::{FrameRenderRequest, StorybookFrameRenderer};
use crate::canvas::Canvas;
use crate::catalog::StorybookFixture;
use crate::layout::{HEADER_HEIGHT, SIDEBAR_WIDTH, preview_content_width};
use crate::preview::{PreviewBuilder, PreviewScene};
use crate::sidebar_settings_state::StorybookSettingsState;
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use katana_ui_core::render_model::UiNode;
use katana_ui_core_storybook::{UiTreeNodeHit, UiTreeRenderArea, UiTreeSurfaceHost};
use std::{collections::BTreeSet, io, path::PathBuf};

const FRAME_WIDTH: usize = 1280;
const FRAME_HEIGHT: usize = 720;
const PREVIEW_WIDTH: f32 = 900.0;
const PREVIEW_HEIGHT: f32 = 600.0;

#[test]
fn hover_highlight_covers_common_markdown_block_kinds() -> Result<(), Box<dyn std::error::Error>> {
    let path = "katana/sample_basic.md";
    let scene = build_scene(path)?;
    let hit_index = HoverHitIndex::new(&scene);
    for case in hover_cases() {
        assert_hover_case(path, &scene, &hit_index, case)?;
    }
    Ok(())
}

#[test]
fn hover_node_hit_stays_aligned_with_viewer_target_rect() -> Result<(), Box<dyn std::error::Error>>
{
    let scene = build_scene("katana/sample.md")?;
    let hit_index = HoverHitIndex::new(&scene);
    let mut failures = Vec::new();
    for needle in [
        "KatanA Rendering Regression Test",
        "This document is a comprehensive sample",
        "5.5 Short + Long + Short Columns",
        "Nested item 2-1",
    ] {
        let (hit, _) = visible_node_hit_containing_with_index(&scene, &hit_index, needle)?;
        let Some(target) = scene.target_for_node_id(hit.node_id.as_str()) else {
            failures.push(format!(
                "needle={needle:?} node_id={} has no viewer target matching_ids={:?}",
                hit.node_id.as_str(),
                text_node_ids_containing(scene.tree.root(), needle)
            ));
            continue;
        };
        let target_y = target.rect.y.round() as isize;
        let target_height = target.rect.height.round() as isize;
        let hit_y = hit.rect.y as isize;
        let hit_height = hit.rect.height as isize;
        let target_bottom = target_y + target_height;
        let hit_bottom = hit_y + hit_height;
        let inside_target = hit_y >= target_y.saturating_sub(1) && hit_bottom <= target_bottom + 1;
        if !inside_target {
            failures.push(format!(
                "needle={needle:?} node_id={} target_y={} hit_y={} dy={} target_height={} hit_height={} target_bottom={} hit_bottom={}",
                hit.node_id.as_str(),
                target_y,
                hit_y,
                hit_y - target_y,
                target_height,
                hit_height,
                target_bottom,
                hit_bottom
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "KDV viewer targets and KUC node hits drifted:\n{}",
        failures.join("\n")
    );
    Ok(())
}

#[test]
fn hover_highlight_keeps_column_content_visible() -> Result<(), Box<dyn std::error::Error>> {
    let path = "katana/sample.md";
    let scene = build_scene(path)?;
    let hit_index = HoverHitIndex::new(&scene);
    let (hit, scroll_y) = visible_node_hit_containing_with_index(
        &scene,
        &hit_index,
        "5.5 Short + Long + Short Columns",
    )?;
    let normal = render(path, &scene, None, scroll_y);
    let hovered = render(path, &scene, Some(hit.node_id.as_str()), scroll_y);
    let bounds = node_hit_block_row_preview_bounds(&hit, scroll_y, &normal);
    let retention = foreground_ink_retention_ratio(&normal, &hovered, bounds);

    assert!(
        retention >= 90,
        "hover highlight must not obscure column content: node_id={} retention={} bounds={:?} hit={:?}",
        hit.node_id.as_str(),
        retention,
        bounds,
        hit.rect
    );
    Ok(())
}

#[test]
fn hover_highlight_for_column_block_stays_inside_kuc_hit_rect()
-> Result<(), Box<dyn std::error::Error>> {
    let path = "katana/sample.md";
    let scene = build_scene(path)?;
    let hit_index = HoverHitIndex::new(&scene);
    let (hit, scroll_y) = visible_node_hit_containing_with_index(
        &scene,
        &hit_index,
        "5.5 Short + Long + Short Columns",
    )?;
    let target = scene
        .target_for_node_id(hit.node_id.as_str())
        .ok_or_else(|| io::Error::other("missing scene target for column hover hit"))?;
    let normal = render(path, &scene, None, scroll_y);
    let hovered = render(path, &scene, Some(hit.node_id.as_str()), scroll_y);
    let target_bounds = node_hit_block_row_preview_bounds(&hit, scroll_y, &normal);
    let (inside, outside) = preview_hover_diff_by_expected_rect(&normal, &hovered, target_bounds);
    let diff_bounds = preview_diff_bounds(&normal, &hovered);

    assert!(
        inside > 0,
        "column hover produced no diff inside KUC hit rect: node_id={} hit={:?} bounds={:?} diff={:?}",
        hit.node_id.as_str(),
        hit.rect,
        target_bounds,
        diff_bounds
    );
    assert_eq!(
        0,
        outside,
        "column hover changed pixels outside KUC hit rect: node_id={} outside={} hit={:?} scene_target={:?} bounds={:?} diff={:?}",
        hit.node_id.as_str(),
        outside,
        hit.rect,
        target.rect,
        target_bounds,
        diff_bounds
    );
    Ok(())
}

struct HoverMatrixCase {
    label: &'static str,
    needle: &'static str,
}

fn hover_cases() -> [HoverMatrixCase; 7] {
    [
        case(
            "paragraph",
            "This fixture exercises core Markdown rendering",
        ),
        case("list", "Nested item 2-1"),
        case("task", "Pending task"),
        case("code", "println!(\"Hello, KatanA!\");"),
        case("table", "Markdown"),
        case("blockquote", "This is a blockquote."),
        case(
            "alert",
            "Highlights information that users should take into account",
        ),
    ]
}

const fn case(label: &'static str, needle: &'static str) -> HoverMatrixCase {
    HoverMatrixCase { label, needle }
}

fn assert_hover_case(
    path: &str,
    scene: &PreviewScene,
    hit_index: &HoverHitIndex,
    case: HoverMatrixCase,
) -> Result<(), Box<dyn std::error::Error>> {
    let (hit, scroll_y) = visible_node_hit_containing_with_index(scene, hit_index, case.needle)?;
    let normal = render(path, scene, None, scroll_y);
    let hovered = render(path, scene, Some(hit.node_id.as_str()), scroll_y);
    let target_bounds = scene_target_preview_bounds(scene, &hit, scroll_y, &normal);
    let (inside, outside) = preview_hover_diff_by_expected_rect(&normal, &hovered, target_bounds);
    let diff_bounds = preview_diff_bounds(&normal, &hovered);

    if inside == 0 && case.label == "blockquote" {
        let retention = foreground_ink_retention_ratio(&normal, &hovered, target_bounds);
        assert!(
            retention >= 90,
            "blockquote hover must not obscure content when its quote background already matches hover surface: retention={} node_id={} hit={:?} bounds={:?} diff={:?}",
            retention,
            hit.node_id.as_str(),
            hit.rect,
            target_bounds,
            diff_bounds
        );
        return Ok(());
    }
    assert!(
        inside > 0,
        "hover case={} produced no diff inside target row node_id={} hit={:?} bounds={:?} diff={:?}",
        case.label,
        hit.node_id.as_str(),
        hit.rect,
        target_bounds,
        diff_bounds
    );
    assert_eq!(
        0, outside,
        "hover case={} changed pixels outside target row outside={} bounds={:?} hit={:?} diff={:?}",
        case.label, outside, target_bounds, hit.rect, diff_bounds
    );
    Ok(())
}

fn build_scene(path: &str) -> Result<PreviewScene, Box<dyn std::error::Error>> {
    PreviewBuilder::default().build(
        &fixture(path),
        ViewerViewport {
            width: PREVIEW_WIDTH,
            height: PREVIEW_HEIGHT,
        },
        true,
        ViewerInteractionConfig::default(),
    )
}

fn render(
    path: &str,
    scene: &PreviewScene,
    hovered_node_id: Option<&str>,
    scroll_y: f32,
) -> Canvas {
    let settings_state = StorybookSettingsState::default();
    StorybookFrameRenderer::render(FrameRenderRequest {
        width: FRAME_WIDTH,
        height: FRAME_HEIGHT,
        fixtures: &[fixture(path)],
        selected_index: 0,
        scene: Some(scene),
        scroll_y,
        sidebar_scroll: Default::default(),
        file_tree_state: Default::default(),
        settings_state: &settings_state,
        dark: true,
        interaction: &ViewerInteractionConfig::default(),
        typography: Default::default(),
        last_command_label: "none",
        task_context_menu: None,
        hovered_node_id,
        hovered_action_node_id: None,
        animation_phase: 0,
    })
}

fn visible_node_hit_containing_with_index(
    scene: &PreviewScene,
    hit_index: &HoverHitIndex,
    needle: &str,
) -> Result<(UiTreeNodeHit, f32), io::Error> {
    let node_ids = text_node_ids_containing(scene.tree.root(), needle);
    let document_hits = hit_index.document_node_hits_for_ids(&node_ids);
    if let Some(hit) = document_hits.iter().max_by_key(|hit| hit.rect.area()) {
        let refined_scroll_y = hit.rect.y.saturating_sub(120) as f32;
        if let Some(refined_hit) = viewport_node_hit_containing(scene, &node_ids, refined_scroll_y)
        {
            return Ok((refined_hit, refined_scroll_y));
        }
    }
    Err(io::Error::other(format!(
        "missing visible KUC node hit for hover needle={needle} matching_ids={:?} document_hits={:?}",
        node_ids, document_hits
    )))
}

struct HoverHitIndex {
    document_hits: Vec<UiTreeNodeHit>,
}

impl HoverHitIndex {
    fn new(scene: &PreviewScene) -> Self {
        Self {
            document_hits: UiTreeSurfaceHost::new(scene.theme.clone()).document_node_hits(
                scene.tree.root(),
                UiTreeRenderArea {
                    x: 0,
                    y: 0,
                    width: preview_content_width(FRAME_WIDTH),
                    height: scene.content_height.ceil().max(1.0) as usize,
                    scroll_y: 0.0,
                },
            ),
        }
    }

    fn document_node_hits_for_ids(&self, node_ids: &BTreeSet<String>) -> Vec<UiTreeNodeHit> {
        self.document_hits
            .iter()
            .filter(|hit| hit_matches_node_ids(hit, node_ids))
            .cloned()
            .collect()
    }
}

fn hit_matches_node_ids(hit: &UiTreeNodeHit, node_ids: &BTreeSet<String>) -> bool {
    node_ids.contains(hit.node_id.as_str())
        || hit
            .semantic_node_id
            .as_ref()
            .is_some_and(|node_id| node_ids.contains(node_id.as_str()))
}

fn text_node_ids_containing(root: &UiNode, needle: &str) -> BTreeSet<String> {
    let mut node_ids = BTreeSet::new();
    collect_text_node_ids_containing(root, needle, &mut node_ids);
    node_ids
}

fn collect_text_node_ids_containing(node: &UiNode, needle: &str, node_ids: &mut BTreeSet<String>) {
    if node_text(node).contains(needle) {
        node_ids.insert(node.id().as_str().to_string());
    }
    for child in node.children() {
        collect_text_node_ids_containing(child, needle, node_ids);
    }
}

fn node_text(node: &UiNode) -> String {
    let mut text = node.props().label.clone();
    for span in &node.props().text.spans {
        text.push_str(&span.text);
    }
    text
}

fn viewport_node_hit_containing(
    scene: &PreviewScene,
    node_ids: &BTreeSet<String>,
    scroll_y: f32,
) -> Option<UiTreeNodeHit> {
    let scroll_offset = scroll_y.round().max(0.0) as usize;
    let hits = UiTreeSurfaceHost::new(scene.theme.clone())
        .viewport_node_hits(
            scene.tree.root(),
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: preview_content_width(FRAME_WIDTH),
                height: PREVIEW_HEIGHT as usize,
                scroll_y,
            },
        )
        .into_iter()
        .collect::<Vec<_>>();
    let hover_node_ids = hits
        .iter()
        .filter(|hit| node_ids.contains(hit.node_id.as_str()))
        .map(|hit| {
            hit.semantic_node_id
                .clone()
                .unwrap_or_else(|| hit.node_id.clone())
                .as_str()
                .to_string()
        })
        .collect::<BTreeSet<_>>();
    hits.into_iter()
        .filter(|hit| hover_node_ids.contains(resolved_hover_node_id(hit).as_str()))
        .map(|mut hit| {
            hit.rect.y = hit.rect.y.saturating_add(scroll_offset);
            hit.node_id = resolved_hover_node_id(&hit);
            hit
        })
        .max_by_key(|hit| hit.rect.area())
}

fn resolved_hover_node_id(hit: &UiTreeNodeHit) -> katana_ui_core::render_model::UiNodeId {
    hit.semantic_node_id
        .clone()
        .unwrap_or_else(|| hit.node_id.clone())
}

fn scene_target_preview_bounds(
    scene: &PreviewScene,
    target: &UiTreeNodeHit,
    scroll_y: f32,
    canvas: &Canvas,
) -> (usize, usize, usize, usize) {
    let Some(scene_target) = scene.target_for_node_id(target.node_id.as_str()) else {
        return node_hit_block_row_preview_bounds(target, scroll_y, canvas);
    };
    let preview_left = SIDEBAR_WIDTH + 16;
    let preview_top = HEADER_HEIGHT + 16;
    let preview_right = canvas.width().saturating_sub(16);
    let preview_bottom = canvas.height().saturating_sub(16);
    let top = preview_top + ((scene_target.rect.y - scroll_y).round().max(0.0) as usize);
    let height = scene_target.rect.height.round().max(1.0) as usize;
    (
        preview_left.min(preview_right),
        top.min(preview_bottom),
        preview_right.saturating_sub(preview_left),
        height.min(preview_bottom.saturating_sub(top)),
    )
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

fn preview_hover_diff_by_expected_rect(
    left: &Canvas,
    right: &Canvas,
    bounds: (usize, usize, usize, usize),
) -> (usize, usize) {
    let (left_offset, top_offset, width_offset, height_offset) = bounds;
    let preview_left = SIDEBAR_WIDTH + 16;
    let preview_top = HEADER_HEIGHT + 16;
    let preview_right = left.width().saturating_sub(16);
    let preview_bottom = left.height().saturating_sub(16);
    let mut inside = 0usize;
    let mut outside = 0usize;
    for (index, (left_pixel, right_pixel)) in
        left.pixels().iter().zip(right.pixels().iter()).enumerate()
    {
        let x = index % left.width();
        let y = index / left.width();
        if x < preview_left || x >= preview_right || y < preview_top || y >= preview_bottom {
            continue;
        }
        if left_pixel == right_pixel {
            continue;
        }
        if inside_bounds(x, y, left_offset, top_offset, width_offset, height_offset) {
            inside += 1;
        } else {
            outside += 1;
        }
    }
    (inside, outside)
}

fn preview_diff_bounds(left: &Canvas, right: &Canvas) -> Option<(usize, usize, usize, usize)> {
    let mut min_x = left.width();
    let mut min_y = left.height();
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    let mut found = false;
    for (index, (left_pixel, right_pixel)) in
        left.pixels().iter().zip(right.pixels().iter()).enumerate()
    {
        if left_pixel == right_pixel {
            continue;
        }
        let x = index % left.width();
        let y = index / left.width();
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        found = true;
    }
    found.then_some((min_x, min_y, max_x, max_y))
}

fn foreground_ink_retention_ratio(
    normal: &Canvas,
    hovered: &Canvas,
    bounds: (usize, usize, usize, usize),
) -> usize {
    let normal_ink = foreground_ink_count(normal, bounds);
    let hovered_ink = foreground_ink_count(hovered, bounds);
    if normal_ink == 0 {
        return 0;
    }
    hovered_ink.saturating_mul(100) / normal_ink
}

fn foreground_ink_count(canvas: &Canvas, bounds: (usize, usize, usize, usize)) -> usize {
    let (left, top, width, height) = bounds;
    let background = dominant_color(canvas, bounds);
    let mut foreground = 0usize;
    for y in top..top.saturating_add(height).min(canvas.height()) {
        for x in left..left.saturating_add(width).min(canvas.width()) {
            let index = y * canvas.width() + x;
            if canvas.pixels()[index] != background {
                foreground = foreground.saturating_add(1);
            }
        }
    }
    foreground
}

fn dominant_color(canvas: &Canvas, bounds: (usize, usize, usize, usize)) -> u32 {
    let (left, top, width, height) = bounds;
    let mut counts = std::collections::BTreeMap::<u32, usize>::new();
    for y in top..top.saturating_add(height).min(canvas.height()) {
        for x in left..left.saturating_add(width).min(canvas.width()) {
            let color = canvas.pixels()[y * canvas.width() + x];
            let count = counts.entry(color).or_default();
            *count = count.saturating_add(1);
        }
    }
    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(color, _)| color)
        .unwrap_or(0)
}

fn inside_bounds(x: usize, y: usize, left: usize, top: usize, width: usize, height: usize) -> bool {
    x >= left && x < left.saturating_add(width) && y >= top && y < top.saturating_add(height)
}

fn fixture(path: &str) -> StorybookFixture {
    StorybookFixture {
        label: path.to_string(),
        path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../assets/fixtures/{path}")),
    }
}
