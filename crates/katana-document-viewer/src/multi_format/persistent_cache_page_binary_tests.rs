use super::CachedPage;
use crate::ViewerImageSurface;
use crate::multi_format::{
    PdfPageRenderRequest, PdfRenderedPage, PdfViewerLimits, PersistentCacheError,
    PersistentDocumentCache,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn large_rgba_page_round_trips_within_the_strict_cache_capacity() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(
        root.path().join("cache"),
        128 * 1024 * 1024,
        "page-binary-v1".into(),
    )?;
    let request = PdfPageRenderRequest::new(0, 1.0);
    let key = super::super::key::CacheKey::page("large-rgba", 0, 1.0, "strict");
    let expected = large_white_page();

    CachedPage::save(&cache, &key, &expected)?;
    let loaded = CachedPage::load(&cache, &key, request, PdfViewerLimits::strict())?
        .ok_or("large page cache entry is missing")?;

    assert_eq!(expected.page_index, loaded.page_index);
    assert_eq!(expected.scale, loaded.scale);
    assert_eq!(expected.surface.width, loaded.surface.width);
    assert_eq!(expected.surface.height, loaded.surface.height);
    assert_eq!(expected.surface.rgba, loaded.surface.rgba);
    Ok(())
}

fn large_white_page() -> PdfRenderedPage {
    let side = 4096_usize;
    PdfRenderedPage {
        page_index: 0,
        scale: 1.0,
        surface: ViewerImageSurface {
            fingerprint: "large-white".into(),
            width: side as u32,
            height: side as u32,
            display_width: side as f32,
            display_height: side as f32,
            content_scale: 100,
            rgba: vec![255; side * side * 4],
        },
    }
}

fn assert_corrupt(cache: &PersistentDocumentCache, key: &str, bytes: &[u8]) -> TestResult {
    cache.save(key, bytes)?;
    assert!(matches!(
        CachedPage::load(
            cache,
            key,
            PdfPageRenderRequest::new(0, 1.0),
            PdfViewerLimits::strict()
        ),
        Err(PersistentCacheError::Corrupt)
    ));
    Ok(())
}

#[test]
fn malformed_page_codec_payloads_are_rejected() -> TestResult {
    let root = tempfile::tempdir()?;
    let cache = PersistentDocumentCache::new(root.path().join("cache"), 1024, "codec-v1".into())?;
    assert_corrupt(&cache, "truncated", &[0; 7])?;
    assert_corrupt(&cache, "overflow", &u64::MAX.to_le_bytes())?;
    assert_corrupt(&cache, "missing-metadata", &1_u64.to_le_bytes())?;
    assert_corrupt(&cache, "metadata", &[1, 0, 0, 0, 0, 0, 0, 0, b'{'])?;

    let embedded = PdfRenderedPage {
        page_index: 0,
        scale: 1.0,
        surface: ViewerImageSurface {
            fingerprint: "embedded".into(),
            width: 1,
            height: 1,
            display_width: 1.0,
            display_height: 1.0,
            content_scale: 100,
            rgba: vec![255; 4],
        },
    };
    let metadata = serde_json::to_vec(&embedded)?;
    let mut bytes = (metadata.len() as u64).to_le_bytes().to_vec();
    bytes.extend_from_slice(&metadata);
    assert_corrupt(&cache, "embedded-rgba", &bytes)
}
