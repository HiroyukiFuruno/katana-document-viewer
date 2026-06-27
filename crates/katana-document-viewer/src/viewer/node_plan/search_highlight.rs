use super::types::ViewerTextSpan;
use crate::{ViewerSearchState, ViewerSearchTextMatch, ViewerSearchTextMatcher, ViewerTextRange};
use katana_markdown_model::KmmNodeId;

pub(super) struct ViewerSearchHighlighter;

impl ViewerSearchHighlighter {
    pub(super) fn highlight(
        spans: Vec<ViewerTextSpan>,
        query: &str,
        current_range: Option<ViewerTextRange>,
    ) -> Vec<ViewerTextSpan> {
        if query.is_empty() {
            return spans;
        }
        let mut offset = 0;
        let mut output = Vec::new();
        for span in spans {
            let span_len = span.text.len();
            output.extend(Self::highlight_span(span, query, offset, current_range));
            offset += span_len;
        }
        output
    }

    pub(super) fn current_range(
        state: &ViewerSearchState,
        node_id: &KmmNodeId,
    ) -> Option<ViewerTextRange> {
        let current = state.matches.get(state.current_index?)?;
        (&current.matched.node_id == node_id).then_some(current.matched.range)
    }

    fn highlight_span(
        span: ViewerTextSpan,
        query: &str,
        span_offset: usize,
        current_range: Option<ViewerTextRange>,
    ) -> Vec<ViewerTextSpan> {
        let matches = ViewerSearchTextMatcher::find(query, &span.text);
        if matches.is_empty() {
            return vec![span];
        }
        let mut output = Vec::new();
        let mut cursor = 0;
        for matched in matches {
            cursor = Self::push_match_parts(
                &mut output,
                &span,
                matched,
                cursor,
                span_offset,
                current_range,
            );
        }
        Self::push_tail(&mut output, &span, cursor);
        output
    }

    fn push_match_parts(
        output: &mut Vec<ViewerTextSpan>,
        span: &ViewerTextSpan,
        matched: ViewerSearchTextMatch,
        cursor: usize,
        span_offset: usize,
        current_range: Option<ViewerTextRange>,
    ) -> usize {
        if matched.start > cursor {
            output.push(Self::span_part(span, cursor, matched.start, false, false));
        }
        output.push(Self::span_part(
            span,
            matched.start,
            matched.end,
            true,
            Self::is_current_match(
                span_offset + matched.start,
                span_offset + matched.end,
                current_range,
            ),
        ));
        matched.end
    }

    fn push_tail(output: &mut Vec<ViewerTextSpan>, span: &ViewerTextSpan, cursor: usize) {
        if cursor < span.text.len() {
            output.push(Self::span_part(span, cursor, span.text.len(), false, false));
        }
    }

    fn is_current_match(start: usize, end: usize, current_range: Option<ViewerTextRange>) -> bool {
        matches!(current_range, Some(range) if range.start == start && range.end == end)
    }

    fn span_part(
        span: &ViewerTextSpan,
        start: usize,
        end: usize,
        highlighted: bool,
        current: bool,
    ) -> ViewerTextSpan {
        let mut style = span.style;
        if current {
            style = style.current_highlight();
        } else if highlighted {
            style = style.highlight();
        }
        ViewerTextSpan {
            text: span.text[start..end].to_string(),
            style,
            link_target: span.link_target.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::viewer::node_plan::types::ViewerTextStyle;

    #[test]
    fn highlights_search_query_inside_plain_span() {
        let spans = ViewerSearchHighlighter::highlight(
            vec![ViewerTextSpan::plain("alpha beta alpha")],
            "alpha",
            None,
        );

        assert_eq!(3, spans.len());
        assert!(spans[0].style.highlight);
        assert!(!spans[1].style.highlight);
        assert!(spans[2].style.highlight);
    }

    #[test]
    fn marks_current_search_range_separately() {
        let spans = ViewerSearchHighlighter::highlight(
            vec![ViewerTextSpan::plain("alpha beta alpha")],
            "alpha",
            Some(ViewerTextRange { start: 11, end: 16 }),
        );

        assert!(spans[0].style.highlight);
        assert!(!spans[0].style.current_highlight);
        assert!(spans[2].style.current_highlight);
    }

    #[test]
    fn keeps_link_target_when_highlighting_link_span() {
        let spans = ViewerSearchHighlighter::highlight(
            vec![ViewerTextSpan::linked(
                "open link",
                "https://example.com",
                ViewerTextStyle::default().link(),
            )],
            "link",
            None,
        );

        assert_eq!("https://example.com", spans[1].link_target);
        assert!(spans[1].style.highlight);
    }

    #[test]
    fn highlights_case_insensitive_search_query() {
        let spans =
            ViewerSearchHighlighter::highlight(vec![ViewerTextSpan::plain("Hello")], "hello", None);

        assert_eq!("Hello", spans[0].text);
        assert!(spans[0].style.highlight);
    }

    #[test]
    fn keeps_span_unchanged_when_query_not_found() {
        let input = ViewerTextSpan::plain("search target");
        let spans = ViewerSearchHighlighter::highlight(vec![input.clone()], "missing", None);

        assert_eq!(1, spans.len());
        assert_eq!(input.text, spans[0].text);
        assert_eq!(input.style, spans[0].style);
    }

    #[test]
    fn keeps_tail_text_after_match() {
        let spans = ViewerSearchHighlighter::highlight(
            vec![ViewerTextSpan::plain("meta data")],
            "eta",
            None,
        );

        assert_eq!(3, spans.len());
        assert_eq!("m", spans[0].text);
        assert!(spans[1].style.highlight);
        assert_eq!(" data", spans[2].text);
    }
}
