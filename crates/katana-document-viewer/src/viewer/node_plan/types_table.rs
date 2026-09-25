use crate::viewer::types::ViewerInput;
use katana_markdown_model::{TableAlignment, TableNode, TableRow};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerTableAlignment {
    Left,
    Center,
    Right,
    Unspecified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewerTableVerticalAlignment {
    Top,
    Center,
    Bottom,
    Unspecified,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerTableCellProjection {
    pub text: String,
    pub alignment: ViewerTableAlignment,
    pub vertical_alignment: ViewerTableVerticalAlignment,
    pub row_span: usize,
    pub column_span: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerTableRowProjection {
    pub cells: Vec<ViewerTableCellProjection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerTableProjection {
    pub rows: Vec<ViewerTableRowProjection>,
    pub column_count: usize,
}

impl ViewerTableProjection {
    #[must_use]
    pub fn from_input(input: &ViewerInput) -> BTreeMap<String, Self> {
        fn collect(
            nodes: &[katana_markdown_model::KmmNode],
            projections: &mut BTreeMap<String, ViewerTableProjection>,
        ) {
            for node in nodes {
                if let katana_markdown_model::KmmNodeKind::Table(table) = &node.kind {
                    projections.insert(node.id.0.clone(), ViewerTableProjection::from_kmm(table));
                }
                collect(&node.children, projections);
            }
        }
        let mut projections = BTreeMap::new();
        collect(&input.snapshot.document.nodes, &mut projections);
        projections
    }

    #[must_use]
    pub fn from_kmm(table: &TableNode) -> Self {
        let rows = table
            .rows
            .iter()
            .filter(|row| !is_separator_row(row))
            .map(|row| ViewerTableRowProjection {
                cells: row
                    .cells
                    .iter()
                    .enumerate()
                    .map(|(column, cell)| ViewerTableCellProjection {
                        text: cell.text.clone(),
                        alignment: table
                            .alignments
                            .get(column)
                            .map(table_alignment)
                            .unwrap_or(ViewerTableAlignment::Unspecified),
                        vertical_alignment: ViewerTableVerticalAlignment::Center,
                        row_span: 1,
                        column_span: 1,
                    })
                    .collect(),
            })
            .collect::<Vec<_>>();
        let column_count = rows.iter().map(|row| row.cells.len()).max().unwrap_or(0);
        Self { rows, column_count }
    }
}

const fn table_alignment(value: &TableAlignment) -> ViewerTableAlignment {
    match value {
        TableAlignment::Left => ViewerTableAlignment::Left,
        TableAlignment::Center => ViewerTableAlignment::Center,
        TableAlignment::Right => ViewerTableAlignment::Right,
        TableAlignment::Unspecified => ViewerTableAlignment::Unspecified,
    }
}

fn is_separator_row(row: &TableRow) -> bool {
    !row.cells.is_empty()
        && row.cells.iter().all(|cell| {
            let value = cell.text.trim();
            !value.is_empty()
                && value
                    .chars()
                    .all(|character| matches!(character, '-' | ':'))
        })
}

#[path = "types_table_geometry.rs"]
mod geometry;

#[cfg(test)]
#[path = "types_table_tests.rs"]
mod tests;
