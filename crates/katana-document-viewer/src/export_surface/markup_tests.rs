use katana_markdown_model::{
    ByteRange, KmmNode, KmmNodeId, KmmNodeKind, LineColumn, LineColumnRange, ListItemNode,
    RawSnippet, SourceSpan, TextSpan,
};

use super::*;

const EMPTY_ID: &str = "id-0";

fn source_span(text: &str) -> SourceSpan {
    SourceSpan {
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

fn text_node(text: &str) -> KmmNode {
    KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind: KmmNodeKind::Text(TextSpan {
            text: text.to_string(),
        }),
        source: source_span(text),
        children: Vec::new(),
    }
}

#[test]
fn parses_details_summary_and_body_variants() {
    let fragment = "<details><summary>Summary</summary><div>Body text</div></details>";
    let parts = SurfaceDetailsParts::parse(fragment);
    assert!(parts.is_some(), "details block should be parseable");
    let Some(parts) = parts else {
        return;
    };
    assert_eq!(parts.summary, "Summary");
    assert_eq!(parts.body, "Body text");

    let fragment_without_div = "<details>\n<summary>Summary</summary> Body only </details>";
    let parts_without_div = SurfaceDetailsParts::parse(fragment_without_div);
    assert!(
        parts_without_div.is_some(),
        "details block should be parseable"
    );
    let Some(parts_without_div) = parts_without_div else {
        return;
    };
    assert_eq!(parts_without_div.body, "Body only");
}

#[test]
fn list_marker_text_uses_expected_branches() {
    let mut ordered_item = ListItemNode {
        marker: "1.".to_string(),
        ordered_number: None,
        task_marker: Some("[x]".to_string()),
        body: Vec::new(),
        children: Vec::new(),
        source: source_span(""),
    };
    assert_eq!(list_marker_text(&ordered_item, false), "☑ ");

    ordered_item.task_marker = None;
    assert_eq!(list_marker_text(&ordered_item, true), "1. ");

    ordered_item.ordered_number = Some(4);
    assert_eq!(list_marker_text(&ordered_item, true), "4. ");
}

#[test]
fn alert_properties_use_all_variants() {
    assert_eq!(alert_title("TIP"), "Tip");
    assert_eq!(alert_title("IMPORTANT"), "Important");
    assert_eq!(alert_title("WARNING"), "Warning");
    assert_eq!(alert_title("CAUTION"), "Caution");
    assert_eq!(alert_title("OTHER"), "Note");
    assert_eq!(alert_label_text("WARNING"), "Warning");
    assert_eq!(alert_color("TIP"), image::Rgba([26, 127, 55, 255]));
    assert_eq!(alert_icon_name("TIP"), "tip-bulb");
    assert_eq!(alert_icon_name("IMPORTANT"), "important-callout");
    assert_eq!(alert_icon_name("WARNING"), "warning-triangle");
    assert_eq!(alert_icon_name("CAUTION"), "caution-octagon");
    assert_eq!(alert_icon_name("OTHER"), "note-circle");
}

#[test]
fn task_marker_text_falls_back_to_checkbox_for_unknown_marker() {
    assert_eq!(list_marker_text(&list_item("[@]", false), false), "☐ ");
}

fn list_item(task_marker: &str, ordered: bool) -> ListItemNode {
    let mut item = ListItemNode {
        marker: if ordered {
            "1.".to_string()
        } else {
            "-".to_string()
        },
        ordered_number: Some(1),
        task_marker: Some(task_marker.to_string()),
        body: Vec::new(),
        children: Vec::new(),
        source: source_span(""),
    };
    if task_marker.is_empty() {
        item.task_marker = None;
    }
    item
}

#[test]
fn alert_body_lines_takes_children_and_raw_fallback() {
    let children_node = KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind: KmmNodeKind::BlockQuote,
        source: source_span("Note"),
        children: vec![text_node("first"), text_node(""), text_node("second")],
    };
    let child_lines = alert_body_lines(&children_node);
    assert_eq!(child_lines, vec!["first".to_string(), "second".to_string()]);

    let fallback_node = KmmNode {
        id: KmmNodeId(EMPTY_ID.to_string()),
        kind: KmmNodeKind::BlockQuote,
        source: source_span("> [!NOTE]\n> body text\n> inline **bold**"),
        children: Vec::new(),
    };
    let fallback_lines = alert_body_lines(&fallback_node);
    assert_eq!(
        fallback_lines,
        vec!["body text".to_string(), "inline bold".to_string()]
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
        kind: KmmNodeKind::Text(TextSpan {
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
