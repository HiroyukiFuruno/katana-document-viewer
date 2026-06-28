const SCROLL_STEP: f32 = 48.0;

pub struct StorybookScroll;

impl StorybookScroll {
    pub fn apply(
        current: f32,
        wheel_delta_y: f32,
        content_height: f32,
        viewport_height: f32,
    ) -> (f32, bool) {
        if wheel_delta_y == 0.0 {
            return (current, false);
        }
        let max_scroll = Self::max_offset(content_height, viewport_height);
        let next = (current - wheel_delta_y * SCROLL_STEP).clamp(0.0, max_scroll);
        (next, (next - current).abs() > f32::EPSILON)
    }

    pub fn wheel_delta_pixels(wheel_delta_y: f32) -> f32 {
        -wheel_delta_y * SCROLL_STEP
    }

    pub fn clamp_offset(target: f32, content_height: f32, viewport_height: f32) -> f32 {
        target.clamp(0.0, Self::max_offset(content_height, viewport_height))
    }

    pub fn max_offset(content_height: f32, viewport_height: f32) -> f32 {
        (content_height - viewport_height).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::StorybookScroll;

    #[test]
    fn apply_scroll_respects_viewport_bounds() {
        let (top, top_changed) = StorybookScroll::apply(0.0, 1.0, 1000.0, 400.0);
        let (bottom, bottom_changed) = StorybookScroll::apply(590.0, -1.0, 1000.0, 400.0);
        let (inside, inside_changed) = StorybookScroll::apply(96.0, -1.0, 1000.0, 400.0);

        assert_eq!(0.0, top);
        assert!(!top_changed);
        assert_eq!(600.0, bottom);
        assert!(bottom_changed);
        assert_eq!(144.0, inside);
        assert!(inside_changed);
    }

    #[test]
    fn wheel_delta_pixels_matches_scroll_step_direction() {
        assert_eq!(48.0, StorybookScroll::wheel_delta_pixels(-1.0));
        assert_eq!(-48.0, StorybookScroll::wheel_delta_pixels(1.0));
    }

    #[test]
    fn clamp_offset_respects_viewport_bounds() {
        assert_eq!(0.0, StorybookScroll::clamp_offset(-12.0, 1000.0, 400.0));
        assert_eq!(600.0, StorybookScroll::clamp_offset(900.0, 1000.0, 400.0));
    }
}
