use super::PlannedNode;

#[test]
fn html_margin_rejects_missing_colon_and_invalid_number() {
    assert_eq!(0, PlannedNode::html_margin_left_px("margin-left 2px"));
    assert_eq!(0, PlannedNode::html_margin_left_px("margin-left: nope"));
}

#[test]
fn html_margin_converts_relative_units_to_pixels() {
    assert_eq!(24, PlannedNode::html_margin_left_px("margin-left: 1.5rem"));
    assert_eq!(32, PlannedNode::html_margin_left_px("margin-left: 2em"));
}
