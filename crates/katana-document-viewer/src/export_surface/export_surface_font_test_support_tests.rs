use super::*;

#[test]
fn estimated_text_width_uses_category_factors() {
    assert_eq!(
        estimated_text_width(" ", 10.0),
        (HALF_WIDTH_SPACE_FACTOR * 10.0).ceil() as u32
    );
    assert_eq!(
        estimated_text_width("!", 10.0),
        (HALF_WIDTH_PUNCTUATION_FACTOR * 10.0).ceil() as u32
    );
    assert_eq!(
        estimated_text_width("a", 10.0),
        (HALF_WIDTH_TEXT_FACTOR * 10.0).ceil() as u32
    );
    assert_eq!(
        estimated_text_width("α", 10.0),
        (HALF_WIDTH_MATH_FACTOR * 10.0).ceil() as u32
    );
    assert_eq!(
        estimated_text_width("文", 10.0),
        (DEFAULT_WIDTH_FACTOR * 10.0).ceil() as u32
    );
}

#[test]
fn recognizes_half_width_math_symbol() {
    assert!(is_half_width_math_symbol('α'));
    assert!(!is_half_width_math_symbol('A'));
}
