use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::{image, node, text_node};
use katana_markdown_model::{DescriptionItem, FootnoteDefinitionNode, KmmNode, KmmNodeKind};

#[test]
fn node_text_covers_description_image_and_rule_text() {
    let description = node(description_kind(), "Term\n: Definition", Vec::new());
    assert_eq!(
        "Term: Definition",
        text(&description, ViewerNodeKind::Paragraph)
    );

    let image_node = node(
        KmmNodeKind::Image(image("image alt")),
        "![image alt](a.png)",
        Vec::new(),
    );
    assert_eq!("image alt", text(&image_node, ViewerNodeKind::Image));

    let rule = node(KmmNodeKind::ThematicBreak, "---", Vec::new());
    assert_eq!("", text(&rule, ViewerNodeKind::Rule));
}

#[test]
fn node_text_covers_footnote_definition_text() {
    let footnote = node(footnote_kind(), "[^note]: 注釈本文", Vec::new());

    assert_eq!(
        "note. 注釈本文",
        text(
            &footnote,
            ViewerNodeKind::FootnoteDefinition {
                label: "note".to_string(),
            },
        )
    );
}

#[test]
fn node_text_covers_inline_children_and_raw_fallback() {
    let parent = node(KmmNodeKind::Paragraph, "fallback", vec![text_node("child")]);
    assert_eq!("child", text(&parent, ViewerNodeKind::Paragraph));

    let fallback = node(
        KmmNodeKind::Paragraph,
        "fallback",
        vec![node(KmmNodeKind::Paragraph, "", Vec::new())],
    );
    assert_eq!("fallback", text(&fallback, ViewerNodeKind::Paragraph));
}

#[test]
fn unresolved_metadata_raw_block_keeps_body_text() {
    let raw = "<unknown-kmm-metadata data-id=\"a\">本文を残す</unknown-kmm-metadata>";
    let node = node(
        KmmNodeKind::RawBlock {
            reason: "unresolved metadata".to_string(),
        },
        raw,
        Vec::new(),
    );

    assert_eq!(raw, text(&node, ViewerNodeKind::Raw));
}

fn description_kind() -> KmmNodeKind {
    KmmNodeKind::DescriptionList {
        items: vec![DescriptionItem {
            term: "Term".to_string(),
            description: "Definition".to_string(),
        }],
    }
}

fn footnote_kind() -> KmmNodeKind {
    KmmNodeKind::FootnoteDefinition(FootnoteDefinitionNode {
        label: "note".to_string(),
        text: "注釈本文".to_string(),
    })
}

fn text(node: &KmmNode, kind: ViewerNodeKind) -> String {
    ViewerNodeClassifier::node_text(node, &kind)
}
