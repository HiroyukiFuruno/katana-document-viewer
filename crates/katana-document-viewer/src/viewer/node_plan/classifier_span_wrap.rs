use super::super::super::types::ViewerTextSpan;
use crate::export_surface_helpers::{BODY_MAX_CHARS, WrappedText};

pub(super) struct ViewerSpanWrapper;

impl ViewerSpanWrapper {
    pub(super) fn wrap_plain_surface_text(text: String) -> Vec<ViewerTextSpan> {
        let mut wrapped = Vec::new();
        for (line_index, line) in text.split('\n').enumerate() {
            if line_index > 0 {
                wrapped.push(ViewerTextSpan::plain("\n"));
            }
            Self::push_wrapped_line(&mut wrapped, line);
        }
        wrapped
    }

    fn push_wrapped_line(wrapped: &mut Vec<ViewerTextSpan>, line: &str) {
        for (chunk_index, chunk) in WrappedText::new(line, BODY_MAX_CHARS).enumerate() {
            if chunk_index > 0 {
                wrapped.push(ViewerTextSpan::plain("\n"));
            }
            wrapped.push(ViewerTextSpan::plain(chunk));
        }
    }
}

#[cfg(test)]
#[path = "classifier_span_wrap_tests.rs"]
mod tests;
