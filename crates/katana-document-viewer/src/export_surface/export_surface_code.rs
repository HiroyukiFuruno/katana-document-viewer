use crate::export_surface_span::{SurfaceTextSpan, SurfaceTextStyle};
use image::Rgba;
use std::sync::LazyLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

const SYNTAX_THEME: &str = "InspiredGitHub";

pub(crate) struct SurfaceCodeHighlighter;

impl SurfaceCodeHighlighter {
    pub(crate) fn highlight(language: Option<&str>, body: &str) -> Vec<Vec<SurfaceTextSpan>> {
        match language.filter(|value| !value.is_empty()) {
            Some(language) => Self::highlight_language(language, body),
            None => body.lines().map(Self::plain_line).collect(),
        }
    }

    fn highlight_language(language: &str, body: &str) -> Vec<Vec<SurfaceTextSpan>> {
        let syntax = syntax(language);
        let mut highlighter = HighlightLines::new(syntax, theme());
        LinesWithEndings::from(body)
            .map(|line| Self::highlight_line(&mut highlighter, line))
            .collect()
    }

    fn highlight_line(highlighter: &mut HighlightLines<'_>, line: &str) -> Vec<SurfaceTextSpan> {
        let line = line.trim_end_matches(['\r', '\n']);
        let ranges = highlighter
            .highlight_line(line, syntax_set())
            .unwrap_or_else(|_| vec![(Style::default(), line)]);
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
    SurfaceTextStyle::default().monospace().with_color(Rgba([
        style.foreground.r,
        style.foreground.g,
        style.foreground.b,
        style.foreground.a,
    ]))
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

fn theme() -> &'static Theme {
    &theme_set().themes[SYNTAX_THEME]
}
