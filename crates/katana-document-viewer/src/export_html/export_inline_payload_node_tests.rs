use super::*;

#[test]
fn append_link_appends_autolink_marker_when_enabled() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();

    InlineHtmlWriter::append_node(&mut html, &link_node("link", "label", true), &theme);
    InlineHtmlWriter::append_node(&mut html, &link_node("link2", "plain", false), &theme);

    assert!(html.contains("data-kdv-autolink=\"true\""));
    assert!(html.contains("title=\"title\""));
}

fn link_node(id: &str, label: &str, autolink: bool) -> KmmNode {
    let title = if autolink {
        Some("title".to_string())
    } else {
        None
    };
    let destination = if autolink {
        "https://example.com"
    } else {
        "/path"
    };
    KmmNode {
        id: KmmNodeId(id.to_string()),
        kind: KmmNodeKind::Link(LinkNode {
            label: label.to_string(),
            destination: destination.to_string(),
            title,
            autolink,
        }),
        source: source_span(""),
        children: Vec::new(),
    }
}

#[test]
fn append_span_uses_raw_text_when_childless() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let node = KmmNode {
        id: KmmNodeId("span".to_string()),
        kind: KmmNodeKind::Emphasis(InlineSpan {
            text: "emphasis".to_string(),
        }),
        source: source_span("emphasis"),
        children: Vec::new(),
    };
    InlineHtmlWriter::append_span(&mut html, &node, "em", "emphasis", &theme);
    assert_eq!(html, "<em>emphasis</em>");
}

#[test]
fn append_image_with_title() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let node = KmmNode {
        id: KmmNodeId("img".to_string()),
        kind: KmmNodeKind::Image(katana_markdown_model::ImageNode {
            alt: "alt".to_string(),
            src: "icon.png".to_string(),
            title: Some("icon".to_string()),
        }),
        source: source_span(""),
        children: Vec::new(),
    };
    InlineHtmlWriter::append_node(&mut html, &node, &theme);
    assert!(html.contains("title=\"icon\""));
}

#[test]
fn append_fragment_without_nodes_uses_raw_text() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();

    InlineHtmlWriter::append_fragment(&mut html, "", &theme);

    assert_eq!(html, "");
}

#[test]
fn append_code_uses_tag_and_escapes_text() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let node = KmmNode {
        id: KmmNodeId("code".to_string()),
        kind: KmmNodeKind::InlineCode(katana_markdown_model::InlineCodeNode {
            code: "a < b".to_string(),
        }),
        source: source_span("a < b"),
        children: Vec::new(),
    };

    InlineHtmlWriter::append_node(&mut html, &node, &theme);

    assert_eq!(html, "<code>a &lt; b</code>");
}

#[test]
fn append_image_without_title_has_no_title_attribute() {
    let mut html = String::new();
    let theme = KdvThemeSnapshot::katana_light();
    let node = KmmNode {
        id: KmmNodeId("img".to_string()),
        kind: KmmNodeKind::Image(katana_markdown_model::ImageNode {
            alt: "alt".to_string(),
            src: "icon.png".to_string(),
            title: None,
        }),
        source: source_span(""),
        children: Vec::new(),
    };

    InlineHtmlWriter::append_node(&mut html, &node, &theme);

    assert!(html.contains("<img src=\"icon.png\" alt=\"alt\">"));
    assert!(!html.contains("title="));
}

#[test]
fn try_append_inline_text_paths() {
    let theme = KdvThemeSnapshot::katana_light();

    let inline = EvaluatedMarkdownFragment::evaluate("inline-text.md", "raw text");
    assert!(!InlineHtmlWriter::try_append_inline_text(
        &mut String::new(),
        &inline,
        &theme
    ));

    let structured = EvaluatedMarkdownFragment::evaluate("inline-text.md", "**bold**");
    assert!(InlineHtmlWriter::try_append_inline_text(
        &mut String::new(),
        &structured,
        &theme
    ));
}
