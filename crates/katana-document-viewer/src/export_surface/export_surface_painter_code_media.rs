use super::{
    CODE_BLOCK_MARGIN, CODE_HORIZONTAL_PADDING, CODE_VERTICAL_PADDING, DIAGRAM_VERTICAL_MARGIN,
    IMAGE_VERTICAL_MARGIN, LINE_CENTERED_TEXT_GUESS_CHAR_WIDTH, LIST_MARKER_COLUMN_WIDTH,
    MATH_FALLBACK_TEXT_SIZE, MATH_VERTICAL_MARGIN, PAGE_PADDING, QUOTE_INDENT,
    SURFACE_CONTENT_WIDTH, SURFACE_WIDTH, SurfaceCodeBlock, SurfaceDiagramBlock, SurfaceHelpers,
    SurfaceImageBlock, SurfaceLine, SurfaceMathBlock, SurfacePaintPalette, SurfacePainter,
    SurfaceSvgImage, SurfaceTextLayout, SurfaceTextPainter,
};
use image::RgbaImage;

impl SurfacePainter {
    pub(super) fn paint_code_block(
        image: &mut RgbaImage,
        block: &SurfaceCodeBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
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
        painter: &mut Option<SurfaceTextPainter>,
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
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let x = box_x + CODE_HORIZONTAL_PADDING;
        match painter {
            Some(it) => it.draw_spans(
                image,
                &line.spans,
                x,
                line_y,
                line.font_size(),
                palette.text,
            ),
            None => SurfaceHelpers::draw_fallback_text(image, x, line_y, &line.text, palette.text),
        }
    }

    pub(super) fn paint_math_block(
        image: &mut RgbaImage,
        block: &SurfaceMathBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        if let Some(rendered) = &block.image {
            Self::paint_rendered_math(image, rendered, y);
            return;
        }
        Self::paint_fallback_math(image, block, y, painter, palette);
    }

    pub(super) fn paint_rendered_math(image: &mut RgbaImage, rendered: &SurfaceSvgImage, y: u32) {
        let x = PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(rendered.image.width()) / 2;
        SurfaceHelpers::paste_rgba(image, &rendered.image, x, y + MATH_VERTICAL_MARGIN);
    }

    pub(super) fn paint_fallback_math(
        image: &mut RgbaImage,
        block: &SurfaceMathBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        match painter {
            Some(it) => it.draw_text(
                image,
                block.fallback_text(),
                SurfaceTextLayout {
                    x: PAGE_PADDING,
                    y: y + MATH_VERTICAL_MARGIN,
                    size: MATH_FALLBACK_TEXT_SIZE,
                    color: palette.text,
                    max_width: Some(SURFACE_CONTENT_WIDTH as f32),
                },
            ),
            None => SurfaceHelpers::draw_fallback_text(
                image,
                PAGE_PADDING,
                y + MATH_VERTICAL_MARGIN,
                block.fallback_text(),
                palette.text,
            ),
        }
    }

    pub(super) fn line_text_x(line: &SurfaceLine) -> u32 {
        if line.is_code() {
            return line.x() + CODE_HORIZONTAL_PADDING;
        }
        if line.is_centered() {
            let estimated_width = (line.text.chars().count() as u32)
                .saturating_mul(LINE_CENTERED_TEXT_GUESS_CHAR_WIDTH);
            return PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(estimated_width) / 2;
        }
        line.x()
    }

    pub(super) fn paint_diagram(
        image: &mut RgbaImage,
        diagram: &SurfaceDiagramBlock,
        y: u32,
        palette: &SurfacePaintPalette,
    ) {
        let Some(rendered) = &diagram.image else {
            SurfaceHelpers::draw_fallback_text(
                image,
                PAGE_PADDING,
                y + DIAGRAM_VERTICAL_MARGIN,
                diagram.fallback_text(),
                palette.text,
            );
            return;
        };
        let x = PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(rendered.image.width()) / 2;
        SurfaceHelpers::paste_rgba(image, &rendered.image, x, y + DIAGRAM_VERTICAL_MARGIN);
    }

    pub(super) fn paint_image(image: &mut RgbaImage, block: &SurfaceImageBlock, y: u32) {
        let x = PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(block.image.width()) / 2;
        SurfaceHelpers::paste_rgba(image, &block.image, x, y + IMAGE_VERTICAL_MARGIN);
    }
}

#[cfg(test)]
#[path = "export_surface_painter_code_media_tests.rs"]
mod tests;
