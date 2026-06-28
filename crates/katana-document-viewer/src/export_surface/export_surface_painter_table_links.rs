use super::{
    PAGE_PADDING, SURFACE_CONTENT_WIDTH, SurfacePainter, SurfaceSpanMetrics, SurfaceTableBlock,
    SurfaceTableLayout, TABLE_CELL_PADDING,
};

impl SurfacePainter {
    pub(super) fn append_table_link_annotations(
        annotations: &mut Vec<super::SurfaceLinkAnnotation>,
        table: &SurfaceTableBlock,
        page_index: usize,
        y: u32,
    ) {
        let column_widths = table.column_widths_for_width(SURFACE_CONTENT_WIDTH);
        let mut row_y = y;
        for (row_index, row) in table.rows().iter().enumerate() {
            let row_height = table.row_height_with_widths(row_index, &column_widths);
            Self::append_table_row_link_annotations(
                annotations,
                TableRowLinkMetadataRequest {
                    table,
                    row,
                    row_index,
                    page_index,
                    y: row_y,
                    row_height,
                    column_widths: &column_widths,
                },
            );
            row_y += row_height;
        }
    }

    fn append_table_row_link_annotations(
        annotations: &mut Vec<super::SurfaceLinkAnnotation>,
        request: TableRowLinkMetadataRequest<'_>,
    ) {
        let mut cell_x = PAGE_PADDING;
        for (column_index, cell) in request.row.iter().enumerate() {
            let width = request
                .column_widths
                .get(column_index)
                .copied()
                .unwrap_or(0);
            Self::append_table_cell_link_annotations(
                annotations,
                TableCellLinkMetadataRequest {
                    table: request.table,
                    cell,
                    row_index: request.row_index,
                    column_index,
                    page_index: request.page_index,
                    x: cell_x,
                    y: request.y,
                    width,
                    row_height: request.row_height,
                },
            );
            cell_x += width;
        }
    }

    fn append_table_cell_link_annotations(
        annotations: &mut Vec<super::SurfaceLinkAnnotation>,
        request: TableCellLinkMetadataRequest<'_>,
    ) {
        let spans = request
            .table
            .cell_spans(request.row_index, request.column_index);
        if spans.iter().all(|span| span.link_target.is_none()) {
            return;
        }
        let mut x = table_cell_text_x(request);
        let max_x = request
            .x
            .saturating_add(request.width)
            .saturating_sub(TABLE_CELL_PADDING);
        for span in spans {
            let span_width = SurfaceSpanMetrics::estimated_width(span, request.table.font_size());
            if let Some(target) = &span.link_target {
                push_table_link_annotation(annotations, request, target, x, max_x, span_width);
            }
            x += span_width;
        }
    }
}

struct TableRowLinkMetadataRequest<'a> {
    table: &'a SurfaceTableBlock,
    row: &'a [String],
    row_index: usize,
    page_index: usize,
    y: u32,
    row_height: u32,
    column_widths: &'a [u32],
}

#[derive(Clone, Copy)]
struct TableCellLinkMetadataRequest<'a> {
    table: &'a SurfaceTableBlock,
    cell: &'a str,
    row_index: usize,
    column_index: usize,
    page_index: usize,
    x: u32,
    y: u32,
    width: u32,
    row_height: u32,
}

fn table_cell_text_y(request: TableCellLinkMetadataRequest<'_>) -> u32 {
    let line_count = SurfaceTableLayout::cell_lines(request.cell, request.width).len();
    request.y
        + SurfaceTableLayout::cell_text_y_with_line_height(
            request.row_height,
            line_count,
            request.table.line_height(),
        )
}

fn table_cell_text_x(request: TableCellLinkMetadataRequest<'_>) -> u32 {
    SurfaceTableLayout::cell_text_x(
        request.cell,
        &request.table.alignment(request.column_index),
        request.x,
        request.width,
    )
}

fn push_table_link_annotation(
    annotations: &mut Vec<super::SurfaceLinkAnnotation>,
    request: TableCellLinkMetadataRequest<'_>,
    target: &str,
    x: u32,
    max_x: u32,
    span_width: u32,
) {
    if target.is_empty() {
        return;
    }
    annotations.push(super::SurfaceLinkAnnotation {
        page_index: request.page_index,
        x,
        y: table_cell_text_y(request),
        width: span_width.min(max_x.saturating_sub(x)),
        height: request.table.line_height(),
        target: target.to_string(),
    });
}
