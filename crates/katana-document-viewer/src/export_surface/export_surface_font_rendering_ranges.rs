use super::SpanVisualRange;

pub(super) fn span_visual_ranges(
    buffer: &cosmic_text::Buffer,
    span_count: usize,
) -> Vec<Option<SpanVisualRange>> {
    let mut ranges: Vec<Option<SpanVisualRange>> = vec![None; span_count];
    for run in buffer.layout_runs() {
        for glyph in run.glyphs {
            SpanVisualRangeCollector::collect_glyph(&mut ranges, span_count, glyph);
        }
    }
    ranges
}

struct SpanVisualRangeCollector;

impl SpanVisualRangeCollector {
    fn collect_glyph(
        ranges: &mut [Option<SpanVisualRange>],
        span_count: usize,
        glyph: &cosmic_text::LayoutGlyph,
    ) {
        if glyph.metadata == 0 {
            return;
        }
        let span_index = glyph.metadata.saturating_sub(1);
        if span_index >= span_count {
            return;
        }
        let start_x = glyph.x;
        let end_x = glyph.x + glyph.w;
        ranges[span_index] = Some(match ranges[span_index] {
            Some(range) => range.extend(start_x, end_x),
            None => SpanVisualRange::new(start_x, end_x),
        });
    }
}
