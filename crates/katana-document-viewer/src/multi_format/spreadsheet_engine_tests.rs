use super::{
    SpreadsheetEngineError, SpreadsheetEngineSession, SpreadsheetEngineSupport, load_model,
};
use crate::multi_format::SpreadsheetViewerLimits;
use crate::multi_format::spreadsheet_engine_sheet::SpreadsheetSheetBuilder;
use crate::multi_format::spreadsheet_filter_test_support::{
    representative_with_merge_cell_references, representative_with_nested_merge_cells,
};
use crate::multi_format::spreadsheet_filter_xml::SpreadsheetFilterCatalog;
use crate::multi_format::{SpreadsheetCoordinate, SpreadsheetMergedCellArtifact};
use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn engine_support_rejects_invalid_sizes_and_indices() {
    assert_eq!(0.0, SpreadsheetEngineSupport::track_size(f64::NAN));
    assert_eq!(0.0, SpreadsheetEngineSupport::track_size(-1.0));
    assert!(matches!(
        SpreadsheetEngineSupport::check_limit("cells", 2, 1),
        Err(SpreadsheetEngineError::ResourceLimit { .. })
    ));
    assert!(SpreadsheetEngineSupport::zero_based(0).is_err());
    assert!(SpreadsheetEngineSupport::engine_index(usize::MAX).is_err());
}

#[test]
fn engine_support_preserves_external_error_context() {
    let conversion = usize::try_from(-1_i32);
    assert!(conversion.is_err());
    if let Err(error) = conversion {
        assert!(matches!(
            SpreadsheetEngineSupport::model_error(error),
            SpreadsheetEngineError::Model(_)
        ));
    }
    assert!(matches!(
        SpreadsheetEngineSupport::engine_error("engine failed".to_owned()),
        SpreadsheetEngineError::Model(_)
    ));
}

#[test]
fn engine_import_rejects_a_structurally_incomplete_workbook()
-> Result<(), Box<dyn std::error::Error>> {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for (name, bytes) in [
        (
            "xl/workbook.xml",
            br#"<workbook xmlns:r="urn:r"><sheets><sheet name="Data" r:id="r1"/></sheets></workbook>"#.as_slice(),
        ),
        (
            "xl/_rels/workbook.xml.rels",
            br#"<Relationships><Relationship Id="r1" Target="worksheets/sheet1.xml"/></Relationships>"#.as_slice(),
        ),
        (
            "xl/worksheets/sheet1.xml",
            br#"<worksheet><sheetData/></worksheet>"#.as_slice(),
        ),
    ] {
        writer.start_file(name, SimpleFileOptions::default())?;
        writer.write_all(bytes)?;
    }
    let bytes = writer.finish()?.into_inner();
    assert!(matches!(
        SpreadsheetEngineSession::open(bytes, "incomplete.xlsx", SpreadsheetViewerLimits::strict(),),
        Err(SpreadsheetEngineError::Import(_))
    ));
    Ok(())
}

#[test]
fn engine_rejects_a_malformed_merge_reference_in_a_real_xlsx() -> TestResult {
    for reference in ["A1", "A1:", "B2:A1", "XFE1:XFE2", "A1048577:A1048578"] {
        let bytes = representative_with_merge_cell_references(&[reference])?;
        assert!(matches!(
            SpreadsheetEngineSession::open(
                bytes,
                "malformed-merge-reference.xlsx",
                SpreadsheetViewerLimits::strict(),
            ),
            Err(SpreadsheetEngineError::InvalidMergedCell(_))
        ));
    }
    Ok(())
}

#[test]
fn engine_preserves_single_cell_and_overlapping_merge_geometry() -> TestResult {
    let bytes = representative_with_merge_cell_references(&["A1:A1", "C3:D4", "D4:E5"])?;
    let session = SpreadsheetEngineSession::open(
        bytes,
        "single-and-overlapping-merges.xlsx",
        SpreadsheetViewerLimits::strict(),
    )?;

    assert_eq!(
        session.sheets()[0].merged_cells,
        vec![
            SpreadsheetMergedCellArtifact {
                anchor: SpreadsheetCoordinate::new(0, 0),
                row_span: 1,
                column_span: 1,
            },
            SpreadsheetMergedCellArtifact {
                anchor: SpreadsheetCoordinate::new(2, 2),
                row_span: 2,
                column_span: 2,
            },
            SpreadsheetMergedCellArtifact {
                anchor: SpreadsheetCoordinate::new(3, 3),
                row_span: 2,
                column_span: 2,
            },
        ]
    );
    Ok(())
}

#[test]
fn engine_ignores_nested_merge_cells_outside_the_worksheet_merge_section() -> TestResult {
    let bytes = representative_with_nested_merge_cells()?;
    let session = SpreadsheetEngineSession::open(
        bytes,
        "nested-merge-cells.xlsx",
        SpreadsheetViewerLimits::strict(),
    )?;

    assert_eq!(
        session.sheets()[0].merged_cells,
        vec![SpreadsheetMergedCellArtifact {
            anchor: SpreadsheetCoordinate::new(0, 0),
            row_span: 1,
            column_span: 6,
        }]
    );
    Ok(())
}

#[test]
fn engine_rejects_worksheet_metadata_count_mismatch() -> TestResult {
    let bytes = representative_xlsx_bytes()?;
    let limits = SpreadsheetViewerLimits::strict();
    let mut metadata = SpreadsheetFilterCatalog::read(&bytes, limits.max_sheets, true)?;
    metadata.pop();
    assert!(matches!(
        SpreadsheetEngineSession::open_model(
            bytes.clone(),
            "metadata-count-mismatch.xlsx",
            limits,
            vec![None, None],
            metadata,
        ),
        Err(SpreadsheetEngineError::Import(_))
    ));
    Ok(())
}

#[test]
fn engine_rejects_worksheet_metadata_name_mismatch() -> TestResult {
    let bytes = representative_xlsx_bytes()?;
    let limits = SpreadsheetViewerLimits::strict();
    let mut metadata = SpreadsheetFilterCatalog::read(&bytes, limits.max_sheets, true)?;
    metadata[0].name = "unexpected-name".to_owned();
    assert!(matches!(
        SpreadsheetEngineSession::open_model(
            bytes,
            "metadata-name-mismatch.xlsx",
            limits,
            vec![None, None],
            metadata,
        ),
        Err(SpreadsheetEngineError::Import(_))
    ));
    Ok(())
}

#[test]
fn sheet_builder_rejects_misaligned_merge_metadata() -> TestResult {
    let bytes = representative_xlsx_bytes()?;
    let model = load_model(&bytes, "metadata-count-mismatch.xlsx")?;
    assert!(matches!(
        SpreadsheetSheetBuilder::build(
            &model,
            SpreadsheetViewerLimits::strict().max_sheets,
            SpreadsheetViewerLimits::strict().max_logical_cells,
            Vec::new(),
        ),
        Err(SpreadsheetEngineError::Import(_))
    ));
    Ok(())
}

fn representative_xlsx_bytes() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.xlsx");
    Ok(std::fs::read(fixture)?)
}
