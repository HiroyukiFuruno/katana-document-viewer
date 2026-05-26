use crate::export_html_ops::ExportHtmlOps;
use crate::export_inline_payload::InlineHtmlWriter;
use crate::theme::KdvThemeSnapshot;

pub(crate) struct LegacyNoteHtmlWriter;

impl LegacyNoteHtmlWriter {
    pub(crate) fn append(html: &mut String, raw: &str, theme: &KdvThemeSnapshot) {
        html.push_str("<blockquote data-kdv-blockquote=\"quote\">");
        match LegacyNoteBlock::from_raw(raw) {
            Some(block) => block.append(html, theme),
            None => html.push_str(&ExportHtmlOps::render_text(raw)),
        }
        html.push_str("</blockquote>\n");
    }
}

struct LegacyNoteBlock {
    title: String,
    body: String,
}

impl LegacyNoteBlock {
    fn from_raw(raw: &str) -> Option<Self> {
        let mut lines = raw.lines().filter_map(strip_quote_line);
        let title = lines.next().and_then(strong_label)?;
        let body = lines
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        Some(Self { title, body })
    }

    fn append(&self, html: &mut String, theme: &KdvThemeSnapshot) {
        html.push_str("<p><strong>");
        html.push_str(&ExportHtmlOps::escape_html(&self.title));
        html.push_str("</strong>");
        if !self.body.is_empty() {
            html.push(' ');
            InlineHtmlWriter::append_fragment(html, &self.body, theme);
        }
        html.push_str("</p>");
    }
}

fn strip_quote_line(line: &str) -> Option<&str> {
    let text = line.trim_start().strip_prefix('>')?.trim_start();
    (!text.is_empty()).then_some(text)
}

fn strong_label(line: &str) -> Option<String> {
    let trimmed = line.trim();
    trimmed
        .strip_prefix("**")
        .and_then(|value| value.strip_suffix("**"))
        .map(ToString::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::KdvThemeSnapshot;

    #[test]
    fn append_parses_title_and_body_from_legacy_note() {
        let mut html = String::new();
        let note = "> **Note**\n> body text";
        LegacyNoteHtmlWriter::append(&mut html, note, &KdvThemeSnapshot::katana_light());
        assert!(html.contains("<blockquote data-kdv-blockquote=\"quote\">"));
        assert!(html.contains("<strong>Note</strong>"));
        assert!(html.contains("body text"));
    }

    #[test]
    fn append_fallback_raw_text_when_title_not_found() {
        let mut html = String::new();
        LegacyNoteHtmlWriter::append(&mut html, "not note", &KdvThemeSnapshot::katana_light());
        assert_eq!(
            html,
            "<blockquote data-kdv-blockquote=\"quote\">not note</blockquote>\n"
        );
    }

    #[test]
    fn from_raw_handles_empty_body() {
        assert!(LegacyNoteBlock::from_raw("> **Note**").is_some());
    }
}
