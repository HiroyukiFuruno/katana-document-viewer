use super::SpreadsheetGridSurface;
use crate::{DocumentSurfaceError, DocumentViewport, SpreadsheetSheetArtifact};
use katana_ui_core::molecule::{GenericGrid, GridAction, GridCoordinate, GridSelection};

impl SpreadsheetGridSurface {
    pub(crate) fn replace_sheet_filtered(
        &mut self,
        sheet: &SpreadsheetSheetArtifact,
        filtered_out_rows: &[usize],
        viewport: DocumentViewport,
    ) -> Result<(), DocumentSurfaceError> {
        let selection = self.grid.selection();
        let layout = self.grid.layout();
        let mut replacement = Self::new_filtered(sheet, filtered_out_rows, viewport)?;
        let _ = replacement.grid.apply_action(GridAction::ScrollTo {
            x: layout.columns.scroll_offset,
            y: layout.rows.scroll_offset,
        });
        restore_selection(&mut replacement.grid, sheet, filtered_out_rows, selection);
        *self = replacement;
        Ok(())
    }
}

fn restore_selection(
    grid: &mut GenericGrid,
    sheet: &SpreadsheetSheetArtifact,
    filtered_out_rows: &[usize],
    selection: Option<GridSelection>,
) {
    let Some(selection) = selection else {
        return;
    };
    let anchor = visible_coordinate(sheet, filtered_out_rows, selection.anchor);
    let active = visible_coordinate(sheet, filtered_out_rows, selection.active);
    let _ = grid.apply_action(GridAction::Select {
        coordinate: anchor,
        extend: false,
    });
    if active != anchor {
        let _ = grid.apply_action(GridAction::Select {
            coordinate: active,
            extend: true,
        });
    }
}

fn visible_coordinate(
    sheet: &SpreadsheetSheetArtifact,
    filtered_out_rows: &[usize],
    coordinate: GridCoordinate,
) -> GridCoordinate {
    GridCoordinate::new(
        nearest_visible_row(sheet, filtered_out_rows, coordinate.row),
        coordinate.column.min(sheet.column_count.saturating_sub(1)),
    )
}

fn nearest_visible_row(
    sheet: &SpreadsheetSheetArtifact,
    filtered_out_rows: &[usize],
    row: usize,
) -> usize {
    let row = row.min(sheet.row_count.saturating_sub(1));
    if !row_is_hidden(sheet, filtered_out_rows, row) {
        return row;
    }
    (row.saturating_add(1)..sheet.row_count)
        .find(|candidate| !row_is_hidden(sheet, filtered_out_rows, *candidate))
        .or_else(|| {
            (0..row)
                .rev()
                .find(|candidate| !row_is_hidden(sheet, filtered_out_rows, *candidate))
        })
        .unwrap_or(row)
}

fn row_is_hidden(
    sheet: &SpreadsheetSheetArtifact,
    filtered_out_rows: &[usize],
    row: usize,
) -> bool {
    sheet.row_tracks.get(row).is_some_and(|track| track.hidden)
        || filtered_out_rows.binary_search(&row).is_ok()
}

#[cfg(test)]
#[path = "spreadsheet_grid_state_tests.rs"]
mod tests;
