use image::RgbaImage;
use resvg::usvg;

mod export_surface_svg_preprocess;
mod export_surface_svg_raster;

use export_surface_svg_preprocess::preprocess_for_rasterizer;
use export_surface_svg_raster::RasterTarget;

pub(crate) struct SurfaceSvgImage {
    pub(crate) image: RgbaImage,
    pub(crate) display_width: f32,
    pub(crate) display_height: f32,
}

impl SurfaceSvgImage {
    pub(crate) fn display_width_px(&self) -> u32 {
        logical_pixel_extent(self.display_width)
    }

    pub(crate) fn display_height_px(&self) -> u32 {
        logical_pixel_extent(self.display_height)
    }

    #[cfg(test)]
    pub(crate) fn from_image(image: RgbaImage) -> Self {
        Self {
            display_width: image.width() as f32,
            display_height: image.height() as f32,
            image,
        }
    }
}

pub(crate) struct SurfaceSvgRasterizer;

const EXPORT_SURFACE_SVG_CONTENT_SCALE: u32 = 200;

impl SurfaceSvgRasterizer {
    pub(crate) fn rasterize(svg_text: &str, max_width: u32) -> Option<SurfaceSvgImage> {
        Self::rasterize_with_root_font_size(svg_text, max_width, None)
    }

    pub(crate) fn rasterize_for_export_surface(
        svg_text: &str,
        max_width: u32,
    ) -> Option<SurfaceSvgImage> {
        Self::rasterize_export_surface_with_root_font_size(svg_text, max_width, None)
    }

    pub(crate) fn rasterize_export_surface_with_root_font_size(
        svg_text: &str,
        max_width: u32,
        root_font_size: Option<f32>,
    ) -> Option<SurfaceSvgImage> {
        Self::rasterize_with_options(
            svg_text,
            max_width,
            root_font_size,
            EXPORT_SURFACE_SVG_CONTENT_SCALE,
            true,
        )
    }

    pub(crate) fn rasterize_with_root_font_size(
        svg_text: &str,
        max_width: u32,
        root_font_size: Option<f32>,
    ) -> Option<SurfaceSvgImage> {
        Self::rasterize_with_root_font_size_and_content_scale(
            svg_text,
            max_width,
            root_font_size,
            100,
        )
    }

    pub(crate) fn rasterize_with_root_font_size_and_content_scale(
        svg_text: &str,
        max_width: u32,
        root_font_size: Option<f32>,
        content_scale: u32,
    ) -> Option<SurfaceSvgImage> {
        Self::rasterize_with_options(svg_text, max_width, root_font_size, content_scale, false)
    }

    fn rasterize_with_options(
        svg_text: &str,
        max_width: u32,
        root_font_size: Option<f32>,
        content_scale: u32,
        preserve_layout_width: bool,
    ) -> Option<SurfaceSvgImage> {
        let tree = parse_svg_tree(svg_text, root_font_size)?;
        let size = tree.size();
        let target = raster_target(size, max_width, content_scale, preserve_layout_width);
        let pixmap = target.render(&tree)?;
        let image = RgbaImage::from_raw(target.width(), target.height(), pixmap.take())?;
        let (display_width, display_height) = display_extent(size, &target, preserve_layout_width);
        Some(SurfaceSvgImage {
            image,
            display_width,
            display_height,
        })
    }

    pub(crate) fn display_size(svg_text: &str, root_font_size: Option<f32>) -> Option<(f32, f32)> {
        let tree = parse_svg_tree(svg_text, root_font_size)?;
        let size = tree.size();
        Some((logical_extent(size.width()), logical_extent(size.height())))
    }
}

fn parse_svg_tree(svg_text: &str, root_font_size: Option<f32>) -> Option<usvg::Tree> {
    let compatible_svg = preprocess_for_rasterizer(
        svg_text,
        root_font_size.filter(|size| size.is_finite() && *size > 0.0),
    );
    usvg::Tree::from_str(
        &compatible_svg,
        &export_surface_svg_raster::rasterizer_options(),
    )
    .ok()
}

fn raster_target(
    size: usvg::Size,
    max_width: u32,
    content_scale: u32,
    preserve_layout_width: bool,
) -> RasterTarget {
    if preserve_layout_width {
        return RasterTarget::new_export_surface(size, max_width, content_scale);
    }
    RasterTarget::new_with_content_scale(size, max_width, content_scale)
}

fn display_extent(
    size: usvg::Size,
    target: &RasterTarget,
    preserve_layout_width: bool,
) -> (f32, f32) {
    if preserve_layout_width {
        return (
            target.display_width() as f32,
            target.display_height() as f32,
        );
    }
    (logical_extent(size.width()), logical_extent(size.height()))
}

fn logical_extent(value: f32) -> f32 {
    if !value.is_finite() || value <= 0.0 {
        return 1.0;
    }
    value
}

fn logical_pixel_extent(value: f32) -> u32 {
    logical_extent(value).ceil() as u32
}

#[cfg(test)]
#[path = "export_surface_svg_tests.rs"]
mod tests;
