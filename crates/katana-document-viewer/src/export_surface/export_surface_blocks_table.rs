use super::{TABLE_CELL_PADDING, TABLE_LINE_HEIGHT, TABLE_ROW_HEIGHT, TABLE_ROW_VERTICAL_PADDING};
use crate::export_surface_helpers::{SURFACE_CONTENT_WIDTH, WrappedText};
use crate::export_surface_text::SurfaceTextParser;
use katana_markdown_model::{TableAlignment, TableNode, TableRow};

const ASCII_CELL_CHAR_WIDTH: u32 = 12;
const WIDE_CELL_CHAR_WIDTH: u32 = 22;
const TABLE_MIN_CELL_CHARS: usize = 8;

pub(crate) struct SurfaceTableBlock {
    rows: Vec<Vec<String>>,
    alignments: Vec<TableAlignment>,
}

impl SurfaceTableBlock {
    pub(crate) fn new(table: &TableNode) -> Self {
        Self {
            rows: table
                .rows
                .iter()
                .filter(|row| !SurfaceTableLayout::is_separator_row(row))
                .map(Self::row_texts)
                .collect(),
            alignments: table.alignments.clone(),
        }
    }

    fn row_texts(row: &TableRow) -> Vec<String> {
        row.cells
            .iter()
            .map(|cell| SurfaceTextParser::inline_markdown_text(&cell.text))
            .collect()
    }

    pub(crate) fn height(&self) -> u32 {
        let column_width = SURFACE_CONTENT_WIDTH / self.column_count().max(1) as u32;
        self.rows
            .iter()
            .enumerate()
            .map(|(index, _)| self.row_height(index, column_width))
            .sum()
    }

    pub(crate) fn column_count(&self) -> usize {
        self.rows.iter().map(Vec::len).max().unwrap_or(1)
    }

    pub(crate) fn alignment(&self, index: usize) -> TableAlignment {
        self.alignments
            .get(index)
            .cloned()
            .unwrap_or(TableAlignment::Unspecified)
    }

    pub(crate) fn text(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.join("  "))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub(crate) fn row_height(&self, row_index: usize, column_width: u32) -> u32 {
        let line_count = self
            .rows
            .get(row_index)
            .map(|row| Self::row_line_count(row, column_width))
            .unwrap_or(1);
        let dynamic_height = line_count as u32 * TABLE_LINE_HEIGHT + TABLE_ROW_VERTICAL_PADDING * 2;
        dynamic_height.max(TABLE_ROW_HEIGHT)
    }

    fn row_line_count(row: &[String], column_width: u32) -> usize {
        row.iter()
            .map(|cell| WrappedText::new(cell, SurfaceTableLayout::cell_max_chars(column_width)))
            .map(Iterator::count)
            .max()
            .unwrap_or(1)
    }

    pub(crate) fn rows(&self) -> &Vec<Vec<String>> {
        &self.rows
    }
}

pub(crate) struct SurfaceTableLayout;

impl SurfaceTableLayout {
    pub(crate) fn is_separator_row(row: &TableRow) -> bool {
        row.cells.iter().all(|cell| {
            let trimmed = cell.text.trim();
            !trimmed.is_empty()
                && trimmed
                    .chars()
                    .all(|character| matches!(character, '-' | ':'))
        })
    }

    pub(crate) fn has_contract(table: &TableNode) -> bool {
        table.rows.get(1).is_some_and(Self::is_separator_row) && table.rows.len() >= 2
    }

    pub(crate) fn cell_text_x(cell: &str, alignment: &TableAlignment, x: u32, width: u32) -> u32 {
        let content_width = width.saturating_sub(TABLE_CELL_PADDING * 2);
        let text_width = Self::estimated_cell_text_width(cell).min(content_width);
        let left = x + TABLE_CELL_PADDING;
        match alignment {
            TableAlignment::Center => left + content_width.saturating_sub(text_width) / 2,
            TableAlignment::Right => left + content_width.saturating_sub(text_width),
            TableAlignment::Left | TableAlignment::Unspecified => left,
        }
    }

    pub(crate) fn cell_text_y(row_height: u32, line_count: usize) -> u32 {
        let content_height = line_count.max(1) as u32 * TABLE_LINE_HEIGHT;
        row_height.saturating_sub(content_height) / 2
    }

    pub(crate) fn estimated_cell_text_width(cell: &str) -> u32 {
        cell.chars()
            .map(|character| {
                if character.is_ascii() {
                    ASCII_CELL_CHAR_WIDTH
                } else {
                    WIDE_CELL_CHAR_WIDTH
                }
            })
            .sum()
    }

    pub(crate) fn cell_max_chars(width: u32) -> usize {
        (width.saturating_sub(TABLE_CELL_PADDING * 2) / WIDE_CELL_CHAR_WIDTH)
            .max(TABLE_MIN_CELL_CHARS as u32)
            .try_into()
            .unwrap_or(TABLE_MIN_CELL_CHARS)
    }

    pub(crate) fn row_fill(
        row_index: usize,
        palette: &crate::export_surface::SurfacePaintPalette,
    ) -> Option<image::Rgba<u8>> {
        if row_index == 0 {
            return Some(palette.table_header);
        }
        if row_index.is_multiple_of(2) {
            return Some(palette.table_even);
        }
        None
    }
}

#[derive(Clone)]
pub(crate) struct SurfaceTableCellPaint<'a> {
    pub(crate) cell: &'a str,
    pub(crate) alignment: TableAlignment,
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) width: u32,
    pub(crate) row_height: u32,
}
