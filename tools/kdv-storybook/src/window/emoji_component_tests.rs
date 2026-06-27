use crate::args::StorybookArgs;
use crate::catalog::{FixtureCatalog, StorybookFixture};
use crate::layout::{HEADER_HEIGHT, SIDEBAR_WIDTH};
use crate::mouse::mouse_test_support::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::preview::PreviewBuilder;
use katana_document_viewer::ViewerTarget;
use katana_ui_core::render_model::UiTreeSemantics;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const MIN_EMOJI_SPANS: usize = 8;
const MIN_CHROMATIC_EMOJI_PIXELS: usize = 32;

#[test]
fn storybook_frame_preserves_os_color_emoji_pixels() -> Result<(), Box<dyn std::error::Error>> {
    let source = TempMarkdown::new("# Emoji\n\nEmoji: 🦀 ⚡ 📝 🔧 ✅ ❌ ⚠️ 💡\n")?;
    let mut storybook = super::StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![source.fixture()],
        },
        PreviewBuilder::default(),
    );

    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let scene = storybook.scene.as_ref().ok_or("scene missing")?;
    let emoji_spans = UiTreeSemantics::emoji_span_count(scene.tree.root());
    assert!(
        emoji_spans >= MIN_EMOJI_SPANS,
        "KDV must preserve emoji spans for KUC OS emoji rendering: {emoji_spans}"
    );

    let canvas = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    let chromatic_pixels = chromatic_preview_pixel_count(&canvas);
    assert!(
        chromatic_pixels > MIN_CHROMATIC_EMOJI_PIXELS,
        "Storybook frame must keep OS color emoji pixels instead of monochrome glyphs: {chromatic_pixels}"
    );
    Ok(())
}

#[test]
fn katana_sample_basic_special_characters_preserve_os_color_emoji_pixels()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = super::StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![real_katana_fixture("katana/sample_basic.md")],
        },
        PreviewBuilder::default(),
    );

    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let target = storybook
        .scene
        .as_ref()
        .ok_or("scene missing")?
        .targets
        .iter()
        .find(|target| target.source.raw.text.contains("Emoji: 🦀"))
        .cloned()
        .ok_or("KatanA sample_basic emoji target missing")?;
    storybook.scroll_y = (target.rect.y - 80.0).max(0.0);
    let canvas = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    let chromatic_pixels = chromatic_target_pixel_count(&canvas, &target, storybook.scroll_y);

    assert!(
        chromatic_pixels > MIN_CHROMATIC_EMOJI_PIXELS,
        "KatanA sample_basic Special Characters emoji must render as OS color emoji pixels: {chromatic_pixels}"
    );
    Ok(())
}

#[test]
fn katana_sample_special_characters_multiline_selection_uses_text_flow()
-> Result<(), Box<dyn std::error::Error>> {
    let mut storybook = super::StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![real_katana_fixture("katana/sample.md")],
        },
        PreviewBuilder::default(),
    );

    storybook.update_scene(WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let target = storybook
        .scene
        .as_ref()
        .ok_or("scene missing")?
        .targets
        .iter()
        .find(|target| target.source.raw.text.contains("Emoji: 🦀"))
        .cloned()
        .ok_or("KatanA sample Special Characters emoji target missing")?;
    storybook.scroll_y = (target.rect.y - 120.0).max(0.0);

    let canvas = storybook.render_canvas(WINDOW_WIDTH, WINDOW_HEIGHT);
    let html = text_run_bounds_containing(&canvas, "HTML entities")
        .ok_or("HTML entities text run missing")?;
    let math = text_run_bounds_containing(&canvas, "Math symbols")
        .ok_or("Math symbols text run missing")?;

    let start = (html.x + html.width / 2, html.y + html.height / 2);
    let end = (math.x + math.width / 3, math.y + math.height / 2);
    storybook.set_text_selection_for_tests(start, end);
    let payload = storybook
        .selected_text_payload_for_tests(WINDOW_WIDTH, WINDOW_HEIGHT)
        .ok_or("selection payload missing")?;

    assert!(
        payload.contains("Japanese: こんにちは世界"),
        "middle Special Characters lines must be selected as text flow, not endpoint x-slices: {payload:?}"
    );
    assert!(
        payload.contains("Emoji:"),
        "middle Special Characters emoji label must remain selectable as a complete visual line: {payload:?}"
    );
    Ok(())
}

fn chromatic_preview_pixel_count(canvas: &crate::canvas::Canvas) -> usize {
    let x_start = SIDEBAR_WIDTH.saturating_add(16).min(canvas.width());
    let x_end = canvas.width().saturating_sub(16).max(x_start);
    let y_start = HEADER_HEIGHT.saturating_add(16).min(canvas.height());
    let y_end = canvas.height().saturating_sub(32).max(y_start);
    let mut count = 0;
    for y in y_start..y_end {
        for x in x_start..x_end {
            count += usize::from(is_chromatic(canvas.pixels()[y * canvas.width() + x]));
        }
    }
    count
}

fn chromatic_target_pixel_count(
    canvas: &crate::canvas::Canvas,
    target: &ViewerTarget,
    scroll_y: f32,
) -> usize {
    let x_start = SIDEBAR_WIDTH
        .saturating_add(16)
        .saturating_add(target.rect.x.max(0.0).round() as usize)
        .min(canvas.width());
    let y_start = HEADER_HEIGHT
        .saturating_add(16)
        .saturating_add((target.rect.y - scroll_y).max(0.0).round() as usize)
        .min(canvas.height());
    let x_end = x_start
        .saturating_add(target.rect.width.max(0.0).round() as usize)
        .min(canvas.width());
    let y_end = y_start
        .saturating_add(target.rect.height.max(0.0).round() as usize)
        .min(canvas.height());
    let mut count = 0;
    for y in y_start..y_end {
        for x in x_start..x_end {
            count += usize::from(is_chromatic(canvas.pixels()[y * canvas.width() + x]));
        }
    }
    count
}

fn is_chromatic(pixel: u32) -> bool {
    let red = (pixel >> 16) & 0xff;
    let green = (pixel >> 8) & 0xff;
    let blue = pixel & 0xff;
    red != green || green != blue
}

fn real_katana_fixture(label: &str) -> StorybookFixture {
    StorybookFixture {
        label: label.to_string(),
        path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../assets/fixtures/{label}")),
    }
}

struct TextRunBounds {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn text_run_bounds_containing(
    canvas: &crate::canvas::Canvas,
    needle: &str,
) -> Option<TextRunBounds> {
    canvas
        .text_runs()
        .iter()
        .find(|run| run.text().contains(needle))
        .map(|run| TextRunBounds {
            x: run.x(),
            y: run.y(),
            width: run.width(),
            height: run.height(),
        })
}

struct TempMarkdown {
    path: PathBuf,
}

impl TempMarkdown {
    fn new(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let created_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let path = std::env::temp_dir().join(format!("kdv-storybook-emoji-{created_at}.md"));
        std::fs::write(&path, content)?;
        Ok(Self { path })
    }

    fn fixture(&self) -> StorybookFixture {
        StorybookFixture {
            label: "test/emoji.md".to_string(),
            path: self.path.clone(),
        }
    }
}

impl Drop for TempMarkdown {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
