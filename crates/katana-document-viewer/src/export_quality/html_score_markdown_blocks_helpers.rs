pub(super) fn html_has_heading(html: &str) -> bool {
    html_heading_count(html) > 0
}

pub(super) fn html_heading_count(html: &str) -> usize {
    ["<h1", "<h2", "<h3", "<h4", "<h5", "<h6"]
        .iter()
        .map(|tag| html.match_indices(tag).count())
        .sum()
}

pub(super) fn html_has_list(html: &str) -> bool {
    (html.contains("<ul") || html.contains("<ol")) && html.contains("<li")
}

pub(super) fn html_list_item_count(html: &str) -> usize {
    html.match_indices("<li").count()
}

pub(super) fn html_has_nested_list(html: &str) -> bool {
    html.match_indices("<li").any(|(start, _)| {
        let segment = &html[start..];
        let end = segment.find("</li>").unwrap_or(segment.len());
        segment[..end].contains("<ul") || segment[..end].contains("<ol")
    })
}

pub(super) fn html_has_code(html: &str) -> bool {
    html.contains("<pre") && html.contains("<code")
}

pub(super) fn html_code_block_count(html: &str) -> usize {
    html.match_indices("<pre").count()
}

pub(super) fn html_table_count(html: &str) -> usize {
    html.match_indices("<table").count()
}

pub(super) fn is_external_block_fence(trimmed: &str) -> bool {
    let language = trimmed
        .trim_start_matches('`')
        .trim_start_matches('~')
        .trim()
        .to_ascii_lowercase();
    matches!(
        language.as_str(),
        "mermaid" | "plantuml" | "drawio" | "math" | "latex"
    )
}

pub(super) fn starts_with_bullet_list(trimmed: &str) -> bool {
    ["- ", "* ", "+ "]
        .iter()
        .any(|prefix| trimmed.starts_with(prefix))
}

pub(super) fn starts_with_ordered_list(trimmed: &str) -> bool {
    let Some(marker_index) = trimmed.find(['.', ')']) else {
        return false;
    };
    let (digits, rest) = trimmed.split_at(marker_index);
    (1..=9).contains(&digits.len())
        && digits.chars().all(|value| value.is_ascii_digit())
        && rest[1..].starts_with([' ', '\t'])
}
