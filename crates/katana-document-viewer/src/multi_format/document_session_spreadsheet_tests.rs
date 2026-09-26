use super::*;
use crate::{SpreadsheetCoordinate, SpreadsheetMergedCellArtifact, ViewerSourceIdentity};
use std::path::{Path, PathBuf};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn corrupted_sheet_index_fails_closed_before_surface_replacement() -> TestResult {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.xlsx");
    let source = OfficeDocumentSource::new(
        ViewerSourceIdentity::new("file:///unit.xlsx", "sha256:unit-xlsx"),
        super::super::OfficeDocumentFormat::Xlsx,
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        std::fs::read(fixture)?,
    );
    let mut session = SpreadsheetDocumentSession::open(
        source,
        OfficeWorkerConfig::new(worker_binary_path()?),
        DocumentViewport::new(640, 480),
    )?;
    session.state.active_index = usize::MAX;

    assert!(matches!(
        session.replace_surface(),
        Err(DocumentSessionError::State(
            super::super::DocumentViewerStateError::IndexOutsideDocument { .. }
        ))
    ));
    Ok(())
}

#[test]
fn spreadsheet_frame_exposes_worksheet_names_for_host_tabs() -> TestResult {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.xlsx");
    let source = OfficeDocumentSource::new(
        ViewerSourceIdentity::new("file:///unit.xlsx", "sha256:unit-xlsx"),
        super::super::OfficeDocumentFormat::Xlsx,
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        std::fs::read(fixture)?,
    );
    let mut session = SpreadsheetDocumentSession::open(
        source,
        OfficeWorkerConfig::new(worker_binary_path()?),
        DocumentViewport::new(640, 480),
    )?;

    let frame = session.frame()?;
    assert_eq!(frame.state.item_count, frame.surface.item_labels().len());
    assert!(
        frame
            .surface
            .item_labels()
            .iter()
            .all(|name| !name.trim().is_empty())
    );
    Ok(())
}

#[test]
fn malformed_worker_sheet_geometry_fails_at_the_surface_boundary() -> TestResult {
    let mut session = open_session()?;
    session.engine.artifact_mut().sheets[0].merged_cells = vec![SpreadsheetMergedCellArtifact {
        anchor: SpreadsheetCoordinate::new(usize::MAX, usize::MAX),
        row_span: 1,
        column_span: 1,
    }];

    assert!(matches!(
        session.replace_surface(),
        Err(DocumentSessionError::Surface(_))
    ));
    Ok(())
}

fn open_session() -> Result<SpreadsheetDocumentSession, Box<dyn std::error::Error>> {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/multi-format/representative.xlsx");
    let source = OfficeDocumentSource::new(
        ViewerSourceIdentity::new("file:///unit.xlsx", "sha256:unit-xlsx"),
        super::super::OfficeDocumentFormat::Xlsx,
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        std::fs::read(fixture)?,
    );
    Ok(SpreadsheetDocumentSession::open(
        source,
        OfficeWorkerConfig::new(worker_binary_path()?),
        DocumentViewport::new(640, 480),
    )?)
}

fn worker_binary_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_exe = std::env::current_exe()?;
    let deps = current_exe
        .parent()
        .ok_or("unit test binary has no parent directory")?;
    let worker = deps
        .parent()
        .ok_or("unit test binary has no target directory")?
        .join("kdv-office-worker");
    #[cfg(windows)]
    let worker = worker.with_extension("exe");
    Ok(worker)
}
