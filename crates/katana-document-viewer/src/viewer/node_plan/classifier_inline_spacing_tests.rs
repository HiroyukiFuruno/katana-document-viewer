use super::super::ViewerNodeKind;
use super::inline_test_support::*;
use super::test_support::node;
use crate::ViewerNodePlan;
use katana_markdown_model::KmmNodeKind;

#[test]
fn parsed_markdown_soft_breaks_and_heading_spaces_match_katana_preview()
-> Result<(), Box<dyn std::error::Error>> {
    let plan = plan_for(
        "# H1 Heading\n\nThis document exercises every rendering\nfeature of KatanA.\n\n## 1. HTML Centering\n",
    )?;

    assert_eq!(
        "This document exercises every rendering feature of KatanA.",
        paragraph_text(&plan)?
    );
    assert_eq!(
        vec!["H1 Heading", "1. HTML Centering"],
        heading_texts(&plan)
    );
    assert_eq!(
        vec!["H1 Heading", "1. HTML Centering"],
        heading_span_texts(&plan)
    );
    Ok(())
}

#[test]
fn long_rich_paragraph_spans_do_not_bake_export_surface_line_breaks()
-> Result<(), Box<dyn std::error::Error>> {
    let text = "A long fallback paragraph should wrap with the export surface estimate before it reaches the far right edge.";
    let spans = paragraph_spans_for(text);

    assert!(!span_text(&spans).contains('\n'));
    assert_eq!(text, span_text(&spans));
    Ok(())
}

#[test]
fn katana_sample_centering_note_keeps_period_in_same_span_line()
-> Result<(), Box<dyn std::error::Error>> {
    let text = "↑ \"English | 日本語\" should appear on the same line, centered.";
    let spans = paragraph_spans_for(text);

    assert_eq!(text, span_text(&spans));
    assert!(!span_text(&spans).contains('\n'));
    Ok(())
}

fn paragraph_text(plan: &ViewerNodePlan) -> Result<String, Box<dyn std::error::Error>> {
    let paragraph = plan
        .nodes
        .iter()
        .find(|node| matches!(node.kind, ViewerNodeKind::Paragraph))
        .ok_or("paragraph missing")?;
    assert_eq!(paragraph.text, span_text(&paragraph.spans));
    Ok(paragraph.text.clone())
}

fn heading_texts(plan: &ViewerNodePlan) -> Vec<&str> {
    heading_nodes(plan)
        .iter()
        .map(|node| node.text.as_str())
        .collect()
}

fn heading_span_texts(plan: &ViewerNodePlan) -> Vec<String> {
    heading_nodes(plan)
        .iter()
        .map(|node| span_text(&node.spans))
        .collect()
}

fn heading_nodes(plan: &ViewerNodePlan) -> Vec<&crate::ViewerNode> {
    plan.nodes
        .iter()
        .filter(|node| matches!(node.kind, ViewerNodeKind::Heading { .. }))
        .collect()
}

fn paragraph_spans_for(text: &str) -> Vec<crate::ViewerTextSpan> {
    let current = node(
        KmmNodeKind::Paragraph,
        text,
        vec![node(text_kind(text), text, Vec::new())],
    );
    super::super::ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph)
}
