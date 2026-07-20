use super::SpanTextWidthMeasurer;
use crate::{ViewerTextSpan, ViewerTextStyle};
use std::cell::RefCell;
use std::collections::HashMap;

const WIDTH_CACHE_LIMIT: usize = 4096;

thread_local! {
    static SPAN_WIDTH_CACHE: RefCell<SpanTextWidthCache> =
        RefCell::new(SpanTextWidthCache::new());
}

pub(super) fn cached_width(span: &ViewerTextSpan, text: &str, font_size: f32) -> u32 {
    SPAN_WIDTH_CACHE.with(|cache| cache.borrow_mut().width(span, text, font_size))
}

struct SpanTextWidthCache {
    measurer: SpanTextWidthMeasurer,
    widths: HashMap<SpanTextWidthCacheKey, u32>,
}

impl SpanTextWidthCache {
    fn new() -> Self {
        Self {
            measurer: SpanTextWidthMeasurer::new(),
            widths: HashMap::new(),
        }
    }

    fn width(&mut self, span: &ViewerTextSpan, text: &str, font_size: f32) -> u32 {
        let key = SpanTextWidthCacheKey::new(text, span.style, font_size);
        if let Some(width) = self.widths.get(&key) {
            return *width;
        }
        if self.widths.len() >= WIDTH_CACHE_LIMIT {
            self.widths.clear();
        }
        let width = self.measurer.width(span, text, font_size);
        self.widths.insert(key, width);
        width
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SpanTextWidthCacheKey {
    text: String,
    style: ViewerTextStyle,
    font_size_bits: u32,
}

impl SpanTextWidthCacheKey {
    fn new(text: &str, style: ViewerTextStyle, font_size: f32) -> Self {
        Self {
            text: text.to_string(),
            style,
            font_size_bits: font_size.to_bits(),
        }
    }
}

#[cfg(test)]
#[path = "builder_span_text_width_cache_tests.rs"]
mod tests;
