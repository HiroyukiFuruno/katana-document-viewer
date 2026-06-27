use super::super::metrics::ViewerNodeMetrics;
use super::super::planned_node::PlannedNode;
use super::super::types::{ViewerHtmlRole, ViewerNodeKind, ViewerTextSpan};
use super::ViewerMediaHeight;
use super::span_line_counter::SpanLineCounter;
use crate::viewer::settings_update::ViewerTypographyConfig;

impl ViewerMediaHeight {
    pub(super) fn text_height(
        planned: &PlannedNode,
        typography: ViewerTypographyConfig,
        content_width: u32,
    ) -> Option<f32> {
        if Self::uses_no_wrap_html_text_height(planned) {
            return Some(ViewerNodeMetrics::body_line_height(typography));
        }
        if Self::uses_span_text_height(&planned.kind) {
            return Some(Self::span_or_block_text_height(
                planned,
                typography,
                content_width,
            ));
        }
        Self::list_or_table_text_height(planned, typography, content_width)
    }

    pub(super) fn accordion_height(
        planned: &PlannedNode,
        typography: ViewerTypographyConfig,
    ) -> Option<f32> {
        let body = Self::details_body(&planned.source.raw.text)?;
        let body_lines = body.lines().filter(|line| !line.trim().is_empty()).count();
        let line_count = body_lines.saturating_add(1).max(2);
        Some(line_count as f32 * ViewerNodeMetrics::body_line_height(typography))
    }

    fn span_or_block_text_height(
        planned: &PlannedNode,
        typography: ViewerTypographyConfig,
        content_width: u32,
    ) -> f32 {
        Self::span_text_height(planned, typography, content_width).unwrap_or_else(|| {
            Self::block_text_height(&planned.kind, &planned.text, typography, content_width)
        })
    }

    fn list_or_table_text_height(
        planned: &PlannedNode,
        typography: ViewerTypographyConfig,
        content_width: u32,
    ) -> Option<f32> {
        matches!(planned.kind, ViewerNodeKind::List | ViewerNodeKind::Table).then(|| {
            Self::block_text_height(&planned.kind, &planned.text, typography, content_width)
        })
    }

    fn block_text_height(
        kind: &ViewerNodeKind,
        text: &str,
        typography: ViewerTypographyConfig,
        content_width: u32,
    ) -> f32 {
        ViewerNodeMetrics::block_height_with_width(kind, text, typography, content_width as usize)
    }

    fn uses_span_text_height(kind: &ViewerNodeKind) -> bool {
        matches!(
            kind,
            ViewerNodeKind::Paragraph
                | ViewerNodeKind::BlockQuote
                | ViewerNodeKind::Alert { .. }
                | ViewerNodeKind::FootnoteDefinition { .. }
                | ViewerNodeKind::Raw
                | ViewerNodeKind::Html {
                    role: ViewerHtmlRole::Generic
                        | ViewerHtmlRole::Left
                        | ViewerHtmlRole::Centered
                        | ViewerHtmlRole::Right
                }
        )
    }

    fn uses_no_wrap_html_text_height(planned: &PlannedNode) -> bool {
        matches!(planned.kind, ViewerNodeKind::Html { .. })
            && planned.source.raw.text.contains("data:image/svg+xml")
            && planned.source.raw.text.contains("xmlns=%22<http")
    }

    fn span_text_height(
        planned: &PlannedNode,
        typography: ViewerTypographyConfig,
        content_width: u32,
    ) -> Option<f32> {
        if matches!(planned.kind, ViewerNodeKind::Alert { .. }) {
            return Some(Self::block_text_height(
                &planned.kind,
                &planned.text,
                typography,
                content_width,
            ));
        }
        let fallback_spans;
        let spans = if planned.spans.is_empty() {
            if planned.text.is_empty() {
                return None;
            }
            fallback_spans = vec![ViewerTextSpan::plain(&planned.text)];
            &fallback_spans
        } else {
            &planned.spans
        };
        let line_count = SpanLineCounter::count(spans, content_width, typography);
        Some(line_count as f32 * ViewerNodeMetrics::body_line_height(typography))
    }

    fn details_body(raw: &str) -> Option<String> {
        let lower = raw.to_ascii_lowercase();
        let summary_end = lower.find("</summary>")? + "</summary>".len();
        let details_end = lower.rfind("</details>")?;
        let body = raw[summary_end..details_end]
            .replace("<div>", "")
            .replace("</div>", "")
            .replace("<p>", "")
            .replace("</p>", "")
            .replace("<br>", "\n")
            .replace("<br/>", "\n")
            .replace("<br />", "\n");
        Some(body.trim().to_string())
    }
}
