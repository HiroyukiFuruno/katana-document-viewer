use super::SpreadsheetFilterCriterion;
use super::spreadsheet_engine::SpreadsheetEngineError;
use super::spreadsheet_filter_test_support::{
    filter_catalog_package, representative_with_worksheet_xml,
};
use super::spreadsheet_filter_xml::SpreadsheetFilterCatalog;
use super::spreadsheet_filter_xml_parser::parse_worksheet;
use crate::multi_format::SpreadsheetViewerLimits;
use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn parses_value_blank_and_unsupported_filter_metadata() -> TestResult {
    let xml = br#"<worksheet><sheetData/><autoFilter ref="A1:C4"><filterColumn colId="1"><filters blank="1"><filter val="Open"/><filter val="Closed"/></filters></filterColumn><filterColumn colId="2"><customFilters/></filterColumn></autoFilter></worksheet>"#;
    let filter = parse_worksheet(Cursor::new(xml.as_slice()), false)?
        .auto_filter
        .ok_or("auto filter was not parsed")?;
    assert_eq!(1, filter.columns[0].column);
    assert_eq!(
        SpreadsheetFilterCriterion::Blank,
        filter.columns[0].criteria[0]
    );
    assert!(matches!(
        filter.columns[1].criteria.as_slice(),
        [SpreadsheetFilterCriterion::Unsupported(kind)] if kind == "customFilters"
    ));
    assert_eq!(1, filter.diagnostics.len());
    Ok(())
}

#[test]
fn marks_date_group_items_unsupported_until_typed_semantics_exist() -> TestResult {
    let xml = br#"<worksheet><autoFilter ref="A1:B4"><filterColumn colId="0"><filters><dateGroupItem year="2026" month="9" dateTimeGrouping="month"/></filters></filterColumn></autoFilter></worksheet>"#;
    let filter = parse_worksheet(Cursor::new(xml.as_slice()), false)?
        .auto_filter
        .ok_or("auto filter was not parsed")?;

    assert!(matches!(
        filter.columns[0].criteria.as_slice(),
        [SpreadsheetFilterCriterion::Unsupported(kind)] if kind == "dateGroupItem"
    ));
    assert_eq!(
        vec!["unsupported AutoFilter criterion `dateGroupItem`".to_string()],
        filter.diagnostics
    );
    Ok(())
}

#[test]
fn reads_filter_metadata_in_workbook_sheet_order() -> TestResult {
    let workbook =
        br#"<workbook xmlns:r="urn:r"><sheets><sheet name="Data" r:id="r1"/></sheets></workbook>"#;
    let relationships =
        br#"<Relationships><Relationship Id="r1" Target="worksheets/sheet1.xml"/></Relationships>"#;
    let worksheet = br#"<worksheet><sheetData/><autoFilter ref="B2:D8"><filterColumn colId="0"><filters><filter val="East"/></filters></filterColumn></autoFilter></worksheet>"#;
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for (name, bytes) in [
        ("xl/workbook.xml", workbook.as_slice()),
        ("xl/_rels/workbook.xml.rels", relationships.as_slice()),
        ("xl/worksheets/sheet1.xml", worksheet.as_slice()),
    ] {
        writer.start_file(name, SimpleFileOptions::default())?;
        writer.write_all(bytes)?;
    }
    let bytes = writer.finish()?.into_inner();
    let sheets = SpreadsheetFilterCatalog::read(&bytes, 1, false)?;
    let filter = sheets[0]
        .auto_filter
        .as_ref()
        .ok_or("filter was not attached")?;
    assert_eq!(1, filter.range.start.row);
    assert_eq!(1, filter.columns[0].column);
    Ok(())
}

#[test]
fn parser_rejects_malformed_ranges_and_xml() {
    assert!(parse_worksheet(Cursor::new(b"<worksheet><".as_slice()), false).is_err());
    assert!(
        parse_worksheet(
            Cursor::new(br#"<worksheet><autoFilter ref="invalid!"/></worksheet>"#.as_slice()),
            false
        )
        .is_err()
    );
}

#[test]
fn parser_handles_empty_columns_and_elements_before_filter_metadata() -> TestResult {
    let xml = br#"<worksheet><filterColumn colId="2"/><filter val="ignored"/><autoFilter ref="A1:B2"><filterColumn colId="0"/></autoFilter></worksheet>"#;
    let filter = parse_worksheet(Cursor::new(xml.as_slice()), false)?
        .auto_filter
        .ok_or("filter missing")?;
    assert_eq!(1, filter.columns.len());
    assert!(filter.columns[0].criteria.is_empty());
    Ok(())
}

#[test]
fn parser_preserves_legacy_duplicate_merge_section_behavior() -> TestResult {
    let xml = br#"<worksheet><mergeCells><mergeCell ref="invalid"/></mergeCells><mergeCells><mergeCell ref="A1:A1"/></mergeCells></worksheet>"#;
    let parsed = parse_worksheet(Cursor::new(xml.as_slice()), true)?;
    assert!(parsed.merged_cells.is_empty());
    let xml = br#"<worksheet><mergeCells/></worksheet>"#;
    let empty = parse_worksheet(Cursor::new(xml.as_slice()), true)?;
    assert!(empty.merged_cells.is_empty());
    Ok(())
}

#[test]
fn parser_rejects_a_merge_cell_without_a_reference() {
    let xml = br#"<worksheet><mergeCells><mergeCell/></mergeCells></worksheet>"#;
    assert!(matches!(
        parse_worksheet(Cursor::new(xml.as_slice()), true),
        Err(SpreadsheetEngineError::Import(_))
    ));
}

#[test]
fn catalog_rejects_invalid_archives_missing_entries_and_missing_relationships() -> TestResult {
    assert!(SpreadsheetFilterCatalog::read(b"not a ZIP", 1, false).is_err());

    let workbook =
        br#"<workbook xmlns:r="urn:r"><sheets><sheet name="Data" r:id="r1"/></sheets></workbook>"#;
    let relationship =
        br#"<Relationships><Relationship Id="r1" Target="worksheets/sheet1.xml"/></Relationships>"#;
    let missing_workbook = filter_catalog_package(None, Some(relationship), None)?;
    assert!(SpreadsheetFilterCatalog::read(&missing_workbook, 1, false).is_err());

    let malformed_workbook = filter_catalog_package(Some(b"<"), Some(relationship), None)?;
    assert!(SpreadsheetFilterCatalog::read(&malformed_workbook, 1, false).is_err());

    let malformed_relationships = filter_catalog_package(Some(workbook), Some(b"<"), None)?;
    assert!(SpreadsheetFilterCatalog::read(&malformed_relationships, 1, false).is_err());

    let missing_sheet = filter_catalog_package(Some(workbook), Some(relationship), None)?;
    assert!(SpreadsheetFilterCatalog::read(&missing_sheet, 1, false).is_err());

    let missing_relationship = filter_catalog_package(
        Some(workbook),
        Some(br#"<Relationships/>"#),
        Some(br#"<worksheet><sheetData/></worksheet>"#),
    )?;
    assert!(SpreadsheetFilterCatalog::read(&missing_relationship, 1, false).is_err());
    Ok(())
}

#[test]
fn catalog_rejects_malformed_worksheet_xml() -> TestResult {
    let bytes = representative_with_worksheet_xml(
        b"<worksheet><sheetData attr='unterminated></worksheet>",
    )?;
    assert!(matches!(
        SpreadsheetFilterCatalog::read(&bytes, SpreadsheetViewerLimits::strict().max_sheets, true),
        Err(SpreadsheetEngineError::Import(_))
    ));
    Ok(())
}
#[test]
fn catalog_rejects_invalid_start_auto_filter_range() -> TestResult {
    let bytes = representative_with_worksheet_xml(
        br#"<worksheet><autoFilter ref="invalid!"></autoFilter></worksheet>"#,
    )?;
    assert!(matches!(
        SpreadsheetFilterCatalog::read(&bytes, SpreadsheetViewerLimits::strict().max_sheets, true),
        Err(SpreadsheetEngineError::Import(message)) if message.contains("invalid AutoFilter range")
    ));
    Ok(())
}
#[test]
fn catalog_rejects_malformed_merge_cell_reference_entity() -> TestResult {
    let bytes = representative_with_worksheet_xml(
        br#"<worksheet><mergeCells><mergeCell ref="A1&bogus;:B1"/></mergeCells></worksheet>"#,
    )?;
    assert!(matches!(
        SpreadsheetFilterCatalog::read(&bytes, SpreadsheetViewerLimits::strict().max_sheets, true),
        Err(SpreadsheetEngineError::Import(_))
    ));
    Ok(())
}

#[test]
fn catalog_rejects_a_merge_cell_reference_without_a_range_end() -> TestResult {
    let bytes = representative_with_worksheet_xml(
        br#"<worksheet><mergeCells><mergeCell ref="A1"/></mergeCells></worksheet>"#,
    )?;
    assert!(matches!(
        SpreadsheetFilterCatalog::read(&bytes, SpreadsheetViewerLimits::strict().max_sheets, true),
        Err(SpreadsheetEngineError::InvalidMergedCell(reference)) if reference == "A1"
    ));
    Ok(())
}
