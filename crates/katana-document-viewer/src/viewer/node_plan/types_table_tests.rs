use super::*;
use crate::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan, ViewerRect,
    ViewerTypographyConfig,
};

#[test]
fn table_projection_preserves_rows_alignment_and_export_geometry() -> Result<(), &'static str> {
    let projection = table_node().table_projection().ok_or("table projection")?;
    assert_eq!(2, projection.rows.len());
    assert_eq!(3, projection.column_count);
    assert_alignments(&projection);
    assert_eq!(vec![389, 389, 390], projection.column_widths(1168));
    assert_eq!(
        2,
        projection
            .row_heights(1168, ViewerTypographyConfig::default())
            .len()
    );
    assert!(
        projection
            .rows
            .iter()
            .flat_map(|row| &row.cells)
            .all(|cell| { cell.row_span == 1 && cell.column_span == 1 })
    );
    Ok(())
}

fn assert_alignments(projection: &ViewerTableProjection) {
    assert_eq!(
        ViewerTableAlignment::Left,
        projection.rows[0].cells[0].alignment
    );
    assert_eq!(
        ViewerTableAlignment::Center,
        projection.rows[0].cells[1].alignment
    );
    assert_eq!(
        ViewerTableAlignment::Right,
        projection.rows[0].cells[2].alignment
    );
}

fn table_node() -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId("table".to_owned()),
        kind: ViewerNodeKind::Table,
        source: table_source(),
        text: "Left | Center | Right\nA | long long long | C".to_owned(),
        spans: Vec::new(),
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: 1_168.0,
            height: 104.0,
        },
        artifact_id: None,
    }
}

fn table_source() -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange { start: 0, end: 77 },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 3,
                column: 27,
            },
        },
        raw: RawSnippet::new(
            "| Left | Center | Right |\n| :--- | :---: | ---: |\n| A | long long long | C |",
        ),
    }
}
