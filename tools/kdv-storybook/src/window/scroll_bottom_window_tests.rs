use super::StorybookWindow;
use crate::args::StorybookArgs;
use crate::canvas::Canvas;
use crate::catalog::{FixtureCatalog, StorybookFixture};
use crate::layout::{StorybookPreviewArea, preview_viewport_height};
use crate::preview::PreviewBuilder;
use crate::preview_theme_bridge::KucThemeBridge;
use katana_document_viewer::KdvThemeSnapshot;
use std::path::PathBuf;

const WINDOW_WIDTH: usize = 1000;
const WINDOW_HEIGHT: usize = 640;
const WIDE_WINDOW_WIDTH: usize = 1320;
const NARROW_WINDOW_WIDTH: usize = 720;
const PREVIEW_BACKGROUND: u32 = 0x151515;

#[test]
fn window_bottom_scroll_renders_tail_space_from_storybook_state()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("katana/sample_basic.md");
    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let max_scroll = max_scroll(&storybook, WINDOW_HEIGHT)?;

    assert!(storybook.apply_preview_scroll(-10_000.0, WINDOW_HEIGHT));
    assert_scroll_near(storybook.scroll_y, max_scroll, "bottom scroll");

    let canvas = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    let ratio = tail_background_ratio(&canvas, dark_theme_rgb("background")?);
    assert!(
        ratio > 90,
        "window bottom scroll must expose KatanA-style tail space: ratio={ratio}"
    );
    Ok(())
}

#[test]
fn window_scroll_to_bottom_keeps_diagram_asset_job_scope() -> Result<(), Box<dyn std::error::Error>>
{
    let mut storybook = storybook("katana/sample_diagrams.md");
    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let first_scene = storybook.scene.as_ref().ok_or("scene missing")?;
    let first_scope = first_scene.asset_request_key.clone();
    let first_job_key = storybook
        .asset_job
        .as_ref()
        .ok_or("asset job missing")?
        .key()
        .clone();
    let first_stats = storybook.preview.builder_cache_stats()?;

    assert!(storybook.apply_preview_scroll(-10_000.0, WINDOW_HEIGHT));
    let _canvas = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);

    let scrolled_scene = storybook.scene.as_ref().ok_or("scrolled scene missing")?;
    let scrolled_job_key = storybook
        .asset_job
        .as_ref()
        .ok_or("asset job missing after bottom scroll")?
        .key();
    let scrolled_stats = storybook.preview.builder_cache_stats()?;

    assert_eq!(first_scope, scrolled_scene.asset_request_key);
    assert_eq!(&first_job_key, scrolled_job_key);
    assert_eq!(
        first_stats.lazy_scene_misses, scrolled_stats.lazy_scene_misses,
        "bottom scroll must not rebuild the lazy scene"
    );
    Ok(())
}

#[test]
fn window_resize_width_preserves_bottom_anchor_when_already_at_tail()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = storybook("katana/sample_basic.md");
    storybook.update_scene(WIDE_WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let wide_max_scroll = max_scroll(&storybook, WINDOW_HEIGHT)?;

    assert!(storybook.apply_preview_scroll(-10_000.0, WINDOW_HEIGHT));
    assert_scroll_near(storybook.scroll_y, wide_max_scroll, "wide bottom scroll");

    storybook.update_scene(NARROW_WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let narrow_max_scroll = max_scroll(&storybook, WINDOW_HEIGHT)?;

    assert_scroll_near(
        storybook.scroll_y,
        narrow_max_scroll,
        "narrow resize bottom anchor",
    );
    let canvas = storybook.render_canvas(NARROW_WINDOW_WIDTH, WINDOW_HEIGHT);
    let ratio = tail_background_ratio(&canvas, dark_theme_rgb("background")?);
    assert!(
        ratio > 90,
        "resized bottom scroll must keep tail space visible: ratio={ratio}"
    );
    Ok(())
}

fn storybook(path: &str) -> StorybookWindow {
    StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![StorybookFixture {
                label: path.to_string(),
                path: fixture_path(path),
            }],
        },
        PreviewBuilder::default(),
    )
}

fn max_scroll(
    storybook: &StorybookWindow,
    window_height: usize,
) -> Result<f32, Box<dyn std::error::Error>> {
    let content_height = storybook
        .scene
        .as_ref()
        .ok_or("scene missing")?
        .content_height;
    Ok((content_height - preview_viewport_height(window_height) as f32).max(0.0))
}

fn assert_scroll_near(actual: f32, expected: f32, label: &str) {
    assert!(
        (actual - expected).abs() <= 0.5,
        "{label} mismatch: actual={actual} expected={expected}"
    );
}

fn tail_background_ratio(canvas: &Canvas, theme_background: u32) -> usize {
    let area = StorybookPreviewArea::for_window(canvas.width(), canvas.height(), 0.0);
    let mut background = 0usize;
    let mut total = 0usize;
    for y in area.y + area.height / 2..area.y + area.height {
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

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../assets/fixtures/{path}"))
}
