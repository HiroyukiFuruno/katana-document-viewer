use crate::canvas::{Canvas, SurfaceArea};
use crate::layout::{HEADER_HEIGHT, SIDEBAR_WIDTH};
use crate::palette::StorybookPalette;

const CONTENT_BOTTOM_RESERVED: usize = 40;
const MIN_CONTENT_PIXELS: usize = 32;
const MIN_CONTENT_WIDTH: usize = 8;
const MIN_CONTENT_HEIGHT: usize = 4;

pub struct StorybookFramePixelGuard;

impl StorybookFramePixelGuard {
    pub fn assert_fixture_content(
        label: &str,
        canvas: &Canvas,
        dark: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let shape = Self::preview_content_shape(canvas, dark);
        if shape.is_valid_content() {
            return Ok(());
        }
        Err(format!("storybook fixture has no shaped preview content: {label}").into())
    }

    #[cfg(test)]
    pub(crate) fn preview_content_pixel_count(canvas: &Canvas, dark: bool) -> usize {
        Self::preview_content_shape(canvas, dark).pixels
    }

    pub(crate) fn preview_content_shape(canvas: &Canvas, dark: bool) -> PreviewContentShape {
        let background = StorybookPalette::new(dark).preview_background();
        let area = Self::preview_content_area(canvas.width(), canvas.height());
        let mut shape = PreviewContentShape::default();
        for y in area.y..area.y.saturating_add(area.height) {
            let row = y.saturating_mul(canvas.width());
            for x in area.x..area.x.saturating_add(area.width) {
                if canvas.pixels()[row + x] != background {
                    shape.include(x, y);
                }
            }
        }
        shape
    }

    fn preview_content_area(width: usize, height: usize) -> SurfaceArea {
        let x = SIDEBAR_WIDTH + 16;
        let y = HEADER_HEIGHT + 16;
        let bottom = height.saturating_sub(CONTENT_BOTTOM_RESERVED);
        SurfaceArea {
            x,
            y,
            width: width.saturating_sub(SIDEBAR_WIDTH + 32),
            height: bottom.saturating_sub(y),
            scroll_y: 0.0,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PreviewContentShape {
    pub(crate) pixels: usize,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl PreviewContentShape {
    fn include(&mut self, x: usize, y: usize) {
        if self.pixels == 0 {
            self.min_x = x;
            self.max_x = x;
            self.min_y = y;
            self.max_y = y;
        } else {
            self.min_x = self.min_x.min(x);
            self.max_x = self.max_x.max(x);
            self.min_y = self.min_y.min(y);
            self.max_y = self.max_y.max(y);
        }
        self.pixels += 1;
    }

    fn is_valid_content(&self) -> bool {
        self.pixels >= MIN_CONTENT_PIXELS
            && self.width() >= MIN_CONTENT_WIDTH
            && self.height() >= MIN_CONTENT_HEIGHT
    }

    pub(crate) fn width(&self) -> usize {
        if self.pixels == 0 {
            return 0;
        }
        self.max_x - self.min_x + 1
    }

    pub(crate) fn height(&self) -> usize {
        if self.pixels == 0 {
            return 0;
        }
        self.max_y - self.min_y + 1
    }
}

#[cfg(test)]
mod tests {
    use super::StorybookFramePixelGuard;
    use crate::canvas::Canvas;
    use crate::layout::{HEADER_HEIGHT, SIDEBAR_WIDTH};
    use crate::palette::StorybookPalette;

    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    #[test]
    fn guard_rejects_empty_preview_area() {
        let canvas = Canvas::new(
            WIDTH,
            HEIGHT,
            StorybookPalette::new(true).preview_background(),
        );

        let result = StorybookFramePixelGuard::assert_fixture_content("empty", &canvas, true);

        assert!(result.is_err());
    }

    #[test]
    fn guard_accepts_preview_content_pixels() -> Result<(), Box<dyn std::error::Error>> {
        let mut canvas = Canvas::new(
            WIDTH,
            HEIGHT,
            StorybookPalette::new(true).preview_background(),
        );
        canvas.fill_rect(SIDEBAR_WIDTH + 24, HEADER_HEIGHT + 24, 16, 4, 0xffffff);

        StorybookFramePixelGuard::assert_fixture_content("content", &canvas, true)
    }

    #[test]
    fn guard_rejects_single_row_noise_in_preview_area() {
        let mut canvas = Canvas::new(
            WIDTH,
            HEIGHT,
            StorybookPalette::new(true).preview_background(),
        );
        canvas.fill_rect(SIDEBAR_WIDTH + 24, HEADER_HEIGHT + 24, 64, 1, 0xffffff);

        let result = StorybookFramePixelGuard::assert_fixture_content("noise", &canvas, true);

        assert!(result.is_err());
    }

    #[test]
    fn guard_ignores_status_area_only_pixels() {
        let mut canvas = Canvas::new(
            WIDTH,
            HEIGHT,
            StorybookPalette::new(true).preview_background(),
        );
        canvas.fill_rect(SIDEBAR_WIDTH + 24, HEIGHT - 24, 16, 4, 0xffffff);

        let result = StorybookFramePixelGuard::assert_fixture_content("status", &canvas, true);

        assert!(result.is_err());
    }
}
