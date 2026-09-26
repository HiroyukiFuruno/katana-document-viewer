use super::{CODE_FONT_ROLE, KucNodeFactory, KucNodeLabels};
use katana_document_viewer::{ViewerNode, ViewerNodeKind, ViewerTextSpan, ViewerTextStyle};
use katana_ui_core::atom::Text;
use katana_ui_core::render_model::{UiNode, UiTextSpan, UiTextSpanStyle, UiTextWrapMode};

const KATANA_LONG_MARKDOWN_HEADING_2_MIN_HEIGHT_PX: f32 = 47.0;
const KATANA_LONG_MARKDOWN_HEADING_2_TEXT_ROLE: &str = "heading-2-long";
const EXPORT_TEXT_MAX_CHARS: usize = 58;
const EXPORT_TEXT_WRAP_WIDTH_PX: u32 = 1168;
const EXPORT_TEXT_LAYOUT_FONT_SIZE_PX: f32 = 24.0;
const ASCII_TEXT_WIDTH_FACTOR: f32 = 0.58;
const SPACE_TEXT_WIDTH_FACTOR: f32 = 0.35;
const WIDE_TEXT_WIDTH_FACTOR: f32 = 1.0;

impl KucNodeFactory<'_> {
    pub(super) fn export_wrapped_centered_html_label(&self, node: &ViewerNode) -> Option<String> {
        if !self.export_surface
            || !matches!(
                node.kind,
                ViewerNodeKind::Html {
                    role: katana_document_viewer::ViewerHtmlRole::Centered
                }
            )
            || node.text.chars().count() <= EXPORT_TEXT_MAX_CHARS
        {
            return None;
        }
        // KDV export は centered HTML の長文を58文字単位で行へ分割する。
        // KUC側にも同じ明示改行を渡し、広いhost幅で一行へ戻ることを防ぐ。
        Some(
            node.text
                .chars()
                .collect::<Vec<_>>()
                .chunks(EXPORT_TEXT_MAX_CHARS)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    pub(super) fn export_wrapped_paragraph_spans(
        &self,
        node: &ViewerNode,
    ) -> Option<Vec<UiTextSpan>> {
        if !self.export_surface
            || !matches!(node.kind, ViewerNodeKind::Paragraph)
            || node.spans.is_empty()
        {
            return None;
        }
        let mut line_width = 0;
        let mut wrapped = Vec::new();
        for span in &node.spans {
            for segment in Self::export_text_segments(span) {
                let segment_width = Self::export_text_width(&segment.text);
                if line_width > 0 && line_width + segment_width > EXPORT_TEXT_WRAP_WIDTH_PX {
                    wrapped.push(UiTextSpan {
                        text: "\n".to_string(),
                        style: Self::text_span_style(span.style),
                        link_target: String::new(),
                    });
                    line_width = 0;
                }
                if line_width == 0 && segment.text.trim().is_empty() {
                    continue;
                }
                line_width += segment_width;
                wrapped.push(UiTextSpan {
                    text: segment.text,
                    style: Self::text_span_style(segment.style),
                    link_target: segment.link_target,
                });
            }
        }
        Some(wrapped)
    }

    fn export_text_segments(span: &ViewerTextSpan) -> Vec<ViewerTextSpan> {
        let mut segments = Vec::new();
        let mut current = String::new();
        for character in span.text.chars() {
            current.push(character);
            if character.is_whitespace() {
                segments.push(ViewerTextSpan {
                    text: std::mem::take(&mut current),
                    style: span.style,
                    link_target: span.link_target.clone(),
                });
            }
        }
        if !current.is_empty() {
            segments.push(ViewerTextSpan {
                text: current,
                style: span.style,
                link_target: span.link_target.clone(),
            });
        }
        segments
    }

    fn export_text_width(text: &str) -> u32 {
        text.chars()
            .map(|character| {
                let factor = if character.is_ascii_whitespace() {
                    SPACE_TEXT_WIDTH_FACTOR
                } else if Self::is_east_asian_wide(character) {
                    WIDE_TEXT_WIDTH_FACTOR
                } else {
                    ASCII_TEXT_WIDTH_FACTOR
                };
                EXPORT_TEXT_LAYOUT_FONT_SIZE_PX * factor
            })
            .sum::<f32>()
            .ceil() as u32
    }

    fn is_east_asian_wide(character: char) -> bool {
        matches!(
            character as u32,
            0x1100..=0x115F
                | 0x2329..=0x232A
                | 0x2E80..=0xA4CF
                | 0xAC00..=0xD7A3
                | 0xF900..=0xFAFF
                | 0xFE10..=0xFE19
                | 0xFE30..=0xFE6F
                | 0xFF00..=0xFF60
                | 0xFFE0..=0xFFE6
        )
    }

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
        if Self::is_normalizable_html_data_image_source(node) {
            return String::new();
        }
        if Self::is_html_image_source_recovery(node) {
            return node.text.clone();
        }
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
        if matches!(node.kind, ViewerNodeKind::Code { .. })
            || Self::is_html_image_source_recovery(node)
        {
            return UiTextWrapMode::NoWrap;
        }
        UiTextWrapMode::Wrap
    }

    pub(super) fn is_html_image_source_recovery(node: &ViewerNode) -> bool {
        matches!(node.kind, ViewerNodeKind::Html { .. })
            && katana_document_viewer::HtmlFragmentNormalizer::has_malformed_image_source_attribute(
                &node.source.raw.text,
            )
    }

    fn is_normalizable_html_data_image_source(node: &ViewerNode) -> bool {
        if !Self::is_html_image_source_recovery(node) {
            return false;
        }
        let normalized =
            katana_document_viewer::HtmlFragmentNormalizer::normalize(&node.source.raw.text);
        normalized.contains("data:image/svg+xml")
            && !katana_document_viewer::HtmlFragmentNormalizer::has_malformed_image_source_attribute(
                &normalized,
            )
    }

    pub(super) fn normalizable_html_data_image_source_spans(
        node: &ViewerNode,
    ) -> Option<Vec<UiTextSpan>> {
        if !Self::is_normalizable_html_data_image_source(node) {
            return None;
        }
        Some(vec![UiTextSpan {
            text: node.text.clone(),
            style: UiTextSpanStyle::default(),
            link_target: String::new(),
        }])
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
