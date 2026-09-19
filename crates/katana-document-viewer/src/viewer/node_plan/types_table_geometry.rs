use super::ViewerTableProjection;
use crate::ViewerTypographyConfig;

const TABLE_ROW_HEIGHT: u32 = 52;
const TABLE_CELL_PADDING: u32 = 16;
const TABLE_ROW_VERTICAL_PADDING: u32 = 16;
const ASCII_CELL_CHAR_WIDTH: u32 = 12;
const WIDE_CELL_CHAR_WIDTH: u32 = 22;
const TABLE_GUARANTEED_MIN_WIDTH: u32 = 40;

impl ViewerTableProjection {
    #[must_use]
    pub fn column_widths(&self, content_width: u32) -> Vec<u32> {
        if self.column_count == 0 {
            return Vec::new();
        }
        let mut ideal = self.ideal_column_widths();
        ideal.sort_by_key(|(width, _)| *width);
        let mut widths = self.allocate_column_widths(content_width, ideal);
        add_remainder(&mut widths, content_width);
        widths
    }

    #[must_use]
    pub fn row_heights(&self, content_width: u32, typography: ViewerTypographyConfig) -> Vec<u32> {
        let widths = self.column_widths(content_width);
        let line_height = crate::ViewerCodeBlockMetrics::line_height_px(typography);
        self.rows
            .iter()
            .map(|row| row_height(row, &widths, line_height))
            .collect()
    }

    fn ideal_column_widths(&self) -> Vec<(u32, usize)> {
        (0..self.column_count)
            .map(|column| {
                let characters = self
                    .rows
                    .iter()
                    .filter_map(|row| row.cells.get(column))
                    .map(|cell| cell.text.chars().count() as u32)
                    .max()
                    .map_or(0, |value| value);
                (
                    characters
                        .saturating_mul(ASCII_CELL_CHAR_WIDTH)
                        .saturating_add(TABLE_CELL_PADDING * 2),
                    column,
                )
            })
            .collect()
    }

    fn allocate_column_widths(&self, content_width: u32, ideal: Vec<(u32, usize)>) -> Vec<u32> {
        let mut widths = vec![0; self.column_count];
        let fair_width = content_width / self.column_count as u32;
        if ideal.iter().all(|(width, _)| *width <= fair_width) {
            widths.fill(fair_width);
            return widths;
        }
        allocate_variable_widths(&mut widths, content_width, ideal);
        widths
    }
}

fn allocate_variable_widths(widths: &mut [u32], content_width: u32, ideal: Vec<(u32, usize)>) {
    let mut remaining_width = content_width;
    let mut remaining_columns = widths.len() as u32;
    for (ideal_width, column) in ideal {
        let fair_share = remaining_width / remaining_columns.max(1);
        let reserved =
            TABLE_GUARANTEED_MIN_WIDTH.saturating_mul(remaining_columns.saturating_sub(1));
        let max_current = remaining_width
            .saturating_sub(reserved)
            .max(TABLE_GUARANTEED_MIN_WIDTH);
        let width = ideal_width.min(fair_share).min(max_current);
        widths[column] = width;
        remaining_width = remaining_width.saturating_sub(width);
        remaining_columns = remaining_columns.saturating_sub(1);
    }
}

fn add_remainder(widths: &mut [u32], content_width: u32) {
    let used = widths.iter().copied().sum::<u32>();
    if let Some(last) = widths.last_mut() {
        *last = last.saturating_add(content_width.saturating_sub(used));
    }
}

fn row_height(row: &super::ViewerTableRowProjection, widths: &[u32], line_height: u32) -> u32 {
    let lines = row
        .cells
        .iter()
        .enumerate()
        .map(|(column, cell)| {
            wrapped_line_count(
                &cell.text,
                widths.get(column).copied().map_or(0, |value| value),
            )
        })
        .max()
        .map_or(1, |value| value) as u32;
    (lines * line_height + TABLE_ROW_VERTICAL_PADDING * 2).max(TABLE_ROW_HEIGHT)
}

fn wrapped_line_count(text: &str, column_width: u32) -> usize {
    let max_width = column_width
        .saturating_sub(TABLE_CELL_PADDING * 2)
        .max(ASCII_CELL_CHAR_WIDTH);
    let mut current_width = 0_u32;
    let mut lines = 1usize;
    for character in text.chars() {
        let width = if character.is_ascii() {
            ASCII_CELL_CHAR_WIDTH
        } else {
            WIDE_CELL_CHAR_WIDTH
        };
        if current_width > 0 && current_width.saturating_add(width) > max_width {
            lines = lines.saturating_add(1);
            current_width = 0;
        }
        current_width = current_width.saturating_add(width);
    }
    lines
}
