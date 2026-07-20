use super::{DEFAULT_SYNTAX_THEME, SurfaceCodeHighlighter, span_style, theme};
use crate::KdvThemeSnapshot;
use image::Rgba;
use syntect::highlighting::Style;

#[test]
fn missing_syntect_theme_uses_default_theme() {
    assert!(std::ptr::eq(
        theme("missing-theme"),
        theme(DEFAULT_SYNTAX_THEME)
    ));
}

#[test]
fn highlights_plain_body_lines_when_language_is_absent() {
    let lines = SurfaceCodeHighlighter::highlight(None, "line one\nline two\n");

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0][0].text, "line one");
    assert!(lines[0][0].style.monospace);
    assert!(!lines[0][0].style.bold);
    assert_eq!(lines[1][0].text, "line two");
}

#[test]
fn code_token_color_reflects_received_syntax_style() {
    let mut token_style = Style::default();
    token_style.foreground.r = 1;
    token_style.foreground.g = 2;
    token_style.foreground.b = 3;
    token_style.foreground.a = 4;
    let style = span_style(token_style);

    assert_eq!(
        style.color,
        Some(Rgba([1, 2, 3, 4])),
        "code token colors must reflect the selected syntax theme without substitution"
    );
}

#[test]
fn code_token_omits_transparent_default_color() {
    let style = span_style(Style::default());

    assert_eq!(
        style.color, None,
        "transparent syntect default colors must fall back to the PDF theme text color"
    );
}

#[test]
fn syntax_theme_falls_back_to_default_for_empty_names() {
    let mut theme = KdvThemeSnapshot::katana_light();
    theme.syntax_theme_light.clear();
    let mut dark_theme = KdvThemeSnapshot::katana_dark();
    dark_theme.syntax_theme_dark.clear();

    assert_eq!(super::syntax_theme_name(&theme), "InspiredGitHub");
    assert_eq!(super::syntax_theme_name(&dark_theme), "InspiredGitHub");
}
