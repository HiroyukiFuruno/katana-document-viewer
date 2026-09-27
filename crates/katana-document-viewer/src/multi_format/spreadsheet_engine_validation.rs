use super::{
    SpreadsheetCoordinate, SpreadsheetEngineError, SpreadsheetEngineSession,
    SpreadsheetSheetArtifact,
};
use std::collections::HashSet;

pub(super) struct SpreadsheetEngineValidation;

impl SpreadsheetEngineValidation {
    pub(super) fn validate_request(
        engine: &SpreadsheetEngineSession,
        sheet_index: usize,
        coordinates: &[SpreadsheetCoordinate],
    ) -> Result<(), SpreadsheetEngineError> {
        super::SpreadsheetEngineSupport::check_limit(
            "materialized_cell_count",
            coordinates.len(),
            engine.limits.max_materialized_cells,
        )?;
        let sheet = engine.sheet(sheet_index)?;
        let mut seen = HashSet::with_capacity(coordinates.len());
        for coordinate in coordinates {
            if coordinate.row >= sheet.row_count || coordinate.column >= sheet.column_count {
                return Err(Self::outside_cell(sheet_index, *coordinate));
            }
            if !seen.insert(*coordinate) {
                return Err(SpreadsheetEngineError::DuplicateCell {
                    row: coordinate.row,
                    column: coordinate.column,
                });
            }
        }
        Ok(())
    }

    fn outside_cell(
        sheet_index: usize,
        coordinate: SpreadsheetCoordinate,
    ) -> SpreadsheetEngineError {
        SpreadsheetEngineError::CellOutsideSheet {
            sheet_index,
            row: coordinate.row,
            column: coordinate.column,
        }
    }
}

impl SpreadsheetEngineSession {
    pub(crate) fn validate_request(
        &self,
        sheet_index: usize,
        coordinates: &[SpreadsheetCoordinate],
    ) -> Result<(), SpreadsheetEngineError> {
        SpreadsheetEngineValidation::validate_request(self, sheet_index, coordinates)
    }

    pub(crate) fn sheet(
        &self,
        requested: usize,
    ) -> Result<&SpreadsheetSheetArtifact, SpreadsheetEngineError> {
        self.sheets
            .get(requested)
            .ok_or(SpreadsheetEngineError::SheetOutsideDocument {
                requested,
                sheet_count: self.sheets.len(),
            })
    }
}
