use super::KucNodeFactory;
use katana_document_viewer::{
    ViewerNode, ViewerTableAlignment, ViewerTableProjection, ViewerTableVerticalAlignment,
};
use katana_ui_core::molecule::{
    GenericGrid, GridCellAppearance, GridCellContent, GridCellSpan, GridCoordinate,
    GridHorizontalAlignment, GridTrackSizeProvider, GridVerticalAlignment, GridViewport,
};
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind, UiTextProps};

impl KucNodeFactory<'_> {
    pub(super) fn table_node(&self, node: &ViewerNode) -> UiNode {
        let Some(projection) = node.table_projection() else {
            return self.text_node(node);
        };
        let row_heights = projection.row_heights(self.content_width, self.typography);
        let column_widths = projection.column_widths(self.content_width);
        let total_height = row_heights.iter().copied().sum::<u32>().max(1);
        let cells = projection
            .rows
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.cells
                    .iter()
                    .enumerate()
                    .map(move |(column_index, cell)| {
                        GridCellContent::new(
                            GridCoordinate::new(row_index, column_index),
                            cell.text.clone(),
                        )
                        .appearance(GridCellAppearance {
                            font_size_px: self.typography.preview_font_size,
                            bold: row_index == 0,
                            horizontal_alignment: grid_alignment(cell.alignment),
                            vertical_alignment: grid_vertical_alignment(cell.vertical_alignment),
                            wrap_text: true,
                            ..GridCellAppearance::default()
                        })
                    })
            })
            .collect();
        let grid = GenericGrid::new(
            &Self::text_label(node),
            projection.rows.len(),
            projection.column_count,
        )
        .row_tracks(GridTrackSizeProvider::Variable {
            sizes: row_heights,
            fallback_size: 1,
        })
        .column_tracks(GridTrackSizeProvider::Variable {
            sizes: column_widths,
            fallback_size: 1,
        })
        .viewport(GridViewport::new(self.content_width, total_height))
        .overscan(projection.rows.len(), projection.column_count)
        .show_grid_lines(true);
        let grid = match grid.with_cell_spans(table_grid_spans(&projection)) {
            Ok(grid) => grid,
            Err(_) => return self.text_node(node),
        };
        match grid.with_visible_cells(cells) {
            Ok(grid) => UiNode::new(UiNodeKind::Row, Self::text_label(node))
                .child(
                    UiNode::from(grid)
                        .height(UiDimension::Px(total_height.min(u32::from(u16::MAX)) as u16)),
                )
                .font_role(self.font_role_for_node(node))
                .text(UiTextProps {
                    role: self.text_role_for_node(node).to_owned(),
                    ..UiTextProps::default()
                }),
            Err(_) => self.text_node(node),
        }
    }
}

pub(super) fn table_grid_spans(projection: &ViewerTableProjection) -> Vec<GridCellSpan> {
    projection
        .rows
        .iter()
        .enumerate()
        .flat_map(|(row, cells)| {
            cells
                .cells
                .iter()
                .enumerate()
                .filter_map(move |(column, cell)| {
                    (cell.row_span > 1 || cell.column_span > 1).then_some(GridCellSpan::new(
                        GridCoordinate::new(row, column),
                        cell.row_span,
                        cell.column_span,
                    ))
                })
        })
        .collect()
}

const fn grid_alignment(value: ViewerTableAlignment) -> GridHorizontalAlignment {
    match value {
        ViewerTableAlignment::Left => GridHorizontalAlignment::Left,
        ViewerTableAlignment::Center => GridHorizontalAlignment::Center,
        ViewerTableAlignment::Right => GridHorizontalAlignment::Right,
        ViewerTableAlignment::Unspecified => GridHorizontalAlignment::General,
    }
}

const fn grid_vertical_alignment(value: ViewerTableVerticalAlignment) -> GridVerticalAlignment {
    match value {
        ViewerTableVerticalAlignment::Top => GridVerticalAlignment::Top,
        ViewerTableVerticalAlignment::Center => GridVerticalAlignment::Center,
        ViewerTableVerticalAlignment::Bottom => GridVerticalAlignment::Bottom,
        ViewerTableVerticalAlignment::Unspecified => GridVerticalAlignment::Bottom,
    }
}
