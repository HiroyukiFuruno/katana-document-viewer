use super::{FrameRenderRequest, StorybookFrameRenderer};
use crate::canvas::Canvas;
use crate::catalog::StorybookFixture;
use crate::frame_pixel_guard::StorybookFramePixelGuard;
use crate::layout::StorybookPreviewArea;
use crate::preview::PreviewBuilder;
use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use crate::preview_theme_bridge::KucThemeBridge;
use katana_document_viewer::KdvThemeSnapshot;
use katana_document_viewer::{
    ViewerInteractionConfig, ViewerMode, ViewerSearchState, ViewerViewport,
};
use std::path::PathBuf;

const FRAME_WIDTH: usize = 1280;
const FRAME_HEIGHT: usize = 720;
const COMPACT_FRAME_HEIGHT: usize = 640;
const PREVIEW_WIDTH: f32 = 900.0;
const PREVIEW_BACKGROUND: u32 = 0x151515;

#[test]
fn scrolled_katana_sample_frame_changes_preview_without_overdrawing_shell()
-> Result<(), Box<dyn std::error::Error>> {
    let top = ScrollFrameTestSupport::render_fixture("katana/sample.md", 0.0)?;
    let scrolled = ScrollFrameTestSupport::render_fixture("katana/sample.md", 480.0)?;

    assert!(
        preview_diff_pixel_count(&top, &scrolled) > 512,
        "scroll did not move visible preview content"
    );
    assert!(
        StorybookFramePixelGuard::preview_content_pixel_count(&scrolled, true) > 256,
        "scrolled preview content disappeared"
    );
    assert_shell_unchanged(&top, &scrolled);
    Ok(())
}

#[test]
fn bottom_scroll_shows_viewport_tail_space() -> Result<(), Box<dyn std::error::Error>> {
    assert_bottom_tail_space("katana/sample.md")?;
    assert_bottom_tail_space("katana/sample_basic.md")?;
    Ok(())
}

#[test]
fn bottom_scroll_aligns_last_target_top_before_tail_space() -> Result<(), Box<dyn std::error::Error>>
{
    assert_last_target_top_at_bottom_scroll("katana/sample.md")?;
    assert_last_target_top_at_bottom_scroll("katana/sample_basic.md")?;
    Ok(())
}

fn assert_bottom_tail_space(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let prepared = ScrollFrameTestSupport::prepare_scene(path, COMPACT_FRAME_HEIGHT)?;
    let bottom_scroll = (prepared.content_height - preview_height(COMPACT_FRAME_HEIGHT)).max(0.0);
    let bottom = ScrollFrameTestSupport::render_prepared(prepared, bottom_scroll);
    let theme_background = dark_theme_rgb("background")?;
    let ratio = tail_background_ratio(&bottom, theme_background);

    assert!(
        ratio > 90,
        "bottom scroll did not expose stable viewport tail space: ratio={ratio}"
    );
    Ok(())
}

fn assert_last_target_top_at_bottom_scroll(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let prepared = ScrollFrameTestSupport::prepare_scene(path, COMPACT_FRAME_HEIGHT)?;
    let last_target = prepared
        .scene
        .targets
        .iter()
        .max_by(|left, right| {
            (left.rect.y + left.rect.height)
                .partial_cmp(&(right.rect.y + right.rect.height))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .ok_or("last target missing")?;
    let last_node_id = last_target.node_id.0.clone();
    let last_rect = last_target.rect;
    let bottom_scroll = (prepared.content_height - preview_height(COMPACT_FRAME_HEIGHT)).max(0.0);

    assert!(
        (bottom_scroll - last_rect.y).abs() <= 0.5,
        "bottom scroll must align the last target with the preview top: path={path} bottom_scroll={bottom_scroll} node_id={} last_rect={:?}",
        last_node_id,
        last_rect
    );
    Ok(())
}

#[test]
fn same_scroll_renders_identically_for_rebuilt_scroll_independent_scene()
-> Result<(), Box<dyn std::error::Error>> {
    let stale_scene = ScrollFrameTestSupport::prepare_scene("katana/sample.md", FRAME_HEIGHT)?;
    let rebuilt_scene = ScrollFrameTestSupport::prepare_scene("katana/sample.md", FRAME_HEIGHT)?;

    let stale_canvas = ScrollFrameTestSupport::render_prepared(stale_scene, 240.0);
    let rebuilt_canvas = ScrollFrameTestSupport::render_prepared(rebuilt_scene, 240.0);

    assert_eq!(
        0,
        preview_diff_pixel_count(&stale_canvas, &rebuilt_canvas),
        "scroll offset was applied differently after scene rebuild"
    );
    Ok(())
}

#[test]
fn pending_asset_animation_does_not_disable_scroll_delta_redraw()
-> Result<(), Box<dyn std::error::Error>> {
    let prepared =
        ScrollFrameTestSupport::prepare_scene("katana/sample_diagrams.md", FRAME_HEIGHT)?;
    let mut canvas = ScrollFrameTestSupport::render_prepared(prepared.clone(), 480.0);
    let interaction = ViewerInteractionConfig::default();
    let settings_state = Default::default();
    let request = FrameRenderRequest {
        width: FRAME_WIDTH,
        height: prepared.frame_height,
        fixtures: std::slice::from_ref(&prepared.fixture),
        selected_index: 0,
        scene: Some(&prepared.scene),
        scroll_y: 528.0,
        sidebar_scroll: Default::default(),
        file_tree_state: Default::default(),
        settings_state: &settings_state,
        dark: true,
        interaction: &interaction,
        typography: Default::default(),
        last_command_label: "none",
        task_context_menu: None,
        hovered_node_id: None,
        hovered_action_node_id: None,
        animation_phase: 1,
    };

    assert!(
        StorybookFrameRenderer::redraw_preview_scroll_delta(&mut canvas, &request, 480.0),
        "pending asset animation must not force full preview redraw during scroll"
    );
    Ok(())
}

struct ScrollFrameTestSupport;

impl ScrollFrameTestSupport {
    fn render_fixture(path: &str, scroll_y: f32) -> Result<Canvas, Box<dyn std::error::Error>> {
        let prepared = Self::prepare_scene(path, FRAME_HEIGHT)?;
        Ok(Self::render_prepared(prepared, scroll_y))
    }

    fn prepare_scene(
        path: &str,
        frame_height: usize,
    ) -> Result<PreparedFrame, Box<dyn std::error::Error>> {
        let fixture = Self::fixture(path);
        let scene = PreviewBuilder::default().build_scene(PreviewBuildRequest {
            fixture: &fixture,
            viewport: ViewerViewport {
                width: PREVIEW_WIDTH,
                height: preview_height(frame_height),
            },
            dark: true,
            theme: None,
            interaction: ViewerInteractionConfig::default(),
            mode: ViewerMode::Document,
            typography: Default::default(),
            search: ViewerSearchState::default(),
            diagram_viewports: Default::default(),
            image_viewports: Default::default(),
            task_state_overrides: Default::default(),
            accordion_open_overrides: Default::default(),
            copied_code_node_ids: Default::default(),
            asset_mode: PreviewBuildAssetMode::Lazy,
            attach_surface: false,
            export_surface: false,
        })?;
        Ok(PreparedFrame {
            fixture,
            content_height: scene.content_height,
            frame_height,
            scene,
        })
    }

    fn render_prepared(prepared: PreparedFrame, scroll_y: f32) -> Canvas {
        StorybookFrameRenderer::render(FrameRenderRequest {
            width: FRAME_WIDTH,
            height: prepared.frame_height,
            fixtures: &[prepared.fixture],
            selected_index: 0,
            scene: Some(&prepared.scene),
            scroll_y,
            sidebar_scroll: Default::default(),
            file_tree_state: Default::default(),
            settings_state: &Default::default(),
            dark: true,
            interaction: &ViewerInteractionConfig::default(),
            typography: Default::default(),
            last_command_label: "none",
            task_context_menu: None,
            hovered_node_id: None,
            hovered_action_node_id: None,
            animation_phase: 0,
        })
    }

    fn fixture(path: &str) -> StorybookFixture {
        StorybookFixture {
            label: path.to_string(),
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(format!("../../assets/fixtures/{path}")),
        }
    }
}

struct PreparedFrame {
    fixture: StorybookFixture,
    scene: crate::preview::PreviewScene,
    content_height: f32,
    frame_height: usize,
}

impl Clone for PreparedFrame {
    fn clone(&self) -> Self {
        Self {
            fixture: self.fixture.clone(),
            scene: self.scene.clone(),
            content_height: self.content_height,
            frame_height: self.frame_height,
        }
    }
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

fn tail_background_ratio(canvas: &Canvas, theme_background: u32) -> usize {
    let mut background = 0;
    let mut total = 0;
    let area = StorybookPreviewArea::for_window(canvas.width(), canvas.height(), 0.0);
    let start_y = area.y + area.height / 2;
    let end_y = area.y + area.height;
    for y in start_y..end_y {
        for x in area.x..area.x + area.width {
            total += 1;
            if is_blank_tail_pixel(canvas.pixels()[y * canvas.width() + x], theme_background) {
                background += 1;
            }
        }
    }
    if total == 0 {
        return 0;
    }
    background * 100 / total
}

fn is_blank_tail_pixel(pixel: u32, theme_background: u32) -> bool {
    pixel == PREVIEW_BACKGROUND || pixel == theme_background
}

fn dark_theme_rgb(name: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let theme = KucThemeBridge::from_kdv(&KdvThemeSnapshot::katana_dark())?;
    let rgba = theme
        .color(name)
        .ok_or_else(|| format!("missing KUC theme color token: {name}"))?;
    Ok(((rgba[0] as u32) << 16) | ((rgba[1] as u32) << 8) | rgba[2] as u32)
}

fn assert_shell_unchanged(shell: &Canvas, actual: &Canvas) {
    for (index, (shell_pixel, actual_pixel)) in shell
        .pixels()
        .iter()
        .zip(actual.pixels().iter())
        .enumerate()
    {
        if inside_preview(index, shell.width()) {
            continue;
        }
        assert_eq!(
            shell_pixel, actual_pixel,
            "preview rendering changed shell pixel index {index}"
        );
    }
}

fn inside_preview(index: usize, width: usize) -> bool {
    let x = index % width;
    let y = index / width;
    let area = StorybookPreviewArea::for_window(width, COMPACT_FRAME_HEIGHT, 0.0);
    x >= area.x && y >= area.y
}

fn preview_height(frame_height: usize) -> f32 {
    StorybookPreviewArea::for_window(FRAME_WIDTH, frame_height, 0.0).height as f32
}
