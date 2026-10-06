use super::super::{PersistentCacheError, PersistentDocumentCache, key::CacheKey};
use super::CachedPage;
use crate::{PdfPageRenderRequest, PdfRenderedPage, PdfViewerLimits, ViewerImageSurface};

#[test]
fn corrupt_surface_dimensions_cannot_overflow_rgba_length() -> Result<(), Box<dyn std::error::Error>>
{
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(root.path().join("cache"), 1024, "one".into())?;
    let page = invalid_page();
    let key = CacheKey::page("overflow", 0, 1.0, "large-limits");
    cache.save(&key, &serde_json::to_vec(&page)?)?;
    let mut limits = PdfViewerLimits::strict();
    limits.max_render_pixels = u64::MAX;
    limits.max_render_dimension = u32::MAX;
    assert!(matches!(
        CachedPage::load(&cache, &key, PdfPageRenderRequest::new(0, 1.0), limits),
        Err(PersistentCacheError::Corrupt)
    ));
    Ok(())
}

fn invalid_page() -> PdfRenderedPage {
    PdfRenderedPage {
        page_index: 0,
        scale: 1.0,
        surface: ViewerImageSurface {
            fingerprint: "overflow".into(),
            width: 1 << 31,
            height: 1 << 31,
            display_width: 1.0,
            display_height: 1.0,
            content_scale: 100,
            rgba: Vec::new(),
        },
    }
}
