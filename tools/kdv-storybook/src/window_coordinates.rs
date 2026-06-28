use katana_ui_core::window::WindowInputNormalizer;
pub(crate) use katana_ui_core::window::{
    WindowCanvasPoint as CanvasPoint, WindowSurfacePoint as WindowPoint,
    WindowSurfaceSize as SurfaceSize,
};
use minifb::{MouseMode, Window};

pub(crate) fn canvas_mouse_position(
    window: &Window,
    surface_width: usize,
    surface_height: usize,
    canvas_width: usize,
    canvas_height: usize,
) -> Option<(f32, f32)> {
    let (x, y) = window.get_unscaled_mouse_pos(MouseMode::Discard)?;
    let point = window_point_to_canvas_point(
        WindowPoint::new(x, y),
        SurfaceSize::new(surface_width, surface_height),
        SurfaceSize::new(canvas_width, canvas_height),
    )?;
    Some((point.x as f32, point.y as f32))
}

pub(crate) fn window_point_to_canvas_point(
    point: WindowPoint,
    window: SurfaceSize,
    canvas: SurfaceSize,
) -> Option<CanvasPoint> {
    WindowInputNormalizer::canvas_point_for_surface_point(point, window, canvas)
}

#[cfg(test)]
mod tests {
    use super::{CanvasPoint, SurfaceSize, WindowPoint, window_point_to_canvas_point};

    #[test]
    fn kuc_core_normalizer_maps_scaled_window_surface_point() {
        let canvas = SurfaceSize::new(1440, 920);
        let window = SurfaceSize::new(2160, 1380);
        let point = WindowPoint::new(465.0, 156.0);

        assert_eq!(
            Some(CanvasPoint { x: 310, y: 104 }),
            window_point_to_canvas_point(point, window, canvas)
        );
    }

    #[test]
    fn kuc_storybook_unscaled_mouse_point_uses_window_surface_not_physical_buffer() {
        let canvas = SurfaceSize::new(1440, 920);
        let window = SurfaceSize::new(1440, 920);
        let point = WindowPoint::new(620.0, 208.0);

        assert_eq!(
            Some(CanvasPoint { x: 620, y: 208 }),
            window_point_to_canvas_point(point, window, canvas)
        );
    }

    #[test]
    fn kuc_storybook_unscaled_mouse_point_keeps_logical_point_when_buffer_is_unscaled() {
        let canvas = SurfaceSize::new(1440, 920);
        let window = SurfaceSize::new(1440, 920);
        let point = WindowPoint::new(310.0, 104.0);

        assert_eq!(
            Some(CanvasPoint { x: 310, y: 104 }),
            window_point_to_canvas_point(point, window, canvas)
        );
    }
}
