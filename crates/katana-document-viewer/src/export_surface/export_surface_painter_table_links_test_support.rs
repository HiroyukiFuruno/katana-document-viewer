use super::super::{SurfaceTableBlock, TableCellLinkMetadataRequest};
use katana_markdown_model::{
    ByteRange, LineColumn, LineColumnRange, RawSnippet, SourceSpan, TableAlignment, TableCell,
    TableNode, TableRow,
};

pub(super) fn empty_table_block() -> SurfaceTableBlock {
    SurfaceTableBlock::new(&TableNode {
        alignments: vec![TableAlignment::Left],
        rows: vec![TableRow {
            cells: vec![table_cell("a")],
        }],
    })
}

fn table_cell(text: &str) -> TableCell {
    TableCell {
        text: text.to_string(),
        source: source_span(text),
    }
}

fn source_span(text: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: text.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: text.len() + 1,
            },
        },
        raw: RawSnippet {
            text: text.to_string(),
        },
    }
}

pub(super) fn annotation_request<'a>(
    table: &'a SurfaceTableBlock,
    row_index: usize,
    column_index: usize,
) -> TableCellLinkMetadataRequest<'a> {
    let row_height = table.row_height_with_widths(row_index, &[100]);
    TableCellLinkMetadataRequest {
        table,
        row_index,
        column_index,
        page_index: 1,
        x: 10,
        y: 20,
        width: 100,
        row_height,
    }
}
