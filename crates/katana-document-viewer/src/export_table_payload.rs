use crate::export_html_ops::{escape_html, table_alignment_label};
use crate::export_inline_payload::InlineHtmlWriter;

pub(crate) struct TableHtmlWriter;

impl TableHtmlWriter {
    pub(crate) fn append(
        html: &mut String,
        table: &katana_markdown_model::TableNode,
        fallback_text: &str,
    ) {
        if !Self::has_table_contract(table) {
            html.push_str(&format!("<p>{}</p>\n", escape_html(fallback_text)));
            return;
        }
        let column_sizes = ColumnSizeProfile::from_table(table);
        html.push_str("<table data-kdv-table=\"katana\">\n");
        Self::append_colgroup(html, &column_sizes);
        if let Some(header) = table.rows.first() {
            html.push_str("<thead><tr>");
            for (index, cell) in header.cells.iter().enumerate() {
                Self::append_cell(html, "th", index, table, &column_sizes, &cell.text);
            }
            html.push_str("</tr></thead>\n");
        }
        html.push_str("<tbody>\n");
        for row in table.rows.iter().skip(2) {
            html.push_str("<tr>");
            for (index, cell) in row.cells.iter().enumerate() {
                Self::append_cell(html, "td", index, table, &column_sizes, &cell.text);
            }
            html.push_str("</tr>\n");
        }
        html.push_str("</tbody>\n</table>\n");
    }

    fn has_table_contract(table: &katana_markdown_model::TableNode) -> bool {
        table.rows.len() >= 2
    }

    fn append_colgroup(html: &mut String, column_sizes: &[ColumnSize]) {
        html.push_str("<colgroup>");
        for size in column_sizes {
            html.push_str(size.col_html());
        }
        html.push_str("</colgroup>\n");
    }

    fn append_cell(
        html: &mut String,
        tag: &str,
        index: usize,
        table: &katana_markdown_model::TableNode,
        column_sizes: &[ColumnSize],
        text: &str,
    ) {
        let align = table
            .alignments
            .get(index)
            .unwrap_or(&katana_markdown_model::TableAlignment::Unspecified);
        let column_size = column_sizes.get(index).unwrap_or(&ColumnSize::Wide);
        html.push_str(&format!(
            "<{tag} data-align=\"{}\" data-kdv-column-size=\"{}\">",
            table_alignment_label(align),
            column_size.label()
        ));
        InlineHtmlWriter::append_fragment(html, text);
        html.push_str(&format!("</{tag}>"));
    }
}

struct ColumnSizeProfile;

impl ColumnSizeProfile {
    fn from_table(table: &katana_markdown_model::TableNode) -> Vec<ColumnSize> {
        let column_count = table
            .rows
            .iter()
            .map(|row| row.cells.len())
            .max()
            .unwrap_or_default();
        (0..column_count)
            .map(|index| Self::column_size(table, index))
            .collect()
    }

    fn column_size(table: &katana_markdown_model::TableNode, index: usize) -> ColumnSize {
        let max_width = table
            .rows
            .iter()
            .filter_map(|row| row.cells.get(index))
            .map(|cell| cell.text.chars().count())
            .max()
            .unwrap_or_default();
        if max_width <= 8 {
            ColumnSize::Short
        } else {
            ColumnSize::Wide
        }
    }
}

enum ColumnSize {
    Short,
    Wide,
}

impl ColumnSize {
    fn label(&self) -> &'static str {
        match self {
            Self::Short => "short",
            Self::Wide => "wide",
        }
    }

    fn col_html(&self) -> &'static str {
        match self {
            Self::Short => "<col data-kdv-column-size=\"short\">",
            Self::Wide => "<col data-kdv-column-size=\"wide\">",
        }
    }
}
