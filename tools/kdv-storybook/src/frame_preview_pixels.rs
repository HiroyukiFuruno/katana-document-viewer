use crate::canvas::Canvas;
use crate::layout::{HEADER_HEIGHT, SIDEBAR_WIDTH};
use crate::palette::StorybookPalette;

pub(crate) struct FramePreviewPixels;

impl FramePreviewPixels {
    pub(crate) fn count_color(canvas: &Canvas, color: u32) -> usize {
        let mut count = 0;
        for y in Self::y_range(canvas) {
            count += Self::row_color_count(canvas, y, color);
        }
        count
    }

    pub(crate) fn count_non_background(canvas: &Canvas, dark: bool) -> usize {
        let background = StorybookPalette::new(dark).preview_background();
        let mut count = 0;
        for y in Self::y_range(canvas) {
            count += Self::x_range(canvas)
                .filter(|x| canvas.pixels()[y * canvas.width() + *x] != background)
                .count();
        }
        count
    }

    fn row_color_count(canvas: &Canvas, y: usize, color: u32) -> usize {
        Self::x_range(canvas)
            .filter(|x| canvas.pixels()[y * canvas.width() + *x] == color)
            .count()
    }

    fn x_range(canvas: &Canvas) -> std::ops::Range<usize> {
        let start = SIDEBAR_WIDTH.saturating_add(16).min(canvas.width());
        let end = canvas.width().saturating_sub(16).max(start);
        start..end
    }

    fn y_range(canvas: &Canvas) -> std::ops::Range<usize> {
        let start = HEADER_HEIGHT.saturating_add(16).min(canvas.height());
        let end = canvas.height().saturating_sub(32).max(start);
        start..end
    }
}
