use crate::export_html_ops::ExportHtmlOps;
use crate::{KdvThemeMode, KdvThemeSnapshot};
use std::sync::LazyLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::{IncludeBackground, styled_line_to_highlighted_html};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

const SYNTAX_THEME: &str = "InspiredGitHub";

pub(crate) struct CodeHtmlWriter;

impl CodeHtmlWriter {
    pub(crate) fn append_plain(
        html: &mut String,
        language: &Option<String>,
        text: &str,
        theme: &KdvThemeSnapshot,
    ) {
        let body = ExportHtmlOps::fenced_body(text);
        match language.as_deref().filter(|value| !value.is_empty()) {
            Some(language) => Self::append_highlighted(html, language, &body, theme),
            None => Self::append_plain_text(html, &body),
        }
    }

    fn append_highlighted(html: &mut String, language: &str, body: &str, theme: &KdvThemeSnapshot) {
        let syntax_theme = syntax_theme_name(theme);
        html.push_str(&format!(
            "<pre data-kdv-code-role=\"plain\" data-kdv-code-language=\"{}\" data-kdv-code-highlighter=\"syntect\" data-kdv-syntax-theme=\"{}\"><code class=\"language-{}\">",
            ExportHtmlOps::escape_html(language),
            ExportHtmlOps::escape_html(syntax_theme),
            ExportHtmlOps::escape_html(language)
        ));
        if let Some(highlighted) = Self::highlighted_html(language, body, syntax_theme) {
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

    fn highlighted_html(language: &str, body: &str, syntax_theme: &str) -> Option<String> {
        if body.is_empty() {
            return None;
        }
        let syntax = Self::syntax(language);
        let mut highlighter = HighlightLines::new(syntax, theme(syntax_theme));
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

fn theme(name: &str) -> &'static Theme {
    match theme_set().themes.get(name) {
        Some(theme) => theme,
        None => &theme_set().themes[SYNTAX_THEME],
    }
}

fn syntax_theme_name(theme: &KdvThemeSnapshot) -> &str {
    let name = match theme.mode {
        KdvThemeMode::Light => theme.syntax_theme_light.as_str(),
        KdvThemeMode::Dark => theme.syntax_theme_dark.as_str(),
    };
    if name.trim().is_empty() {
        return SYNTAX_THEME;
    }
    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_plain_uses_plain_text_without_language() {
        let mut html = String::new();
        let language = Some(String::new());
        CodeHtmlWriter::append_plain(
            &mut html,
            &language,
            "a & b",
            &KdvThemeSnapshot::katana_light(),
        );
        assert!(html.contains("<pre data-kdv-code-role=\"plain\"><code>a &amp; b</code></pre>"));
    }

    #[test]
    fn append_plain_uses_highlighted_code_when_language_specified() {
        let mut html = String::new();
        CodeHtmlWriter::append_plain(
            &mut html,
            &Some("rust".to_string()),
            "fn main() {}",
            &KdvThemeSnapshot::katana_light(),
        );
        assert!(html.contains("data-kdv-code-language=\"rust\""));
        assert!(html.contains("data-kdv-code-highlighter=\"syntect\""));
    }

    #[test]
    fn append_plain_uses_plain_text_when_highlighted_code_is_empty() {
        let mut html = String::new();
        CodeHtmlWriter::append_highlighted(
            &mut html,
            "rust",
            "",
            &KdvThemeSnapshot::katana_light(),
        );
        assert!(
            html.starts_with(
                "<pre data-kdv-code-role=\"plain\" data-kdv-code-language=\"rust\" data-kdv-code-highlighter=\"syntect\" data-kdv-syntax-theme=\"InspiredGitHub\"><code class=\"language-rust\">"
            )
        );
        assert!(html.ends_with("</code></pre>\n"));
    }

    #[test]
    fn fenced_body_still_supported() {
        let fenced = "```rust\nfn main() {}\n```";
        let body = ExportHtmlOps::fenced_body(fenced);
        assert_eq!(body, "fn main() {}");
    }

    #[test]
    fn append_highlighted_uses_default_theme_when_theme_is_empty() {
        let mut custom_theme = KdvThemeSnapshot::katana_dark();
        custom_theme.syntax_theme_dark.clear();

        let mut html = String::new();

        CodeHtmlWriter::append_highlighted(&mut html, "rust", "let x = 1;", &custom_theme);
        let expected = format!("data-kdv-syntax-theme=\"{}\"", SYNTAX_THEME);

        assert!(html.contains(&expected));
    }

    #[test]
    fn missing_syntect_theme_uses_default_theme() {
        assert!(std::ptr::eq(theme("missing-theme"), theme(SYNTAX_THEME)));
    }
}
