use crate::export_html_ops::ExportHtmlOps;
use std::sync::LazyLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::{IncludeBackground, styled_line_to_highlighted_html};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

const SYNTAX_THEME: &str = "InspiredGitHub";

pub(crate) struct CodeHtmlWriter;

impl CodeHtmlWriter {
    pub(crate) fn append_plain(html: &mut String, language: &Option<String>, text: &str) {
        let body = ExportHtmlOps::fenced_body(text);
        match language.as_deref().filter(|value| !value.is_empty()) {
            Some(language) => Self::append_highlighted(html, language, &body),
            None => Self::append_plain_text(html, &body),
        }
    }

    fn append_highlighted(html: &mut String, language: &str, body: &str) {
        html.push_str(&format!(
            "<pre data-kdv-code-role=\"plain\" data-kdv-code-language=\"{}\" data-kdv-code-highlighter=\"syntect\" data-kdv-syntax-theme=\"{SYNTAX_THEME}\"><code class=\"language-{}\">",
            ExportHtmlOps::escape_html(language),
            ExportHtmlOps::escape_html(language)
        ));
        if let Some(highlighted) = Self::highlighted_html(language, body) {
            html.push_str(&highlighted);
        } else {
            html.push_str(&ExportHtmlOps::escape_html(body));
        }
        html.push_str("</code></pre>\n");
    }

    fn append_plain_text(html: &mut String, body: &str) {
        html.push_str(&format!(
            "<pre data-kdv-code-role=\"plain\"><code>{}</code></pre>\n",
            ExportHtmlOps::escape_html(body)
        ));
    }

    fn highlighted_html(language: &str, body: &str) -> Option<String> {
        let syntax = Self::syntax(language);
        let mut highlighter = HighlightLines::new(syntax, theme());
        let mut html = String::new();
        for line in LinesWithEndings::from(body) {
            let ranges = highlighter.highlight_line(line, syntax_set()).ok()?;
            let highlighted =
                styled_line_to_highlighted_html(&ranges, IncludeBackground::No).ok()?;
            html.push_str(&highlighted);
        }
        Some(html)
    }

    fn syntax(language: &str) -> &'static SyntaxReference {
        syntax_set()
            .find_syntax_by_token(language)
            .or_else(|| syntax_set().find_syntax_by_extension(language))
            .unwrap_or_else(|| syntax_set().find_syntax_plain_text())
    }
}

fn syntax_set() -> &'static SyntaxSet {
    static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
    &SYNTAX_SET
}

fn theme_set() -> &'static ThemeSet {
    static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);
    &THEME_SET
}

fn theme() -> &'static Theme {
    &theme_set().themes[SYNTAX_THEME]
}
