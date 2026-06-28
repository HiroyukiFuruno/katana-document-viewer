use super::ViewerNodeClassifier;
use crate::{ViewerTextSpan, ViewerTextStyle};
use katana_markdown_model::TableNode;

impl ViewerNodeClassifier {
    pub(super) fn table_spans(table: &TableNode) -> Vec<ViewerTextSpan> {
        let mut spans = Vec::new();
        for (row_index, row) in table.rows.iter().enumerate() {
            if Self::is_table_separator_row(row) {
                continue;
            }
            if !spans.is_empty() && row_index > 0 {
                spans.push(ViewerTextSpan::plain("\n"));
            }
            for (cell_index, cell) in row.cells.iter().enumerate() {
                if cell_index > 0 {
                    spans.push(ViewerTextSpan::plain(" | "));
                }
                spans.extend(Self::table_cell_spans(&cell.text));
            }
        }
        spans
    }

    fn table_cell_spans(text: &str) -> Vec<ViewerTextSpan> {
        let mut spans = Vec::new();
        let mut cursor = 0;
        while let Some(start_offset) = text[cursor..].find('`') {
            let content_start = cursor + start_offset + 1;
            let Some(end_offset) = text[content_start..].find('`') else {
                break;
            };
            Self::push_table_plain(&mut spans, &text[cursor..cursor + start_offset]);
            Self::push_table_inline_code(
                &mut spans,
                &text[content_start..content_start + end_offset],
            );
            cursor = content_start + end_offset + 1;
        }
        Self::push_table_plain(&mut spans, &text[cursor..]);
        spans
    }

    fn push_table_plain(spans: &mut Vec<ViewerTextSpan>, value: &str) {
        if value.is_empty() {
            return;
        }
        spans.push(ViewerTextSpan::plain(value));
    }

    fn push_table_inline_code(spans: &mut Vec<ViewerTextSpan>, value: &str) {
        if value.is_empty() {
            return;
        }
        spans.push(ViewerTextSpan::styled(
            value,
            ViewerTextStyle::default().inline_code(),
        ));
    }

    pub(super) fn inline_marker_text(value: &str) -> String {
        value.replace("**", "").replace('*', "").replace("~~", "")
    }
}
