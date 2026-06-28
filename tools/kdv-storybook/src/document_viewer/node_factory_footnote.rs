use super::KucNodeFactory;
use katana_document_viewer::{ViewerNode, ViewerNodeKind, ViewerTextSpan};
use katana_ui_core::atom::Text;
use katana_ui_core::layout::Row;
use katana_ui_core::render_model::{UiDimension, UiNode, UiTextSpan, UiTextWrapMode};

const FOOTNOTE_MARKER_WIDTH_PX: u16 = 36;

impl KucNodeFactory<'_> {
    pub(super) fn footnote_node(&self, node: &ViewerNode) -> UiNode {
        let marker = Self::footnote_marker(node);
        let body = Self::footnote_body(node, &marker);
        let body_spans = Self::footnote_body_spans(node, &marker);
        let body_node = self.footnote_body_node(node, body, body_spans);
        UiNode::from(
            Row::new()
                .child(Self::footnote_marker_node(&marker))
                .child(body_node),
        )
        .height(UiDimension::Px(self.body_line_height_px()))
    }

    fn footnote_marker(node: &ViewerNode) -> String {
        let ViewerNodeKind::FootnoteDefinition { label } = &node.kind else {
            return String::new();
        };
        format!("{label}.")
    }

    fn footnote_marker_node(marker: &str) -> UiNode {
        UiNode::from(Text::new(marker.to_string()).text_role("list-marker"))
            .width(UiDimension::Px(FOOTNOTE_MARKER_WIDTH_PX))
    }

    fn footnote_body_node(
        &self,
        node: &ViewerNode,
        body: String,
        body_spans: Vec<UiTextSpan>,
    ) -> UiNode {
        let mut text = Text::new(body)
            .text_role("footnote")
            .wrap(UiTextWrapMode::Wrap)
            .selectable(self.interaction.selection_enabled);
        if !body_spans.is_empty() {
            text = text.text_spans(body_spans);
        }
        let rendered: UiNode = text.into();
        if node.spans.iter().any(|span| !span.link_target.is_empty()) {
            return rendered
                .stable_node_id(node.node_id.0.clone())
                .stable_state_id(node.node_id.0.clone());
        }
        rendered
    }

    fn footnote_body(node: &ViewerNode, marker: &str) -> String {
        let marker_prefix = format!("{marker} ");
        node.text
            .strip_prefix(&marker_prefix)
            .unwrap_or(node.text.as_str())
            .to_string()
    }

    fn footnote_body_spans(node: &ViewerNode, marker: &str) -> Vec<UiTextSpan> {
        let marker_prefix = format!("{marker} ");
        Self::text_spans(&Self::spans_without_marker_prefix(
            &node.spans,
            &marker_prefix,
        ))
    }

    fn spans_without_marker_prefix(
        spans: &[ViewerTextSpan],
        marker_prefix: &str,
    ) -> Vec<ViewerTextSpan> {
        let mut remaining_prefix = marker_prefix;
        let mut stripped = Vec::new();
        for span in spans {
            if remaining_prefix.is_empty() {
                stripped.push(span.clone());
                continue;
            }
            let next_prefix = Self::strip_span_prefix(span, remaining_prefix, &mut stripped);
            remaining_prefix = next_prefix;
        }
        stripped
    }

    fn strip_span_prefix<'a>(
        span: &ViewerTextSpan,
        remaining_prefix: &'a str,
        stripped: &mut Vec<ViewerTextSpan>,
    ) -> &'a str {
        if span.text.is_empty() {
            return remaining_prefix;
        }
        if let Some(prefix_rest) = remaining_prefix.strip_prefix(span.text.as_str()) {
            return prefix_rest;
        }
        let Some(text_rest) = span.text.strip_prefix(remaining_prefix) else {
            stripped.push(span.clone());
            return "";
        };
        if !text_rest.is_empty() {
            let mut body_span = span.clone();
            body_span.text = text_rest.to_string();
            stripped.push(body_span);
        }
        ""
    }
}
