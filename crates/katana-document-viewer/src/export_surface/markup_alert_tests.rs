use super::super::{
    alert_body_lines, alert_color, alert_icon_name, alert_label_text, alert_title,
    legacy_note_children, legacy_note_quote,
};
use super::support::{EMPTY_ID, source_span, text_node};
use katana_markdown_model::{KmmNode, KmmNodeId, KmmNodeKind};

#[test]
fn alert_properties_use_all_variants() {
    assert_eq!(alert_title("TIP"), "Tip");
    assert_eq!(alert_title("IMPORTANT"), "Important");
    assert_eq!(alert_title("WARNING"), "Warning");
    assert_eq!(alert_title("CAUTION"), "Caution");
    assert_eq!(alert_title("OTHER"), "Note");
    assert_eq!(alert_label_text("WARNING"), "Warning");
    assert_eq!(alert_color("TIP"), image::Rgba([26, 127, 55, 255]));
    assert_eq!(alert_color("WARNING"), image::Rgba([191, 135, 0, 255]));
    assert_eq!(alert_color("CAUTION"), image::Rgba([209, 36, 47, 255]));
    assert_eq!(alert_icon_name("TIP"), "tip-bulb");
    assert_eq!(alert_icon_name("IMPORTANT"), "important-callout");
    assert_eq!(alert_icon_name("WARNING"), "warning-triangle");
    assert_eq!(alert_icon_name("CAUTION"), "caution-circle-slash");
    assert_eq!(alert_icon_name("OTHER"), "note-circle");
}

#[test]
fn alert_body_lines_takes_children_and_raw_fallback() {
    let children_node = KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind: KmmNodeKind::BlockQuote,
        source: source_span("Note"),
        children: vec![text_node("first"), text_node(""), text_node("second")],
    };
    let child_lines = alert_body_lines(&children_node, "NOTE");
    assert_eq!(child_lines, vec!["first".to_string(), "second".to_string()]);

    let fallback_node = KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind: KmmNodeKind::BlockQuote,
        source: source_span("> [!NOTE] same line body\n> body text\n> inline **bold**"),
        children: Vec::new(),
    };
    let fallback_lines = alert_body_lines(&fallback_node, "NOTE");
    assert_eq!(
        fallback_lines,
        vec![
            "same line body".to_string(),
            "body text".to_string(),
            "inline bold".to_string()
        ]
    );
}

#[test]
fn alert_body_lines_skip_generated_title_child() {
    let children_node = KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind: KmmNodeKind::BlockQuote,
        source: source_span("> [!WARNING]\n> body"),
        children: vec![
            text_node("Warning"),
            text_node("body"),
            text_node("Warning"),
        ],
    };

    assert_eq!(
        alert_body_lines(&children_node, "WARNING"),
        vec!["body", "Warning"]
    );
}

#[test]
fn legacy_note_parsing_uses_title_and_body() {
    let quote = "> **Note**\n> one\n>\n> two";
    let note = legacy_note_quote(quote);
    assert!(note.is_some(), "legacy note quote should parse");
    let Some(note) = note else {
        return;
    };
    assert_eq!(note.0, "Note");
    assert_eq!(note.1, "one two");

    let node = KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind: KmmNodeKind::Text(katana_markdown_model::TextSpan {
            text: String::new(),
        }),
        source: source_span("Note"),
        children: vec![text_node("Note"), text_node("first"), text_node("second")],
    };
    let from_children = legacy_note_children(&node.children);
    assert!(from_children.is_some(), "legacy note children should parse");
    let Some(from_children) = from_children else {
        return;
    };
    assert_eq!(from_children.0, "Note");
    assert_eq!(from_children.1, "first second");
}

#[test]
fn alert_body_line_from_quote_keeps_non_prefixed_line() {
    let node = KmmNode {
        id: KmmNodeId("legacy".to_string()),
        kind: KmmNodeKind::BlockQuote,
        source: source_span("> body line"),
        children: Vec::new(),
    };

    assert_eq!(
        alert_body_lines(&node, "NOTE"),
        vec!["body line".to_string()]
    );
}

#[test]
fn alert_body_line_from_quote_keeps_malformed_quote_marker() {
    let node = KmmNode {
        id: KmmNodeId("legacy".to_string()),
        kind: KmmNodeKind::BlockQuote,
        source: source_span("> [!NOTE\n> body"),
        children: Vec::new(),
    };

    assert_eq!(
        alert_body_lines(&node, "NOTE"),
        vec!["[!NOTE".to_string(), "body".to_string()]
    );
}

#[test]
fn alert_body_line_from_quote_skips_empty_matched_marker_line() {
    let node = KmmNode {
        id: KmmNodeId("legacy".to_string()),
        kind: KmmNodeKind::BlockQuote,
        source: source_span("> [!NOTE] \n> body"),
        children: Vec::new(),
    };

    assert_eq!(alert_body_lines(&node, "NOTE"), vec!["body".to_string()]);
}
