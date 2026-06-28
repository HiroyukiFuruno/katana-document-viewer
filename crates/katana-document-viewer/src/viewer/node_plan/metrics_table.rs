use super::ViewerNodeMetrics;
use crate::viewer::settings_update::ViewerTypographyConfig;

const TABLE_ROW_HEIGHT: f32 = 52.0;
const TABLE_LINE_HEIGHT: f32 = 34.0;
const TABLE_CELL_PADDING: usize = 16;
const TABLE_ROW_VERTICAL_PADDING: f32 = 16.0;
const TABLE_ASCII_CELL_CHAR_WIDTH: usize = 12;
const TABLE_WIDE_CELL_CHAR_WIDTH: usize = 22;
const KATANA_TABLE_BASE_WIDTH_OFFSET: usize = TABLE_CELL_PADDING * 2;
const KATANA_TABLE_GUARANTEED_MIN_WIDTH: usize = 40;

impl ViewerNodeMetrics {
    pub(super) fn table_block_height(
        text: &str,
        typography: ViewerTypographyConfig,
        content_width: usize,
    ) -> f32 {
        let rows = text
            .lines()
            .filter(|line| !Self::is_table_separator_line(line))
            .map(Self::table_cells)
            .collect::<Vec<_>>();
        let column_count = rows.iter().map(Vec::len).max().unwrap_or(1).max(1);
        let column_widths = Self::katana_table_column_widths(&rows, column_count, content_width);
        rows.iter()
            .map(|row| Self::table_row_height(row, &column_widths, typography))
            .sum::<f32>()
            .max(TABLE_ROW_HEIGHT * Self::body_scale(typography))
    }

    fn table_row_height(
        row: &[&str],
        column_widths: &[usize],
        typography: ViewerTypographyConfig,
    ) -> f32 {
        let line_count = row
            .iter()
            .enumerate()
            .map(|(index, cell)| {
                Self::wrapped_cell_line_count(cell, column_widths.get(index).copied().unwrap_or(0))
            })
            .max()
            .unwrap_or(1) as f32;
        let line_height = TABLE_LINE_HEIGHT * Self::code_scale(typography);
        let padding = TABLE_ROW_VERTICAL_PADDING;
        let min_height = TABLE_ROW_HEIGHT;
        (line_count * line_height + padding * 2.0).max(min_height)
    }

    fn wrapped_cell_line_count(cell: &str, column_width: usize) -> usize {
        let max_width = column_width
            .saturating_sub(TABLE_CELL_PADDING * 2)
            .max(TABLE_ASCII_CELL_CHAR_WIDTH);
        let mut current_width = 0usize;
        let mut line_count = 1usize;
        for character in cell.chars() {
            let character_width = Self::estimated_cell_char_width(character);
            if current_width > 0 && current_width.saturating_add(character_width) > max_width {
                line_count = line_count.saturating_add(1);
                current_width = 0;
            }
            current_width = current_width.saturating_add(character_width);
        }
        line_count.max(1)
    }

    fn table_cells(line: &str) -> Vec<&str> {
        let mut cells = line.split('|').collect::<Vec<_>>();
        if cells.first().is_some_and(|cell| cell.trim().is_empty()) {
            cells.remove(0);
        }
        if cells.last().is_some_and(|cell| cell.trim().is_empty()) {
            cells.pop();
        }
        cells.iter().map(|cell| cell.trim()).collect()
    }

    fn is_table_separator_line(line: &str) -> bool {
        let cells = Self::table_cells(line);
        !cells.is_empty()
            && cells.iter().all(|cell| {
                let trimmed = cell.trim();
                !trimmed.is_empty()
                    && trimmed
                        .chars()
                        .all(|character| matches!(character, '-' | ':'))
            })
    }

    fn katana_table_column_widths(
        rows: &[Vec<&str>],
        column_count: usize,
        table_width: usize,
    ) -> Vec<usize> {
        if column_count == 0 {
            return Vec::new();
        }
        let max_chars = Self::table_column_max_chars(rows, column_count);
        let mut ideal = max_chars
            .iter()
            .enumerate()
            .map(|(index, chars)| {
                (
                    chars
                        .saturating_mul(TABLE_ASCII_CELL_CHAR_WIDTH)
                        .saturating_add(KATANA_TABLE_BASE_WIDTH_OFFSET),
                    index,
                )
            })
            .collect::<Vec<_>>();
        ideal.sort_by_key(|(width, _)| *width);
        Self::allocate_katana_table_column_widths(column_count, table_width.max(1), &ideal)
    }

    fn table_column_max_chars(rows: &[Vec<&str>], column_count: usize) -> Vec<usize> {
        let mut values = vec![0; column_count];
        for row in rows {
            for (index, cell) in row.iter().enumerate().take(column_count) {
                values[index] = values[index].max(cell.chars().count());
            }
        }
        values
    }

    fn allocate_katana_table_column_widths(
        column_count: usize,
        available_width: usize,
        ideal_widths: &[(usize, usize)],
    ) -> Vec<usize> {
        let mut widths = vec![0; column_count];
        if column_count == 0 {
            return widths;
        }
        let fair_width = available_width / column_count;
        if ideal_widths.iter().all(|(width, _)| *width <= fair_width) {
            widths.fill(fair_width);
            Self::add_table_remainder_to_last_column(&mut widths, available_width);
            return widths;
        }

        let mut remaining_width = available_width;
        let mut remaining_columns = column_count;
        for &(ideal_width, column_index) in ideal_widths {
            let fair_share = remaining_width / remaining_columns.max(1);
            let width = if ideal_width < fair_share {
                ideal_width
            } else {
                let reserved = KATANA_TABLE_GUARANTEED_MIN_WIDTH
                    .saturating_mul(remaining_columns.saturating_sub(1));
                let max_current = remaining_width
                    .saturating_sub(reserved)
                    .max(KATANA_TABLE_GUARANTEED_MIN_WIDTH);
                fair_share.min(max_current)
            };
            widths[column_index] = width;
            remaining_width = remaining_width.saturating_sub(width);
            remaining_columns = remaining_columns.saturating_sub(1);
        }
        Self::add_table_remainder_to_last_column(&mut widths, available_width);
        widths
    }

    fn add_table_remainder_to_last_column(widths: &mut [usize], available_width: usize) {
        let used = widths.iter().sum::<usize>();
        if let Some(last) = widths.last_mut() {
            *last = last.saturating_add(available_width.saturating_sub(used));
        }
    }

    fn estimated_cell_char_width(character: char) -> usize {
        if character.is_ascii() {
            TABLE_ASCII_CELL_CHAR_WIDTH
        } else {
            TABLE_WIDE_CELL_CHAR_WIDTH
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ViewerNodeMetrics;

    #[test]
    fn table_block_height_uses_katana_short_long_short_column_allocation() {
        let text = [
            "Short | Long Column Test | Short",
            "--- | --- | ---",
            "ID | This text is a very long line to verify horizontal scrolling and word wrapping are working correctly. | Notes",
        ]
        .join("\n");

        assert_eq!(
            268.0,
            ViewerNodeMetrics::table_block_height(
                &text,
                ViewerNodeMetrics::default_typography(),
                500
            )
        );
    }
}
