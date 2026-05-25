use super::{
    DEFAULT_WIDTH_FACTOR, HALF_WIDTH_MATH_FACTOR, HALF_WIDTH_PUNCTUATION_FACTOR,
    HALF_WIDTH_SPACE_FACTOR, HALF_WIDTH_TEXT_FACTOR,
};

pub(super) fn estimated_text_width(text: &str, size: f32) -> u32 {
    text.chars()
        .map(|character| character_width_factor(character) * size)
        .sum::<f32>()
        .ceil() as u32
}

fn character_width_factor(character: char) -> f32 {
    if character.is_ascii_whitespace() {
        return HALF_WIDTH_SPACE_FACTOR;
    }
    if character.is_ascii_punctuation() {
        return HALF_WIDTH_PUNCTUATION_FACTOR;
    }
    if character.is_ascii() {
        return HALF_WIDTH_TEXT_FACTOR;
    }
    if is_half_width_math_symbol(character) {
        return HALF_WIDTH_MATH_FACTOR;
    }
    DEFAULT_WIDTH_FACTOR
}

fn is_half_width_math_symbol(character: char) -> bool {
    HALF_WIDTH_MATH_SYMBOLS.contains(&character)
}

const HALF_WIDTH_MATH_SYMBOLS: &[char] = &[
    'α', 'β', 'γ', 'δ', '∑', '∫', '√', '∞', '⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸',
    '⁹', 'ⁿ', 'ˣ', '₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉', 'ₖ',
];
