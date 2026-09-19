use super::KucNodeFactory;
use super::node_factory_tests_support::viewer_node;
use katana_document_viewer::{
    ViewerNodeKind, ViewerTableAlignment, ViewerTableCellProjection, ViewerTableProjection,
    ViewerTableRowProjection, ViewerTableVerticalAlignment,
};
use katana_markdown_model::RawSnippet;
use katana_ui_core::molecule::{GridHorizontalAlignment, GridVerticalAlignment};
use katana_ui_core::render_model::{UiDimension, UiNodeKind};

#[test]
fn table_node_preserves_rows_cells_alignment_wrapping_and_geometry()
-> Result<(), Box<dyn std::error::Error>> {
    let factory = KucNodeFactory::new(&[], 320);
    let mut node = viewer_node(
        ViewerNodeKind::Table,
        "Left | Center | Right\nA | long long long | C",
    );
    node.source.raw = RawSnippet::new(
        "| Left | Center | Right |\n| :--- | :---: | ---: |\n| A | long long long | C |",
    );
    node.rect.height = node
        .table_projection()
        .ok_or("table projection")?
        .row_heights(320, Default::default())
        .into_iter()
        .sum::<u32>() as f32;

    let rendered = factory.viewer_node(&node);

    assert_eq!(UiNodeKind::Row, rendered.kind());
    assert_eq!(UiDimension::Px(320), rendered.props().common.width);
    assert_eq!(
        UiDimension::Px(node.rect.height as u16),
        rendered.props().common.height
    );
    let grid_node = &rendered.children()[0];
    assert_eq!(UiNodeKind::Grid, grid_node.kind());
    let grid = &grid_node.props().grid;
    assert_eq!(2, grid.row_count);
    assert_eq!(3, grid.column_count);
    assert_eq!(6, grid.cells.len());
    assert_eq!(320, grid.total_width);
    assert_eq!(node.rect.height as u32, grid.total_height);
    assert_eq!("Left", grid.cells[0].text);
    assert_eq!(
        GridHorizontalAlignment::Left,
        grid.cells[0].appearance.horizontal_alignment
    );
    assert_eq!(
        GridHorizontalAlignment::Center,
        grid.cells[1].appearance.horizontal_alignment
    );
    assert_eq!(
        GridHorizontalAlignment::Right,
        grid.cells[2].appearance.horizontal_alignment
    );
    assert_eq!(
        GridVerticalAlignment::Center,
        grid.cells[4].appearance.vertical_alignment
    );
    assert!(grid.cells.iter().all(|cell| cell.appearance.wrap_text));
    assert!(grid.cells[..3].iter().all(|cell| cell.appearance.bold));
    assert!(grid.cells[3..].iter().all(|cell| !cell.appearance.bold));
    assert!(
        grid.cells
            .iter()
            .all(|cell| cell.row_span == 1 && cell.column_span == 1)
    );
    Ok(())
}

#[test]
fn table_projection_cell_merges_become_kuc_grid_spans() {
    let projection = ViewerTableProjection {
        rows: vec![ViewerTableRowProjection {
            cells: vec![ViewerTableCellProjection {
                text: "merged".to_owned(),
                alignment: ViewerTableAlignment::Center,
                vertical_alignment: ViewerTableVerticalAlignment::Center,
                row_span: 2,
                column_span: 3,
            }],
        }],
        column_count: 3,
    };

    let spans = super::super::table::table_grid_spans(&projection);
    assert_eq!(1, spans.len());
    assert_eq!(2, spans[0].row_span);
    assert_eq!(3, spans[0].column_span);
}
