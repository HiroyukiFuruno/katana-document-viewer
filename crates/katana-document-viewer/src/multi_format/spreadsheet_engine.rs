use super::{
    SpreadsheetAutoFilterArtifact, SpreadsheetCoordinate, SpreadsheetMaterializedCell,
    SpreadsheetOpenedSheet, SpreadsheetSheetArtifact, SpreadsheetViewerLimits,
    spreadsheet_engine_cell::SpreadsheetCellMaterializer,
    spreadsheet_engine_sheet::SpreadsheetSheetBuilder,
    spreadsheet_filter_engine::SpreadsheetActiveFilters,
    spreadsheet_filter_xml::SpreadsheetFilterCatalog,
    spreadsheet_streaming::StreamingSpreadsheetSession,
};
use ironcalc::base::Model;

pub(crate) use super::spreadsheet_engine_support::SpreadsheetEngineSupport;
#[path = "spreadsheet_engine_error.rs"]
mod error;
pub(super) use error::SpreadsheetEngineError;
#[path = "spreadsheet_engine_model.rs"]
mod model;
use model::load_model;

pub(super) struct SpreadsheetEngineSession {
    backend: SpreadsheetEngineBackend,
    sheets: Vec<SpreadsheetSheetArtifact>,
    filters: Vec<Option<SpreadsheetAutoFilterArtifact>>,
    limits: SpreadsheetViewerLimits,
    active_filters: SpreadsheetActiveFilters,
}

enum SpreadsheetEngineBackend {
    Model(Box<Model<'static>>),
    Streaming(StreamingSpreadsheetSession),
}

impl SpreadsheetEngineSession {
    pub(super) fn open(
        bytes: Vec<u8>,
        name: &str,
        limits: SpreadsheetViewerLimits,
    ) -> Result<Self, SpreadsheetEngineError> {
        let filters = {
            let _trace = super::debug_trace::DebugTrace::start("spreadsheet.filter_catalog");
            SpreadsheetFilterCatalog::read(&bytes, limits.max_sheets)?
        };
        let streaming_required = {
            let _trace = super::debug_trace::DebugTrace::start("spreadsheet.streaming_detection");
            StreamingSpreadsheetSession::is_required(&bytes)?
        };
        if streaming_required {
            return Self::open_streaming(bytes, limits, filters);
        }
        Self::open_model(bytes, name, limits, filters)
    }

    fn open_model(
        bytes: Vec<u8>,
        name: &str,
        limits: SpreadsheetViewerLimits,
        filters: Vec<Option<super::SpreadsheetAutoFilterArtifact>>,
    ) -> Result<Self, SpreadsheetEngineError> {
        let model = load_model(&bytes, name)?;
        let sheets = {
            let _trace = super::debug_trace::DebugTrace::start("spreadsheet.sheet_artifacts");
            SpreadsheetSheetBuilder::build(&model, limits.max_sheets, limits.max_logical_cells)?
        };
        let mut session = Self {
            backend: SpreadsheetEngineBackend::Model(Box::new(model)),
            active_filters: Vec::new(),
            sheets,
            filters,
            limits,
        };
        {
            let _trace = super::debug_trace::DebugTrace::start("spreadsheet.persisted_filters");
            session.initialize_persisted_filters()?;
        }
        Ok(session)
    }

    fn open_streaming(
        bytes: Vec<u8>,
        limits: SpreadsheetViewerLimits,
        filters: Vec<Option<super::SpreadsheetAutoFilterArtifact>>,
    ) -> Result<Self, SpreadsheetEngineError> {
        let streaming = {
            let _trace = super::debug_trace::DebugTrace::start("spreadsheet.streaming_open");
            StreamingSpreadsheetSession::open(bytes, limits)?
        };
        let sheets = streaming.sheets().to_vec();
        let mut session = Self {
            backend: SpreadsheetEngineBackend::Streaming(streaming),
            active_filters: Vec::new(),
            sheets,
            filters,
            limits,
        };
        {
            let _trace = super::debug_trace::DebugTrace::start("spreadsheet.persisted_filters");
            session.initialize_persisted_filters()?;
        }
        Ok(session)
    }

    #[cfg(test)]
    pub(super) fn sheets(&self) -> &[SpreadsheetSheetArtifact] {
        &self.sheets
    }

    pub(super) fn opened_sheets(&self) -> Vec<SpreadsheetOpenedSheet> {
        self.sheets
            .iter()
            .cloned()
            .zip(self.filters.iter().cloned())
            .map(|(sheet, auto_filter)| SpreadsheetOpenedSheet { sheet, auto_filter })
            .collect()
    }

    #[cfg(test)]
    pub(super) fn auto_filters(&self) -> &[Option<SpreadsheetAutoFilterArtifact>] {
        &self.filters
    }

    pub(super) fn auto_filter(
        &self,
        sheet_index: usize,
    ) -> Result<Option<&SpreadsheetAutoFilterArtifact>, SpreadsheetEngineError> {
        if sheet_index >= self.sheets.len() {
            return Err(SpreadsheetEngineError::SheetOutsideDocument {
                requested: sheet_index,
                sheet_count: self.sheets.len(),
            });
        }
        Ok(self.filters.get(sheet_index).and_then(Option::as_ref))
    }

    #[cfg(test)]
    pub(super) fn auto_filter_mut(
        &mut self,
        sheet_index: usize,
    ) -> Result<Option<&mut SpreadsheetAutoFilterArtifact>, SpreadsheetEngineError> {
        let sheet_count = self.sheets.len();
        if sheet_index >= sheet_count {
            return Err(SpreadsheetEngineError::SheetOutsideDocument {
                requested: sheet_index,
                sheet_count,
            });
        }
        Ok(self.filters.get_mut(sheet_index).and_then(Option::as_mut))
    }

    pub(super) fn materialize(
        &self,
        sheet_index: usize,
        coordinates: &[SpreadsheetCoordinate],
    ) -> Result<Vec<SpreadsheetMaterializedCell>, SpreadsheetEngineError> {
        self.validate_request(sheet_index, coordinates)?;
        match &self.backend {
            SpreadsheetEngineBackend::Model(model) => coordinates
                .iter()
                .copied()
                .map(|coordinate| {
                    SpreadsheetCellMaterializer::materialize(model, sheet_index, coordinate)
                })
                .collect(),
            SpreadsheetEngineBackend::Streaming(streaming) => streaming
                .materialize(sheet_index, coordinates)
                .map(|cells| {
                    cells
                        .into_iter()
                        .map(SpreadsheetMaterializedCell::without_borders)
                        .collect()
                }),
        }
    }
}

#[path = "spreadsheet_engine_filter.rs"]
mod filter;
#[path = "spreadsheet_engine_validation.rs"]
mod validation;

#[cfg(test)]
#[path = "spreadsheet_engine_filter_error_tests.rs"]
mod filter_error_tests;
#[cfg(test)]
#[path = "spreadsheet_engine_filter_persisted_tests.rs"]
mod filter_persisted_tests;
#[cfg(test)]
#[path = "spreadsheet_engine_filter_tests.rs"]
mod filter_tests;
#[cfg(test)]
#[path = "spreadsheet_engine_tests.rs"]
mod tests;
