use super::{KdvListMarker, KdvListRow};

#[test]
fn parse_without_whitespace_uses_default_marker_and_full_line_body() {
    let row = KdvListRow::parse("todo");

    assert!(matches!(row.marker, KdvListMarker::Text("-")));
    assert_eq!("todo", row.body);
    assert_eq!(0, row.depth);
}

#[test]
fn parse_text_marker_keeps_marker_and_body() {
    let row = KdvListRow::parse("todo item");

    assert!(matches!(row.marker, KdvListMarker::Text("todo")));
    assert_eq!("item", row.body);
}

#[test]
fn parse_uses_tab_count_for_indent_depth() {
    let row = KdvListRow::parse("\t-- task");

    assert_eq!(1, row.depth);
}
