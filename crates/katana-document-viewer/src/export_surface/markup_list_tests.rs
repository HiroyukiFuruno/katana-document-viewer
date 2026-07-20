use super::super::list_marker_text;
use super::support::{list_item, source_span};

#[test]
fn list_marker_text_uses_expected_branches() {
    let mut ordered_item = katana_markdown_model::ListItemNode {
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
fn task_marker_text_falls_back_to_checkbox_for_unknown_marker() {
    assert_eq!(list_marker_text(&list_item("[@]", false), false), "☐ ");
}
