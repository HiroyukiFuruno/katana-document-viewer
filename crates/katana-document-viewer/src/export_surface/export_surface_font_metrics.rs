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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_text_color_returns_exact_rgba_channels() {
        let color = buffer_text_color(Rgba([10, 20, 30, 40]));
        assert_eq!(color.r(), 10);
        assert_eq!(color.g(), 20);
        assert_eq!(color.b(), 30);
        assert_eq!(color.a(), 40);
    }

    #[test]
    fn span_ranges_width_uses_min_and_max_boundaries() {
        let ranges = vec![
            Some(rendering::SpanVisualRange::new(10.2, 20.8)),
            Some(rendering::SpanVisualRange::new(5.1, 25.0)),
            None,
        ];

        assert_eq!(span_ranges_width(&ranges), 20);
    }

    #[test]
    fn span_ranges_width_is_zero_when_empty() {
        assert_eq!(span_ranges_width(&[]), 0);
    }
}
