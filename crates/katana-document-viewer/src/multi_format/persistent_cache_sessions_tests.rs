use super::super::{PersistentCacheError, PersistentDocumentCache, key::CacheKey};
use super::CachedPage;
use crate::ViewerImageSurface;
use crate::multi_format::{PdfPageRenderRequest, PdfRenderedPage, PdfViewerLimits};
use std::fs;
type TestResult = Result<(), Box<dyn std::error::Error>>;
fn cache() -> Result<(tempfile::TempDir, PersistentDocumentCache), Box<dyn std::error::Error>> {
    let root = tempfile::tempdir()?;
    let cache =
        PersistentDocumentCache::new(root.path().join("cache"), 1024 * 1024, "env-v1".into())?;
    Ok((root, cache))
}
fn page(index: usize, scale: f32, width: u32, height: u32) -> PdfRenderedPage {
    PdfRenderedPage {
        page_index: index,
        scale,
        surface: ViewerImageSurface {
            fingerprint: format!("page-{index}"),
            width,
            height,
            display_width: width as f32,
            display_height: height as f32,
            content_scale: 100,
            rgba: vec![0; width as usize * height as usize * 4],
        },
    }
}
fn key(index: usize, scale: f32) -> String {
    CacheKey::page("document", index, scale, "strict")
}
fn save_json(cache: &PersistentDocumentCache, key: &str, page: &PdfRenderedPage) -> TestResult {
    cache.save(key, &serde_json::to_vec(page)?)?;
    Ok(())
}
fn assert_corrupt(
    cache: &PersistentDocumentCache,
    key: &str,
    page: PdfRenderedPage,
    request: PdfPageRenderRequest,
    limits: PdfViewerLimits,
) -> TestResult {
    save_json(cache, key, &page)?;
    assert!(matches!(
        CachedPage::load(cache, key, request, limits),
        Err(PersistentCacheError::Corrupt)
    ));
    Ok(())
}
fn assert_corrupt_bytes(
    cache: &PersistentDocumentCache,
    key: &str,
    bytes: &[u8],
    request: PdfPageRenderRequest,
    limits: PdfViewerLimits,
) -> TestResult {
    cache.save(key, bytes)?;
    assert!(matches!(
        CachedPage::load(cache, key, request, limits),
        Err(PersistentCacheError::Corrupt)
    ));
    Ok(())
}
#[test]
fn cached_page_round_trips_and_missing_is_a_miss() -> TestResult {
    let (_root, cache) = cache()?;
    let request = PdfPageRenderRequest::new(2, 0.5);
    let limits = PdfViewerLimits::strict();
    let key = key(request.page_index, request.scale);
    assert_eq!(None, CachedPage::load(&cache, &key, request, limits)?);
    let expected = page(request.page_index, request.scale, 2, 1);
    CachedPage::save(&cache, &key, &expected)?;
    assert_eq!(
        Some(expected),
        CachedPage::load(&cache, &key, request, limits)?
    );
    Ok(())
}
#[test]
fn invalid_json_is_corrupt() -> TestResult {
    let (_root, cache) = cache()?;
    let key = key(0, 1.0);
    cache.save(&key, b"not-json")?;
    assert!(matches!(
        CachedPage::load(
            &cache,
            &key,
            PdfPageRenderRequest::new(0, 1.0),
            PdfViewerLimits::strict()
        ),
        Err(PersistentCacheError::Corrupt)
    ));
    Ok(())
}
#[test]
fn request_and_surface_shape_mismatches_are_corrupt() -> TestResult {
    let (_root, cache) = cache()?;
    let limits = PdfViewerLimits::strict();
    assert_corrupt(
        &cache,
        &key(1, 1.0),
        page(0, 1.0, 1, 1),
        PdfPageRenderRequest::new(1, 1.0),
        limits,
    )?;
    assert_corrupt(
        &cache,
        &format!("scale-{}", key(0, 1.0)),
        page(0, 2.0, 1, 1),
        PdfPageRenderRequest::new(0, 1.0),
        limits,
    )?;
    let mut rgba = page(0, 1.0, 1, 1);
    rgba.surface.rgba.clear();
    assert_corrupt(
        &cache,
        &format!("rgba-{}", key(0, 1.0)),
        rgba,
        PdfPageRenderRequest::new(0, 1.0),
        limits,
    )?;
    Ok(())
}
#[test]
fn zero_display_values_are_corrupt() -> TestResult {
    let (_root, cache) = cache()?;
    let limits = PdfViewerLimits::strict();
    for (name, width, height) in [("zero-width", 0.0, 1.0), ("zero-height", 1.0, 0.0)] {
        let mut value = page(0, 1.0, 1, 1);
        value.surface.display_width = width;
        value.surface.display_height = height;
        assert_corrupt(
            &cache,
            &format!("{name}-{}", key(0, 1.0)),
            value,
            PdfPageRenderRequest::new(0, 1.0),
            limits,
        )?;
    }
    Ok(())
}
#[test]
fn nonfinite_display_values_are_corrupt() -> TestResult {
    let (_root, cache) = cache()?;
    let limits = PdfViewerLimits::strict();
    for (name, width, height) in [
        ("nan-width", "NaN", "1.0"),
        ("inf-height", "1.0", "Infinity"),
    ] {
        let bytes = format!(
            r#"{{"page_index":0,"scale":1.0,"surface":{{"fingerprint":"x","width":1,"height":1,"display_width":{},"display_height":{},"content_scale":100,"rgba":[0,0,0,0]}}}}"#,
            width, height,
        );
        assert_corrupt_bytes(
            &cache,
            &format!("{name}-{}", key(0, 1.0)),
            bytes.as_bytes(),
            PdfPageRenderRequest::new(0, 1.0),
            limits,
        )?;
    }
    Ok(())
}
#[test]
fn custom_dimension_and_pixel_limits_are_enforced() -> TestResult {
    let (_root, cache) = cache()?;
    let request = PdfPageRenderRequest::new(0, 1.0);
    let mut limits = PdfViewerLimits::strict();
    limits.max_render_dimension = 1;
    assert_corrupt(&cache, &key(0, 1.0), page(0, 1.0, 2, 1), request, limits)?;
    let mut limits = PdfViewerLimits::strict();
    limits.max_render_pixels = 3;
    assert_corrupt(
        &cache,
        &format!("pixels-{}", key(0, 1.0)),
        page(0, 1.0, 2, 2),
        request,
        limits,
    )?;
    let mut zero_pixels = page(0, 1.0, 1, 1);
    zero_pixels.surface.width = 0;
    assert_corrupt(
        &cache,
        &format!("zero-{}", key(0, 1.0)),
        zero_pixels,
        request,
        PdfViewerLimits::strict(),
    )?;
    Ok(())
}
#[test]
fn saved_artifact_is_real_json_backed_by_cache_key() -> TestResult {
    let (_root, cache) = cache()?;
    let key = key(0, 1.0);
    let value = page(0, 1.0, 1, 1);
    CachedPage::save(&cache, &key, &value)?;
    let bytes = fs::read(cache.directory().join(&key))?;
    assert!(serde_json::from_slice::<PdfRenderedPage>(&bytes[72..]).is_ok());
    Ok(())
}
