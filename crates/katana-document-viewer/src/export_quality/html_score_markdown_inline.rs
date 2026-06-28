use crate::export_quality::markdown_link_source::MarkdownLinkSource;

pub(super) fn evaluates_inline_markdown(html: &str, source: &str) -> bool {
    let requirements = InlineRequirements::from_source(source);
    (!requirements.strong || html.contains("<strong>"))
        && (!requirements.emphasis || html.contains("<em>"))
        && (!requirements.strikethrough || html.contains("<s>") || html.contains("<del>"))
        && (!requirements.link || html_contains_anchor_with_href(html))
        && (!requirements.inline_code || html.contains("<code"))
}

struct InlineRequirements {
    strong: bool,
    emphasis: bool,
    strikethrough: bool,
    link: bool,
    inline_code: bool,
}

impl InlineRequirements {
    fn from_source(source: &str) -> Self {
        Self {
            strong: requires_strong(source),
            emphasis: requires_emphasis(source),
            strikethrough: requires_strikethrough(source),
            link: contains_markdown_link(source),
            inline_code: requires_inline_code(source),
        }
    }
}

fn contains_markdown_link(source: &str) -> bool {
    MarkdownLinkSource::contains_markdown_link(source) || source_contains_html_anchor(source)
}

fn source_contains_html_anchor(source: &str) -> bool {
    let source = source_lines_outside_fences(source).join("\n");
    html_contains_anchor_with_href(&source)
}

fn html_contains_anchor_with_href(html: &str) -> bool {
    let lower = html.to_ascii_lowercase();
    let mut rest = lower.as_str();
    while let Some(anchor_start) = rest.find("<a") {
        let after_anchor = &rest[anchor_start + 2..];
        if !tag_starts_with_boundary(after_anchor) {
            rest = &rest[anchor_start + 2..];
            continue;
        }
        let Some(tag_end) = after_anchor.find('>') else {
            return false;
        };
        if tag_has_href_attribute(&after_anchor[..tag_end]) {
            return true;
        }
        rest = &after_anchor[tag_end + 1..];
    }
    false
}

fn tag_starts_with_boundary(rest: &str) -> bool {
    if rest.starts_with('>') {
        return true;
    }
    rest.chars().next().is_some_and(char::is_whitespace)
}

fn tag_has_href_attribute(tag: &str) -> bool {
    let mut rest = tag;
    while let Some(href_start) = rest.find("href") {
        let before = href_start
            .checked_sub(1)
            .and_then(|index| rest.as_bytes().get(index))
            .copied();
        let after = rest[href_start + 4..].trim_start();
        if before.is_some_and(|value| value.is_ascii_whitespace()) && after.starts_with('=') {
            return true;
        }
        rest = &rest[href_start + 4..];
    }
    false
}

fn requires_strong(source: &str) -> bool {
    source_lines_outside_fences(source)
        .iter()
        .any(|line| contains_delimited_inline(line, "**") || contains_delimited_inline(line, "__"))
}

fn requires_emphasis(source: &str) -> bool {
    source_lines_outside_fences(source).iter().any(|line| {
        contains_single_delimited_inline(line, '*') || contains_single_delimited_inline(line, '_')
    })
}

fn requires_strikethrough(source: &str) -> bool {
    source_lines_outside_fences(source)
        .iter()
        .any(|line| contains_delimited_inline(line, "~~"))
}

fn requires_inline_code(source: &str) -> bool {
    source_lines_outside_fences(source)
        .iter()
        .any(|line| line.contains('`'))
}

fn source_lines_outside_fences(source: &str) -> Vec<&str> {
    let mut inside_fence = false;
    let mut lines = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            inside_fence = !inside_fence;
            continue;
        }
        if !inside_fence {
            lines.push(line);
        }
    }
    lines
}

fn contains_delimited_inline(line: &str, delimiter: &str) -> bool {
    let mut search_from = 0;
    while let Some(start_offset) = line[search_from..].find(delimiter) {
        let content_start = search_from + start_offset + delimiter.len();
        let Some(end_offset) = line[content_start..].find(delimiter) else {
            return false;
        };
        if !line[content_start..content_start + end_offset]
            .trim()
            .is_empty()
        {
            return true;
        }
        search_from = content_start + end_offset + delimiter.len();
    }
    false
}

fn contains_single_delimited_inline(line: &str, delimiter: char) -> bool {
    let bytes = line.as_bytes();
    for (index, character) in line.char_indices() {
        if character != delimiter || touches_same_delimiter(bytes, index, delimiter) {
            continue;
        }
        if single_delimiter_has_closing(line, index + delimiter.len_utf8(), delimiter) {
            return true;
        }
    }
    false
}

fn single_delimiter_has_closing(line: &str, start: usize, delimiter: char) -> bool {
    for (offset, character) in line[start..].char_indices() {
        let index = start + offset;
        if character == delimiter
            && !touches_same_delimiter(line.as_bytes(), index, delimiter)
            && !line[start..index].trim().is_empty()
        {
            return true;
        }
    }
    false
}

fn touches_same_delimiter(bytes: &[u8], index: usize, delimiter: char) -> bool {
    let marker = delimiter as u8;
    index
        .checked_sub(1)
        .is_some_and(|left| bytes[left] == marker)
        || bytes.get(index + 1).is_some_and(|right| *right == marker)
}

#[cfg(test)]
#[path = "html_score_markdown_inline_tests.rs"]
mod tests;
