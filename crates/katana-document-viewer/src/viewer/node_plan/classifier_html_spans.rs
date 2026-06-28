use super::super::super::types::{ViewerTextSpan, ViewerTextStyle};
use super::ViewerNodeClassifier;
use crate::export_surface_text::SurfaceTextParser as TextParser;

impl ViewerNodeClassifier {
    pub(super) fn inline_html_spans(html: &str, style: ViewerTextStyle) -> Vec<ViewerTextSpan> {
        let text = TextParser::html_fragment_text(html);
        let html_style = html_style(html, style);
        if let Some(target) = html_link_target(html) {
            return Self::linked_span(text, target, html_style);
        }
        Self::styled_span(text, html_style)
    }

    pub(super) fn html_block_spans(raw: &str, fallback: String) -> Vec<ViewerTextSpan> {
        let spans = Self::html_link_spans(raw);
        if spans.is_empty() {
            return vec![ViewerTextSpan::plain(fallback)];
        }
        spans
    }

    fn html_link_spans(raw: &str) -> Vec<ViewerTextSpan> {
        let lower = raw.to_ascii_lowercase();
        let mut cursor = 0;
        let mut spans = Vec::new();
        while let Some(relative_start) = lower[cursor..].find("<a ") {
            let link_start = cursor + relative_start;
            let Some(tag_end_delta) = raw[link_start..].find('>') else {
                break;
            };
            let tag_end = link_start + tag_end_delta;
            let body_start = tag_end + 1;
            let Some(close_delta) = lower[body_start..].find("</a>") else {
                break;
            };
            let close_start = body_start + close_delta;
            Self::push_html_plain(&raw[cursor..link_start], &mut spans);
            let target = html_link_target(&raw[link_start..=tag_end]);
            let text = TextParser::html_fragment_text(&raw[body_start..close_start]);
            if let Some(target) = target {
                spans.extend(Self::linked_span(text, target, ViewerTextStyle::default()));
            } else {
                spans.extend(Self::plain_span(&text, ViewerTextStyle::default()));
            }
            cursor = close_start + "</a>".len();
        }
        Self::push_html_plain(&raw[cursor..], &mut spans);
        spans
    }

    fn push_html_plain(raw: &str, spans: &mut Vec<ViewerTextSpan>) {
        let text = TextParser::html_fragment_text(raw);
        if text.is_empty() {
            return;
        }
        spans.push(ViewerTextSpan::plain(text));
    }
}

fn html_link_target(html: &str) -> Option<String> {
    let lower = html.to_ascii_lowercase();
    let href_index = lower.find("href")?;
    let after_href = &html[href_index + "href".len()..];
    let equals_index = after_href.find('=')?;
    let value = after_href[equals_index + 1..].trim_start();
    let quote = value.chars().next()?;
    if quote == '"' || quote == '\'' {
        let target = &value[quote.len_utf8()..];
        let end = target.find(quote)?;
        return Some(target[..end].to_string());
    }
    let end = value
        .find(|character: char| character.is_whitespace() || character == '>')
        .unwrap_or(value.len());
    Some(value[..end].trim_matches('/').to_string())
}

fn html_style(html: &str, style: ViewerTextStyle) -> ViewerTextStyle {
    let lower = html.to_ascii_lowercase();
    if lower.contains("<code") {
        return style.inline_code();
    }
    if lower.contains("<strong") || lower.contains("<b") {
        return style.bold();
    }
    if lower.contains("<em") || lower.contains("<i") {
        return style.italic();
    }
    if lower.contains("<u") {
        return style.underline();
    }
    if lower.contains("<mark") {
        return style.highlight();
    }
    if lower.contains("<s") || lower.contains("<del") {
        return style.strikethrough();
    }
    style
}
