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
        filtered_out_row_bitmap,
        legacy_filtered_out_rows,
    } = response
    else {
        return None;
    };
    (*response_id == request_id).then(|| SpreadsheetFilterEvent::VisibilityChanged {
        sheet_index: *sheet_index,
        applied_columns: applied_columns.clone(),
        visible_row_count: *visible_row_count,
        filtered_out_rows: visibility_rows(filtered_out_row_bitmap, legacy_filtered_out_rows),
    })
}

fn visibility_rows(bitmap: &[u8], legacy_rows: &[usize]) -> Vec<usize> {
    if bitmap.is_empty() {
        return legacy_rows.to_vec();
    }
    filtered_out_rows_from_bitmap(bitmap)
}

fn filtered_out_rows_from_bitmap(bitmap: &[u8]) -> Vec<usize> {
    bitmap
        .iter()
        .enumerate()
        .flat_map(|(byte_index, byte)| {
            (0..8)
                .filter_map(move |bit| (byte & (1_u8 << bit) != 0).then_some(byte_index * 8 + bit))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{filtered_out_rows_from_bitmap, visibility_rows};

    #[test]
    fn visibility_bitmap_restores_each_filtered_row() {
        assert_eq!(
            vec![0, 3, 5, 15],
            filtered_out_rows_from_bitmap(&[0b0010_1001, 0b1000_0000])
        );
    }

    #[test]
    fn visibility_rows_accepts_the_previous_worker_wire_shape() {
        assert_eq!(vec![3, 5], visibility_rows(&[], &[3, 5]));
    }
}
