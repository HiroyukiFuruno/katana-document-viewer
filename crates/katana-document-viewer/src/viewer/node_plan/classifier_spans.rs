use super::super::types::{ViewerNodeKind, ViewerTextSpan, ViewerTextStyle};
use super::ViewerNodeClassifier;
use katana_markdown_model::{DescriptionItem, KmmNode, KmmNodeKind};

impl ViewerNodeClassifier {
    pub(in crate::viewer::node_plan) fn node_spans(
        node: &KmmNode,
        kind: &ViewerNodeKind,
    ) -> Vec<ViewerTextSpan> {
        match (&node.kind, kind) {
            (KmmNodeKind::Heading(heading), _) => Self::heading_spans(node, &heading.text),
            (KmmNodeKind::HtmlBlock(_), _) => {
                Self::html_block_spans(&node.source.raw.text, Self::node_text(node, kind))
            }
            (KmmNodeKind::List(list), _) => Self::list_spans(list),
            (KmmNodeKind::BlockQuote, _)
            | (KmmNodeKind::Alert { .. }, ViewerNodeKind::BlockQuote) => {
                Self::block_quote_spans(node)
            }
            (KmmNodeKind::DescriptionList { items }, _) => Self::description_list_spans(items),
            (KmmNodeKind::FootnoteDefinition(definition), _) => {
                Self::footnote_definition_spans(definition)
            }
            (KmmNodeKind::CodeBlock(role), _) => Self::code_block_node_spans(role, node, kind),
            (KmmNodeKind::Table(table), ViewerNodeKind::Table) => Self::table_spans(table),
            (KmmNodeKind::Table(_), ViewerNodeKind::Paragraph) => {
                Self::surface_table_fallback_spans(node, kind)
            }
            (KmmNodeKind::ThematicBreak, _) => Vec::new(),
            _ => Self::inline_spans_or_plain(node, Self::node_text(node, kind)),
        }
    }

    fn block_quote_spans(node: &KmmNode) -> Vec<ViewerTextSpan> {
        vec![ViewerTextSpan::plain(Self::block_quote_text(node))]
    }

    fn description_list_spans(items: &[DescriptionItem]) -> Vec<ViewerTextSpan> {
        vec![ViewerTextSpan::plain(Self::description_list_text(items))]
    }

    fn surface_table_fallback_spans(node: &KmmNode, kind: &ViewerNodeKind) -> Vec<ViewerTextSpan> {
        span_wrap::ViewerSpanWrapper::wrap_plain_surface_text(Self::node_text(node, kind))
    }
    fn heading_spans(node: &KmmNode, fallback: &str) -> Vec<ViewerTextSpan> {
        if node.children.is_empty() {
            return vec![ViewerTextSpan::plain(fallback.to_string())];
        }
        let spans = Self::strip_heading_marker_spans(Self::inline_nodes_spans(
            &node.children,
            ViewerTextStyle::default(),
        ));
        if spans.is_empty() {
            return vec![ViewerTextSpan::plain(fallback.to_string())];
        }
        spans
    }

    fn strip_heading_marker_spans(spans: Vec<ViewerTextSpan>) -> Vec<ViewerTextSpan> {
        let mut stripped = Vec::new();
        let mut marker_consumed = false;
        for mut span in spans {
            if !marker_consumed {
                let text = strip_heading_marker_text(&span.text);
                if text.is_empty() {
                    continue;
                }
                span.text = text;
                marker_consumed = true;
            }
            stripped.push(span);
        }
        stripped
    }
    fn monospace_block_spans(node: &KmmNode, kind: &ViewerNodeKind) -> Vec<ViewerTextSpan> {
        vec![ViewerTextSpan::styled(
            Self::node_text(node, kind),
            ViewerTextStyle::default().inline_code(),
        )]
    }
    fn inline_spans_or_plain(node: &KmmNode, fallback: String) -> Vec<ViewerTextSpan> {
        let spans = Self::inline_node_spans(node, ViewerTextStyle::default());
        if spans.is_empty() {
            return vec![ViewerTextSpan::plain(fallback)];
        }
        let spans_text = Self::spans_text(&spans);
        if spans_text != fallback && spans_text.replace('\n', "") == fallback {
            return Self::normalize_inline_span_breaks(spans, "");
        }
        if spans_text != fallback && spans_text.replace('\n', " ") == fallback {
            return Self::normalize_inline_span_breaks(spans, " ");
        }
        if fallback.contains('\n') && spans_text != fallback {
            return vec![ViewerTextSpan::plain(fallback)];
        }
        spans
    }

    fn spans_text(spans: &[ViewerTextSpan]) -> String {
        spans
            .iter()
            .map(|span| span.text.as_str())
            .collect::<String>()
    }

    fn normalize_inline_span_breaks(
        spans: Vec<ViewerTextSpan>,
        replacement: &str,
    ) -> Vec<ViewerTextSpan> {
        spans
            .into_iter()
            .filter_map(|mut span| {
                span.text = span.text.replace('\n', replacement);
                if span.text.is_empty() {
                    return None;
                }
                Some(span)
            })
            .collect()
    }

    fn inline_node_spans(node: &KmmNode, style: ViewerTextStyle) -> Vec<ViewerTextSpan> {
        match &node.kind {
            KmmNodeKind::Strong(_) if !node.children.is_empty() => {
                return Self::inline_nodes_spans(&node.children, style.bold());
            }
            KmmNodeKind::Emphasis(_) if !node.children.is_empty() => {
                return Self::inline_nodes_spans(&node.children, style.italic());
            }
            KmmNodeKind::Strikethrough(_) if !node.children.is_empty() => {
                return Self::inline_nodes_spans(&node.children, style.strikethrough());
            }
            _ => {}
        }
        if !node.children.is_empty() {
            return Self::inline_nodes_spans(&node.children, style);
        }
        Self::inline_atom_spans(&node.kind, style)
    }

    fn inline_nodes_spans(nodes: &[KmmNode], style: ViewerTextStyle) -> Vec<ViewerTextSpan> {
        let mut spans = Vec::new();
        for node in nodes {
            spans.extend(Self::inline_node_spans(node, style));
        }
        spans
    }

    fn inline_atom_spans(kind: &KmmNodeKind, style: ViewerTextStyle) -> Vec<ViewerTextSpan> {
        if let Some(spans) = Self::styled_inline_atom_spans(kind, style) {
            return spans;
        }
        match kind {
            KmmNodeKind::Text(text) => Self::plain_span(&text.text, style),
            KmmNodeKind::InlineHtml(html) => Self::inline_html_spans(&html.html, style),
            KmmNodeKind::Link(link) => {
                Self::linked_span(link.label.clone(), link.destination.clone(), style)
            }
            KmmNodeKind::Image(image) => Self::styled_span(image.alt.clone(), style),
            KmmNodeKind::FootnoteReference(reference) => {
                Self::footnote_reference_span(&reference.label, style)
            }
            KmmNodeKind::FootnoteDefinition(definition) => {
                Self::styled_span(definition.text.clone(), style)
            }
            _ => Vec::new(),
        }
    }
}

fn strip_heading_marker_text(text: &str) -> String {
    let trimmed = text.trim_start();
    let hash_count = trimmed.bytes().take_while(|it| *it == b'#').count();
    if hash_count == 0 {
        return trimmed.to_string();
    }
    if !(1..=6).contains(&hash_count) {
        return text.to_string();
    }
    let after_marker = &trimmed[hash_count..];
    after_marker.trim_start().to_string()
}

#[path = "classifier_code_spans.rs"]
mod code_spans;
#[path = "classifier_html_spans.rs"]
mod html_spans;
#[path = "classifier_inline_atom_spans.rs"]
mod inline_atom_spans;
#[path = "classifier_span_helpers.rs"]
mod span_helpers;
#[path = "classifier_span_wrap.rs"]
mod span_wrap;
#[path = "classifier_table_spans.rs"]
mod table_spans;

#[cfg(test)]
#[path = "classifier_spans_tests_private.rs"]
mod tests;
