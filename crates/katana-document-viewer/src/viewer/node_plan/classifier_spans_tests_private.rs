use crate::viewer::node_plan::builder::test_support::{node, text_node};
use crate::viewer::node_plan::classifier::ViewerNodeClassifier;
use crate::viewer::node_plan::types::{ViewerNodeKind, ViewerTextSpan, ViewerTextStyle};
use katana_markdown_model::{DescriptionItem, KmmNodeKind};

#[test]
fn strip_heading_marker_text_keeps_non_marker_or_invalid_marker_text() {
    assert_eq!(
        "#######title",
        super::strip_heading_marker_text("#######title")
    );
    assert_eq!("", super::strip_heading_marker_text("###   "));
}

#[test]
fn strip_heading_marker_spans_drops_only_marker_span_text() {
    let spans = vec![
        ViewerTextSpan {
            text: "##".to_string(),
            style: ViewerTextStyle::default(),
            link_target: String::new(),
        },
        ViewerTextSpan::plain("Title"),
    ];
    let normalized = ViewerNodeClassifier::strip_heading_marker_spans(spans);
    assert_eq!(
        vec!["Title"],
        normalized
            .iter()
            .map(|span| span.text.as_str())
            .collect::<Vec<_>>()
    );
}

#[test]
fn node_spans_renders_footnote_definition_as_body() {
    let node = node(
        KmmNodeKind::FootnoteDefinition(katana_markdown_model::FootnoteDefinitionNode {
            label: "1".to_string(),
            text: "definition".to_string(),
        }),
        "[^1] definition",
        Vec::new(),
    );
    let spans = ViewerNodeClassifier::node_spans(&node, &ViewerNodeKind::Paragraph);
    assert_eq!(4, spans.len());
    assert_eq!("1. definition ↩", ViewerNodeClassifier::spans_text(&spans));
    assert_eq!("#fnref-1", spans[3].link_target);
}

#[test]
fn description_list_node_spans_use_description_format() {
    let node = node(
        KmmNodeKind::DescriptionList {
            items: vec![
                DescriptionItem {
                    term: "Term".to_string(),
                    description: "Description".to_string(),
                },
                DescriptionItem {
                    term: "Next".to_string(),
                    description: "Item".to_string(),
                },
            ],
        },
        "Term: Description\nNext: Item",
        Vec::new(),
    );
    let spans = ViewerNodeClassifier::node_spans(&node, &ViewerNodeKind::Paragraph);
    assert_eq!(1, spans.len());
    assert_eq!("Term: Description\nNext: Item", spans[0].text);
}

#[test]
fn inline_node_spans_omits_empty_child_text() {
    let node = node(KmmNodeKind::Paragraph, "", vec![text_node("")]);
    assert!(ViewerNodeClassifier::inline_node_spans(&node, ViewerTextStyle::default()).is_empty());
}

#[test]
fn inline_spans_normalize_soft_break_variants() {
    let node = node(KmmNodeKind::Paragraph, "", vec![text_node("a\nb")]);

    let joined = ViewerNodeClassifier::inline_spans_or_plain(&node, "ab".to_string());
    assert_eq!("ab", ViewerNodeClassifier::spans_text(&joined));

    let spaced = ViewerNodeClassifier::inline_spans_or_plain(&node, "a b".to_string());
    assert_eq!("a b", ViewerNodeClassifier::spans_text(&spaced));

    let fallback = ViewerNodeClassifier::inline_spans_or_plain(&node, "other\ntext".to_string());
    assert_eq!(vec![ViewerTextSpan::plain("other\ntext")], fallback);
}

#[test]
fn normalize_inline_span_breaks_drops_empty_spans() {
    let normalized = ViewerNodeClassifier::normalize_inline_span_breaks(
        vec![ViewerTextSpan::plain("\n"), ViewerTextSpan::plain("a\nb")],
        "",
    );
    assert_eq!(vec![ViewerTextSpan::plain("ab")], normalized);
}
