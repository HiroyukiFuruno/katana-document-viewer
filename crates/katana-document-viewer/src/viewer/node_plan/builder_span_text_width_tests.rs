use super::{SpanTextWidthMeasurer, whitespace_width};
use crate::{ViewerTextSpan, ViewerTextStyle};

#[test]
fn cached_width_is_zero_for_empty_text() {
    assert_eq!(
        0,
        SpanTextWidthMeasurer::cached_width(&ViewerTextSpan::plain(""), "", 16.0)
    );
}

#[test]
fn cached_width_is_deterministic_for_repeated_calls() {
    let span = ViewerTextSpan::plain("sample");
    let width = SpanTextWidthMeasurer::cached_width(&span, "sample", 16.0);
    assert_eq!(
        width,
        SpanTextWidthMeasurer::cached_width(&span, "sample", 16.0)
    );
}

#[test]
fn cached_width_handles_preserve_whitespace_width_factor() {
    let span = ViewerTextSpan::styled(" ", ViewerTextStyle::default().inline_code());
    let monospace_width = SpanTextWidthMeasurer::cached_width(&span, " ", 5.0);
    let normal_width = SpanTextWidthMeasurer::cached_width(&ViewerTextSpan::plain(" "), " ", 5.0);
    assert!(monospace_width >= normal_width);
}

#[test]
fn cached_width_shapes_italic_and_emoji_styles() {
    let italic = ViewerTextSpan::styled("italic-coverage", ViewerTextStyle::default().italic());
    let emoji = ViewerTextSpan::styled("emoji-coverage", ViewerTextStyle::default().emoji());

    assert!(SpanTextWidthMeasurer::cached_width(&italic, &italic.text, 17.25) > 0);
    assert!(SpanTextWidthMeasurer::cached_width(&emoji, &emoji.text, 18.25) > 0);
}

#[test]
fn whitespace_width_scales_with_font_size() {
    assert_eq!(2, whitespace_width(5.0, false));
    assert_eq!(3, whitespace_width(5.0, true));
}
