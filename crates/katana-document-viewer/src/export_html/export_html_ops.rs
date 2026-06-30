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
        if body_end == 0 {
            return text.to_string();
        }
        lines[1..body_end]
            .iter()
            .map(|line| Self::strip_indent(line, indent))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub(crate) fn alert_body(text: &str) -> String {
        text.lines()
            .map(Self::strip_blockquote_marker)
            .filter_map(|line| Self::alert_body_line(line.as_str()))
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

    fn alert_body_line(line: &str) -> Option<String> {
        let trimmed = line.trim();
        if let Some(marker) = trimmed.strip_prefix("[!")
            && let Some((_label, rest)) = marker.split_once(']')
        {
            let body = rest.trim();
            return (!body.is_empty()).then(|| body.to_string());
        }
        (!Self::alert_marker(line)).then(|| line.to_string())
    }

    fn fence_line(line: &str) -> bool {
        let trimmed = line.trim_start();
        trimmed.starts_with("```") || trimmed.starts_with("~~~")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fenced_body_returns_plain_text_without_fence() {
        assert_eq!(
            ExportHtmlOps::fenced_body("not a fenced code block"),
            "not a fenced code block"
        );
    }

    #[test]
    fn fenced_body_removes_fence_and_prefix_quotes() {
        let input = "> ```rust\n>   fn main() {}\n> ```";
        assert_eq!(ExportHtmlOps::fenced_body(input), "fn main() {}");
    }

    #[test]
    fn fenced_body_keeps_unclosed_fence_text() {
        assert_eq!(ExportHtmlOps::fenced_body("```"), "```");
    }

    #[test]
    fn fenced_body_keeps_unclosed_multiline_fence_text() {
        assert_eq!(ExportHtmlOps::fenced_body("```\nbody"), "```\nbody");
    }

    #[test]
    fn alert_body_removes_gfm_marker_and_trim() {
        let input = "> [!NOTE]\n> note body";
        assert_eq!(ExportHtmlOps::alert_body(input), "note body");
    }

    #[test]
    fn alert_body_keeps_same_line_marker_body() {
        let input = "> [!NOTE] same line body\n> note body";
        assert_eq!(
            ExportHtmlOps::alert_body(input),
            "same line body\nnote body"
        );
    }

    #[test]
    fn alert_body_removes_markdown_emphasis_marker() {
        let input = "> *NOTE*\n> note body";
        assert_eq!(ExportHtmlOps::alert_body(input), "note body");
    }

    #[test]
    fn render_text_decodes_common_entities() {
        assert_eq!(
            ExportHtmlOps::render_text("a &quot;X&quot; &lt; Y &#62; Z &apos;W&apos; &amp; V"),
            "a &quot;X&quot; &lt; Y &gt; Z &#39;W&#39; &amp; V"
        );
    }

    #[test]
    fn strip_indent_respects_indent_width() {
        assert_eq!(ExportHtmlOps::strip_indent("  text", 2), "text");
        assert_eq!(ExportHtmlOps::strip_indent(" text", 2), "text");
    }
}
