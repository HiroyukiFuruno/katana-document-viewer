use crate::export_surface_span::{SurfaceTextSpan, SurfaceTextStyle};
use crate::{KdvThemeMode, KdvThemeSnapshot};
use image::Rgba;
use std::sync::LazyLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

const DEFAULT_SYNTAX_THEME: &str = "InspiredGitHub";

pub(crate) struct SurfaceCodeHighlighter;

impl SurfaceCodeHighlighter {
    pub(crate) fn highlight(language: Option<&str>, body: &str) -> Vec<Vec<SurfaceTextSpan>> {
        Self::highlight_with_theme_name(language, body, DEFAULT_SYNTAX_THEME)
    }

    pub(crate) fn highlight_with_theme(
        language: Option<&str>,
        body: &str,
        theme: &KdvThemeSnapshot,
    ) -> Vec<Vec<SurfaceTextSpan>> {
        Self::highlight_with_theme_config(language, body, syntax_theme_name(theme))
    }

    fn highlight_with_theme_name(
        language: Option<&str>,
        body: &str,
        syntax_theme: &str,
    ) -> Vec<Vec<SurfaceTextSpan>> {
        Self::highlight_with_theme_config(language, body, syntax_theme)
    }

    fn highlight_with_theme_config(
        language: Option<&str>,
        body: &str,
        syntax_theme: &str,
    ) -> Vec<Vec<SurfaceTextSpan>> {
        match language.filter(|value| !value.is_empty()) {
            Some(language) => Self::highlight_language(language, body, syntax_theme),
            None => body.lines().map(Self::plain_line).collect(),
        }
    }

    fn highlight_language(
        language: &str,
        body: &str,
        syntax_theme: &str,
    ) -> Vec<Vec<SurfaceTextSpan>> {
        let syntax = syntax(language);
        let mut highlighter = HighlightLines::new(syntax, theme(syntax_theme));
        LinesWithEndings::from(body)
            .map(|line| Self::highlight_line(&mut highlighter, line))
            .collect()
    }

    fn highlight_line(highlighter: &mut HighlightLines<'_>, line: &str) -> Vec<SurfaceTextSpan> {
        let line = line.trim_end_matches(['\r', '\n']);
        let fallback = vec![(Style::default(), line)];
        let ranges = highlighter
            .highlight_line(line, syntax_set())
            .unwrap_or(fallback);
        ranges
            .into_iter()
            .map(|(style, text)| SurfaceTextSpan::styled(text, span_style(style)))
            .collect()
    }

    fn plain_line(line: &str) -> Vec<SurfaceTextSpan> {
        let line = line.trim_end_matches(['\r', '\n']);
        vec![SurfaceTextSpan::styled(
            line,
            SurfaceTextStyle::default().monospace(),
        )]
    }
}

fn span_style(style: Style) -> SurfaceTextStyle {
    if style.foreground.a == 0
        || (style.foreground.r == 0
            && style.foreground.g == 0
            && style.foreground.b == 0
            && style.foreground.a == 255)
    {
        return SurfaceTextStyle::default().monospace();
    }
    let color = Rgba([
        style.foreground.r,
        style.foreground.g,
        style.foreground.b,
        style.foreground.a,
    ]);
    SurfaceTextStyle::default().monospace().with_color(color)
}

fn syntax(language: &str) -> &'static SyntaxReference {
    syntax_set()
        .find_syntax_by_token(language)
        .or_else(|| syntax_set().find_syntax_by_extension(language))
        .unwrap_or_else(|| syntax_set().find_syntax_plain_text())
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
    theme_set()
        .themes
        .get(name)
        .unwrap_or_else(|| &theme_set().themes[DEFAULT_SYNTAX_THEME])
}

fn syntax_theme_name(theme: &KdvThemeSnapshot) -> &str {
    let name = match theme.mode {
        KdvThemeMode::Light => theme.syntax_theme_light.as_str(),
        KdvThemeMode::Dark => theme.syntax_theme_dark.as_str(),
    };
    if name.trim().is_empty() {
        return DEFAULT_SYNTAX_THEME;
    }
    name
}

#[cfg(test)]
#[path = "export_surface_code_tests.rs"]
mod tests;
