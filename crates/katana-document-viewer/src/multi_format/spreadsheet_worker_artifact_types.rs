use super::{SpreadsheetCoordinate, SpreadsheetMergedCellArtifact, SpreadsheetTrackArtifact};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadsheetDataBarArtifact {
    pub positive_color: Option<String>,
    pub negative_color: Option<String>,
    pub value: f64,
    pub axis_position: f64,
    pub gradient: bool,
    pub show_value: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadsheetIconArtifact {
    pub name: String,
    pub color: Option<String>,
    pub show_value: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadsheetRatingArtifact {
    pub icon_name: String,
    pub count: u32,
    pub maximum: u32,
    pub color: Option<String>,
    pub show_value: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadsheetConditionalFormattingArtifact {
    pub applied: bool,
    pub data_bar: Option<SpreadsheetDataBarArtifact>,
    pub icon: Option<SpreadsheetIconArtifact>,
    pub rating: Option<SpreadsheetRatingArtifact>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpreadsheetFilterRange {
    pub start: SpreadsheetCoordinate,
    pub end: SpreadsheetCoordinate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum SpreadsheetFilterCriterion {
    Values(Vec<String>),
    Blank,
    NonBlank,
    Unsupported(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpreadsheetFilterColumnArtifact {
    pub column: usize,
    pub criteria: Vec<SpreadsheetFilterCriterion>,
    pub candidates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpreadsheetAutoFilterArtifact {
    pub range: SpreadsheetFilterRange,
    pub columns: Vec<SpreadsheetFilterColumnArtifact>,
    pub filtered_out_rows: Vec<usize>,
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadsheetSheetArtifact {
    pub index: usize,
    pub name: String,
    pub row_count: usize,
    pub column_count: usize,
    pub row_tracks: Vec<SpreadsheetTrackArtifact>,
    pub column_tracks: Vec<SpreadsheetTrackArtifact>,
    pub frozen_rows: usize,
    pub frozen_columns: usize,
    pub merged_cells: Vec<SpreadsheetMergedCellArtifact>,
    pub show_grid_lines: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct SpreadsheetOpenedSheet {
    pub(crate) sheet: SpreadsheetSheetArtifact,
    #[serde(default)]
    pub(crate) auto_filter: Option<SpreadsheetAutoFilterArtifact>,
}
