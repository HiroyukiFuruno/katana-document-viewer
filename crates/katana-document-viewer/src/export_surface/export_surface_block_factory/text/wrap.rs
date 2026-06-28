use crate::export_surface_line::BODY_FONT_SIZE;
use crate::export_surface_span::SurfaceTextSpan;

const MIN_LINE_WRAP_WIDTH: u32 = 120;

pub(in crate::export_surface::export_surface_block_factory) struct SurfaceInlineLineWrapper;

impl SurfaceInlineLineWrapper {
    pub(in crate::export_surface::export_surface_block_factory) fn wrap(
        spans: Vec<SurfaceTextSpan>,
        max_width: u32,
    ) -> Vec<Vec<SurfaceTextSpan>> {
        let max_width = max_width.max(MIN_LINE_WRAP_WIDTH);
        let mut state = LineWrapState::new(max_width);
        for segment in spans
            .into_iter()
            .flat_map(|span| Self::segments(span, max_width))
        {
            state.push(segment);
        }
        state.finish()
    }

    fn segments(span: SurfaceTextSpan, max_width: u32) -> Vec<SurfaceTextSpan> {
        if span.inline_image.is_some() {
            return vec![span];
        }
        Self::text_segments(span, max_width)
    }

    fn text_segments(span: SurfaceTextSpan, max_width: u32) -> Vec<SurfaceTextSpan> {
        let mut segments = Vec::new();
        let mut current = String::new();
        for character in span.text.chars() {
            current.push(character);
            if character.is_whitespace() {
                Self::push_width_segments(
                    &mut segments,
                    &span,
                    std::mem::take(&mut current),
                    max_width,
                );
            }
        }
        if !current.is_empty() {
            Self::push_width_segments(&mut segments, &span, current, max_width);
        }
        segments
    }

    fn push_width_segments(
        segments: &mut Vec<SurfaceTextSpan>,
        span: &SurfaceTextSpan,
        text: String,
        max_width: u32,
    ) {
        if Self::estimated_text_width(span, &text) <= max_width {
            segments.push(Self::with_text(span, text));
            return;
        }
        Self::push_character_chunks(segments, span, text, max_width);
    }

    fn push_character_chunks(
        segments: &mut Vec<SurfaceTextSpan>,
        span: &SurfaceTextSpan,
        text: String,
        max_width: u32,
    ) {
        let mut current = String::new();
        for character in text.chars() {
            Self::push_character(segments, span, &mut current, character, max_width);
        }
        if !current.is_empty() {
            segments.push(Self::with_text(span, current));
        }
    }

    fn push_character(
        segments: &mut Vec<SurfaceTextSpan>,
        span: &SurfaceTextSpan,
        current: &mut String,
        character: char,
        max_width: u32,
    ) {
        let candidate = format!("{current}{character}");
        if !current.is_empty() && Self::estimated_text_width(span, &candidate) > max_width {
            segments.push(Self::with_text(span, std::mem::take(current)));
        }
        current.push(character);
    }

    fn estimated_text_width(span: &SurfaceTextSpan, text: &str) -> u32 {
        Self::with_text(span, text.to_string()).estimated_width(BODY_FONT_SIZE)
    }

    fn with_text(span: &SurfaceTextSpan, text: String) -> SurfaceTextSpan {
        let mut segment = span.clone();
        segment.text = text;
        segment
    }
}

struct LineWrapState {
    max_width: u32,
    lines: Vec<Vec<SurfaceTextSpan>>,
    current_line: Vec<SurfaceTextSpan>,
    current_width: u32,
}

impl LineWrapState {
    fn new(max_width: u32) -> Self {
        Self {
            max_width,
            lines: Vec::new(),
            current_line: Vec::new(),
            current_width: 0,
        }
    }

    fn push(&mut self, segment: SurfaceTextSpan) {
        let segment_width = segment.estimated_width(BODY_FONT_SIZE);
        if self.should_start_new_line(segment_width) {
            self.start_new_line();
        }
        if self.should_skip_leading_space(&segment) {
            return;
        }
        self.current_width += segment_width;
        self.current_line.push(segment);
    }

    fn finish(mut self) -> Vec<Vec<SurfaceTextSpan>> {
        self.start_new_line();
        if self.lines.is_empty() {
            vec![vec![SurfaceTextSpan::plain(String::new())]]
        } else {
            self.lines
        }
    }

    fn should_start_new_line(&self, segment_width: u32) -> bool {
        self.current_width > 0 && self.current_width + segment_width > self.max_width
    }

    fn should_skip_leading_space(&self, segment: &SurfaceTextSpan) -> bool {
        self.current_width == 0 && segment.text.trim().is_empty()
    }

    fn start_new_line(&mut self) {
        if self.current_line.is_empty() {
            return;
        }
        self.lines.push(std::mem::take(&mut self.current_line));
        self.current_width = 0;
    }
}
