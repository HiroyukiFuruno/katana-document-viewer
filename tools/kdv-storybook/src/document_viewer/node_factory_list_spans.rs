use katana_document_viewer::{ViewerTextSpan, ViewerTextStyle};

pub(super) struct ListSpanRows;

impl ListSpanRows {
    pub(super) fn body_text_spans(body: &str, spans: Vec<ViewerTextSpan>) -> Vec<ViewerTextSpan> {
        if spans.is_empty() {
            return vec![ViewerTextSpan {
                text: body.to_string(),
                style: ViewerTextStyle::default(),
                link_target: String::new(),
            }];
        }
        spans
    }

    pub(super) fn split_by_line(spans: &[ViewerTextSpan]) -> Vec<Vec<ViewerTextSpan>> {
        let mut rows = vec![Vec::new()];
        for span in spans {
            push_span_lines(span, &mut rows);
        }
        rows.into_iter()
            .filter(|row| !Self::text(row).is_empty())
            .collect()
    }

    pub(super) fn after_offset(spans: Vec<ViewerTextSpan>, offset: usize) -> Vec<ViewerTextSpan> {
        let mut remaining = offset;
        let mut values = Vec::new();
        for span in spans {
            push_span_after_offset(span, &mut remaining, &mut values);
        }
        values
    }

    pub(super) fn text(spans: &[ViewerTextSpan]) -> String {
        spans.iter().map(|span| span.text.as_str()).collect()
    }
}

fn push_span_lines(span: &ViewerTextSpan, rows: &mut Vec<Vec<ViewerTextSpan>>) {
    let parts = span.text.split('\n').collect::<Vec<_>>();
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            rows.push(Vec::new());
        }
        push_nonempty_span_part(span, part, rows);
    }
}

fn push_nonempty_span_part(span: &ViewerTextSpan, part: &str, rows: &mut [Vec<ViewerTextSpan>]) {
    if part.is_empty() {
        return;
    }
    let mut next = span.clone();
    next.text = part.to_string();
    if let Some(row) = rows.last_mut() {
        row.push(next);
    }
}

fn push_span_after_offset(
    mut span: ViewerTextSpan,
    remaining: &mut usize,
    values: &mut Vec<ViewerTextSpan>,
) {
    if *remaining >= span.text.len() {
        *remaining -= span.text.len();
        return;
    }
    if *remaining > 0 {
        span.text = span.text[*remaining..].to_string();
        *remaining = 0;
    }
    if !span.text.is_empty() {
        values.push(span);
    }
}
