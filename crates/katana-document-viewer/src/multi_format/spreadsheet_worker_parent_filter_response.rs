use super::SpreadsheetFilterEvent;
use crate::OfficeWorkerError;
use crate::multi_format::spreadsheet_worker_parent::SpreadsheetWorkerParentResponse;
use crate::multi_format::spreadsheet_worker_protocol::SpreadsheetWorkerResponse;

pub(super) fn filter_success_event(
    request_id: u64,
    response: SpreadsheetWorkerResponse,
) -> Result<SpreadsheetFilterEvent, OfficeWorkerError> {
    if let Some(event) = candidate_response(request_id, &response) {
        return Ok(event);
    }
    if let Some(event) = visibility_response(request_id, &response) {
        return Ok(event);
    }
    Err(SpreadsheetWorkerParentResponse::unexpected_response(
        "filter", response,
    ))
}

fn candidate_response(
    request_id: u64,
    response: &SpreadsheetWorkerResponse,
) -> Option<SpreadsheetFilterEvent> {
    let SpreadsheetWorkerResponse::FilterCandidates {
        request_id: response_id,
        sheet_index,
        column,
        values,
        truncated,
    } = response
    else {
        return None;
    };
    (*response_id == request_id).then(|| SpreadsheetFilterEvent::Candidates {
        sheet_index: *sheet_index,
        column: *column,
        values: values.clone(),
        truncated: *truncated,
    })
}

fn visibility_response(
    request_id: u64,
    response: &SpreadsheetWorkerResponse,
) -> Option<SpreadsheetFilterEvent> {
    let SpreadsheetWorkerResponse::FilterVisibility {
        request_id: response_id,
        sheet_index,
        applied_columns,
        visible_row_count,
        filtered_out_rows,
    } = response
    else {
        return None;
    };
    (*response_id == request_id).then(|| SpreadsheetFilterEvent::VisibilityChanged {
        sheet_index: *sheet_index,
        applied_columns: applied_columns.clone(),
        visible_row_count: *visible_row_count,
        filtered_out_rows: filtered_out_rows.clone(),
    })
}
