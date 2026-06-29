use super::*;
use crate::theme::KdvThemeSnapshot;
use image::Rgba;
use katana_markdown_model::{
    ByteRange, LineColumn, LineColumnRange, RawSnippet, SourceSpan, TableAlignment, TableCell,
    TableNode, TableRow,
};

fn table_block() -> SurfaceTableBlock {
    SurfaceTableBlock::new(&TableNode {
        alignments: vec![
            TableAlignment::Left,
            TableAlignment::Center,
            TableAlignment::Right,
        ],
        rows: vec![
            row(&["key", "value", "type"]),
            row(&["a", "b", "c"]),
            row(&["12", "34", "56"]),
        ],
    })
}

fn row(cells: &[&str]) -> TableRow {
    TableRow {
        cells: cells
            .iter()
            .map(|text| TableCell {
                text: (*text).to_string(),
                source: SourceSpan {
                    byte_range: ByteRange { start: 0, end: 0 },
                    line_column_range: LineColumnRange {
                        start: LineColumn { line: 0, column: 0 },
                        end: LineColumn { line: 0, column: 0 },
                    },
                    raw: RawSnippet {
                        text: (*text).to_string(),
                    },
                },
            })
            .collect(),
    }
}

fn palette() -> SurfacePaintPalette {
    SurfacePaintPalette::from_theme(&KdvThemeSnapshot::katana_light())
}

fn system_text_painter() -> crate::export_surface_font::SurfaceTextPainter {
    crate::export_surface_font::SurfaceTextPainter::from_system_fonts()
}

#[test]
fn paint_table_row_background_colors_header_and_even_rows() {
    let mut image = image::RgbaImage::from_pixel(240, 80, Rgba([255, 255, 255, 255]));
    let palette = palette();

    SurfacePainter::paint_table_row_background(&mut image, 120, 0, 20, 0, &palette);
    SurfacePainter::paint_table_row_background(&mut image, 120, 24, 20, 1, &palette);
    SurfacePainter::paint_table_row_background(&mut image, 120, 48, 20, 2, &palette);

    assert_eq!(image.get_pixel(PAGE_PADDING, 10).0, palette.table_header.0);
    assert_eq!(image.get_pixel(PAGE_PADDING, 34).0, [255, 255, 255, 255]);
    assert_eq!(image.get_pixel(PAGE_PADDING, 58).0, palette.table_even.0);
}

#[test]
fn paint_table_cell_line_advances_text_y_with_table_line_height() {
    let mut image = image::RgbaImage::from_pixel(200, 80, Rgba([255, 255, 255, 255]));
    let spans = vec![crate::export_surface_span::SurfaceTextSpan::plain("value")];
    let cell = table_cell_paint_for_test(&spans);
    let mut next_text_y = cell.y + 2;

    paint_value_cell_line(&mut image, &cell, &mut next_text_y);

    assert_eq!(next_text_y, 44);
    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
}

fn table_cell_paint_for_test<'a>(
    spans: &'a [crate::export_surface_span::SurfaceTextSpan],
) -> SurfaceTableCellPaint<'a> {
    SurfaceTableCellPaint {
        spans,
        alignment: TableAlignment::Left,
        x: 10,
        y: 8,
        width: 120,
        row_height: 40,
        table_font_size: 22.0,
        table_line_height: 34,
    }
}

fn paint_value_cell_line(
    image: &mut image::RgbaImage,
    cell: &SurfaceTableCellPaint<'_>,
    next_text_y: &mut u32,
) {
    let mut painter = system_text_painter();
    SurfacePainter::paint_table_cell_line(
        image,
        cell,
        vec![crate::export_surface_span::SurfaceTextSpan::plain("value")],
        *next_text_y,
        next_text_y,
        &mut painter,
        &palette(),
    );
}

#[test]
fn paint_table_cell_paint_is_positioned_by_alignment() {
    let table = table_block();
    let row = &table.rows()[1];
    let request = SurfaceTableRowPaintRequest {
        table: &table,
        row,
        row_index: 1,
        row_y: 16,
        row_height: 40,
        column_widths: &[80, 80, 80],
        row_width: SURFACE_CONTENT_WIDTH,
    };
    let cell = SurfacePainter::table_cell_paint(&request, 1, "value", PAGE_PADDING + 80, 80);
    assert_eq!(cell.alignment, TableAlignment::Center);
    assert_eq!(cell.x, PAGE_PADDING + 80);
}

#[test]
fn paint_table_row_paints_cells_and_content() -> Result<(), Box<dyn std::error::Error>> {
    let table = table_block();
    let row = table.rows()[0].clone();
    let request = SurfaceTableRowPaintRequest {
        table: &table,
        row: &row,
        row_index: 0,
        row_y: 0,
        row_height: 40,
        column_widths: &[80, 80, 80],
        row_width: SURFACE_CONTENT_WIDTH,
    };
    let mut painter = system_text_painter();
    let mut image = image::RgbaImage::from_pixel(240, 80, Rgba([255, 255, 255, 255]));
    SurfacePainter::paint_table_row(&mut image, request, &mut painter, &palette());
    assert_ne!(image.get_pixel(PAGE_PADDING, 1).0, [255, 255, 255, 255]);
    assert_ne!(image.get_pixel(PAGE_PADDING + 1, 1).0, [255, 255, 255, 255]);
    Ok(())
}

#[test]
fn paint_table_renders_multiple_rows() {
    let table = table_block();
    let mut image = image::RgbaImage::from_pixel(240, 200, Rgba([255, 255, 255, 255]));
    let mut painter = system_text_painter();
    SurfacePainter::paint_table(&mut image, &table, 0, &mut painter, &palette());
    assert!(
        image
            .pixels()
            .any(|pixel| *pixel != Rgba([255, 255, 255, 255]))
    );
}
