use super::{
    SpreadsheetAutoFilterArtifact, SpreadsheetCoordinate, SpreadsheetSheetArtifact,
    SpreadsheetViewerLimits, spreadsheet_filter_engine::SpreadsheetActiveFilters,
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

#[path = "spreadsheet_engine_session.rs"]
mod session;

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
