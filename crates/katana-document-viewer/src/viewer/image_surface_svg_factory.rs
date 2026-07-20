use super::image_surface_cache::ViewerImageSurfaceCache;
use super::{ViewerImageSurface, ViewerImageSurfaceError, ViewerImageSurfaceFactory};
use crate::export_surface_svg::SurfaceSvgRasterizer;
use image::Rgba;
use image::imageops::FilterType;

const MAX_TEXTURE_SIDE: u32 = 2048;
const ALPHA_CHANNEL_INDEX: usize = 3;

impl ViewerImageSurfaceFactory {
    pub(crate) fn svg_surface_fingerprint(
        fingerprint: String,
        svg: &str,
        max_width: u32,
        root_font_size: Option<f32>,
        content_scale: u32,
        background: Option<[u8; 4]>,
    ) -> String {
        Self::surface_fingerprint(
            fingerprint,
            svg.as_bytes(),
            max_width,
            root_font_size,
            content_scale,
            background,
        )
    }

    pub(crate) fn rasterize_svg(
        svg: &str,
        max_width: u32,
        root_font_size: Option<f32>,
        content_scale: u32,
    ) -> Result<crate::export_surface_svg::SurfaceSvgImage, ViewerImageSurfaceError> {
        let image = SurfaceSvgRasterizer::rasterize_with_root_font_size_and_content_scale(
            svg,
            max_width,
            root_font_size,
            content_scale,
        )
        .ok_or(ViewerImageSurfaceError::InvalidSvg)?;
        Ok(image)
    }

    pub(crate) fn cache_surface(
        fingerprint: String,
        image: crate::export_surface_svg::SurfaceSvgImage,
        content_scale: u32,
        background: Option<[u8; 4]>,
    ) -> ViewerImageSurface {
        let display_width = image.display_width;
        let display_height = image.display_height;
        let mut image = image.image;
        if let Some(background) = background {
            composite_over_background(&mut image, background);
        }
        let (image, content_scale) = texture_ready_surface(image, display_width, content_scale);
        ViewerImageSurfaceCache::put(ViewerImageSurface {
            fingerprint,
            width: image.width(),
            height: image.height(),
            display_width,
            display_height,
            content_scale,
            rgba: image.into_raw(),
        })
    }
}

fn texture_ready_surface(
    image: image::RgbaImage,
    display_width: f32,
    content_scale: u32,
) -> (image::RgbaImage, u32) {
    let max_side = image.width().max(image.height());
    if max_side <= MAX_TEXTURE_SIDE {
        let physical_width = image.width();
        return (
            image,
            content_scale_for_display_width(physical_width, display_width, content_scale),
        );
    }
    let resize_scale = MAX_TEXTURE_SIDE as f32 / max_side as f32;
    let resized_width = ((image.width() as f32 * resize_scale).round() as u32).max(1);
    let resized_height = ((image.height() as f32 * resize_scale).round() as u32).max(1);
    let resized =
        image::imageops::resize(&image, resized_width, resized_height, FilterType::Triangle);
    let next_content_scale =
        content_scale_for_display_width(resized_width, display_width, content_scale);
    (resized, next_content_scale)
}

fn content_scale_for_display_width(
    physical_width: u32,
    display_width: f32,
    requested_scale: u32,
) -> u32 {
    if !display_width.is_finite() || display_width <= 0.0 {
        return requested_scale.max(1);
    }
    ((physical_width as f32 * 100.0 / display_width).round() as u32)
        .max(1)
        .min(requested_scale.max(1))
}

fn composite_over_background(image: &mut image::RgbaImage, background: [u8; 4]) {
    let background = Rgba([background[0], background[1], background[2], u8::MAX]);
    for pixel in image.pixels_mut() {
        let alpha = pixel[ALPHA_CHANNEL_INDEX];
        if alpha == u8::MAX {
            continue;
        }
        pixel[0] = composite_channel(pixel[0], alpha, background[0]);
        pixel[1] = composite_channel(pixel[1], alpha, background[1]);
        pixel[2] = composite_channel(pixel[2], alpha, background[2]);
        pixel[ALPHA_CHANNEL_INDEX] = u8::MAX;
    }
}

fn composite_channel(foreground: u8, alpha: u8, background: u8) -> u8 {
    let foreground = u16::from(foreground) * u16::from(alpha);
    let background = u16::from(background) * u16::from(u8::MAX - alpha);
    ((foreground + background) / u16::from(u8::MAX)) as u8
}

#[cfg(test)]
#[path = "image_surface_svg_factory_tests.rs"]
mod tests;
