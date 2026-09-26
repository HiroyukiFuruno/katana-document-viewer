#[path = "spreadsheet_grid_mapping.rs"]
mod mapping;

use super::{
    DocumentGridCommand, DocumentGridNavigation, DocumentSurfaceError, DocumentSurfaceFrame,
    DocumentViewport,
};
use crate::multi_format::SpreadsheetMaterializedCell;
use crate::{SpreadsheetCellArtifact, SpreadsheetCoordinate, SpreadsheetSheetArtifact};
use katana_ui_core::molecule::{GenericGrid, GridCoordinate, GridEvent, GridViewport};
use katana_ui_core::render_model::UiGridValidationError;
use mapping::{
    cell_content, cell_span, materialized_cell_content, row_track_provider, spreadsheet_coordinate,
    track_provider,
};
use support::{adjusted_frozen_panes, grid_action};

const DEFAULT_ROW_SIZE: u32 = 20;
const DEFAULT_COLUMN_SIZE: u32 = 80;

impl From<UiGridValidationError> for DocumentSurfaceError {
    fn from(value: UiGridValidationError) -> Self {
        Self::InvalidGrid {
            detail: format!("{value:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpreadsheetGridSurface {
    sheet_index: usize,
    grid: GenericGrid,
}

impl SpreadsheetGridSurface {
    pub fn new(
        sheet: &SpreadsheetSheetArtifact,
        viewport: DocumentViewport,
    ) -> Result<Self, DocumentSurfaceError> {
        Self::new_filtered(sheet, &[], viewport)
    }

    pub(crate) fn new_filtered(
        sheet: &SpreadsheetSheetArtifact,
        filtered_out_rows: &[usize],
        viewport: DocumentViewport,
    ) -> Result<Self, DocumentSurfaceError> {
        let (frozen_rows, frozen_columns) = adjusted_frozen_panes(sheet);
        let mut grid = GenericGrid::new(&sheet.name, sheet.row_count, sheet.column_count)
            .row_tracks(row_track_provider(
                &sheet.row_tracks,
                DEFAULT_ROW_SIZE,
                filtered_out_rows,
            ))
            .column_tracks(track_provider(&sheet.column_tracks, DEFAULT_COLUMN_SIZE))
            .viewport(GridViewport::new(viewport.width, viewport.height))
            .overscan(1, 1)
            .frozen(frozen_rows, frozen_columns)
            .show_grid_lines(sheet.show_grid_lines)
            .with_cell_spans(sheet.merged_cells.iter().copied().map(cell_span).collect())?;
        if sheet.row_count > 0 && sheet.column_count > 0 {
            grid = grid.active_cell(GridCoordinate::new(0, 0));
        }
        Ok(Self {
            sheet_index: sheet.index,
            grid,
        })
    }

    #[must_use]
    pub const fn sheet_index(&self) -> usize {
        self.sheet_index
    }

    #[must_use]
    pub fn materialization_request(&self) -> Vec<SpreadsheetCoordinate> {
        self.grid
            .visible_coordinates()
            .into_iter()
            .map(spreadsheet_coordinate)
            .collect()
    }

    pub fn supply_cells(
        &mut self,
        cells: Vec<SpreadsheetCellArtifact>,
    ) -> Result<(), DocumentSurfaceError> {
        self.grid = self
            .grid
            .clone()
            .with_visible_cells(cells.into_iter().map(cell_content).collect())?;
        Ok(())
    }

    pub(crate) fn supply_materialized_cells(
        &mut self,
        cells: Vec<SpreadsheetMaterializedCell>,
    ) -> Result<(), DocumentSurfaceError> {
        self.grid = self
            .grid
            .clone()
            .with_visible_cells(cells.into_iter().map(materialized_cell_content).collect())?;
        Ok(())
    }

    pub fn apply_command(&mut self, command: DocumentGridCommand) -> super::DocumentGridEvent {
        let Some(action) = grid_action(&self.grid, command) else {
            return super::DocumentGridEvent::None;
        };
        document_grid_event(self.grid.apply_action(action))
    }

    pub fn frame(&self) -> Result<DocumentSurfaceFrame, DocumentSurfaceError> {
        DocumentSurfaceFrame::from_node(self.grid.clone().into())
    }
}

const fn document_grid_event(event: GridEvent) -> super::DocumentGridEvent {
    match event {
        GridEvent::None => super::DocumentGridEvent::None,
        GridEvent::SelectionChanged(_) => super::DocumentGridEvent::SelectionChanged,
        GridEvent::Scrolled(_) => super::DocumentGridEvent::Scrolled,
    }
}

#[path = "spreadsheet_grid_support.rs"]
mod support;

#[cfg(test)]
#[path = "spreadsheet_grid_alignment_tests.rs"]
mod alignment_tests;
#[cfg(test)]
#[path = "spreadsheet_grid_appearance_tests.rs"]
mod appearance_tests;
#[cfg(test)]
#[path = "spreadsheet_grid_command_tests.rs"]
mod command_tests;
#[cfg(test)]
#[path = "spreadsheet_grid_filter_tests.rs"]
mod filter_tests;
#[cfg(test)]
#[path = "spreadsheet_grid_mapping_tests.rs"]
mod mapping_tests;
#[cfg(test)]
#[path = "spreadsheet_grid_pointer_tests.rs"]
mod pointer_tests;
#[path = "spreadsheet_grid_state.rs"]
mod state;
#[cfg(test)]
#[path = "spreadsheet_grid_test_support.rs"]
mod test_support;
#[cfg(test)]
#[path = "spreadsheet_grid_tests.rs"]
mod tests;
