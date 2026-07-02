pub(super) fn debug_size_with_raster_suffix(
    display_width: u32,
    display_height: u32,
    raster_width: u32,
    raster_height: u32,
) -> String {
    let display = format!("{display_width}x{display_height}");
    if display_width == raster_width && display_height == raster_height {
        display
    } else {
        format!("{display}@{raster_width}x{raster_height}")
    }
}
