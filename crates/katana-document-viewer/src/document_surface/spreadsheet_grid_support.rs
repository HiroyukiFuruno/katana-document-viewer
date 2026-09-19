use super::{DocumentGridCommand, DocumentGridNavigation};
use crate::SpreadsheetSheetArtifact;
use katana_ui_core::molecule::{GenericGrid, GridAction, GridCoordinate, GridNavigationIntent};

pub(super) fn adjusted_frozen_panes(sheet: &SpreadsheetSheetArtifact) -> (usize, usize) {
    let mut rows = sheet.frozen_rows.min(sheet.row_count);
    let mut columns = sheet.frozen_columns.min(sheet.column_count);
    loop {
        let previous = (rows, columns);
        for merged in &sheet.merged_cells {
            let row_end = merged.anchor.row.saturating_add(merged.row_span);
            if merged.anchor.row < rows && rows < row_end {
                rows = row_end.min(sheet.row_count);
            }
            let column_end = merged.anchor.column.saturating_add(merged.column_span);
            if merged.anchor.column < columns && columns < column_end {
                columns = column_end.min(sheet.column_count);
            }
        }
        if previous == (rows, columns) {
            return (rows, columns);
        }
    }
}

pub(super) fn grid_action(grid: &GenericGrid, command: DocumentGridCommand) -> Option<GridAction> {
    Some(match command {
        DocumentGridCommand::SelectAt { x, y, extend } => GridAction::Select {
            coordinate: grid.hit_test(x, y)?.coordinate,
            extend,
        },
        DocumentGridCommand::ScrollTo { x, y } => GridAction::ScrollTo { x, y },
        DocumentGridCommand::Select {
            row,
            column,
            extend,
        } => GridAction::Select {
            coordinate: GridCoordinate::new(row, column),
            extend,
        },
        DocumentGridCommand::Navigate { intent, extend } => GridAction::Navigate {
            intent: SpreadsheetGridSupport::navigation_intent(intent),
            extend,
        },
    })
}

pub(crate) struct SpreadsheetGridSupport;

impl SpreadsheetGridSupport {
    pub(crate) const fn navigation_intent(intent: DocumentGridNavigation) -> GridNavigationIntent {
        match intent {
            DocumentGridNavigation::Left => GridNavigationIntent::Left,
            DocumentGridNavigation::Right => GridNavigationIntent::Right,
            DocumentGridNavigation::Up => GridNavigationIntent::Up,
            DocumentGridNavigation::Down => GridNavigationIntent::Down,
            DocumentGridNavigation::Home => GridNavigationIntent::Home,
            DocumentGridNavigation::End => GridNavigationIntent::End,
            DocumentGridNavigation::PageUp => GridNavigationIntent::PageUp,
            DocumentGridNavigation::PageDown => GridNavigationIntent::PageDown,
        }
    }
}
