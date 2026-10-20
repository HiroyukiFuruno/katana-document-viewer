use super::spreadsheet_engine::{SpreadsheetEngineError, SpreadsheetEngineSupport};
use super::spreadsheet_filter_xml_parser::{ParsedWorksheetMetadata, parse_worksheet};
use super::spreadsheet_streaming_xml::{
    WorkbookSheet, parse_relationships, parse_workbook_sheets, read_zip_entry,
};
use super::{SpreadsheetAutoFilterArtifact, SpreadsheetMergedCellArtifact};
use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use zip::ZipArchive;

const METADATA_ENTRY_LIMIT: u64 = 16 * 1024 * 1024;

pub(super) struct SpreadsheetFilterCatalog;

pub(super) struct SpreadsheetWorksheetMetadata {
    pub(super) name: String,
    pub(super) auto_filter: Option<SpreadsheetAutoFilterArtifact>,
    pub(super) merged_cells: Vec<SpreadsheetMergedCellArtifact>,
}

impl SpreadsheetFilterCatalog {
    pub(super) fn read(
        bytes: &[u8],
        max_sheets: usize,
        capture_merged_cells: bool,
    ) -> Result<Vec<SpreadsheetWorksheetMetadata>, SpreadsheetEngineError> {
        let mut archive = match ZipArchive::new(Cursor::new(bytes)) {
            Ok(archive) => archive,
            Err(error) => return Err(SpreadsheetEngineError::Import(error.to_string())),
        };
        let workbook = read_zip_entry(&mut archive, "xl/workbook.xml", METADATA_ENTRY_LIMIT)?;
        let relationships = read_zip_entry(
            &mut archive,
            "xl/_rels/workbook.xml.rels",
            METADATA_ENTRY_LIMIT,
        )?;
        let sheets = parse_workbook_sheets(&workbook)?;
        SpreadsheetEngineSupport::check_limit("sheet_count", sheets.len(), max_sheets)?;
        let targets = parse_relationships(&relationships)?;
        sheets
            .iter()
            .map(|sheet| Self::read_sheet(&mut archive, sheet, &targets, capture_merged_cells))
            .collect()
    }

    fn read_sheet(
        archive: &mut ZipArchive<Cursor<&[u8]>>,
        sheet: &WorkbookSheet,
        targets: &HashMap<String, String>,
        capture_merged_cells: bool,
    ) -> Result<SpreadsheetWorksheetMetadata, SpreadsheetEngineError> {
        let path = worksheet_path(sheet, targets)?;
        let entry = match archive.by_name(&path) {
            Ok(entry) => entry,
            Err(error) => return Err(SpreadsheetEngineError::Import(error.to_string())),
        };
        let ParsedWorksheetMetadata {
            auto_filter,
            merged_cells,
        } = parse_worksheet(BufReader::new(entry), capture_merged_cells)?;
        Ok(SpreadsheetWorksheetMetadata {
            name: sheet.name.clone(),
            auto_filter,
            merged_cells,
        })
    }
}

fn worksheet_path(
    sheet: &WorkbookSheet,
    targets: &HashMap<String, String>,
) -> Result<String, SpreadsheetEngineError> {
    let target = match targets.get(&sheet.relationship_id) {
        Some(target) => target,
        None => {
            return Err(SpreadsheetEngineError::Import(
                "worksheet relationship is missing".into(),
            ));
        }
    };
    let target = target.trim_start_matches('/').trim_start_matches("./");
    if target.starts_with("xl/") {
        Ok(target.to_owned())
    } else {
        Ok(format!("xl/{target}"))
    }
}
