use super::super::PersistentCacheError;
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
        let json = serde_json::to_vec(&metadata).map_err(|_| PersistentCacheError::Corrupt)?;
        let mut bytes = Vec::with_capacity(8 + json.len() + surface.rgba.len());
        bytes.extend_from_slice(&(json.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&json);
        bytes.extend_from_slice(&surface.rgba);
        Ok(bytes)
    }

    pub(super) fn decode(bytes: &[u8]) -> Result<PdfRenderedPage, PersistentCacheError> {
        let prefix = bytes.get(..8).ok_or(PersistentCacheError::Corrupt)?;
        let length = u64::from_le_bytes(
            prefix
                .try_into()
                .map_err(|_| PersistentCacheError::Corrupt)?,
        );
        let end = usize::try_from(length)
            .ok()
            .and_then(|n| n.checked_add(8))
            .ok_or(PersistentCacheError::Corrupt)?;
        let metadata = bytes.get(8..end).ok_or(PersistentCacheError::Corrupt)?;
        let mut page: PdfRenderedPage =
            serde_json::from_slice(metadata).map_err(|_| PersistentCacheError::Corrupt)?;
        if !page.surface.rgba.is_empty() {
            return Err(PersistentCacheError::Corrupt);
        }
        page.surface.rgba = bytes[end..].to_vec();
        Ok(page)
    }
}
