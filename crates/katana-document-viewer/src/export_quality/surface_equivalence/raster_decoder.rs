use super::{DecodedRgbSurface, SurfaceRgbConverter};
use image::GenericImageView;

pub(super) struct RasterSurfaceDecoder;

impl RasterSurfaceDecoder {
    pub(super) fn decode(bytes: &[u8]) -> Result<DecodedRgbSurface, String> {
        let image = image::load_from_memory(bytes).map_err(|error| error.to_string())?;
        let (width, height) = image.dimensions();
        Ok(DecodedRgbSurface {
            width,
            height,
            rgb: SurfaceRgbConverter::rgba_to_rgb(&image.to_rgba8().into_raw())?,
        })
    }
}
