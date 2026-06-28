use super::ViewerNodeClassifier;
use katana_markdown_model::{
    DescriptionItem, KmmNode, KmmNodeKind, ListItemNode, ListNode, TableNode,
};

impl ViewerNodeClassifier {
    #[inline(never)]
    pub(super) fn table_text(table: &TableNode) -> String {
        let mut rows = Vec::new();
        for row in &table.rows {
            if Self::is_table_separator_row(row) {
                continue;
            }
            let mut cells = Vec::new();
            for cell in &row.cells {
                cells.push(Self::table_cell_text(&cell.text));
            }
            rows.push(cells.join(" | "));
        }
        rows.join("\n")
    }

    fn table_cell_text(text: &str) -> String {
        let mut result = String::new();
        let mut cursor = 0;
        while let Some(start_offset) = text[cursor..].find('`') {
            let content_start = cursor + start_offset + 1;
            let Some(end_offset) = text[content_start..].find('`') else {
                break;
            };
            result.push_str(&text[cursor..cursor + start_offset]);
            result.push_str(&text[content_start..content_start + end_offset]);
            cursor = content_start + end_offset + 1;
        }
        result.push_str(&text[cursor..]);
        result
    }

    pub(super) fn is_table_separator_row(row: &katana_markdown_model::TableRow) -> bool {
        row.cells.iter().all(|cell| {
            let trimmed = cell.text.trim();
            !trimmed.is_empty()
                && trimmed
                    .chars()
                    .all(|character| matches!(character, '-' | ':'))
        })
    }

    pub(super) fn has_table_separator_row(table: &TableNode) -> bool {
        table.rows.iter().any(Self::is_table_separator_row)
    }

    #[inline(never)]
    pub(super) fn list_text(list: &ListNode) -> String {
        Self::list_lines(list, 0).join("\n")
    }

    fn list_lines(list: &ListNode, depth: usize) -> Vec<String> {
        let mut items = Vec::new();
        for item in &list.items {
            items.push(Self::list_item_text_at_depth(item, list.ordered, depth));
            for child in &item.children {
                if let KmmNodeKind::List(list) = &child.kind {
                    items.extend(Self::list_lines(list, depth + 1));
                }
            }
        }
        items
    }

    fn list_item_text_at_depth(item: &ListItemNode, ordered: bool, depth: usize) -> String {
        let marker = Self::list_marker(item, ordered);
        let body = Self::inline_nodes_text(&item.body);
        let indent = "  ".repeat(depth);
        if body.is_empty() {
            return format!("{indent}{marker}");
        }
        format!("{indent}{marker} {body}")
    }

    #[inline(never)]
    pub(super) fn list_marker(item: &ListItemNode, ordered: bool) -> String {
        if let Some(task_marker) = &item.task_marker {
            return task_marker.clone();
        }
        if ordered {
            return format!("{}.", item.ordered_number.unwrap_or(1));
        }
        "-".to_string()
    }

    #[inline(never)]
    pub(super) fn block_quote_text(node: &KmmNode) -> String {
        if let Some(text) = Self::legacy_note_quote_text(&node.source.raw.text) {
            return text;
        }
        let quoted = Self::block_quote_raw_lines(&node.source.raw.text);
        if !quoted.is_empty() {
            return quoted.join("\n");
        }
        Self::inline_text(node)
    }

    fn block_quote_raw_lines(raw: &str) -> Vec<String> {
        let mut lines = Vec::new();
        let mut in_code = false;
        for line in raw.lines() {
            let Some((depth, body)) = Self::block_quote_line(line) else {
                continue;
            };
            if body.trim_start().starts_with("```") {
                in_code = !in_code;
                continue;
            }
            if body.trim().is_empty() {
                lines.push(String::new());
                continue;
            }
            lines.push(Self::block_quote_visible_line(depth, body, in_code));
        }
        lines
    }

    fn block_quote_line(line: &str) -> Option<(usize, &str)> {
        let mut rest = line.trim_start();
        let mut depth = 0;
        while let Some(next) = rest.strip_prefix('>') {
            depth += 1;
            rest = next.trim_start();
        }
        (depth > 0).then_some((depth, rest))
    }

    fn block_quote_visible_line(depth: usize, body: &str, in_code: bool) -> String {
        let mut text = if in_code {
            body.to_string()
        } else {
            Self::strip_basic_quote_markers(body)
        };
        if depth > 1 {
            text.insert_str(0, &"  ".repeat(depth - 1));
        }
        text
    }

    fn strip_basic_quote_markers(body: &str) -> String {
        let trimmed = body.trim();
        if let Some(stripped) = trimmed
            .strip_prefix("**")
            .and_then(|value| value.strip_suffix("**"))
        {
            return stripped.to_string();
        }
        trimmed.to_string()
    }

    fn legacy_note_quote_text(raw: &str) -> Option<String> {
        let lines = Self::block_quote_raw_lines(raw)
            .into_iter()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>();
        let [title, body @ ..] = lines.as_slice() else {
            return None;
        };
        if !Self::is_legacy_note_title(title) || body.is_empty() {
            return None;
        }
        Some(format!("{title} {}", body.join(" ")))
    }

    fn is_legacy_note_title(value: &str) -> bool {
        matches!(value, "Note" | "Tip" | "Important" | "Warning" | "Caution")
    }

    #[inline(never)]
    pub(super) fn description_list_text(items: &[DescriptionItem]) -> String {
        let mut lines = Vec::new();
        for item in items {
            lines.push(format!("{}: {}", item.term, item.description));
        }
        lines.join("\n")
    }

    #[inline(never)]
    pub(super) fn inline_nodes_text(nodes: &[KmmNode]) -> String {
        let mut text = String::new();
        for node in nodes {
            text.push_str(&Self::inline_text(node));
        }
        text
    }
}
