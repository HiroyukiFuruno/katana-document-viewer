use super::{
    DocumentGridCellBorders, DocumentGridCoordinate, DocumentGridSurfaceFrame,
    DocumentSurfaceContent, DocumentSurfaceFrame, DocumentSurfaceKind, PdfOutlineItem,
};
use crate::{DocumentGridBorderSide, DocumentGridViewport};

#[test]
fn grid_frame_reports_grid_kind_at_runtime() {
    let frame = sample_frame().with_navigation_metadata(
        vec!["Sheet 1".to_owned()],
        vec![PdfOutlineItem {
            title: "Section".to_owned(),
            level: 0,
            page_index: Some(0),
        }],
    );

    assert_eq!(
        DocumentSurfaceKind::Grid,
        std::hint::black_box(&frame).kind()
    );
    assert_eq!(["Sheet 1"], frame.item_labels());
    assert_eq!("Section", frame.outline_items()[0].title);
}

#[test]
fn grid_border_entries_preserve_order_and_match_single_cell_lookup() {
    let entries = vec![
        (
            DocumentGridCoordinate { row: 1, column: 1 },
            borders(Some(border_side("thin", Some("#102030"))), None, None, None),
        ),
        (
            DocumentGridCoordinate { row: 1, column: 2 },
            DocumentGridCellBorders::default(),
        ),
        (
            DocumentGridCoordinate { row: 3, column: 4 },
            borders(
                None,
                Some(border_side("double", Some("#405060"))),
                Some(border_side("dotted", None)),
                None,
            ),
        ),
    ];
    let frame = frame_with_grid_borders(entries.clone());

    assert_eq!(entries.as_slice(), frame.grid_cell_border_entries());
    for (coordinate, borders) in frame.grid_cell_border_entries() {
        assert_eq!(Some(borders), frame.grid_cell_borders(*coordinate));
    }
}

#[test]
fn large_grid_border_projection_uses_one_batch_traversal() {
    let frame = frame_with_grid_borders(large_grid_border_entries());
    let projected = visible_left_border_projection(&frame);

    assert_eq!(4_096, frame.grid_cell_border_entries().len());
    assert_eq!(1_366, projected.len());
    assert_eq!(
        (
            DocumentGridCoordinate { row: 0, column: 0 },
            "solid".to_owned()
        ),
        projected[0]
    );
    assert_eq!(
        (
            DocumentGridCoordinate {
                row: 63,
                column: 63
            },
            "solid".to_owned()
        ),
        projected[1_365]
    );
}

fn large_grid_border_entries() -> Vec<(DocumentGridCoordinate, DocumentGridCellBorders)> {
    (0..4_096)
        .map(|index| {
            let coordinate = DocumentGridCoordinate {
                row: index / 64,
                column: index % 64,
            };
            let borders = if index % 3 == 0 {
                borders(Some(border_side("solid", None)), None, None, None)
            } else {
                DocumentGridCellBorders::default()
            };
            (coordinate, borders)
        })
        .collect()
}

fn visible_left_border_projection(
    frame: &DocumentSurfaceFrame,
) -> Vec<(DocumentGridCoordinate, String)> {
    frame
        .grid_cell_border_entries()
        .iter()
        .filter_map(|(coordinate, borders)| {
            borders
                .left
                .as_ref()
                .map(|border| (*coordinate, border.style.clone()))
        })
        .collect()
}

fn frame_with_grid_borders(
    grid_borders: Vec<(DocumentGridCoordinate, DocumentGridCellBorders)>,
) -> DocumentSurfaceFrame {
    DocumentSurfaceFrame {
        grid_borders,
        ..sample_frame()
    }
}

fn borders(
    left: Option<DocumentGridBorderSide>,
    right: Option<DocumentGridBorderSide>,
    top: Option<DocumentGridBorderSide>,
    bottom: Option<DocumentGridBorderSide>,
) -> DocumentGridCellBorders {
    DocumentGridCellBorders {
        left,
        right,
        top,
        bottom,
    }
}

fn border_side(style: &str, color: Option<&str>) -> DocumentGridBorderSide {
    DocumentGridBorderSide {
        style: style.to_owned(),
        color: color.map(str::to_owned),
    }
}

fn sample_frame() -> DocumentSurfaceFrame {
    DocumentSurfaceFrame {
        content: DocumentSurfaceContent::Grid(DocumentGridSurfaceFrame {
            row_count: 0,
            column_count: 0,
            total_width: 0,
            total_height: 0,
            viewport: DocumentGridViewport::default(),
            active_cell: None,
            show_grid_lines: true,
            cells: Vec::new(),
        }),
        navigation: super::DocumentNavigationMetadata::default(),
        grid_borders: Vec::new(),
    }
}
