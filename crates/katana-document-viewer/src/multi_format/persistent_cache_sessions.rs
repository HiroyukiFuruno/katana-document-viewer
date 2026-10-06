use super::{PersistentCacheError, PersistentDocumentCache, key::CacheKey};
use crate::multi_format::{
    BinaryDocumentSource, OfficeStaticViewerSession, PdfPageRenderRequest, PdfRenderedPage,
    PdfViewerLimits, PdfViewerSession,
};

#[path = "persistent_cache_office.rs"]
mod office;

#[path = "persistent_cache_page_codec.rs"]
mod page_codec;

pub struct PersistentOfficeViewerSession {
    session: OfficeStaticViewerSession,
    cache: PersistentDocumentCache,
    key: String,
    conversion_cache_hit: bool,
}

pub struct PersistentPdfViewerSession {
    session: PdfViewerSession,
    cache: PersistentDocumentCache,
    key: String,
    limits: PdfViewerLimits,
}

impl PersistentPdfViewerSession {
    pub fn open(
        source: BinaryDocumentSource,
        limits: PdfViewerLimits,
        cache: PersistentDocumentCache,
    ) -> Result<Self, PersistentCacheError> {
        let key = CacheKey::pdf(&cache, &source);
        let session = PdfViewerSession::open_with_limits(source, limits)?;
        Ok(Self {
            session,
            cache,
            key,
            limits,
        })
    }

    #[must_use]
    pub const fn artifact(&self) -> &crate::multi_format::PdfDocumentArtifact {
        self.session.artifact()
    }

    #[must_use]
    pub fn outline(&self) -> &[crate::PdfOutlineItem] {
        self.session.outline()
    }

    pub fn render_page(
        &mut self,
        request: PdfPageRenderRequest,
    ) -> Result<(PdfRenderedPage, bool), PersistentCacheError> {
        self.session.validate_request(request)?;
        let key = CacheKey::page(
            &self.key,
            request.page_index,
            request.scale,
            &format!("{:?}", self.limits),
        );
        if let Some(page) = CachedPage::load(&self.cache, &key, request, self.limits)? {
            return Ok((page, true));
        }
        let page = self.session.render_page(request)?;
        CachedPage::save(&self.cache, &key, &page)?;
        Ok((page, false))
    }
}

struct CachedPage;

impl CachedPage {
    fn load(
        cache: &PersistentDocumentCache,
        key: &str,
        request: PdfPageRenderRequest,
        limits: PdfViewerLimits,
    ) -> Result<Option<PdfRenderedPage>, PersistentCacheError> {
        let Some(bytes) = cache.load(key)? else {
            return Ok(None);
        };
        let page = page_codec::PageCodec::decode(&bytes)?;
        let surface = &page.surface;
        let pixels = u64::from(surface.width) * u64::from(surface.height);
        if page.page_index != request.page_index
            || page.scale.to_bits() != request.scale.to_bits()
            || pixels == 0
            || pixels > limits.max_render_pixels
            || surface.width.max(surface.height) > limits.max_render_dimension
            || pixels.saturating_mul(4) != surface.rgba.len() as u64
            || !surface.display_width.is_finite()
            || !surface.display_height.is_finite()
            || surface.display_width <= 0.0
            || surface.display_height <= 0.0
        {
            return Err(PersistentCacheError::Corrupt);
        }
        Ok(Some(page))
    }

    fn save(
        cache: &PersistentDocumentCache,
        key: &str,
        page: &PdfRenderedPage,
    ) -> Result<(), PersistentCacheError> {
        let bytes = page_codec::PageCodec::encode(page)?;
        cache.save(key, &bytes)
    }
}

#[cfg(test)]
#[path = "persistent_cache_sessions_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "persistent_cache_sessions_overflow_tests.rs"]
mod overflow_tests;

#[cfg(test)]
#[path = "persistent_cache_accessor_tests.rs"]
mod accessor_tests;

#[cfg(test)]
#[path = "persistent_cache_page_binary_tests.rs"]
mod page_binary_tests;
