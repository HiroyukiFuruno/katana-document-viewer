use super::super::super::code_highlighter::ViewerCodeHighlighter;
use super::super::super::types::ViewerTextSpan;
use super::ViewerNodeClassifier;
use crate::KdvThemeSnapshot;
use katana_markdown_model::{CodeBlockRole, KmmNode};

impl ViewerNodeClassifier {
    pub(super) fn code_block_node_spans(
        role: &CodeBlockRole,
        node: &KmmNode,
        kind: &super::super::super::types::ViewerNodeKind,
    ) -> Vec<ViewerTextSpan> {
        let CodeBlockRole::Plain { language } = role else {
            return Self::monospace_block_spans(node, kind);
        };
        Self::code_block_spans(language.as_deref(), &Self::node_text(node, kind))
    }

    pub(in crate::viewer::node_plan) fn code_block_node_spans_with_theme(
        role: &CodeBlockRole,
        node: &KmmNode,
        kind: &super::super::super::types::ViewerNodeKind,
        theme: &KdvThemeSnapshot,
    ) -> Vec<ViewerTextSpan> {
        let CodeBlockRole::Plain { language } = role else {
            return Self::monospace_block_spans(node, kind);
        };
        Self::code_block_spans_with_theme(language.as_deref(), &Self::node_text(node, kind), theme)
    }

    pub(super) fn code_block_spans(language: Option<&str>, body: &str) -> Vec<ViewerTextSpan> {
        ViewerCodeHighlighter::highlight(language, body)
    }

    pub(super) fn code_block_spans_with_theme(
        language: Option<&str>,
        body: &str,
        theme: &KdvThemeSnapshot,
    ) -> Vec<ViewerTextSpan> {
        ViewerCodeHighlighter::highlight_with_theme(language, body, theme)
    }
}
