use super::spreadsheet_engine::{SpreadsheetEngineError, SpreadsheetEngineSupport};
use super::spreadsheet_streaming_xml_values::attribute;
use super::{SpreadsheetAutoFilterArtifact, SpreadsheetCoordinate, SpreadsheetFilterRange};
use ironcalc::base::expressions::utils::parse_reference_a1;
use quick_xml::Reader;
use quick_xml::events::BytesStart;
use std::io::BufRead;

pub(super) fn auto_filter(
    reader: &Reader<impl BufRead>,
    event: &BytesStart<'_>,
) -> Result<Option<SpreadsheetAutoFilterArtifact>, SpreadsheetEngineError> {
    attribute(reader, event, b"ref")?
        .map(|reference| {
            Ok(SpreadsheetAutoFilterArtifact {
                range: filter_range(&reference)?,
                columns: Vec::new(),
                filtered_out_rows: Vec::new(),
                diagnostics: Vec::new(),
            })
        })
        .transpose()
}

fn filter_range(reference: &str) -> Result<SpreadsheetFilterRange, SpreadsheetEngineError> {
    let (start, end) = reference.split_once(':').unwrap_or((reference, reference));
    Ok(SpreadsheetFilterRange {
        start: coordinate(start)?,
        end: coordinate(end)?,
    })
}

fn coordinate(reference: &str) -> Result<SpreadsheetCoordinate, SpreadsheetEngineError> {
    let parsed = parse_reference_a1(reference)
        .ok_or_else(|| SpreadsheetEngineError::Import("invalid AutoFilter range".into()))?;
    Ok(SpreadsheetCoordinate::new(
        SpreadsheetEngineSupport::zero_based(parsed.row)?,
        SpreadsheetEngineSupport::zero_based(parsed.column)?,
    ))
}
