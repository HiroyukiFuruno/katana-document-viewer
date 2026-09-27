use super::*;
use crate::ViewerTypographyConfig;

#[test]
fn table_projection_preserves_rows_alignment_and_export_geometry() -> Result<(), &'static str> {
    let projection = table_projection();
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

#[test]
fn empty_table_projection_has_no_geometry() {
    let projection = ViewerTableProjection {
        rows: Vec::new(),
        column_count: 0,
    };
    assert!(projection.column_widths(640).is_empty());
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

fn table_projection() -> ViewerTableProjection {
    ViewerTableProjection {
        rows: vec![
            ViewerTableRowProjection {
                cells: vec![
                    cell("Left", ViewerTableAlignment::Left),
                    cell("Center", ViewerTableAlignment::Center),
                    cell("Right", ViewerTableAlignment::Right),
                ],
            },
            ViewerTableRowProjection {
                cells: vec![
                    cell("A", ViewerTableAlignment::Left),
                    cell("long long long", ViewerTableAlignment::Center),
                    cell("C", ViewerTableAlignment::Right),
                ],
            },
        ],
        column_count: 3,
    }
}

fn cell(text: &str, alignment: ViewerTableAlignment) -> ViewerTableCellProjection {
    ViewerTableCellProjection {
        text: text.to_owned(),
        alignment,
        vertical_alignment: ViewerTableVerticalAlignment::Center,
        row_span: 1,
        column_span: 1,
    }
}
