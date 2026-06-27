use super::super::classifier::ViewerNodeClassifier;
use super::super::planned_node::PlannedNode;
use super::super::search_highlight::ViewerSearchHighlighter;
use super::super::types::{ViewerNodeKind, ViewerTextSpan};
use super::context::ViewerNodeContext;
use super::{ParagraphLayout, ViewerNodePlanBuilder};
use katana_markdown_model::{KmmNode, KmmNodeKind};

impl<'a> ViewerNodePlanBuilder<'a> {
    pub(super) fn planned_node(
        &self,
        node: &KmmNode,
        context: ViewerNodeContext<'_>,
    ) -> Option<PlannedNode> {
        let kind = self.node_kind_for_node(node, context)?;
        let planned = PlannedNode {
            node_id: node.id.clone(),
            source: node.source.clone(),
            text: self.node_text(node, &kind),
            spans: self.highlighted_spans(node, &kind),
            reference: self.asset_reference(node, &kind),
            kind,
        };
        if self.should_skip_planned_node(&planned) {
            return None;
        }
        Some(planned)
    }

    fn highlighted_spans(&self, node: &KmmNode, kind: &ViewerNodeKind) -> Vec<ViewerTextSpan> {
        ViewerSearchHighlighter::highlight(
            self.node_spans(node, kind),
            &self.input.search.query,
            ViewerSearchHighlighter::current_range(&self.input.search, &node.id),
        )
    }

    fn node_text(&self, node: &KmmNode, kind: &ViewerNodeKind) -> String {
        let text = ViewerNodeClassifier::node_text(node, kind);
        if self.should_normalize_soft_line_breaks(kind) {
            return Self::normalize_soft_line_break_text(&text);
        }
        text
    }

    fn node_spans(&self, node: &KmmNode, kind: &ViewerNodeKind) -> Vec<ViewerTextSpan> {
        let spans = if let KmmNodeKind::CodeBlock(role) = &node.kind {
            ViewerNodeClassifier::code_block_node_spans_with_theme(
                role,
                node,
                kind,
                &self.input.theme,
            )
        } else {
            ViewerNodeClassifier::node_spans(node, kind)
        };
        if self.should_normalize_soft_line_breaks(kind) {
            return Self::normalize_soft_line_break_spans(spans);
        }
        spans
    }

    fn should_normalize_soft_line_breaks(&self, kind: &ViewerNodeKind) -> bool {
        self.paragraph_layout == ParagraphLayout::SoftWrap
            && matches!(kind, ViewerNodeKind::Paragraph)
    }

    fn normalize_soft_line_break_text(text: &str) -> String {
        if !text.contains('\n') {
            return text.to_string();
        }
        text.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn normalize_soft_line_break_spans(spans: Vec<ViewerTextSpan>) -> Vec<ViewerTextSpan> {
        let mut spans = spans.into_iter().peekable();
        let mut normalized = Vec::new();
        while let Some(mut span) = spans.next() {
            Self::normalize_soft_line_break_span(&mut span, spans.peek());
            if !span.text.is_empty() {
                normalized.push(span);
            }
        }
        normalized
    }

    fn normalize_soft_line_break_span(span: &mut ViewerTextSpan, next: Option<&ViewerTextSpan>) {
        if !span.text.contains('\n') {
            return;
        }
        if span.text.trim_matches('\n').trim().is_empty() {
            span.text = Self::soft_line_break_separator(next);
            return;
        }
        span.text = span.text.replace('\n', " ");
    }

    fn soft_line_break_separator(next: Option<&ViewerTextSpan>) -> String {
        if next.is_some_and(|span| Self::starts_with_joining_punctuation(&span.text)) {
            return String::new();
        }
        " ".to_string()
    }

    fn starts_with_joining_punctuation(text: &str) -> bool {
        text.chars()
            .find(|character| !character.is_whitespace())
            .is_some_and(|character| {
                matches!(
                    character,
                    '.' | ',' | ';' | ':' | '!' | '?' | ')' | ']' | '}' | '、' | '。'
                )
            })
    }

    fn node_kind_for_node(
        &self,
        node: &KmmNode,
        context: ViewerNodeContext<'_>,
    ) -> Option<ViewerNodeKind> {
        let kind = ViewerNodeClassifier::node_kind_for_node(node)?;
        if Self::image_paragraph_touches_adjacent_block(node, &kind, context) {
            return Some(ViewerNodeKind::Paragraph);
        }
        Some(kind)
    }

    fn image_paragraph_touches_adjacent_block(
        node: &KmmNode,
        kind: &ViewerNodeKind,
        context: ViewerNodeContext<'_>,
    ) -> bool {
        matches!(kind, ViewerNodeKind::Image)
            && matches!(node.kind, KmmNodeKind::Paragraph)
            && !context.is_blank_line_isolated(node)
    }
}
