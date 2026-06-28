use crate::export_html_ops::ExportHtmlOps;

pub(super) fn append_autolinked_text(html: &mut String, text: &str) {
    let mut offset = 0;
    while let Some(relative_start) = next_url_start(&text[offset..]) {
        let start = offset + relative_start;
        html.push_str(&ExportHtmlOps::render_text(&text[offset..start]));
        let token_end = url_token_end(text, start);
        let token = &text[start..token_end];
        let (url, trailing) = split_trailing_url_punctuation(token);
        append_url_token(html, token, url, trailing);
        offset = token_end;
    }
    html.push_str(&ExportHtmlOps::render_text(&text[offset..]));
}

pub(super) fn next_url_start(text: &str) -> Option<usize> {
    match (text.find("https://"), text.find("http://")) {
        (Some(https), Some(http)) => Some(https.min(http)),
        (Some(https), None) => Some(https),
        (None, Some(http)) => Some(http),
        (None, None) => None,
    }
}

fn append_url_token(html: &mut String, token: &str, url: &str, trailing: &str) {
    if !url_has_scheme_body(url) {
        html.push_str(&ExportHtmlOps::render_text(token));
        return;
    }
    html.push_str(&format!(
        "<a href=\"{}\" data-kdv-autolink=\"true\">{}</a>",
        ExportHtmlOps::escape_html(url),
        ExportHtmlOps::render_text(url)
    ));
    html.push_str(&ExportHtmlOps::render_text(trailing));
}

fn url_token_end(text: &str, start: usize) -> usize {
    text[start..]
        .char_indices()
        .find_map(|(index, character)| {
            if character.is_whitespace() || matches!(character, '<' | '>' | '"' | '\'') {
                Some(start + index)
            } else {
                None
            }
        })
        .unwrap_or(text.len())
}

fn split_trailing_url_punctuation(token: &str) -> (&str, &str) {
    let mut end = token.len();
    while end > 0 {
        let Some(character) = token[..end].chars().next_back() else {
            break;
        };
        if !matches!(character, '.' | ',' | ';' | ':' | '!' | '?' | ')' | ']') {
            break;
        }
        end -= character.len_utf8();
    }
    token.split_at(end)
}

fn url_has_scheme_body(url: &str) -> bool {
    url.strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .is_some_and(|body| !body.is_empty())
}
