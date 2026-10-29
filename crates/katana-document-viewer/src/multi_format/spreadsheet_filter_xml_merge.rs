use super::spreadsheet_engine::{SpreadsheetEngineError, SpreadsheetEngineSupport};
use super::spreadsheet_streaming_xml_values::attribute;
use super::{SpreadsheetCoordinate, SpreadsheetMergedCellArtifact};
use ironcalc::base::expressions::utils::{
    is_valid_column_number, is_valid_row, parse_reference_a1,
};
use quick_xml::Reader;
use quick_xml::events::BytesStart;
use std::io::BufRead;

#[derive(Default)]
pub(super) struct SpreadsheetMergeParser {
    capture: bool,
    section_depths: Vec<usize>,
    section_count: usize,
    references: Option<Vec<Option<String>>>,
}

impl SpreadsheetMergeParser {
    pub(super) fn new(capture: bool) -> Self {
        Self {
            capture,
            ..Self::default()
        }
    }

    pub(super) fn is_worksheet_section(&self, xml_depth: usize) -> bool {
        self.capture && xml_depth == 1
    }

    pub(super) fn start_section(&mut self, xml_depth: usize, is_empty: bool) {
        self.section_count += 1;
        self.references = if self.section_count == 1 {
            Some(Vec::new())
        } else {
            None
        };
        if !is_empty {
            self.section_depths.push(xml_depth);
        }
    }

    pub(super) fn is_merge_cell(&self, xml_depth: usize) -> bool {
        self.capture
            && self
                .section_depths
                .last()
                .is_some_and(|depth| depth.saturating_add(1) == xml_depth)
    }

    pub(super) fn capture_reference(
        &mut self,
        reader: &Reader<impl BufRead>,
        event: &BytesStart<'_>,
    ) -> Result<(), SpreadsheetEngineError> {
        if let Some(references) = self.references.as_mut() {
            references.push(attribute(reader, event, b"ref")?);
        }
        Ok(())
    }

    pub(super) fn end_section(&mut self, xml_depth: usize) {
        if self
            .section_depths
            .last()
            .is_some_and(|depth| depth.saturating_add(1) == xml_depth)
        {
            self.section_depths.pop();
        }
    }

    pub(super) fn finish(
        self,
    ) -> Result<Vec<SpreadsheetMergedCellArtifact>, SpreadsheetEngineError> {
        match (self.section_count, self.references) {
            (1, Some(references)) => references
                .into_iter()
                .map(|reference| {
                    let reference = reference.ok_or_else(missing_reference_error)?;
                    parse_merge_cell(&reference)
                })
                .collect(),
            _ => Ok(Vec::new()),
        }
    }
}

fn missing_reference_error() -> SpreadsheetEngineError {
    SpreadsheetEngineError::Import("required spreadsheet XML attribute `ref` is missing".into())
}

fn parse_merge_cell(
    reference: &str,
) -> Result<SpreadsheetMergedCellArtifact, SpreadsheetEngineError> {
    let invalid_range = || SpreadsheetEngineError::InvalidMergedCell(reference.to_owned());
    let Some((start, end)) = reference.split_once(':') else {
        return Err(invalid_range());
    };
    let start = parse_reference_a1(start).ok_or_else(invalid_range)?;
    let end = parse_reference_a1(end).ok_or_else(invalid_range)?;
    if end.row < start.row
        || end.column < start.column
        || !is_valid_row(start.row)
        || !is_valid_column_number(start.column)
        || !is_valid_row(end.row)
        || !is_valid_column_number(end.column)
    {
        return Err(invalid_range());
    }
    Ok(SpreadsheetMergedCellArtifact {
        anchor: SpreadsheetCoordinate::new(
            SpreadsheetEngineSupport::zero_based(start.row)?,
            SpreadsheetEngineSupport::zero_based(start.column)?,
        ),
        row_span: SpreadsheetEngineSupport::span(start.row, end.row)?,
        column_span: SpreadsheetEngineSupport::span(start.column, end.column)?,
    })
}
