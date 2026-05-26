use super::{
    PAGE_PADDING, SURFACE_CONTENT_WIDTH, SurfaceHelpers, SurfacePaintPalette, SurfacePainter,
    SurfaceTableBlock, SurfaceTableCellPaint, SurfaceTableLayout, SurfaceTableRowPaintRequest,
    SurfaceTextLayout, SurfaceTextPainter, TABLE_CELL_FONT_SIZE, TABLE_CELL_PADDING,
    TABLE_LINE_HEIGHT, WrappedText,
};
use image::RgbaImage;

impl SurfacePainter {
    pub(super) fn paint_table(
        image: &mut RgbaImage,
        table: &SurfaceTableBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let row_width = SURFACE_CONTENT_WIDTH;
        let column_width = row_width / table.column_count().max(1) as u32;
        let mut row_y = y;
        for (row_index, row) in table.rows().iter().enumerate() {
            let row_height = table.row_height(row_index, column_width);
            Self::paint_table_row(
                image,
                SurfaceTableRowPaintRequest {
                    table,
                    row,
                    row_index,
                    row_y,
                    row_height,
                    column_width,
                    row_width,
                },
                painter,
                palette,
            );
            row_y += row_height;
        }
    }

    pub(super) fn paint_table_row(
        image: &mut RgbaImage,
        request: SurfaceTableRowPaintRequest<'_>,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        Self::paint_table_row_background(
            image,
            request.row_width,
            request.row_y,
            request.row_height,
            request.row_index,
            palette,
        );
        for (column_index, cell) in request.row.iter().enumerate() {
            Self::paint_table_column_cell(image, &request, column_index, cell, painter, palette);
        }
    }

    pub(super) fn paint_table_column_cell(
        image: &mut RgbaImage,
        request: &SurfaceTableRowPaintRequest<'_>,
        column_index: usize,
        cell: &str,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let x = PAGE_PADDING + column_index as u32 * request.column_width;
        Self::paint_table_cell_border(
            image,
            x,
            request.row_y,
            request.column_width,
            request.row_height,
            palette.table_border,
        );
        let cell_paint = Self::table_cell_paint(request, column_index, cell, x);
        Self::paint_table_cell(image, &cell_paint, painter, palette);
    }

    pub(super) fn table_cell_paint<'a>(
        request: &SurfaceTableRowPaintRequest<'a>,
        column_index: usize,
        cell: &'a str,
        x: u32,
    ) -> SurfaceTableCellPaint<'a> {
        SurfaceTableCellPaint {
            cell,
            alignment: request.table.alignment(column_index),
            x,
            y: request.row_y,
            width: request.column_width,
            row_height: request.row_height,
        }
    }

    pub(super) fn paint_table_row_background(
        image: &mut RgbaImage,
        row_width: u32,
        row_y: u32,
        row_height: u32,
        row_index: usize,
        palette: &SurfacePaintPalette,
    ) {
        if let Some(fill) = SurfaceTableLayout::row_fill(row_index, palette) {
            SurfaceHelpers::fill_rect(image, PAGE_PADDING, row_y, row_width, row_height, fill);
        }
    }

    pub(super) fn paint_table_cell_border(
        image: &mut RgbaImage,
        x: u32,
        row_y: u32,
        column_width: u32,
        row_height: u32,
        color: image::Rgba<u8>,
    ) {
        SurfaceHelpers::stroke_rect(image, x, row_y, column_width, row_height, color);
    }

    pub(super) fn paint_table_cell(
        image: &mut RgbaImage,
        cell: &SurfaceTableCellPaint<'_>,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let max_chars = SurfaceTableLayout::cell_max_chars(cell.width);
        let lines = WrappedText::new(cell.cell, max_chars).collect::<Vec<_>>();
        let mut text_y = cell.y + SurfaceTableLayout::cell_text_y(cell.row_height, lines.len());
        for line in &lines {
            Self::paint_table_cell_line(image, cell, line, text_y, &mut text_y, painter, palette);
        }
    }

    pub(super) fn paint_table_cell_line(
        image: &mut RgbaImage,
        cell: &SurfaceTableCellPaint<'_>,
        line: &str,
        text_y: u32,
        next_text_y: &mut u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let layout = SurfaceTextLayout {
            x: SurfaceTableLayout::cell_text_x(line, &cell.alignment, cell.x, cell.width),
            y: text_y,
            size: TABLE_CELL_FONT_SIZE,
            color: palette.text,
            max_width: Some(cell.width.saturating_sub(TABLE_CELL_PADDING * 2) as f32),
        };
        match painter {
            Some(it) => it.draw_text(image, line, layout),
            None => {
                SurfaceHelpers::draw_fallback_text(image, layout.x, layout.y, line, palette.text)
            }
        }
        *next_text_y += TABLE_LINE_HEIGHT;
    }
}

#[cfg(test)]
#[path = "export_surface_painter_table_tests.rs"]
mod tests;
