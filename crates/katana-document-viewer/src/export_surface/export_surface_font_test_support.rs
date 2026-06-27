use super::{Rgba, rendering};
use image::RgbaImage;

const HALF_WIDTH_SPACE_FACTOR: f32 = 0.35;
const HALF_WIDTH_PUNCTUATION_FACTOR: f32 = 0.43;
const HALF_WIDTH_TEXT_FACTOR: f32 = 0.54;
const HALF_WIDTH_MATH_FACTOR: f32 = 0.65;
const DEFAULT_WIDTH_FACTOR: f32 = 0.92;
const FONT_LINE_HEIGHT_MULTIPLIER: f32 = 1.45;
const TEST_BUFFER_WIDTH: f32 = 2048.0;
const FONT_BUFFER_HEIGHT_SCALE: f32 = 1.8;

pub(super) fn estimated_text_width(text: &str, size: f32) -> u32 {
    text.chars()
        .map(|character| character_width_factor(character) * size)
        .sum::<f32>()
        .ceil() as u32
}

fn character_width_factor(character: char) -> f32 {
    if character.is_ascii_whitespace() {
        return HALF_WIDTH_SPACE_FACTOR;
    }
    if character.is_ascii_punctuation() {
        return HALF_WIDTH_PUNCTUATION_FACTOR;
    }
    if character.is_ascii() {
        return HALF_WIDTH_TEXT_FACTOR;
    }
    if is_half_width_math_symbol(character) {
        return HALF_WIDTH_MATH_FACTOR;
    }
    DEFAULT_WIDTH_FACTOR
}

pub(super) fn is_half_width_math_symbol(character: char) -> bool {
    rendering::is_half_width_math_symbol(character)
}

pub(super) fn actual_span_x_range(
    spans: &[super::SurfaceTextSpan],
    span_index: usize,
    size: f32,
) -> Option<(u32, u32)> {
    let mut font_system = cosmic_text::FontSystem::new();
    let metrics = cosmic_text::Metrics::new(size, size * FONT_LINE_HEIGHT_MULTIPLIER);
    let mut buffer = cosmic_text::Buffer::new(&mut font_system, metrics);
    buffer.set_size(
        Some(TEST_BUFFER_WIDTH),
        Some(size * FONT_BUFFER_HEIGHT_SCALE),
    );
    let default_attrs = cosmic_text::Attrs::new();
    let rich = spans.iter().enumerate().map(|(index, span)| {
        (
            span.text.as_str(),
            rendering::attrs_for_span_with_metadata(span, index.saturating_add(1)),
        )
    });
    buffer.set_rich_text(rich, &default_attrs, cosmic_text::Shaping::Advanced, None);
    buffer.shape_until_scroll(&mut font_system, false);

    SpanRangeFinder::find(&buffer, span_index)
}

struct SpanRangeFinder;

impl SpanRangeFinder {
    fn find(buffer: &cosmic_text::Buffer, span_index: usize) -> Option<(u32, u32)> {
        let mut range = SpanRangeAccumulator::new(span_index.saturating_add(1));
        for run in buffer.layout_runs() {
            for glyph in run.glyphs {
                range.collect(glyph);
            }
        }
        range.into_u32_range()
    }
}

struct SpanRangeAccumulator {
    target_metadata: usize,
    min_x: Option<f32>,
    max_x: Option<f32>,
}

impl SpanRangeAccumulator {
    fn new(target_metadata: usize) -> Self {
        Self {
            target_metadata,
            min_x: None,
            max_x: None,
        }
    }

    fn collect(&mut self, glyph: &cosmic_text::LayoutGlyph) {
        if glyph.metadata != self.target_metadata {
            return;
        }
        self.min_x = Some(self.min_x.map_or(glyph.x, |current| current.min(glyph.x)));
        self.max_x = Some(
            self.max_x
                .map_or(glyph.x + glyph.w, |current| current.max(glyph.x + glyph.w)),
        );
    }

    fn into_u32_range(self) -> Option<(u32, u32)> {
        Some((self.min_x?.floor() as u32, self.max_x?.ceil() as u32))
    }
}

pub(super) fn painted_x_range(image: &RgbaImage, color: Rgba<u8>) -> Option<(u32, u32)> {
    let mut min_x = None;
    let mut max_x = None;
    for (x, _, pixel) in image.enumerate_pixels() {
        if *pixel != color {
            continue;
        }
        min_x = Some(min_x.map_or(x, |current: u32| current.min(x)));
        max_x = Some(max_x.map_or(x, |current: u32| current.max(x)));
    }
    Some((min_x?, max_x?))
}

#[cfg(test)]
#[path = "export_surface_font_test_support_tests.rs"]
mod tests;
