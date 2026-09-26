use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpreadsheetCoordinate {
    pub row: usize,
    pub column: usize,
}

impl SpreadsheetCoordinate {
    #[must_use]
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpreadsheetTrackArtifact {
    pub size: f32,
    pub hidden: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpreadsheetHorizontalAlignment {
    General,
    Left,
    Center,
    CenterContinuous,
    Right,
    Fill,
    Justify,
    Distributed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpreadsheetVerticalAlignment {
    Bottom,
    Center,
    Top,
    Justify,
    Distributed,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpreadsheetCellBorderArtifact {
    pub left: Option<SpreadsheetBorderSideArtifact>,
    pub right: Option<SpreadsheetBorderSideArtifact>,
    pub top: Option<SpreadsheetBorderSideArtifact>,
    pub bottom: Option<SpreadsheetBorderSideArtifact>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpreadsheetBorderSideArtifact {
    pub style: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadsheetCellStyleArtifact {
    pub font_name: String,
    pub font_size: f32,
    pub font_color: Option<String>,
    pub fill_color: Option<String>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub horizontal_alignment: SpreadsheetHorizontalAlignment,
    pub vertical_alignment: SpreadsheetVerticalAlignment,
    pub wrap_text: bool,
    pub number_format: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct SpreadsheetMaterializedCell {
    pub(crate) cell: SpreadsheetCellArtifact,
    #[serde(default)]
    pub(crate) borders: SpreadsheetCellBorderArtifact,
}

impl SpreadsheetMaterializedCell {
    pub(super) fn without_borders(cell: SpreadsheetCellArtifact) -> Self {
        Self {
            cell,
            borders: SpreadsheetCellBorderArtifact::default(),
        }
    }
}

impl From<SpreadsheetCellArtifact> for SpreadsheetMaterializedCell {
    fn from(cell: SpreadsheetCellArtifact) -> Self {
        Self::without_borders(cell)
    }
}

impl std::ops::Deref for SpreadsheetMaterializedCell {
    type Target = SpreadsheetCellArtifact;

    fn deref(&self) -> &Self::Target {
        &self.cell
    }
}

impl std::ops::DerefMut for SpreadsheetMaterializedCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cell
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum SpreadsheetCellValue {
    Empty,
    Text(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpreadsheetCellArtifact {
    pub coordinate: SpreadsheetCoordinate,
    pub display_text: String,
    pub value: SpreadsheetCellValue,
    pub formula: Option<String>,
    pub style: SpreadsheetCellStyleArtifact,
    pub conditional_formatting: SpreadsheetConditionalFormattingArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpreadsheetMergedCellArtifact {
    pub anchor: SpreadsheetCoordinate,
    pub row_span: usize,
    pub column_span: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpreadsheetViewerLimits {
    pub max_sheets: usize,
    pub max_logical_cells: usize,
    pub max_materialized_cells: usize,
}

impl SpreadsheetViewerLimits {
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            max_sheets: 256,
            max_logical_cells: 25_000_000,
            max_materialized_cells: 4_096,
        }
    }
}

#[path = "spreadsheet_worker_artifact_types.rs"]
mod types;
pub(crate) use types::SpreadsheetOpenedSheet;
pub use types::{
    SpreadsheetAutoFilterArtifact, SpreadsheetConditionalFormattingArtifact,
    SpreadsheetDataBarArtifact, SpreadsheetFilterColumnArtifact, SpreadsheetFilterCriterion,
    SpreadsheetFilterRange, SpreadsheetIconArtifact, SpreadsheetRatingArtifact,
    SpreadsheetSheetArtifact,
};
