use super::*;

#[test]
fn append_fragment_uses_escape_path_when_no_nodes() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();

    InlineHtmlWriter::append_fragment(&mut html, "raw & text", &theme);

    assert_eq!(html, "raw &amp; text");
}

#[test]
fn append_fragment_parses_and_writes_nodes() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    InlineHtmlWriter::append_fragment(&mut html, "**bold**", &theme);

    assert!(html.contains("<strong>bold</strong>"));
}

#[test]
fn append_fragment_autolinks_bare_urls_in_table_cells() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();

    InlineHtmlWriter::append_fragment(&mut html, "https://example.com/html", &theme);

    assert_eq!(
        html,
        r#"<a href="https://example.com/html" data-kdv-autolink="true">https://example.com/html</a>"#
    );
}

#[test]
fn append_node_falls_back_for_unknown_kind() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let node = KmmNode {
        id: KmmNodeId("desc".to_string()),
        kind: KmmNodeKind::DescriptionList { items: Vec::new() },
        source: source_span("<desc>"),
        children: Vec::new(),
    };

    InlineHtmlWriter::append_node(&mut html, &node, &theme);

    assert_eq!(html, "&lt;desc&gt;");
}

#[test]
fn append_text_uses_plain_text_when_markdown_absent() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();

    InlineHtmlWriter::append_text(&mut html, "plain", &theme);

    assert_eq!(html, "plain");
}

#[test]
fn append_text_falls_back_when_fragment_is_not_structured() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let fragment = EvaluatedMarkdownFragment::evaluate("inline-text.md", "a &amp; b");
    let ok = InlineHtmlWriter::try_append_inline_text(&mut html, &fragment, &theme);

    assert!(!ok);
    assert_eq!(html, "");
}

#[test]
fn append_fragment_node_paragraph_with_and_without_children() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let paragraph_empty = KmmNode {
        id: KmmNodeId("p1".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span("body"),
        children: Vec::new(),
    };
    InlineHtmlWriter::append_fragment_node(&mut html, &paragraph_empty, &theme);

    let child = KmmNode {
        id: KmmNodeId("t1".to_string()),
        kind: KmmNodeKind::Text(TextSpan {
            text: "child".to_string(),
        }),
        source: source_span("child"),
        children: Vec::new(),
    };
    let paragraph_children = KmmNode {
        id: KmmNodeId("p2".to_string()),
        kind: KmmNodeKind::Paragraph,
        source: source_span("p"),
        children: vec![child],
    };
    InlineHtmlWriter::append_fragment_node(&mut html, &paragraph_children, &theme);

    assert_eq!(html, "bodychild");
}

#[test]
fn append_span_uses_nested_children() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let child = KmmNode {
        id: KmmNodeId("child".to_string()),
        kind: KmmNodeKind::Text(TextSpan {
            text: "inner".to_string(),
        }),
        source: source_span("inner"),
        children: Vec::new(),
    };
    let node = KmmNode {
        id: KmmNodeId("span".to_string()),
        kind: KmmNodeKind::Strong(InlineSpan {
            text: "strong".to_string(),
        }),
        source: source_span(""),
        children: vec![child],
    };
    InlineHtmlWriter::append_span(&mut html, &node, "strong", "strong", &theme);
    assert_eq!(html, "<strong>inner</strong>");
}
