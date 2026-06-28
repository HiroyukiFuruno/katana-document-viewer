use super::{FrameRenderRequest, StorybookFrameRenderer};
use crate::canvas::Canvas;
use crate::catalog::StorybookFixture;
use crate::layout::{HEADER_HEIGHT, preview_content_width};
use crate::preview::{PreviewBuilder, PreviewScene};
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use std::path::PathBuf;

#[path = "frame_html_alignment_support.rs"]
mod support;

use support::{PreviewBounds, TextBand, TextBandCollector};

const FRAME_HEIGHT: usize = 960;
const NARROW_FRAME_WIDTH: usize = 1280;
const WIDE_FRAME_WIDTH: usize = 1600;

#[test]
fn html_alignment_uses_preview_area_positions() -> Result<(), Box<dyn std::error::Error>> {
    let narrow = HtmlAlignmentFrameSupport::render(NARROW_FRAME_WIDTH)?;
    let wide = HtmlAlignmentFrameSupport::render(WIDE_FRAME_WIDTH)?;
    let narrow_bands = HtmlAlignmentFrameSupport::semantic_bands(&narrow.canvas)?;
    let wide_bands = HtmlAlignmentFrameSupport::semantic_bands(&wide.canvas)?;
    let narrow_preview = PreviewBounds::for_frame(NARROW_FRAME_WIDTH, narrow.canvas.height());
    let wide_preview = PreviewBounds::for_frame(WIDE_FRAME_WIDTH, wide.canvas.height());

    narrow_bands
        .centered_heading
        .assert_centered(narrow_preview);
    narrow_bands
        .centered_subheading
        .assert_centered(narrow_preview);
    narrow_bands
        .centered_paragraph
        .assert_centered(narrow_preview);
    narrow_bands.right_link.assert_right_aligned(narrow_preview);
    narrow_bands
        .right_uppercase_link
        .assert_right_aligned(narrow_preview);
    narrow_bands.left_text.assert_left_aligned(narrow_preview);
    wide_bands.centered_heading.assert_centered(wide_preview);
    wide_bands.right_link.assert_right_aligned(wide_preview);
    let preview_center_delta = wide_preview.center_x() - narrow_preview.center_x();
    assert!(
        (wide_bands.centered_heading.center_x()
            - narrow_bands.centered_heading.center_x()
            - preview_center_delta)
            .abs()
            <= 4.0,
        "centered HTML text did not follow viewport width: narrow={:?} wide={:?}",
        narrow_bands.centered_heading,
        wide_bands.centered_heading
    );
    let preview_right_delta = wide_preview.right() - narrow_preview.right();
    assert!(
        wide_bands
            .right_link
            .max_x
            .abs_diff(narrow_bands.right_link.max_x + preview_right_delta)
            <= 4,
        "right HTML text did not follow viewport width: narrow={:?} wide={:?}",
        narrow_bands.right_link,
        wide_bands.right_link
    );
    Ok(())
}

#[test]
fn katana_markdown_html_alignment_reaches_frame_positions() -> Result<(), Box<dyn std::error::Error>>
{
    let sample =
        HtmlAlignmentFrameSupport::render_fixture(WIDE_FRAME_WIDTH, "katana/sample_html.md")?;
    let bands = HtmlAlignmentFrameSupport::text_bands(&sample.canvas);
    let preview = PreviewBounds::for_frame(WIDE_FRAME_WIDTH, sample.canvas.height());
    let centered_count = bands
        .iter()
        .filter(|band| band.is_centered(preview))
        .count();

    assert!(
        centered_count >= 5,
        "expected KatanA markdown HTML blocks to keep centered frame positions: centered_count={centered_count} bands={bands:?}"
    );
    Ok(())
}

#[test]
fn katana_language_link_underline_reaches_frame_pixels() -> Result<(), Box<dyn std::error::Error>> {
    let sample = HtmlAlignmentFrameSupport::render_fixture(WIDE_FRAME_WIDTH, "katana/sample.md")?;
    let bands = HtmlAlignmentFrameSupport::text_bands(&sample.canvas);
    let preview = PreviewBounds::for_frame(WIDE_FRAME_WIDTH, sample.canvas.height());
    let link_band = bands
        .iter()
        .copied()
        .find_map(|band| link_blue_bounds(&sample.canvas, band).map(|bounds| (band, bounds)))
        .ok_or_else(|| format!("expected language link underline/text blue pixels: {bands:?}"))?;

    let (band, blue) = link_band;
    assert!(
        band.is_centered(preview),
        "language selector row must stay centered in preview: band={band:?} preview={preview:?}"
    );
    assert!(
        blue.count >= 80,
        "language link must paint enough link-blue pixels for text and underline: blue={blue:?} band={band:?}"
    );
    assert!(
        (20..=140).contains(&blue.width()),
        "language link underline must stay near the linked label, not cover the whole row: blue={blue:?} band={band:?}"
    );
    Ok(())
}

#[test]
fn katana_intro_text_keeps_readable_frame_band_heights() -> Result<(), Box<dyn std::error::Error>> {
    let sample = HtmlAlignmentFrameSupport::render_fixture(WIDE_FRAME_WIDTH, "katana/sample.md")?;
    let bands = HtmlAlignmentFrameSupport::text_bands(&sample.canvas);
    if bands.len() < 3 {
        return Err(format!("expected title and wrapped body text bands: {bands:?}").into());
    }
    let title = bands[0];
    let body_first_line = bands[1];
    let body_second_line = bands[2];

    assert!(
        title.height() >= 20,
        "KatanA title text band is too short and likely crushed: title={title:?}"
    );
    assert!(
        body_first_line.height() >= 12,
        "KatanA body text first line is too short and likely crushed: body={body_first_line:?}"
    );
    assert!(
        body_second_line.height() >= 12,
        "KatanA body text second line is too short and likely crushed: body={body_second_line:?}"
    );
    Ok(())
}

#[test]
fn direct_html_margin_left_fixture_reaches_frame_pixels() -> Result<(), Box<dyn std::error::Error>>
{
    let rendered = HtmlAlignmentFrameSupport::render_fixture(
        WIDE_FRAME_WIDTH,
        "direct/html-margin-left.html",
    )?;
    let bands = HtmlAlignmentFrameSupport::text_bands(&rendered.canvas);
    if bands.len() < 4 {
        return Err(format!("expected heading, baseline, and indented bands: {bands:?}").into());
    }
    let baseline = bands[1];
    let indented_link = bands[2];
    let indented_text = bands[3];
    let link_delta = indented_link.min_x().saturating_sub(baseline.min_x());
    let text_delta = indented_text.min_x().saturating_sub(baseline.min_x());
    let text_over_link_delta = indented_text.min_x().saturating_sub(indented_link.min_x());

    assert!(
        (75..=85).contains(&link_delta),
        "margin-left:80px link must render at the logical CSS offset: baseline={baseline:?} link={indented_link:?} delta={link_delta}"
    );
    assert!(
        (115..=125).contains(&text_delta),
        "margin-left:120px text must render at the logical CSS offset: baseline={baseline:?} text={indented_text:?} delta={text_delta}"
    );
    assert!(
        (35..=45).contains(&text_over_link_delta),
        "margin-left:120px should be 40px beyond margin-left:80px: link={indented_link:?} text={indented_text:?} delta={text_over_link_delta}"
    );
    assert!(
        count_link_blue_pixels(&rendered.canvas, indented_link) >= 80,
        "margin-left link underline/text must reach frame as link-blue pixels: link={indented_link:?}"
    );
    Ok(())
}

fn count_link_blue_pixels(canvas: &Canvas, band: TextBand) -> usize {
    link_blue_bounds(canvas, band)
        .map(|bounds| bounds.count)
        .unwrap_or(0)
}

#[derive(Clone, Copy, Debug)]
struct LinkBlueBounds {
    min_x: usize,
    max_x: usize,
    count: usize,
}

impl LinkBlueBounds {
    fn width(self) -> usize {
        self.max_x - self.min_x + 1
    }
}

fn link_blue_bounds(canvas: &Canvas, band: TextBand) -> Option<LinkBlueBounds> {
    let mut bounds: Option<LinkBlueBounds> = None;
    let mut count = 0usize;
    for y in band.min_y()..=band.max_y {
        for x in band.min_x()..=band.max_x {
            let pixel = canvas.pixels()[y * canvas.width() + x];
            if is_link_blue_pixel(pixel) {
                count += 1;
                bounds = Some(match bounds {
                    Some(current) => LinkBlueBounds {
                        min_x: current.min_x.min(x),
                        max_x: current.max_x.max(x),
                        count,
                    },
                    None => LinkBlueBounds {
                        min_x: x,
                        max_x: x,
                        count,
                    },
                });
            }
        }
    }
    bounds
}

fn is_link_blue_pixel(pixel: u32) -> bool {
    let red = ((pixel >> 16) & 0xff) as i32;
    let green = ((pixel >> 8) & 0xff) as i32;
    let blue = (pixel & 0xff) as i32;
    blue > 90 && blue > red + 20 && blue > green + 5
}

struct HtmlAlignmentFrameSupport;

impl HtmlAlignmentFrameSupport {
    fn render(frame_width: usize) -> Result<RenderedHtmlFixture, Box<dyn std::error::Error>> {
        Self::render_fixture(frame_width, "direct/html-alignment.html")
    }

    fn render_fixture(
        frame_width: usize,
        path: &str,
    ) -> Result<RenderedHtmlFixture, Box<dyn std::error::Error>> {
        let fixture = Self::fixture(path);
        let scene = PreviewBuilder::default().build(
            &fixture,
            ViewerViewport {
                width: Self::preview_width(frame_width) as f32,
                height: Self::preview_height(FRAME_HEIGHT) as f32,
            },
            true,
            ViewerInteractionConfig::default(),
        )?;
        let canvas = Self::render_scene(frame_width, &fixture, &scene);
        Ok(RenderedHtmlFixture { canvas })
    }

    fn render_scene(
        frame_width: usize,
        fixture: &StorybookFixture,
        scene: &PreviewScene,
    ) -> Canvas {
        StorybookFrameRenderer::render(FrameRenderRequest {
            width: frame_width,
            height: FRAME_HEIGHT,
            fixtures: std::slice::from_ref(fixture),
            selected_index: 0,
            scene: Some(scene),
            scroll_y: 0.0,
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

    fn semantic_bands(canvas: &Canvas) -> Result<HtmlAlignmentBands, Box<dyn std::error::Error>> {
        let bands = Self::text_bands(canvas);
        HtmlAlignmentBands::new(&bands)
    }

    fn text_bands(canvas: &Canvas) -> Vec<TextBand> {
        TextBandCollector::new(canvas, PreviewBounds::for_canvas(canvas)).collect()
    }

    fn fixture(path: &str) -> StorybookFixture {
        StorybookFixture {
            label: path.to_string(),
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../assets/fixtures")
                .join(path),
        }
    }

    fn preview_width(frame_width: usize) -> usize {
        preview_content_width(frame_width)
    }

    fn preview_height(frame_height: usize) -> usize {
        frame_height.saturating_sub(HEADER_HEIGHT + 32)
    }
}

#[derive(Clone, Copy, Debug)]
struct HtmlAlignmentBands {
    centered_heading: TextBand,
    centered_subheading: TextBand,
    centered_paragraph: TextBand,
    right_link: TextBand,
    right_uppercase_link: TextBand,
    left_text: TextBand,
}

impl HtmlAlignmentBands {
    fn new(bands: &[TextBand]) -> Result<Self, Box<dyn std::error::Error>> {
        if bands.len() < 6 {
            return Err(format!("expected at least 6 HTML text bands, got {bands:?}").into());
        }
        Ok(Self {
            centered_heading: bands[0],
            centered_subheading: bands[1],
            centered_paragraph: bands[2],
            right_link: bands[3],
            right_uppercase_link: bands[4],
            left_text: bands[5],
        })
    }
}

struct RenderedHtmlFixture {
    canvas: Canvas,
}
