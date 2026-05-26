use super::*;

#[test]
fn append_footnote_definition_uses_text_when_children_are_empty() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    InlineHtmlWriter::append_footnote_definition(
        &mut html,
        &definition_node(),
        "1",
        "body",
        &theme,
    );

    assert!(html.contains("data-kdv-footnote-definition=\"1\""));
    assert!(html.contains("body"));
}

#[test]
fn append_footnote_definition_uses_children_when_present() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let node = KmmNode {
        id: KmmNodeId("def".to_string()),
        kind: KmmNodeKind::FootnoteDefinition(FootnoteDefinitionNode {
            label: "1".to_string(),
            text: "ignored".to_string(),
        }),
        source: source_span("body text"),
        children: vec![KmmNode {
            id: KmmNodeId("child".to_string()),
            kind: KmmNodeKind::Text(TextSpan {
                text: "child body".to_string(),
            }),
            source: source_span("child body"),
            children: Vec::new(),
        }],
    };

    InlineHtmlWriter::append_footnote_definition(&mut html, &node, "1", "fallback", &theme);

    assert!(html.contains("child body"));
    assert!(!html.contains("fallback"));
}

#[test]
fn append_fragment_node_fallbacks_to_append_node_for_non_paragraph() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let node = KmmNode {
        id: KmmNodeId("n".to_string()),
        kind: KmmNodeKind::Text(TextSpan {
            text: "raw text".to_string(),
        }),
        source: source_span("raw text"),
        children: Vec::new(),
    };

    InlineHtmlWriter::append_fragment_node(&mut html, &node, &theme);

    assert_eq!(html, "raw text");
}

#[test]
fn append_footnote_reference_and_definition_branches() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let ref_node = KmmNode {
        id: KmmNodeId("fnref".to_string()),
        kind: KmmNodeKind::FootnoteReference(FootnoteReferenceNode {
            label: "1".to_string(),
        }),
        source: source_span(""),
        children: Vec::new(),
    };
    InlineHtmlWriter::append_node(&mut html, &ref_node, &theme);
    InlineHtmlWriter::append_footnote_definition(
        &mut html,
        &definition_node(),
        "1",
        "body",
        &theme,
    );

    assert!(html.contains("data-kdv-footnote-ref=\"1\""));
    assert!(html.contains("data-kdv-footnote-backref=\"1\""));
}
