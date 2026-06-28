pub(crate) struct DirectHtmlTableNormalizer;

impl DirectHtmlTableNormalizer {
    pub(crate) fn normalize(raw: &str) -> String {
        let rows = HtmlTableRows::parse(raw);
        if rows.is_empty() {
            return raw.to_string();
        }
        MarkdownTable::new(rows).to_markdown()
    }
}

struct HtmlTableRows;

impl HtmlTableRows {
    fn parse(raw: &str) -> Vec<Vec<String>> {
        let lower = raw.to_ascii_lowercase();
        let mut rows = Vec::new();
        let mut cursor = 0;
        while let Some(start_delta) = lower[cursor..].find("<tr") {
            let start = cursor + start_delta;
            let Some(open_delta) = lower[start..].find('>') else {
                break;
            };
            let body_start = start + open_delta + 1;
            let Some(close_delta) = lower[body_start..].find("</tr>") else {
                break;
            };
            let close = body_start + close_delta;
            let cells = HtmlTableCells::parse(&raw[body_start..close]);
            if !cells.is_empty() {
                rows.push(cells);
            }
            cursor = close + "</tr>".len();
        }
        rows
    }
}

struct HtmlTableCells;

impl HtmlTableCells {
    fn parse(row: &str) -> Vec<String> {
        let lower = row.to_ascii_lowercase();
        let mut cells = Vec::new();
        let mut cursor = 0;
        while let Some(cell) = Self::next_cell(row, &lower, cursor) {
            cells.push(HtmlText::text(cell.body));
            cursor = cell.next_cursor;
        }
        cells
    }

    fn next_cell<'a>(row: &'a str, lower: &str, cursor: usize) -> Option<HtmlCell<'a>> {
        let td = lower[cursor..].find("<td").map(|index| (index, "td"));
        let th = lower[cursor..].find("<th").map(|index| (index, "th"));
        let (start_delta, tag) = Self::earliest(td, th)?;
        let start = cursor + start_delta;
        let open_delta = lower[start..].find('>')?;
        let body_start = start + open_delta + 1;
        let close_tag = format!("</{tag}>");
        let close_delta = lower[body_start..].find(&close_tag)?;
        let close = body_start + close_delta;
        Some(HtmlCell {
            body: &row[body_start..close],
            next_cursor: close + close_tag.len(),
        })
    }

    fn earliest(
        left: Option<(usize, &'static str)>,
        right: Option<(usize, &'static str)>,
    ) -> Option<(usize, &'static str)> {
        match (left, right) {
            (Some(left), Some(right)) if left.0 <= right.0 => Some(left),
            (Some(_), Some(right)) => Some(right),
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        }
    }
}

struct HtmlCell<'a> {
    body: &'a str,
    next_cursor: usize,
}

struct MarkdownTable {
    rows: Vec<Vec<String>>,
}

impl MarkdownTable {
    fn new(rows: Vec<Vec<String>>) -> Self {
        Self { rows }
    }

    fn to_markdown(&self) -> String {
        let width = self.rows.iter().map(Vec::len).max().unwrap_or(1);
        let header = Self::row(self.rows.first().map(Vec::as_slice).unwrap_or(&[]), width);
        let divider = Self::divider(width);
        let body = self
            .rows
            .iter()
            .skip(1)
            .map(|row| Self::row(row, width))
            .collect::<Vec<_>>();
        [vec![header, divider], body].concat().join("\n")
    }

    fn divider(width: usize) -> String {
        let cells = (0..width).map(|_| "---").collect::<Vec<_>>();
        Self::pipe_row(&cells)
    }

    fn row(cells: &[String], width: usize) -> String {
        let mut normalized = cells.iter().map(String::as_str).collect::<Vec<_>>();
        while normalized.len() < width {
            normalized.push("");
        }
        Self::pipe_row(&normalized)
    }

    fn pipe_row(cells: &[&str]) -> String {
        format!("| {} |", cells.join(" | "))
    }
}

struct HtmlText;

impl HtmlText {
    fn text(raw: &str) -> String {
        Self::decode(&Self::strip_tags(raw))
            .replace('|', "\\|")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn strip_tags(raw: &str) -> String {
        let mut text = String::new();
        let mut inside_tag = false;
        for character in raw.chars() {
            match character {
                '<' => inside_tag = true,
                '>' if inside_tag => {
                    inside_tag = false;
                    text.push(' ');
                }
                _ if !inside_tag => text.push(character),
                _ => {}
            }
        }
        text
    }

    fn decode(raw: &str) -> String {
        raw.replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
    }
}

#[cfg(test)]
#[path = "direct_html_table_normalizer_tests.rs"]
mod tests;
