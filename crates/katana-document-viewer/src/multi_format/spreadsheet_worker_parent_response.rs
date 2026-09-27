use super::{
    OfficeDocumentSource, OfficeWorkerError, SpreadsheetAutoFilterArtifact,
    SpreadsheetDocumentArtifact, SpreadsheetMaterializedCell, SpreadsheetOpenedSheet,
    SpreadsheetSheetArtifact, SpreadsheetWorkerResponse, ViewerQualityProfile,
};

pub(super) fn spreadsheet_artifact(
    source: OfficeDocumentSource,
    sheets: Vec<SpreadsheetSheetArtifact>,
    preflight_diagnostics: Vec<super::super::ViewerDiagnostic>,
) -> SpreadsheetDocumentArtifact {
    let profile = ViewerQualityProfile::interactive_grid();
    let mut diagnostics = profile.diagnostics();
    diagnostics.extend(preflight_diagnostics);
    SpreadsheetDocumentArtifact {
        identity: source.identity,
        mime: source.mime,
        sheet_count: sheets.len(),
        sheets,
        capabilities: profile.capabilities,
        diagnostics,
    }
}

pub(super) fn opened_sheets(
    response: SpreadsheetWorkerResponse,
) -> Result<Vec<SpreadsheetOpenedSheet>, OfficeWorkerError> {
    match response {
        SpreadsheetWorkerResponse::Opened { sheets } => Ok(sheets),
        response => Err(SpreadsheetWorkerParentResponse::unexpected_response(
            "open", response,
        )),
    }
}

pub(super) fn split_opened_sheets(
    opened: Vec<SpreadsheetOpenedSheet>,
) -> (
    Vec<SpreadsheetSheetArtifact>,
    Vec<Option<SpreadsheetAutoFilterArtifact>>,
) {
    opened
        .into_iter()
        .map(|opened| (opened.sheet, opened.auto_filter))
        .unzip()
}

pub(super) fn materialized_cells(
    request_id: u64,
    response: SpreadsheetWorkerResponse,
) -> Result<Vec<SpreadsheetMaterializedCell>, OfficeWorkerError> {
    match response {
        SpreadsheetWorkerResponse::Materialized {
            request_id: response_id,
            cells,
        } if response_id == request_id => Ok(cells),
        SpreadsheetWorkerResponse::Failed {
            request_id: Some(response_id),
            stage,
            message,
        } if response_id == request_id => Err(OfficeWorkerError::EngineFailure { stage, message }),
        response => Err(SpreadsheetWorkerParentResponse::unexpected_response(
            "materialize",
            response,
        )),
    }
}

pub(crate) struct SpreadsheetWorkerParentResponse;

impl SpreadsheetWorkerParentResponse {
    pub(crate) fn unexpected_response(
        operation: &str,
        response: SpreadsheetWorkerResponse,
    ) -> OfficeWorkerError {
        OfficeWorkerError::protocol(format!(
            "unexpected spreadsheet response during {operation}: {response:?}"
        ))
    }
}
