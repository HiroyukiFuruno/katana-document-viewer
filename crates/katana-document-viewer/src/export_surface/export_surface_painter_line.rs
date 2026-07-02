use super::{
    LIST_MARKER_COLUMN_WIDTH, SurfaceHelpers, SurfaceLine, SurfaceListLinePaintRequest,
    SurfaceMarkerPaintRequest, SurfacePaintPalette, SurfacePainter, SurfaceSpansLayout,
    SurfaceTextPainter, SurfaceTextSpan,
};
use image::RgbaImage;

impl SurfacePainter {
    pub(super) fn paint_line(
        image: &mut RgbaImage,
        line: &SurfaceLine,
        y: u32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        let size = line.font_size();
        let text_x = Self::line_text_x_for_paint(line, painter);
        if line.quote_depth() > 0 {
            self_paint_quote_bars(image, line, y, palette);
        }
        let text_y = line.text_y(y);
        if let Some(marker) = line.list_marker() {
            Self::paint_list_line(
                image,
                SurfaceListLinePaintRequest {
                    line,
                    text_x,
                    text_y,
                    size,
                    marker: &marker,
                },
                painter,
                palette,
            );
            return;
        }
        Self::paint_line_without_marker(image, line, text_x, text_y, size, painter, palette)
    }

    pub(super) fn paint_line_without_marker(
        image: &mut RgbaImage,
        line: &SurfaceLine,
        text_x: u32,
        text_y: u32,
        size: f32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        if line.aligns_with_list_marker() {
            Self::paint_aligned_list_line(image, line, text_x, text_y, size, painter, palette);
            return;
        }
        Self::paint_line_text(image, &line.spans, text_x, text_y, size, painter, palette);
    }

    pub(super) fn paint_list_line(
        image: &mut RgbaImage,
        request: SurfaceListLinePaintRequest<'_>,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        let marker_request = SurfaceMarkerPaintRequest {
            marker: request.marker,
            x: request.text_x,
            y: request.text_y,
            indent_depth: request.line.indent_depth(),
            size: request.size,
        };
        Self::paint_line_marker(image, marker_request, painter, palette);
        Self::paint_line_text(
            image,
            request.line.content_spans(),
            request.text_x + LIST_MARKER_COLUMN_WIDTH,
            request.text_y,
            request.size,
            painter,
            palette,
        );
    }

    pub(super) fn paint_aligned_list_line(
        image: &mut RgbaImage,
        line: &SurfaceLine,
        text_x: u32,
        text_y: u32,
        size: f32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        Self::paint_line_text(
            image,
            &line.spans,
            text_x + LIST_MARKER_COLUMN_WIDTH,
            text_y,
            size,
            painter,
            palette,
        );
    }

    pub(super) fn paint_line_text(
        image: &mut RgbaImage,
        spans: &[SurfaceTextSpan],
        x: u32,
        y: u32,
        size: f32,
        painter: &mut SurfaceTextPainter,
        palette: &SurfacePaintPalette,
    ) {
        painter.draw_spans_with_backgrounds(
            image,
            spans,
            SurfaceSpansLayout {
                x,
                y,
                size,
                color: palette.text,
                backgrounds: palette.text_backgrounds(),
            },
        );
    }
}

fn self_paint_quote_bars(
    image: &mut RgbaImage,
    line: &SurfaceLine,
    y: u32,
    palette: &SurfacePaintPalette,
) {
    SurfaceHelpers::draw_quote_bars(
        image,
        line.quote_depth(),
        y,
        line.line_height(),
        palette.quote,
    );
}

#[cfg(test)]
#[path = "export_surface_painter_line_tests.rs"]
mod tests;
