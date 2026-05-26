use crate::export_surface_text::SurfaceTextParser;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{KmmNode, KmmNodeKind};

use super::export_surface_span_nodes_helpers::{
    append_inline_html, append_inline_math, append_link, append_style_node, append_unknown_node,
    push, push_plain,
};
use super::{SurfaceInlineSpans, SurfaceTextSpan, SurfaceTextStyle};

impl SurfaceInlineSpans {
    pub(crate) fn from_markdown(markdown: &str, theme: &KdvThemeSnapshot) -> Vec<SurfaceTextSpan> {
        let fragment = crate::export_semantics::EvaluatedMarkdownFragment::evaluate(
            "surface-inline.md",
            markdown,
        );
        if !fragment.has_nodes() {
            return vec![SurfaceTextSpan::plain(
                SurfaceTextParser::decode_basic_entities(markdown),
            )];
        }
        Self::from_nodes_without_fallback(fragment.nodes(), theme)
    }

    pub(crate) fn from_node(node: &KmmNode, theme: &KdvThemeSnapshot) -> Vec<SurfaceTextSpan> {
        let mut spans = Vec::new();
        Self::append_node(&mut spans, node, SurfaceTextStyle::default(), theme);
        spans
    }

    pub(crate) fn from_nodes(nodes: &[KmmNode], theme: &KdvThemeSnapshot) -> Vec<SurfaceTextSpan> {
        let mut spans = Vec::new();
        let mut raw_text = String::new();
        for node in nodes {
            Self::append_node_without_fallback(
                &mut spans,
                node,
                SurfaceTextStyle::default(),
                theme,
            );
            append_node_source_text(&mut raw_text, node);
        }
        let fragment = crate::export_semantics::EvaluatedMarkdownFragment::evaluate(
            "surface-raw-inline.md",
            &raw_text,
        );
        if spans.iter().all(SurfaceTextSpan::is_plain) && fragment.contains_inline_markdown() {
            return Self::from_markdown(&raw_text, theme);
        }
        spans
    }

    fn from_nodes_without_fallback(
        nodes: &[KmmNode],
        theme: &KdvThemeSnapshot,
    ) -> Vec<SurfaceTextSpan> {
        let mut spans = Vec::new();
        for node in nodes {
            Self::append_node_without_fallback(
                &mut spans,
                node,
                SurfaceTextStyle::default(),
                theme,
            );
        }
        spans
    }

    fn append_node(
        spans: &mut Vec<SurfaceTextSpan>,
        node: &KmmNode,
        style: SurfaceTextStyle,
        theme: &KdvThemeSnapshot,
    ) {
        Self::append_node_without_fallback(spans, node, style, theme);
    }

    pub(crate) fn append_node_without_fallback(
        spans: &mut Vec<SurfaceTextSpan>,
        node: &KmmNode,
        style: SurfaceTextStyle,
        theme: &KdvThemeSnapshot,
    ) {
        if Self::append_styled_inline_node(spans, node, style, theme) {
            return;
        }
        if Self::append_semantic_inline_node(spans, node, style, theme) {
            return;
        }
        append_unknown_node(spans, node, style, theme);
    }

    fn append_styled_inline_node(
        spans: &mut Vec<SurfaceTextSpan>,
        node: &KmmNode,
        style: SurfaceTextStyle,
        theme: &KdvThemeSnapshot,
    ) -> bool {
        match &node.kind {
            KmmNodeKind::Text(text) => {
                push_plain(spans, &text.text, style);
            }
            KmmNodeKind::Strong(span) => {
                append_style_node(spans, node, &span.text, style.bold(), theme);
            }
            KmmNodeKind::Emphasis(span) => {
                append_style_node(spans, node, &span.text, style.italic(), theme);
            }
            KmmNodeKind::Strikethrough(span) => {
                append_style_node(spans, node, &span.text, style.strikethrough(), theme);
            }
            KmmNodeKind::InlineCode(code) => {
                push(spans, &code.code, style.inline_code());
            }
            _ => return false,
        }
        true
    }

    fn append_semantic_inline_node(
        spans: &mut Vec<SurfaceTextSpan>,
        node: &KmmNode,
        style: SurfaceTextStyle,
        theme: &KdvThemeSnapshot,
    ) -> bool {
        match &node.kind {
            KmmNodeKind::InlineHtml(html) => {
                append_inline_html(spans, &html.html, style);
            }
            KmmNodeKind::Link(link) => {
                append_link(spans, link.label.as_str(), &link.destination, style);
            }
            KmmNodeKind::Image(image) => push(spans, image.alt.as_str(), style),
            KmmNodeKind::FootnoteReference(reference) => {
                append_link(
                    spans,
                    format!("[{}]", reference.label),
                    format!("#fn-{}", reference.label),
                    style,
                );
            }
            KmmNodeKind::InlineMath(math) => {
                append_inline_math(spans, &math.expression, style, theme);
            }
            KmmNodeKind::Emoji(emoji) => push(spans, emoji.value.as_str(), style),
            _ => return false,
        }
        true
    }
}

fn append_node_source_text(output: &mut String, node: &KmmNode) {
    if !node.source.raw.text.is_empty() {
        output.push_str(&node.source.raw.text);
        return;
    }
    for child in &node.children {
        append_node_source_text(output, child);
    }
}

#[cfg(test)]
#[path = "export_surface_span_nodes_variant_tests.rs"]
mod variant_tests;

#[cfg(test)]
#[path = "export_surface_span_nodes_tests.rs"]
mod tests;
