use super::content_scale_for_display_width;

#[test]
fn invalid_display_width_keeps_requested_content_scale() {
    assert_eq!(250, content_scale_for_display_width(100, f32::NAN, 250));
    assert_eq!(1, content_scale_for_display_width(100, 0.0, 0));
}
