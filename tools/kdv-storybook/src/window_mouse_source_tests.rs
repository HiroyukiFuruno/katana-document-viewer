#[test]
fn window_input_uses_kuc_core_mouse_normalizer_for_canvas_space() {
    let mouse_source = include_str!("window_mouse.rs");
    let scroll_source = include_str!("window_scroll.rs");
    let coordinate_source = include_str!("window_coordinates.rs");

    assert!(
        !mouse_source.contains(".get_mouse_pos(")
            && !mouse_source.contains(".get_unscaled_mouse_pos("),
        "window mouse input must go through window_coordinates::canvas_mouse_position"
    );
    assert!(
        !scroll_source.contains(".get_mouse_pos(")
            && !scroll_source.contains(".get_unscaled_mouse_pos("),
        "window scroll hit routing must go through the same coordinate normalizer as mouse clicks"
    );
    assert!(mouse_source.contains("window_coordinates::canvas_mouse_position"));
    assert!(mouse_source.contains("current_canvas_mouse_position"));
    assert!(scroll_source.contains("current_canvas_mouse_position"));
    assert!(
        coordinate_source.contains(".get_unscaled_mouse_pos("),
        "KDV must follow KUC Storybook and read unscaled window coordinates before canvas normalization"
    );
    assert!(
        coordinate_source.contains("WindowInputNormalizer"),
        "KDV must delegate canvas coordinate normalization to katana-ui-core"
    );
    assert!(
        !coordinate_source.contains(".get_mouse_pos("),
        "scaled minifb coordinates shift KUC hit targets on Retina displays"
    );
    assert!(
        mouse_source.contains("window.get_size()"),
        "window input normalization must keep a window-size fallback before the first frame"
    );
    assert!(
        mouse_source.contains("input_surface_size_for_window_point")
            && mouse_source.contains("_frame_canvas: Option<&Canvas>")
            && !mouse_source.contains("return (canvas.width(), canvas.height())"),
        "Retina direct presentation must keep minifb unscaled mouse points in window coordinates"
    );
}
