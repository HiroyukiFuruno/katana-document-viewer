use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::{list_item, node, text_node};
use crate::ViewerTextSpan;
use katana_markdown_model::{FootnoteDefinitionNode, KmmNodeKind, LinkNode, ListNode};

#[test]
fn list_spans_supports_multiple_items_and_empty_body_without_trailing_space() {
    let current = node(
        KmmNodeKind::List(ListNode {
            ordered: false,
            task_markers: Vec::new(),
            items: vec![
                list_item(None, None, Vec::new()),
                list_item(None, None, vec![text_node("two")]),
            ],
        }),
        "-\n- two",
        Vec::new(),
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::List);
    assert_eq!(5, spans.len());
    assert_eq!(
        "-\n- two",
        spans
            .iter()
            .map(|span| span.text.clone())
            .collect::<Vec<_>>()
            .concat()
    );
}

#[test]
fn inline_atom_span_returns_empty_on_empty_link_label() {
    let current = node(
        KmmNodeKind::Paragraph,
        "fallback",
        vec![node(
            KmmNodeKind::Link(LinkNode {
                label: String::new(),
                destination: "https://example.com".to_string(),
                title: None,
                autolink: false,
            }),
            "",
            Vec::new(),
        )],
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);
    assert_eq!(vec![ViewerTextSpan::plain("fallback")], spans);
}

#[test]
fn footnote_definition_span_is_styled_via_inline_atom() {
    let current = node(
        KmmNodeKind::Paragraph,
        "fallback",
        vec![node(
            KmmNodeKind::FootnoteDefinition(FootnoteDefinitionNode {
                label: "1".to_string(),
                text: "note".to_string(),
            }),
            "note",
            Vec::new(),
        )],
    );

    let spans = ViewerNodeClassifier::node_spans(&current, &ViewerNodeKind::Paragraph);

    assert_eq!(1, spans.len());
    assert_eq!("note", spans[0].text);
}
