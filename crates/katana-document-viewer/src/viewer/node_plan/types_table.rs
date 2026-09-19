use super::{ViewerNode, ViewerNodeKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewerTableAlignment {
    Left,
    Center,
    Right,
    Unspecified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewerTableVerticalAlignment {
    Top,
    Center,
    Bottom,
    Unspecified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewerTableCellProjection {
    pub text: String,
    pub alignment: ViewerTableAlignment,
    pub vertical_alignment: ViewerTableVerticalAlignment,
    pub row_span: usize,
    pub column_span: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewerTableRowProjection {
    pub cells: Vec<ViewerTableCellProjection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewerTableProjection {
    pub rows: Vec<ViewerTableRowProjection>,
    pub column_count: usize,
}

impl ViewerNode {
    #[must_use]
    pub fn table_projection(&self) -> Option<ViewerTableProjection> {
        if !matches!(self.kind, ViewerNodeKind::Table) {
            return None;
        }
        let alignments = table_alignments(&self.source.raw.text);
        let mut projection = ViewerTableProjection::from_text(&self.text)?;
        for row in &mut projection.rows {
            for (column, cell) in row.cells.iter_mut().enumerate() {
                cell.alignment = alignments
                    .get(column)
                    .copied()
                    .unwrap_or(ViewerTableAlignment::Unspecified);
            }
        }
        Some(projection)
    }
}

impl ViewerTableProjection {
    pub(crate) fn from_text(text: &str) -> Option<Self> {
        let rows = text
            .lines()
            .map(table_cells)
            .filter(|cells| !cells.is_empty() && !is_separator_row(cells))
            .map(|cells| ViewerTableRowProjection {
                cells: cells
                    .into_iter()
                    .map(|text| ViewerTableCellProjection {
                        text,
                        alignment: ViewerTableAlignment::Unspecified,
                        vertical_alignment: ViewerTableVerticalAlignment::Center,
                        row_span: 1,
                        column_span: 1,
                    })
                    .collect(),
            })
            .collect::<Vec<_>>();
        let column_count = rows.iter().map(|row| row.cells.len()).max().unwrap_or(0);
        (!rows.is_empty() && column_count > 0).then_some(Self { rows, column_count })
    }
}

fn table_alignments(raw: &str) -> Vec<ViewerTableAlignment> {
    let separator = raw
        .lines()
        .map(table_cells)
        .find(|cells| is_separator_row(cells));
    separator
        .into_iter()
        .flatten()
        .map(|cell| {
            let value = cell.trim();
            match (value.starts_with(':'), value.ends_with(':')) {
                (true, true) => ViewerTableAlignment::Center,
                (false, true) => ViewerTableAlignment::Right,
                (true, false) => ViewerTableAlignment::Left,
                (false, false) => ViewerTableAlignment::Unspecified,
            }
        })
        .collect()
}

fn table_cells(line: &str) -> Vec<String> {
    let mut cells = line.split('|').map(str::trim).collect::<Vec<_>>();
    if cells.first().is_some_and(|cell| cell.is_empty()) {
        cells.remove(0);
    }
    if cells.last().is_some_and(|cell| cell.is_empty()) {
        cells.pop();
    }
    cells.into_iter().map(ToOwned::to_owned).collect()
}

fn is_separator_row(cells: &[String]) -> bool {
    !cells.is_empty()
        && cells.iter().all(|cell| {
            !cell.is_empty() && cell.chars().all(|character| matches!(character, '-' | ':'))
        })
}

#[path = "types_table_geometry.rs"]
mod geometry;

#[cfg(test)]
#[path = "types_table_tests.rs"]
mod tests;
