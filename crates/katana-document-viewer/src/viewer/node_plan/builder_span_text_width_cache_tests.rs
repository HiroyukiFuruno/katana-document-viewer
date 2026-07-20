use super::{SpanTextWidthCache, SpanTextWidthCacheKey, WIDTH_CACHE_LIMIT};
use crate::{ViewerTextSpan, ViewerTextStyle};

#[test]
fn width_cache_clears_before_inserting_past_capacity() {
    let mut cache = SpanTextWidthCache::new();
    for index in 0..WIDTH_CACHE_LIMIT {
        cache.widths.insert(
            SpanTextWidthCacheKey::new(
                &format!("cached-{index}"),
                ViewerTextStyle::default(),
                16.0,
            ),
            1,
        );
    }

    let span = ViewerTextSpan::plain("new-entry");
    assert!(cache.width(&span, &span.text, 16.5) > 0);
    assert_eq!(1, cache.widths.len());
}
