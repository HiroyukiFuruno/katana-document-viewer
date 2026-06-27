use super::export_surface_font_rendering as rendering;
use cosmic_text::Color;
use image::Rgba;

const FONT_COLOR_RED_CHANNEL: usize = 0;
const FONT_COLOR_GREEN_CHANNEL: usize = 1;
const FONT_COLOR_BLUE_CHANNEL: usize = 2;
const FONT_COLOR_ALPHA_CHANNEL: usize = 3;

pub(super) fn buffer_text_color(color: Rgba<u8>) -> Color {
    Color::rgba(
        color[FONT_COLOR_RED_CHANNEL],
        color[FONT_COLOR_GREEN_CHANNEL],
        color[FONT_COLOR_BLUE_CHANNEL],
        color[FONT_COLOR_ALPHA_CHANNEL],
    )
}

pub(super) fn span_ranges_width(ranges: &[Option<rendering::SpanVisualRange>]) -> u32 {
    let mut min_x = u32::MAX;
    let mut max_x = 0;
    for range in ranges.iter().flatten() {
        min_x = min_x.min(range.start_x);
        max_x = max_x.max(range.end_x());
    }
    if min_x == u32::MAX {
        return 0;
    }
    max_x.saturating_sub(min_x)
}
