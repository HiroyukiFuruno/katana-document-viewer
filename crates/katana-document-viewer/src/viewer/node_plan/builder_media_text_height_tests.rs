use super::{
    super::super::metrics::ViewerNodeMetrics,
    super::super::planned_node::PlannedNode,
    super::super::types::{ViewerHtmlRole, ViewerNodeKind, ViewerTextSpan},
    ViewerMediaHeight,
};
use crate::viewer::settings_update::ViewerTypographyConfig;
use katana_markdown_model::{ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet};

#[test]
fn text_height_uses_no_wrap_html_text_height() {
    let planned = planned_node(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Generic,
        },
        "data:image/svg+xml,%3Csvg%20xmlns=%22<http",
        16,
    );
    let typography_24 = ViewerTypographyConfig {
        preview_font_size: 24,
    };
    let typography_16 = ViewerTypographyConfig {
        preview_font_size: 16,
    };

    assert_eq!(
        Some(ViewerNodeMetrics::body_line_height(typography_24)),
        ViewerMediaHeight::text_height(&planned, typography_24, 120)
    );
    assert_eq!(
        Some(ViewerNodeMetrics::body_line_height(typography_16)),
        ViewerMediaHeight::text_height(&planned, typography_16, 120)
    );
}

#[test]
fn text_height_wraps_span_text_when_no_wrap_is_not_used() {
    let planned = planned_node(ViewerNodeKind::Paragraph, "first line", 16);
    let typography = ViewerTypographyConfig {
        preview_font_size: 16,
    };

    let height = ViewerMediaHeight::text_height(&planned, typography, 120);
    assert!(matches!(height, Some(value) if value > 0.0));
}

#[test]
fn list_or_table_text_height_is_available_for_list_nodes() {
    let planned = planned_node(ViewerNodeKind::List, "- first\n- second", 16);
    let typography = ViewerTypographyConfig {
        preview_font_size: 16,
    };

    assert!(ViewerMediaHeight::text_height(&planned, typography, 240).is_some());
}

#[test]
fn details_body_replaces_html_tokens() {
    let body = ViewerMediaHeight::details_body(
        "<details><summary>Title</summary><p>Body</p><br/>After<br>done</details>",
    );
    assert_eq!(Some("Body\nAfter\ndone".to_string()), body);
}

#[test]
fn accordion_height_falls_back_to_two_line_minimum() {
    let planned = planned_node(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Accordion,
        },
        "<details><summary>Title</summary></details>",
        16,
    );
    let typography = ViewerTypographyConfig {
        preview_font_size: 16,
    };
    assert_eq!(
        Some(2.0 * ViewerNodeMetrics::body_line_height(typography)),
        ViewerMediaHeight::accordion_height(&planned, typography)
    );
}

#[test]
fn span_text_height_is_none_when_no_spans_and_no_text() {
    let planned = PlannedNode {
        node_id: KmmNodeId("node-empty".to_string()),
        kind: ViewerNodeKind::Paragraph,
        source: source(""),
        text: String::new(),
        spans: Vec::new(),
        reference: None,
    };

    assert!(
        ViewerMediaHeight::span_text_height(
            &planned,
            ViewerTypographyConfig {
                preview_font_size: 16,
            },
            200
        )
        .is_none()
    );
}

#[test]
fn text_height_falls_back_to_block_height_for_empty_paragraph() {
    let planned = PlannedNode {
        node_id: KmmNodeId("node-empty-fallback".to_string()),
        kind: ViewerNodeKind::Paragraph,
        source: source(""),
        text: String::new(),
        spans: Vec::new(),
        reference: None,
    };
    let typography = ViewerTypographyConfig {
        preview_font_size: 16,
    };

    assert!(ViewerMediaHeight::text_height(&planned, typography, 200).is_some());
}

#[test]
fn span_text_height_uses_plain_span_for_empty_spans_and_non_empty_text() {
    let planned = PlannedNode {
        node_id: KmmNodeId("node-empty-span".to_string()),
        kind: ViewerNodeKind::Paragraph,
        source: source("source"),
        text: "fallback text".to_string(),
        spans: Vec::new(),
        reference: None,
    };
    let typography = ViewerTypographyConfig {
        preview_font_size: 16,
    };
    let height = ViewerMediaHeight::span_text_height(&planned, typography, 200);
    assert_eq!(
        Some(ViewerNodeMetrics::body_line_height(typography)),
        height
    );
}

fn planned_node(kind: ViewerNodeKind, text: &str, font: usize) -> PlannedNode {
    PlannedNode {
        node_id: KmmNodeId(format!("node-{font}")),
        kind,
        source: source(text),
        text: text.to_string(),
        spans: vec![ViewerTextSpan::plain("x")],
        reference: None,
    }
}

fn source(text: &str) -> katana_markdown_model::SourceSpan {
    katana_markdown_model::SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: text.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: text.len() + 1,
            },
        },
        raw: RawSnippet {
            text: text.to_string(),
        },
    }
}
