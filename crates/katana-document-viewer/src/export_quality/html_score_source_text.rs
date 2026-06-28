use crate::export_quality::types::{ExportQualityCheck, check};
use html_score_source_plain_text::HtmlSourcePlainText;

const HTML_TEXT_TAGS: [&str; 29] = [
    "<!doctype",
    "<html",
    "<body",
    "<main",
    "<section",
    "<article",
    "<h1",
    "<h2",
    "<h3",
    "<h4",
    "<h5",
    "<h6",
    "<p",
    "<div",
    "<span",
    "<a",
    "<ul",
    "<ol",
    "<li",
    "<table",
    "<tr",
    "<td",
    "<th",
    "<blockquote",
    "<details",
    "<summary",
    "<code",
    "<pre",
    "<figure",
];

pub(super) struct HtmlSourceTextQuality;

impl HtmlSourceTextQuality {
    pub(super) fn checks(html: &str, source: &str) -> Vec<ExportQualityCheck> {
        vec![check(
            "html preserves source visible text",
            !HtmlVisibleText::source_requires_text(source)
                || HtmlVisibleText::output_preserves_source_text(html, source),
            true,
            0,
        )]
    }
}

struct HtmlVisibleText;

impl HtmlVisibleText {
    fn source_requires_text(source: &str) -> bool {
        Self::looks_like_direct_html(source)
            && Self::contains_html_tag(source)
            && !Self::source_tokens(source).is_empty()
    }

    fn output_preserves_source_text(html: &str, source: &str) -> bool {
        let output_tokens = Self::output_tokens(html);
        let source_tokens = Self::source_tokens(source);
        Self::tokens_appear_in_order(&source_tokens, &output_tokens)
    }

    fn source_tokens(html: &str) -> Vec<String> {
        Self::visible_tokens(&Self::plain_text(html))
    }

    fn output_tokens(html: &str) -> Vec<String> {
        Self::visible_tokens(&Self::plain_text(html))
    }

    fn visible_tokens(text: &str) -> Vec<String> {
        text.split(|character: char| character.is_whitespace() || character.is_ascii_punctuation())
            .map(str::trim)
            .filter(|token| token.chars().count() > 1)
            .map(str::to_ascii_lowercase)
            .collect()
    }

    fn tokens_appear_in_order(required: &[String], actual: &[String]) -> bool {
        let mut actual_index = 0;
        for required_token in required {
            let Some(offset) = actual[actual_index..]
                .iter()
                .position(|actual_token| actual_token == required_token)
            else {
                return false;
            };
            actual_index += offset + 1;
        }
        true
    }

    fn plain_text(html: &str) -> String {
        HtmlSourcePlainText::scan(html)
    }

    fn contains_html_tag(source: &str) -> bool {
        let lower = source.to_ascii_lowercase();
        HTML_TEXT_TAGS.iter().any(|tag| lower.contains(tag))
    }

    fn looks_like_direct_html(source: &str) -> bool {
        let trimmed = source.trim_start();
        trimmed.starts_with('<')
    }
}

#[path = "html_score_source_plain_text.rs"]
mod html_score_source_plain_text;
