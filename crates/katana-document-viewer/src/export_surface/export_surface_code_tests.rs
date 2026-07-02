use super::{SurfaceCodeHighlighter, span_style};
use image::Rgba;
use syntect::highlighting::Style;

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
