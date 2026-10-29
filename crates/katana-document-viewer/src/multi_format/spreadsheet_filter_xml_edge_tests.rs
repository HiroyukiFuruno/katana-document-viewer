use super::SpreadsheetFilterCriterion;
use super::spreadsheet_engine::SpreadsheetEngineError;
use super::spreadsheet_filter_test_support::{
    filter_catalog_package, representative_with_worksheet_xml,
};
use super::spreadsheet_filter_xml::SpreadsheetFilterCatalog;
use super::spreadsheet_filter_xml_parser::parse_worksheet;
use std::io::Cursor;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn parser_rejects_invalid_merge_after_collecting_auto_filter() {
    let xml = br#"<worksheet><autoFilter ref="A1:C4"><filterColumn colId="0"><filters><filter val="Open"/></filters></filterColumn></autoFilter><mergeCells><mergeCell ref="A1"/></mergeCells></worksheet>"#;
    assert!(matches!(
        parse_worksheet(Cursor::new(xml), true),
        Err(SpreadsheetEngineError::InvalidMergedCell(reference)) if reference == "A1"
    ));
}

#[test]
fn parser_ignores_unsupported_filter_without_auto_filter_context() -> TestResult {
    let parsed = parse_worksheet(
        Cursor::new(b"<worksheet><customFilters/></worksheet>"),
        false,
    )?;
    assert!(parsed.auto_filter.is_none());
    assert!(parsed.merged_cells.is_empty());
    Ok(())
}

#[test]
fn parser_records_unsupported_filter_without_a_column_context() -> TestResult {
    let parsed = parse_worksheet(
        Cursor::new(
            b"<worksheet><autoFilter ref=\"A1:C4\"><customFilters/></autoFilter></worksheet>",
        ),
        false,
    )?;
    let filter = parsed.auto_filter.ok_or("AutoFilter was not parsed")?;
    assert!(filter.columns.is_empty());
    assert_eq!(
        vec!["unsupported AutoFilter criterion `customFilters`"],
        filter.diagnostics
    );
    Ok(())
}

#[test]
fn catalog_rejects_a_missing_relationships_entry() -> TestResult {
    let workbook =
        br#"<workbook xmlns:r="urn:r"><sheets><sheet name="Data" r:id="r1"/></sheets></workbook>"#;
    let bytes = filter_catalog_package(Some(workbook), None, None)?;
    assert!(matches!(
        SpreadsheetFilterCatalog::read(&bytes, 1, false),
        Err(SpreadsheetEngineError::Import(_))
    ));
    Ok(())
}

#[test]
fn catalog_collects_empty_filter_columns_from_a_real_xlsx() -> TestResult {
    let worksheet =
        br#"<worksheet><autoFilter ref="A1:C4"><filterColumn colId="0"/></autoFilter></worksheet>"#;
    let bytes = representative_with_worksheet_xml(worksheet)?;
    let sheets = SpreadsheetFilterCatalog::read(
        &bytes,
        crate::multi_format::SpreadsheetViewerLimits::strict().max_sheets,
        true,
    )?;
    let filter = sheets[0]
        .auto_filter
        .as_ref()
        .ok_or("AutoFilter was not parsed")?;
    assert_eq!(1, filter.columns.len());
    assert!(filter.columns[0].criteria.is_empty());
    Ok(())
}

#[test]
fn parser_rejects_invalid_and_overflowing_filter_column_ids() {
    for col_id in ["invalid", "999999999999999999999999999999"] {
        let xml = format!(
            r#"<worksheet><autoFilter ref="A1:C4"><filterColumn colId="{col_id}"/></autoFilter></worksheet>"#
        );
        assert!(matches!(
            parse_worksheet(Cursor::new(xml.as_bytes()), false),
            Err(SpreadsheetEngineError::Import(_))
        ));
    }
}

#[test]
fn parser_rejects_malformed_blank_and_value_attribute_entities() {
    for xml in [
        br#"<worksheet><autoFilter ref="A1:C4"><filterColumn colId="0"><filters blank="&bogus;"/></filterColumn></autoFilter></worksheet>"#.as_slice(),
        br#"<worksheet><autoFilter ref="A1:C4"><filterColumn colId="0"><filter val="&bogus;"/></filterColumn></autoFilter></worksheet>"#.as_slice(),
    ] {
        assert!(matches!(
            parse_worksheet(Cursor::new(xml), false),
            Err(SpreadsheetEngineError::Import(_))
        ));
    }
}

#[test]
fn parser_records_true_blank_and_ignores_it_without_a_column() -> TestResult {
    let xml = br#"<worksheet><autoFilter ref="A1:C4"><filterColumn colId="0"><filters blank="true"/></filterColumn></autoFilter></worksheet>"#;
    let parsed = parse_worksheet(Cursor::new(xml.as_slice()), false)?;
    let filter = parsed.auto_filter.ok_or("AutoFilter was not parsed")?;
    assert!(matches!(
        filter.columns[0].criteria.as_slice(),
        [SpreadsheetFilterCriterion::Blank]
    ));

    let parsed = parse_worksheet(
        Cursor::new(br#"<worksheet><filters blank="true"/></worksheet>"#.as_slice()),
        false,
    )?;
    assert!(parsed.auto_filter.is_none());
    Ok(())
}

#[test]
fn parser_ignores_filter_elements_without_value_attributes() -> TestResult {
    let xml = br#"<worksheet><autoFilter ref="A1:C4"><filterColumn colId="0"><filter/></filterColumn></autoFilter></worksheet>"#;
    let parsed = parse_worksheet(Cursor::new(xml.as_slice()), false)?;
    let filter = parsed.auto_filter.ok_or("AutoFilter was not parsed")?;
    assert!(filter.columns[0].criteria.is_empty());
    Ok(())
}
