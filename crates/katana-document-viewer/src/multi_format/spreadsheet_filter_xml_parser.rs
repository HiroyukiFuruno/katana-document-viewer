use super::spreadsheet_engine::SpreadsheetEngineError;
use super::spreadsheet_filter_xml_filter::auto_filter;
use super::spreadsheet_filter_xml_merge::SpreadsheetMergeParser;
use super::spreadsheet_streaming_xml_values::{attribute, attribute_usize, xml_error};
use super::{
    SpreadsheetAutoFilterArtifact, SpreadsheetFilterColumnArtifact, SpreadsheetFilterCriterion,
    SpreadsheetMergedCellArtifact,
};
use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

pub(super) fn parse_worksheet(
    input: impl BufRead,
    capture_merged_cells: bool,
) -> Result<ParsedWorksheetMetadata, SpreadsheetEngineError> {
    let mut parser = FilterParser {
        merged_cells: SpreadsheetMergeParser::new(capture_merged_cells),
        ..FilterParser::default()
    };
    let mut reader = Reader::from_reader(input);
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(event)) => {
                parser.start(&reader, &event, false)?;
                parser.xml_depth += 1;
            }
            Ok(Event::Empty(event)) => {
                parser.start(&reader, &event, true)?;
                if event.local_name().as_ref() == "filterColumn" {
                    parser.finish_column();
                }
            }
            Ok(Event::End(event)) => parser.end(&event),
            Ok(Event::Eof) => return parser.finish(),
            Ok(_) => {}
            Err(error) => return Err(xml_error(error)),
        }
        buffer.clear();
    }
}

pub(super) struct ParsedWorksheetMetadata {
    pub(super) auto_filter: Option<SpreadsheetAutoFilterArtifact>,
    pub(super) merged_cells: Vec<SpreadsheetMergedCellArtifact>,
}

#[derive(Default)]
struct FilterParser {
    xml_depth: usize,
    merged_cells: SpreadsheetMergeParser,
    filter: Option<SpreadsheetAutoFilterArtifact>,
    column: Option<SpreadsheetFilterColumnArtifact>,
}

impl FilterParser {
    fn start(
        &mut self,
        reader: &Reader<impl BufRead>,
        event: &BytesStart<'_>,
        is_empty: bool,
    ) -> Result<(), SpreadsheetEngineError> {
        match event.local_name().as_ref() {
            "autoFilter" => self.filter = auto_filter(reader, event)?,
            "mergeCells" if self.merged_cells.is_worksheet_section(self.xml_depth) => {
                self.merged_cells.start_section(self.xml_depth, is_empty);
            }
            "mergeCell" if self.merged_cells.is_merge_cell(self.xml_depth) => {
                self.merged_cells.capture_reference(reader, event)?;
            }
            "filterColumn" => self.column = filter_column(reader, event, self.filter.as_ref())?,
            "filters" => add_blank(reader, event, self.column.as_mut())?,
            "filter" => add_value(reader, event, self.column.as_mut())?,
            kind if unsupported_filter(kind) => self.add_unsupported(kind),
            _ => {}
        }
        Ok(())
    }

    fn end(&mut self, event: &quick_xml::events::BytesEnd<'_>) {
        if event.local_name().as_ref() == "mergeCells" {
            self.merged_cells.end_section(self.xml_depth);
        }
        if event.local_name().as_ref() == "filterColumn" {
            self.finish_column();
        }
        self.xml_depth = self.xml_depth.saturating_sub(1);
    }

    fn finish(self) -> Result<ParsedWorksheetMetadata, SpreadsheetEngineError> {
        Ok(ParsedWorksheetMetadata {
            auto_filter: self.filter,
            merged_cells: self.merged_cells.finish()?,
        })
    }

    fn finish_column(&mut self) {
        if let (Some(column), Some(filter)) = (self.column.take(), self.filter.as_mut()) {
            filter.columns.push(column);
        }
    }

    fn add_unsupported(&mut self, kind: &str) {
        let kind = kind.to_owned();
        if let Some(column) = self.column.as_mut() {
            column
                .criteria
                .push(SpreadsheetFilterCriterion::Unsupported(kind.clone()));
        }
        if let Some(filter) = self.filter.as_mut() {
            filter
                .diagnostics
                .push(format!("unsupported AutoFilter criterion `{kind}`"));
        }
    }
}

fn filter_column(
    reader: &Reader<impl BufRead>,
    event: &BytesStart<'_>,
    filter: Option<&SpreadsheetAutoFilterArtifact>,
) -> Result<Option<SpreadsheetFilterColumnArtifact>, SpreadsheetEngineError> {
    let Some(filter) = filter else {
        return Ok(None);
    };
    let relative = attribute_usize(reader, event, b"colId")?.unwrap_or(0);
    Ok(Some(SpreadsheetFilterColumnArtifact {
        column: filter.range.start.column.saturating_add(relative),
        criteria: Vec::new(),
        candidates: Vec::new(),
    }))
}

fn add_blank(
    reader: &Reader<impl BufRead>,
    event: &BytesStart<'_>,
    column: Option<&mut SpreadsheetFilterColumnArtifact>,
) -> Result<(), SpreadsheetEngineError> {
    if matches!(
        attribute(reader, event, b"blank")?.as_deref(),
        Some("1" | "true")
    ) && let Some(column) = column
    {
        column.criteria.push(SpreadsheetFilterCriterion::Blank);
    }
    Ok(())
}

fn add_value(
    reader: &Reader<impl BufRead>,
    event: &BytesStart<'_>,
    column: Option<&mut SpreadsheetFilterColumnArtifact>,
) -> Result<(), SpreadsheetEngineError> {
    if let (Some(column), Some(value)) = (column, attribute(reader, event, b"val")?) {
        match column.criteria.last_mut() {
            Some(SpreadsheetFilterCriterion::Values(values)) => values.push(value),
            _ => column
                .criteria
                .push(SpreadsheetFilterCriterion::Values(vec![value])),
        }
    }
    Ok(())
}

fn unsupported_filter(kind: &str) -> bool {
    matches!(
        kind,
        "customFilters"
            | "dynamicFilter"
            | "top10"
            | "colorFilter"
            | "iconFilter"
            | "dateGroupItem"
    )
}
