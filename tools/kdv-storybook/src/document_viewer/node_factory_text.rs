use super::{CODE_FONT_ROLE, KucNodeFactory, KucNodeLabels};
use katana_document_viewer::{ViewerNode, ViewerNodeKind, ViewerTextSpan, ViewerTextStyle};
use katana_ui_core::atom::Text;
use katana_ui_core::render_model::{UiNode, UiTextSpan, UiTextSpanStyle, UiTextWrapMode};

const KATANA_LONG_MARKDOWN_HEADING_2_MIN_HEIGHT_PX: f32 = 47.0;
const KATANA_LONG_MARKDOWN_HEADING_2_TEXT_ROLE: &str = "heading-2-long";

impl KucNodeFactory<'_> {
    pub(super) fn text_role_for_node(&self, node: &ViewerNode) -> &'static str {
        if self.export_surface {
            return KucNodeLabels::export_surface_text_role(&node.kind);
        }
        if Self::uses_katana_long_markdown_heading_2_metrics(node) {
            return KATANA_LONG_MARKDOWN_HEADING_2_TEXT_ROLE;
        }
        KucNodeLabels::text_role(&node.kind)
    }

    pub(super) fn font_role_for_node(&self, node: &ViewerNode) -> &'static str {
        if Self::is_inline_code_only_node(node) {
            return CODE_FONT_ROLE;
        }
        if self.export_surface {
            return KucNodeLabels::export_surface_font_role(&node.kind);
        }
        KucNodeLabels::font_role(&node.kind)
    }

    pub(super) fn text_label(node: &ViewerNode) -> String {
        if !matches!(node.kind, ViewerNodeKind::Table) && !node.spans.is_empty() {
            return node
                .spans
                .iter()
                .map(|span| span.text.as_str())
                .collect::<String>();
        }
        KucNodeLabels::label(node)
    }

    pub(super) fn text_with_role(&self, label: String, text_role: &'static str) -> UiNode {
        Text::new(label)
            .font_role(CODE_FONT_ROLE)
            .text_role(text_role)
            .selectable(self.interaction.selection_enabled)
            .into()
    }

    pub(super) fn text_wrap_for_node(node: &ViewerNode) -> UiTextWrapMode {
        if matches!(node.kind, ViewerNodeKind::Code { .. }) {
            return UiTextWrapMode::NoWrap;
        }
        UiTextWrapMode::Wrap
    }

    fn is_inline_code_only_node(node: &ViewerNode) -> bool {
        !node.spans.is_empty()
            && node
                .spans
                .iter()
                .all(|span| span.style.inline_code || span.text.trim().is_empty())
    }

    fn uses_katana_long_markdown_heading_2_metrics(node: &ViewerNode) -> bool {
        matches!(node.kind, ViewerNodeKind::Heading { level: 2 })
            && node.rect.height >= KATANA_LONG_MARKDOWN_HEADING_2_MIN_HEIGHT_PX
    }

    pub(super) fn text_spans(spans: &[ViewerTextSpan]) -> Vec<UiTextSpan> {
        spans
            .iter()
            .map(|span| UiTextSpan {
                text: span.text.clone(),
                style: Self::text_span_style(span.style),
                link_target: span.link_target.clone(),
            })
            .collect()
    }

    fn text_span_style(style: ViewerTextStyle) -> UiTextSpanStyle {
        UiTextSpanStyle {
            bold: style.bold,
            italic: style.italic,
            monospace: style.monospace,
            underline: style.underline,
            strikethrough: style.strikethrough,
            highlight: style.highlight,
            current_highlight: style.current_highlight,
            inline_code: style.inline_code,
            inline_math: style.inline_math,
            emoji: style.emoji,
            color_rgba: style.color_rgba,
        }
    }
}
