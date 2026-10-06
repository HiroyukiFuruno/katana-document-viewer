use super::super::PersistentCacheError;
use super::super::payload_codec::PayloadCodec;
use crate::{PdfRenderedPage, ViewerImageSurface};

pub(super) struct PageCodec;

impl PageCodec {
    pub(super) fn encode(page: &PdfRenderedPage) -> Result<Vec<u8>, PersistentCacheError> {
        let surface = &page.surface;
        let metadata = PdfRenderedPage {
            page_index: page.page_index,
            scale: page.scale,
            surface: ViewerImageSurface {
                fingerprint: surface.fingerprint.clone(),
                width: surface.width,
                height: surface.height,
                display_width: surface.display_width,
                display_height: surface.display_height,
                content_scale: surface.content_scale,
                rgba: Vec::new(),
            },
        };
        PayloadCodec::encode(&metadata, &surface.rgba)
    }

    pub(super) fn decode(bytes: &[u8]) -> Result<PdfRenderedPage, PersistentCacheError> {
        let (mut page, rgba): (PdfRenderedPage, _) = PayloadCodec::decode(bytes)?;
        if !page.surface.rgba.is_empty() {
            return Err(PersistentCacheError::Corrupt);
        }
        page.surface.rgba = rgba.to_vec();
        Ok(page)
    }
}
