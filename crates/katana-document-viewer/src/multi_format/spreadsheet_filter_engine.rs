use super::spreadsheet_engine::{SpreadsheetEngineError, SpreadsheetEngineSession};
use super::{SpreadsheetAutoFilterArtifact, SpreadsheetCellArtifact, SpreadsheetSheetArtifact};
use std::collections::{BTreeMap, BTreeSet};

const MAX_FILTER_VALUES: usize = 4_096;

#[path = "spreadsheet_filter_engine_candidates.rs"]
mod candidate_values;
#[path = "spreadsheet_filter_engine_persisted.rs"]
mod persisted;

pub(super) use candidate_values::candidates;

pub(super) type SpreadsheetActiveFilters = Vec<BTreeMap<usize, BTreeSet<String>>>;

pub(super) fn persisted_filters(
    filters: &[Option<SpreadsheetAutoFilterArtifact>],
) -> SpreadsheetActiveFilters {
    persisted::SpreadsheetPersistedFilterEngine::persisted_filters(filters)
}

pub(super) struct SpreadsheetFilterResult {
    pub(super) applied_columns: Vec<usize>,
    pub(super) visible_row_count: usize,
    pub(super) filtered_out_rows: Vec<usize>,
}

pub(super) fn apply(
    engine: &SpreadsheetEngineSession,
    active: &mut SpreadsheetActiveFilters,
    sheet_index: usize,
    column: usize,
    values: Vec<String>,
) -> Result<SpreadsheetFilterResult, SpreadsheetEngineError> {
    filter_sheet(engine, sheet_index, column)?;
    if values.len() > MAX_FILTER_VALUES {
        return Err(SpreadsheetEngineError::FilterValueLimit {
            actual: values.len(),
            limit: MAX_FILTER_VALUES,
        });
    }
    active_sheet(active, sheet_index)?.insert(column, values.into_iter().collect());
    evaluate(engine, active, sheet_index)
}

pub(super) fn clear(
    engine: &SpreadsheetEngineSession,
    active: &mut SpreadsheetActiveFilters,
    sheet_index: usize,
    column: Option<usize>,
) -> Result<SpreadsheetFilterResult, SpreadsheetEngineError> {
    engine.sheet(sheet_index)?;
    if engine.auto_filter(sheet_index)?.is_none() {
        return Err(SpreadsheetEngineError::FilterUnavailable { sheet_index });
    }
    let filters = active_sheet(active, sheet_index)?;
    if let Some(column) = column {
        filter_sheet(engine, sheet_index, column)?;
        filters.remove(&column);
    } else {
        filters.clear();
    }
    evaluate(engine, active, sheet_index)
}

pub(super) fn evaluate(
    engine: &SpreadsheetEngineSession,
    active: &SpreadsheetActiveFilters,
    sheet_index: usize,
) -> Result<SpreadsheetFilterResult, SpreadsheetEngineError> {
    let sheet = engine.sheet(sheet_index)?;
    let filters = active
        .get(sheet_index)
        .ok_or(SpreadsheetEngineError::SheetOutsideDocument {
            requested: sheet_index,
            sheet_count: active.len(),
        })?;
    let mut filtered_out_rows = Vec::new();
    if !filters.is_empty() {
        filtered_out_rows = filtered_rows(engine, sheet_index, sheet, filters)?;
    }
    Ok(SpreadsheetFilterResult {
        applied_columns: filters.keys().copied().collect(),
        visible_row_count: visible_row_count(sheet, &filtered_out_rows),
        filtered_out_rows,
    })
}

fn filtered_rows(
    engine: &SpreadsheetEngineSession,
    sheet_index: usize,
    sheet: &SpreadsheetSheetArtifact,
    filters: &BTreeMap<usize, BTreeSet<String>>,
) -> Result<Vec<usize>, SpreadsheetEngineError> {
    let filter = engine
        .auto_filter(sheet_index)?
        .ok_or(SpreadsheetEngineError::FilterUnavailable { sheet_index })?;
    let rows = filter_rows(sheet, filter);
    let columns = filters.keys().copied().collect::<Vec<_>>();
    let mut filtered_out_rows = Vec::new();
    let mut collect_rejected_rows =
        |chunk_rows: std::ops::Range<usize>, cells: Vec<SpreadsheetCellArtifact>| {
            filtered_out_rows.extend(rejected_rows(chunk_rows, &columns, filters, cells));
            Ok(())
        };
    engine.visit_filter_grid(sheet_index, &columns, rows, &mut collect_rejected_rows)?;
    Ok(filtered_out_rows)
}

pub(super) fn rejected_rows(
    rows: std::ops::Range<usize>,
    columns: &[usize],
    filters: &BTreeMap<usize, BTreeSet<String>>,
    cells: Vec<SpreadsheetCellArtifact>,
) -> Vec<usize> {
    let mut cell_values = cells.into_iter().map(|cell| cell.display_text);
    rows.filter(|_| {
        let mut rejected = false;
        for column in columns {
            let Some(value) = cell_values.next() else {
                return true;
            };
            rejected |= filters
                .get(column)
                .is_some_and(|selected| !selected.contains(&value));
        }
        rejected
    })
    .collect()
}

fn filter_sheet(
    engine: &SpreadsheetEngineSession,
    sheet_index: usize,
    column: usize,
) -> Result<(&SpreadsheetSheetArtifact, &SpreadsheetAutoFilterArtifact), SpreadsheetEngineError> {
    let sheet = engine.sheet(sheet_index)?;
    let Some(filter) = engine.auto_filter(sheet_index)? else {
        return Err(SpreadsheetEngineError::FilterUnavailable { sheet_index });
    };
    if column < filter.range.start.column || column > filter.range.end.column {
        return Err(SpreadsheetEngineError::FilterColumnOutsideRange {
            sheet_index,
            column,
        });
    }
    Ok((sheet, filter))
}

pub(super) fn filter_rows(
    sheet: &SpreadsheetSheetArtifact,
    filter: &SpreadsheetAutoFilterArtifact,
) -> std::ops::Range<usize> {
    let start = filter
        .range
        .start
        .row
        .saturating_add(1)
        .min(sheet.row_count);
    let end = filter.range.end.row.saturating_add(1).min(sheet.row_count);
    start..end
}

fn active_sheet(
    active: &mut SpreadsheetActiveFilters,
    sheet_index: usize,
) -> Result<&mut BTreeMap<usize, BTreeSet<String>>, SpreadsheetEngineError> {
    let sheet_count = active.len();
    active
        .get_mut(sheet_index)
        .ok_or(SpreadsheetEngineError::SheetOutsideDocument {
            requested: sheet_index,
            sheet_count,
        })
}

fn visible_row_count(sheet: &SpreadsheetSheetArtifact, filtered: &[usize]) -> usize {
    let hidden = sheet
        .row_tracks
        .iter()
        .enumerate()
        .filter(|(row, track)| track.hidden || filtered.binary_search(row).is_ok())
        .count();
    sheet.row_count.saturating_sub(hidden)
}
