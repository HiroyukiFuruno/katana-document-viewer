use super::{
    CODE_BLOCK_MARGIN, CODE_HORIZONTAL_PADDING, CODE_VERTICAL_PADDING,
    LINE_CENTERED_TEXT_GUESS_CHAR_WIDTH, LIST_MARKER_COLUMN_WIDTH, MATH_VERTICAL_MARGIN,
    PAGE_PADDING, QUOTE_INDENT, SURFACE_CONTENT_WIDTH, SURFACE_WIDTH, SurfaceCodeBlock,
    SurfaceHelpers, SurfaceLine, SurfaceMathBlock, SurfacePaintPalette, SurfacePainter,
    SurfaceSvgImage, SurfaceTextLayout, SurfaceTextPainter,
};
use image::RgbaImage;

impl SurfacePainter {
    pub(super) fn paint_code_block(
        image: &mut RgbaImage,
        block: &SurfaceCodeBlock,
        y: u32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        if block.quote_depth > 0 {
            SurfaceHelpers::draw_quote_bars(
                image,
                block.quote_depth,
                y,
                block.height(),
                palette.quote,
            );
        }
        let (box_x, box_width, box_y) = Self::code_block_geometry(block, y);
        Self::paint_code_background(image, box_x, box_y, box_width, block.box_height(), palette);
        Self::paint_code_lines(image, &block.lines, box_x, box_y, painter, palette);
    }

    pub(super) fn code_block_geometry(block: &SurfaceCodeBlock, y: u32) -> (u32, u32, u32) {
        let box_x = PAGE_PADDING
            + block.quote_depth * QUOTE_INDENT
            + block.indent_depth * LIST_MARKER_COLUMN_WIDTH;
        let box_width = SURFACE_WIDTH.saturating_sub(box_x + PAGE_PADDING);
        let box_y = y + CODE_BLOCK_MARGIN;
        (box_x, box_width, box_y)
    }

    pub(super) fn paint_code_background(
        image: &mut RgbaImage,
        box_x: u32,
        box_y: u32,
        box_width: u32,
        box_height: u32,
        palette: &SurfacePaintPalette,
    ) {
        SurfaceHelpers::fill_rect(
            image,
            box_x,
            box_y,
            box_width,
            box_height,
            palette.code_background,
        );
        SurfaceHelpers::stroke_rect(
            image,
            box_x,
            box_y,
            box_width,
            box_height,
            palette.code_border,
        );
    }

    pub(super) fn paint_code_lines(
        image: &mut RgbaImage,
        lines: &[SurfaceLine],
        box_x: u32,
        box_y: u32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        let mut line_y = box_y + CODE_VERTICAL_PADDING;
        for line in lines {
            Self::paint_code_line(image, line, box_x, line_y, painter, palette);
            line_y += line.line_height();
        }
    }

    pub(super) fn paint_code_line(
        image: &mut RgbaImage,
        line: &SurfaceLine,
        box_x: u32,
        line_y: u32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        let x = box_x + CODE_HORIZONTAL_PADDING;
        painter.draw_spans(
            image,
            &line.spans,
            x,
            line_y,
            line.font_size(),
            palette.text,
        );
    }

    pub(super) fn paint_math_block(
        image: &mut RgbaImage,
        block: &SurfaceMathBlock,
        y: u32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        if let Some(rendered) = &block.image {
            Self::paint_rendered_math(image, rendered, y);
            return;
        }
        Self::paint_raw_math_text(image, block, y, painter, palette);
    }

    pub(super) fn paint_rendered_math(image: &mut RgbaImage, rendered: &SurfaceSvgImage, y: u32) {
        let x = PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(rendered.image.width()) / 2;
        SurfaceHelpers::paste_rgba(image, &rendered.image, x, y + MATH_VERTICAL_MARGIN);
    }

    pub(super) fn paint_raw_math_text(
        image: &mut RgbaImage,
        block: &SurfaceMathBlock,
        y: u32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        painter.draw_text(
            image,
            block.fallback_text(),
            SurfaceTextLayout {
                x: PAGE_PADDING,
                y: y + MATH_VERTICAL_MARGIN,
                size: block.raw_text_size(),
                color: palette.text,
                max_width: Some(SURFACE_CONTENT_WIDTH as f32),
            },
        );
    }

    pub(super) fn line_text_x(line: &SurfaceLine) -> u32 {
        Self::line_text_x_with_width(line, None)
    }

    pub(super) fn line_text_x_for_paint(
        line: &SurfaceLine,
        painter: &mut SurfaceTextPainter,
    ) -> u32 {
        let measured_width = painter.measure_spans_width(
            &line.spans,
            line.font_size(),
            SURFACE_CONTENT_WIDTH as f32,
        );
        Self::line_text_x_with_width(line, Some(measured_width))
    }

    fn line_text_x_with_width(line: &SurfaceLine, measured_width: Option<u32>) -> u32 {
        if line.is_code() {
            return line.x() + CODE_HORIZONTAL_PADDING;
        }
        if line.is_centered() {
            let width = measured_width.unwrap_or_else(|| Self::estimated_aligned_line_width(line));
            return PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(width) / 2;
        }
        if line.is_right_aligned() {
            let width = measured_width.unwrap_or_else(|| Self::estimated_aligned_line_width(line));
            return PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(width);
        }
        line.x()
    }

    fn estimated_aligned_line_width(line: &SurfaceLine) -> u32 {
        (line.text.chars().count() as u32).saturating_mul(LINE_CENTERED_TEXT_GUESS_CHAR_WIDTH)
    }
}

#[cfg(test)]
#[path = "export_surface_painter_code_media_tests.rs"]
mod tests;
