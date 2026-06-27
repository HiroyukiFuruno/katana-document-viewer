pub(super) const DIAGRAM_EXPORT_MAX_WIDTH: u32 = 860;
pub(super) const MATH_MAX_WIDTH: u32 = 760;

const KATANA_FULLSCREEN_PADDING_PX: u32 = 40;

pub(super) fn capped_diagram_width(display_width: f32, max_width: u32) -> f32 {
    display_width.min(max_width as f32).max(1.0)
}

pub(super) fn capped_diagram_height(
    display_width: f32,
    display_height: f32,
    max_width: u32,
) -> f32 {
    let capped_width = capped_diagram_width(display_width, max_width);
    display_height_for_width(display_width, display_height, capped_width)
}

pub(super) fn fullscreen_diagram_width(
    surface: &katana_document_viewer::ViewerImageSurface,
    max_width: u32,
    viewport_width: Option<u32>,
    viewport_height: Option<u32>,
) -> f32 {
    let available_width = viewport_width
        .map(fullscreen_available_extent)
        .unwrap_or(max_width as f32)
        .max(1.0);
    let source_width = surface.display_width.max(1.0);
    let source_height = surface.display_height.max(1.0);
    let width_scale = available_width / source_width;
    let height_scale = viewport_height
        .map(fullscreen_available_height)
        .filter(|height| height.is_finite() && *height > 0.0)
        .map_or(f32::INFINITY, |height| height / source_height);
    let scale = width_scale.min(height_scale).min(1.0);
    (source_width * scale).max(1.0)
}

pub(super) fn display_height_for_width(display_width: f32, display_height: f32, width: f32) -> f32 {
    if !display_width.is_finite() || display_width <= 0.0 {
        return display_height.max(1.0);
    }
    (display_height * width / display_width).max(1.0)
}

fn fullscreen_available_height(viewport_height: u32) -> f32 {
    fullscreen_available_extent(viewport_height)
}

fn fullscreen_available_extent(viewport_extent: u32) -> f32 {
    viewport_extent
        .saturating_sub(KATANA_FULLSCREEN_PADDING_PX.saturating_mul(2))
        .max(1) as f32
}
