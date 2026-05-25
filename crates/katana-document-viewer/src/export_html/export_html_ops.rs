use katana_markdown_model::{DiagramKind, TableAlignment};

pub(crate) struct ExportHtmlOps;

impl ExportHtmlOps {
    pub(crate) fn fenced_body(text: &str) -> String {
        let lines = text
            .lines()
            .map(Self::strip_fence_blockquote_marker)
            .collect::<Vec<_>>();
        if lines.len() < 2 || !Self::fence_line(lines[0]) {
            return text.to_string();
        }
        let indent = Self::line_indent(lines[0]);
        let body_end = lines
            .iter()
            .rposition(|line| Self::fence_line(line))
            .unwrap_or(lines.len());
        lines[1..body_end]
            .iter()
            .map(|line| Self::strip_indent(line, indent))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub(crate) fn alert_body(text: &str) -> String {
        text.lines()
            .map(Self::strip_blockquote_marker)
            .filter(|line| !Self::alert_marker(line))
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }

    pub(crate) fn diagram_kind_label(kind: &DiagramKind) -> &'static str {
        match kind {
            DiagramKind::Mermaid => "mermaid",
            DiagramKind::DrawIo => "drawio",
            DiagramKind::PlantUml => "plantuml",
        }
    }

    pub(crate) fn table_alignment_label(align: &TableAlignment) -> &'static str {
        match align {
            TableAlignment::Left => "left",
            TableAlignment::Center => "center",
            TableAlignment::Right => "right",
            TableAlignment::Unspecified => "unspecified",
        }
    }

    pub(crate) fn escape_html(value: &str) -> String {
        value
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }

    pub(crate) fn render_text(value: &str) -> String {
        Self::escape_html(&Self::decode_html_entities(value))
    }

    fn decode_html_entities(value: &str) -> String {
        value
            .replace("&quot;", "\"")
            .replace("&#34;", "\"")
            .replace("&lt;", "<")
            .replace("&#60;", "<")
            .replace("&gt;", ">")
            .replace("&#62;", ">")
            .replace("&#39;", "'")
            .replace("&apos;", "'")
            .replace("&amp;", "&")
    }

    fn strip_blockquote_marker(line: &str) -> String {
        line.trim_start()
            .strip_prefix('>')
            .map(str::trim_start)
            .unwrap_or(line)
            .to_string()
    }

    fn strip_fence_blockquote_marker(line: &str) -> &str {
        line.trim_start()
            .strip_prefix('>')
            .map(str::trim_start)
            .unwrap_or(line)
    }

    fn alert_marker(line: &str) -> bool {
        let trimmed = line.trim();
        trimmed.starts_with("[!")
            || matches!(
                trimmed.trim_matches('*').to_ascii_uppercase().as_str(),
                "NOTE" | "TIP" | "IMPORTANT" | "WARNING" | "CAUTION"
            )
    }

    fn fence_line(line: &str) -> bool {
        line.trim_start().starts_with("```")
    }

    fn line_indent(line: &str) -> usize {
        line.bytes().take_while(|it| *it == b' ').count()
    }

    fn strip_indent(line: &str, indent: usize) -> &str {
        let removable = line
            .bytes()
            .take(indent)
            .take_while(|it| *it == b' ')
            .count();
        &line[removable..]
    }
}
