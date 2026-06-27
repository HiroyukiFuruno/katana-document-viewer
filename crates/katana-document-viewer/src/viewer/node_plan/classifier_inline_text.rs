use super::ViewerNodeClassifier;
use crate::export_html_ops::ExportHtmlOps as HtmlOps;
use crate::export_surface_text::SurfaceTextParser as TextParser;
use katana_markdown_model::{KmmNode, KmmNodeKind};

impl ViewerNodeClassifier {
    pub(super) fn paragraph_text(node: &KmmNode) -> String {
        let text = Self::inline_text(node);
        if text.contains('\n') {
            return Self::normalize_soft_line_breaks(&text);
        }
        if node.source.raw.text.contains('\n') && Self::is_plain_paragraph(node) {
            return Self::normalize_soft_line_breaks(&TextParser::decode_basic_entities(
                &node.source.raw.text,
            ));
        }
        text
    }

    pub(super) fn inline_text(node: &KmmNode) -> String {
        if node.children.is_empty() {
            return Self::inline_atom_text(&node.kind);
        }
        let text = node
            .children
            .iter()
            .map(Self::inline_text)
            .collect::<String>();
        if text.is_empty() {
            return node.source.raw.text.clone();
        }
        text
    }

    fn inline_atom_text(kind: &KmmNodeKind) -> String {
        match kind {
            KmmNodeKind::Text(text) => TextParser::decode_basic_entities(&text.text),
            KmmNodeKind::Strong(span) => span.text.clone(),
            KmmNodeKind::Emphasis(span) => span.text.clone(),
            KmmNodeKind::Strikethrough(span) => span.text.clone(),
            KmmNodeKind::InlineCode(code) => code.code.clone(),
            KmmNodeKind::InlineHtml(html) => TextParser::html_fragment_text(&html.html),
            KmmNodeKind::Link(link) => link.label.clone(),
            KmmNodeKind::Image(image) => image.alt.clone(),
            KmmNodeKind::FootnoteReference(reference) => Self::footnote_text(&reference.label),
            KmmNodeKind::FootnoteDefinition(definition) => definition.text.clone(),
            KmmNodeKind::InlineMath(math) => math.expression.clone(),
            KmmNodeKind::Emoji(emoji) => emoji.value.clone(),
            _ => String::new(),
        }
    }

    pub(super) fn alert_text(label: &str, raw: &str) -> String {
        let mut text = String::new();
        text.push_str(label);
        text.push_str(": ");
        text.push_str(&HtmlOps::alert_body(raw));
        text
    }

    pub(super) fn footnote_text(label: &str) -> String {
        let mut text = String::from("[");
        text.push_str(label);
        text.push(']');
        text
    }

    pub(super) fn footnote_definition_text(label: &str, body: &str) -> String {
        let mut text = label.to_string();
        text.push_str(". ");
        text.push_str(body);
        text
    }

    fn normalize_soft_line_breaks(text: &str) -> String {
        text.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub(super) fn is_plain_paragraph(node: &KmmNode) -> bool {
        matches!(node.kind, KmmNodeKind::Paragraph)
            && node.children.iter().all(Self::is_plain_paragraph_child)
    }

    fn is_plain_paragraph_child(node: &KmmNode) -> bool {
        matches!(node.kind, KmmNodeKind::Text(_))
    }
}
