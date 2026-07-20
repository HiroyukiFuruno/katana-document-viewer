use super::super::test_support::{input_with_nodes, node};
use super::*;
use crate::ViewerNodePlanner;

#[test]
fn planned_node_skips_empty_paragraph() {
    let input = input_with_nodes(vec![node(KmmNodeKind::Paragraph, "", Vec::new())]);
    let plan = ViewerNodePlanner::create(&input, 0.0);

    assert!(plan.nodes.is_empty());
}

#[test]
fn soft_line_break_spans_normalize_content_and_joining_punctuation() {
    let spans = vec![
        ViewerTextSpan::plain("left\nright"),
        ViewerTextSpan::plain("\n"),
        ViewerTextSpan::plain(" ,"),
        ViewerTextSpan::plain("\n"),
    ];

    let normalized = ViewerNodePlanBuilder::normalize_soft_line_break_spans(spans);
    let texts = normalized
        .iter()
        .map(|span| span.text.as_str())
        .collect::<Vec<_>>();

    assert_eq!(vec!["left right", " ,", " "], texts);
}

#[test]
fn joining_punctuation_detection_covers_punctuation_and_text() {
    assert!(ViewerNodePlanBuilder::starts_with_joining_punctuation(
        " 。"
    ));
    assert!(!ViewerNodePlanBuilder::starts_with_joining_punctuation(
        "word"
    ));
}
