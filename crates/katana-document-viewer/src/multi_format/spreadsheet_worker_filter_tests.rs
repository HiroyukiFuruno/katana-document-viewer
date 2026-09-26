use super::super::filter_bitmap::filtered_out_row_bitmap;
use super::truncate_candidate_values;
use crate::multi_format::spreadsheet_worker_protocol::{
    MAX_SPREADSHEET_REQUEST_BYTES, MAX_SPREADSHEET_RESPONSE_BYTES, SpreadsheetWorkerRequest,
    SpreadsheetWorkerResponse,
};

#[test]
fn candidate_truncation_stays_below_the_response_byte_limit()
-> Result<(), Box<dyn std::error::Error>> {
    let first = "first".repeat(32);
    let second = "second".repeat(32);
    let max_bytes = candidate_response_bytes(vec![first.clone()], false)?;
    let (values, truncated) =
        truncate_candidate_values(1, 0, 0, vec![first.clone(), second], false, max_bytes);

    assert_eq!(vec![first], values);
    assert!(truncated);
    assert!(candidate_response_bytes(values, truncated)? <= max_bytes);
    Ok(())
}

#[test]
fn candidate_truncation_counts_escaped_and_multibyte_json_values()
-> Result<(), Box<dyn std::error::Error>> {
    let first = "plain".to_owned();
    let second = "quote\"\\\u{0007}\n日本".repeat(16);
    let values = vec![first.clone(), second];
    let max_bytes = candidate_response_bytes(values.clone(), false)?;
    let (accepted, truncated) =
        truncate_candidate_values(1, 0, 0, values.clone(), false, max_bytes);

    assert_eq!(values, accepted);
    assert!(!truncated);
    let (accepted, truncated) =
        truncate_candidate_values(1, 0, 0, values, false, max_bytes.saturating_sub(1));
    assert_eq!(vec![first], accepted);
    assert!(truncated);
    assert!(candidate_response_bytes(accepted, truncated)? <= max_bytes.saturating_sub(1));
    Ok(())
}

#[test]
fn candidate_truncation_keeps_every_returned_value_applicable()
-> Result<(), Box<dyn std::error::Error>> {
    let values = (0..2_000)
        .map(|index| format!("candidate-{index}-{}", "x".repeat(300)))
        .collect::<Vec<_>>();

    let (accepted, truncated) = truncate_candidate_values(7, 3, 5, values, false, 16 * 1024 * 1024);

    assert!(truncated);
    assert!(!accepted.is_empty());
    let mut encoded = serde_json::to_vec(&SpreadsheetWorkerRequest::ApplyFilter {
        request_id: u64::MAX,
        sheet_index: 3,
        column: 5,
        values: accepted,
    })?;
    encoded.push(b'\n');
    assert!(encoded.len() <= MAX_SPREADSHEET_REQUEST_BYTES);
    Ok(())
}

#[test]
fn candidate_truncation_drops_a_value_that_cannot_fit_apply_values() {
    let oversized = "x".repeat(MAX_SPREADSHEET_REQUEST_BYTES);
    let (accepted, truncated) =
        truncate_candidate_values(7, 3, 5, vec![oversized], false, 16 * 1024 * 1024);

    assert!(accepted.is_empty());
    assert!(truncated);
}

#[test]
fn visibility_bitmap_stays_within_the_response_limit_at_the_strict_cell_limit()
-> Result<(), Box<dyn std::error::Error>> {
    let filtered_out_row_bitmap = filtered_out_row_bitmap(0..25_000_000);
    let response = SpreadsheetWorkerResponse::FilterVisibility {
        request_id: u64::MAX,
        sheet_index: usize::MAX,
        applied_columns: vec![usize::MAX],
        visible_row_count: 0,
        filtered_out_row_bitmap,
        legacy_filtered_out_rows: Vec::new(),
    };

    let encoded = serde_json::to_vec(&response)?;
    assert!(encoded.len() <= MAX_SPREADSHEET_RESPONSE_BYTES);
    Ok(())
}

fn candidate_response_bytes(
    values: Vec<String>,
    truncated: bool,
) -> Result<usize, serde_json::Error> {
    serde_json::to_vec(&SpreadsheetWorkerResponse::FilterCandidates {
        request_id: 1,
        sheet_index: 0,
        column: 0,
        values,
        truncated,
    })
    .map(|bytes| bytes.len())
}
