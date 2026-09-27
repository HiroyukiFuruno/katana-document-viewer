use katana_markdown_model::SourceSpan;

pub(crate) struct MarkdownLineBreak;

impl MarkdownLineBreak {
    pub(crate) fn can_merge_soft_paragraph_sources(
        previous: &SourceSpan,
        next: &SourceSpan,
    ) -> bool {
        !is_image_paragraph_source(previous)
            && !is_image_paragraph_source(next)
            && !ends_with_hard_line_break(&previous.raw.text)
            && previous.line_column_range.end.line + 1 == next.line_column_range.start.line
    }
}

fn is_image_paragraph_source(source: &SourceSpan) -> bool {
    source.raw.text.trim_start().starts_with("![")
}

fn ends_with_hard_line_break(source: &str) -> bool {
    let source = source.trim_end_matches(['\r', '\n']);
    let trailing_backslashes = source
        .chars()
        .rev()
        .take_while(|character| *character == '\\')
        .count();
    trailing_backslashes % 2 == 1 || source.ends_with("  ")
}
