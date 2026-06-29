use super::{TABLE_CELL_PADDING, TABLE_LINE_HEIGHT, TABLE_ROW_HEIGHT, TABLE_ROW_VERTICAL_PADDING};
use crate::export_surface_helpers::SURFACE_CONTENT_WIDTH;
use crate::export_surface_line::SurfaceTypographyConfig;
use crate::export_surface_span::{SurfaceInlineSpans, SurfaceTextSpan};
use crate::export_surface_text::SurfaceTextParser;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{TableAlignment, TableNode, TableRow};

const ASCII_CELL_CHAR_WIDTH: u32 = 12;
const WIDE_CELL_CHAR_WIDTH: u32 = 22;
const TABLE_MIN_CELL_CHARS: usize = 8;
pub(crate) struct SurfaceTableBlock {
    rows: Vec<Vec<String>>,
    cell_spans: Vec<Vec<Vec<SurfaceTextSpan>>>,
    alignments: Vec<TableAlignment>,
    typography: SurfaceTypographyConfig,
}

impl SurfaceTableBlock {
    #[cfg(test)]
    pub(crate) fn new(table: &TableNode) -> Self {
        Self::new_with_theme(table, &KdvThemeSnapshot::katana_light())
    }

    pub(crate) fn new_with_theme(table: &TableNode, theme: &KdvThemeSnapshot) -> Self {
        Self {
            rows: table
                .rows
                .iter()
                .filter(|row| !SurfaceTableLayout::is_separator_row(row))
                .map(Self::row_texts)
                .collect(),
            cell_spans: table
                .rows
                .iter()
                .filter(|row| !SurfaceTableLayout::is_separator_row(row))
                .map(|row| Self::row_spans(row, theme))
                .collect(),
            alignments: table.alignments.clone(),
            typography: SurfaceTypographyConfig::default(),
        }
    }

    fn row_texts(row: &TableRow) -> Vec<String> {
        row.cells
            .iter()
            .map(|cell| SurfaceTextParser::inline_markdown_text(&cell.text))
            .collect()
    }

    fn row_spans(row: &TableRow, theme: &KdvThemeSnapshot) -> Vec<Vec<SurfaceTextSpan>> {
        row.cells
            .iter()
            .map(|cell| SurfaceInlineSpans::from_markdown(&cell.text, theme))
            .collect()
    }

    pub(crate) fn height(&self) -> u32 {
        self.height_for_width(SURFACE_CONTENT_WIDTH)
    }

    pub(crate) fn height_for_width(&self, row_width: u32) -> u32 {
        let column_widths = self.column_widths_for_width(row_width);
        self.rows
            .iter()
            .enumerate()
            .map(|(index, _)| self.row_height_with_widths(index, &column_widths))
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

    #[cfg(test)]
    pub(crate) fn row_height(&self, row_index: usize, column_width: u32) -> u32 {
        self.row_height_with_widths(row_index, &vec![column_width; self.column_count().max(1)])
    }

    pub(crate) fn row_height_with_widths(&self, row_index: usize, column_widths: &[u32]) -> u32 {
        let line_count = self.row_line_count(row_index, column_widths);
        let dynamic_height =
            line_count as u32 * self.line_height() + TABLE_ROW_VERTICAL_PADDING * 2;
        dynamic_height.max(TABLE_ROW_HEIGHT)
    }

    fn row_line_count(&self, row_index: usize, column_widths: &[u32]) -> usize {
        self.rows
            .get(row_index)
            .map(|row| {
                row.iter()
                    .enumerate()
                    .map(|(index, _cell)| {
                        let width = column_widths.get(index).copied().unwrap_or(0);
                        self.cell_span_line_count(row_index, index, width)
                    })
                    .max()
                    .unwrap_or(1)
            })
            .unwrap_or(1)
    }

    pub(crate) fn rows(&self) -> &Vec<Vec<String>> {
        &self.rows
    }

    pub(crate) fn cell_spans(&self, row_index: usize, column_index: usize) -> &[SurfaceTextSpan] {
        self.cell_spans
            .get(row_index)
            .and_then(|row| row.get(column_index))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub(crate) fn cell_span_lines(
        &self,
        row_index: usize,
        column_index: usize,
        width: u32,
    ) -> Vec<Vec<SurfaceTextSpan>> {
        let spans = self.cell_spans(row_index, column_index).to_vec();
        if spans.is_empty() {
            return vec![vec![SurfaceTextSpan::plain(String::new())]];
        }
        Self::wrap_cell_spans(spans, width)
    }

    pub(crate) fn wrap_cell_spans(
        spans: Vec<SurfaceTextSpan>,
        width: u32,
    ) -> Vec<Vec<SurfaceTextSpan>> {
        SurfaceTableSpanLines::wrap(spans, SurfaceTableLayout::cell_max_chars(width))
    }

    pub(crate) fn cell_span_line_count(
        &self,
        row_index: usize,
        column_index: usize,
        width: u32,
    ) -> usize {
        self.cell_span_lines(row_index, column_index, width).len()
    }

    pub(crate) fn cell_line_text(spans: &[SurfaceTextSpan]) -> String {
        spans
            .iter()
            .map(|span| span.text.as_str())
            .collect::<Vec<_>>()
            .join("")
    }

    pub(crate) fn line_height(&self) -> u32 {
        scale_u32(TABLE_LINE_HEIGHT, self.typography.code_scale())
    }

    pub(crate) fn font_size(&self) -> f32 {
        22.0 * self.typography.code_scale()
    }

    pub(crate) fn apply_typography(&mut self, typography: SurfaceTypographyConfig) {
        self.typography = typography;
    }

    pub(crate) fn column_widths_for_width(&self, row_width: u32) -> Vec<u32> {
        let column_count = self.column_count().max(1);
        vec![row_width / column_count as u32; column_count]
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

    #[cfg(test)]
    pub(crate) fn cell_text_y(row_height: u32, line_count: usize) -> u32 {
        Self::cell_text_y_with_line_height(row_height, line_count, TABLE_LINE_HEIGHT)
    }

    pub(crate) fn cell_text_y_with_line_height(
        row_height: u32,
        line_count: usize,
        line_height: u32,
    ) -> u32 {
        let content_height = line_count.max(1) as u32 * line_height;
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

    fn cell_max_chars(width: u32) -> usize {
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

struct SurfaceTableSpanLines;

impl SurfaceTableSpanLines {
    fn wrap(spans: Vec<SurfaceTextSpan>, max_chars: usize) -> Vec<Vec<SurfaceTextSpan>> {
        let mut state = SurfaceTableSpanLineState::new(max_chars);
        for span in spans {
            state.push_span(&span);
        }
        state.finish()
    }
}

struct SurfaceTableSpanLineState {
    max_chars: usize,
    lines: Vec<Vec<SurfaceTextSpan>>,
    current_line: Vec<SurfaceTextSpan>,
    current_chars: usize,
}

impl SurfaceTableSpanLineState {
    fn new(max_chars: usize) -> Self {
        Self {
            max_chars: max_chars.max(1),
            lines: Vec::new(),
            current_line: Vec::new(),
            current_chars: 0,
        }
    }

    fn push_span(&mut self, span: &SurfaceTextSpan) {
        let mut text = String::new();
        for character in span.text.chars() {
            if self.current_chars == self.max_chars {
                self.push_segment(span, &mut text);
                self.start_new_line();
            }
            text.push(character);
            self.current_chars += 1;
        }
        self.push_segment(span, &mut text);
    }

    fn push_segment(&mut self, span: &SurfaceTextSpan, text: &mut String) {
        if text.is_empty() {
            return;
        }
        let mut segment = span.clone();
        segment.text = std::mem::take(text);
        self.current_line.push(segment);
    }

    fn start_new_line(&mut self) {
        if self.current_line.is_empty() {
            return;
        }
        self.lines.push(std::mem::take(&mut self.current_line));
        self.current_chars = 0;
    }

    fn finish(mut self) -> Vec<Vec<SurfaceTextSpan>> {
        self.start_new_line();
        if self.lines.is_empty() {
            return vec![vec![SurfaceTextSpan::plain(String::new())]];
        }
        self.lines
    }
}

fn scale_u32(value: u32, scale: f32) -> u32 {
    if !scale.is_finite() || scale <= 0.0 {
        return value;
    }
    ((value as f32) * scale).round().max(1.0) as u32
}

#[derive(Clone)]
pub(crate) struct SurfaceTableCellPaint<'a> {
    pub(crate) spans: &'a [SurfaceTextSpan],
    pub(crate) alignment: TableAlignment,
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) width: u32,
    pub(crate) row_height: u32,
    pub(crate) table_font_size: f32,
    pub(crate) table_line_height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use katana_markdown_model::{
        ByteRange, LineColumn, LineColumnRange, RawSnippet, SourceSpan, TableCell, TableRow,
    };

    #[test]
    fn table_surface_uses_equal_width_columns_for_export_png_reference() {
        let table = table_block(&[
            &["Short", "Long Column Test", "Short"],
            &[
                "ID",
                "This text is a very long line to verify horizontal scrolling and word wrapping are working correctly.",
                "Notes",
            ],
        ]);
        let block = SurfaceTableBlock::new(&table);

        assert_eq!(vec![166, 166, 166], block.column_widths_for_width(500));
    }

    #[test]
    fn table_surface_height_uses_export_png_equal_width_wrapping() {
        let table = table_block(&[
            &["Short", "Long Column Test", "Short"],
            &[
                "ID",
                "This text is a very long line to verify horizontal scrolling and word wrapping are working correctly.",
                "Notes",
            ],
        ]);
        let block = SurfaceTableBlock::new(&table);

        assert_eq!(
            574,
            block.height_for_width(500),
            "export PNG table height must keep KatanA reference equal-width wrapping"
        );
    }

    fn table_block(rows: &[&[&str]]) -> TableNode {
        TableNode {
            alignments: Vec::new(),
            rows: rows.iter().map(|row| table_row(row)).collect(),
        }
    }

    fn table_row(cells: &[&str]) -> TableRow {
        TableRow {
            cells: cells
                .iter()
                .map(|text| TableCell {
                    text: (*text).to_string(),
                    source: SourceSpan {
                        byte_range: ByteRange { start: 0, end: 0 },
                        line_column_range: LineColumnRange {
                            start: LineColumn { line: 0, column: 0 },
                            end: LineColumn { line: 0, column: 0 },
                        },
                        raw: RawSnippet {
                            text: (*text).to_string(),
                        },
                    },
                })
                .collect(),
        }
    }
}
