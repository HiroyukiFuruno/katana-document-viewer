#[derive(Clone, Copy, Debug)]
pub(super) struct SpanVisualRange {
    start_x: u32,
    end_x: u32,
}

impl SpanVisualRange {
    pub(super) fn new(start_x: f32, end_x: f32) -> Self {
        Self {
            start_x: start_x.floor().max(0.0) as u32,
            end_x: end_x.ceil().max(0.0) as u32,
        }
    }

    fn merge(self, start_x: f32, end_x: f32) -> Self {
        Self::new(
            (self.start_x as f32).min(start_x),
            (self.end_x as f32).max(end_x),
        )
    }

    pub(super) fn width(self) -> u32 {
        self.end_x.saturating_sub(self.start_x).max(1)
    }

    pub(super) fn start_x(&self) -> u32 {
        self.start_x
    }
}

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
        let end_x = glyph.w + start_x;
        ranges[span_index] = Some(match ranges[span_index] {
            Some(range) => range.merge(start_x, end_x),
            None => SpanVisualRange::new(start_x, end_x),
        });
    }
}
