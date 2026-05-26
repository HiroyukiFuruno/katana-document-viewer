use super::{SurfaceAlertBlock, SurfaceBadge, SurfaceBadgeRowBlock};

#[test]
fn badge_row_height_text_and_total_width_calculated_from_badges() {
    let row = SurfaceBadgeRowBlock::new(vec![
        SurfaceBadge::linked(
            "left".to_string(),
            "1".to_string(),
            image::Rgba([1, 2, 3, 255]),
            None,
        ),
        SurfaceBadge::single("beta".to_string()),
    ]);

    assert_eq!(row.text(), "left=1 | beta");
    assert_eq!(row.badges().len(), 2);
    assert!(row.total_width() > 0);
    assert_eq!(row.height(), 46);
}

#[test]
fn badge_text_and_width_handle_empty_message_as_zero_width_tail() {
    let badge = SurfaceBadge::single("label-only".to_string());

    assert_eq!(badge.text(), "label-only");
    assert_eq!(badge.message_width(), 0);
    assert!(badge.width() > 0);
}

#[test]
fn alert_block_renders_title_and_body_text() {
    let alert = SurfaceAlertBlock::new(
        "WARNING",
        vec!["line one".to_string(), "line two".to_string()],
        0,
    );

    assert!(alert.text().starts_with("Warning"));
    assert!(alert.text().contains("line one"));
    assert!(alert.text().contains("line two"));
    assert!(alert.height() > 0);
}
