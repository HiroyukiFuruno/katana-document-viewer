use crate::export_surface_helpers::SURFACE_CONTENT_WIDTH;
use crate::export_surface_svg::SurfaceSvgRasterizer;
use image::RgbaImage;

pub(crate) struct SurfaceImageBlock {
    pub(crate) image: RgbaImage,
    pub(crate) _alt: String,
}

impl SurfaceImageBlock {
    pub(crate) fn from_path(
        path: &std::path::Path,
        requested_width: Option<u32>,
        alt: String,
    ) -> Option<Self> {
        if is_svg_path(path) {
            return Self::from_svg_path(path, requested_width, alt);
        }
        let image = image::open(path).ok()?.to_rgba8();
        let image = scaled_image(image, requested_width);
        Some(Self { image, _alt: alt })
    }

    fn from_svg_path(
        path: &std::path::Path,
        requested_width: Option<u32>,
        alt: String,
    ) -> Option<Self> {
        let svg = std::fs::read_to_string(path).ok()?;
        let max_width = requested_width.unwrap_or(SURFACE_CONTENT_WIDTH);
        let rendered = SurfaceSvgRasterizer::rasterize(&svg, max_width)?;
        let image = scaled_image(rendered.image, requested_width);
        Some(Self { image, _alt: alt })
    }

    pub(crate) fn height(&self) -> u32 {
        self.image.height() + super::super::IMAGE_VERTICAL_MARGIN * 2
    }

    #[cfg(test)]
    pub(crate) fn alt_for_tests(&self) -> String {
        self._alt.clone()
    }
}

fn is_svg_path(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("svg"))
}

fn scaled_image(image: RgbaImage, requested_width: Option<u32>) -> RgbaImage {
    let max_width = requested_width
        .unwrap_or(image.width())
        .min(SURFACE_CONTENT_WIDTH);
    if image.width() <= max_width {
        return image;
    }
    let height = (image.height() as f32 * max_width as f32 / image.width() as f32)
        .round()
        .max(1.0) as u32;
    image::imageops::resize(
        &image,
        max_width,
        height,
        image::imageops::FilterType::Lanczos3,
    )
}
