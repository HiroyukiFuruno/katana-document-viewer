#[cfg(test)]
pub(super) fn is_half_width_math_symbol(character: char) -> bool {
    HALF_WIDTH_MATH_SYMBOLS.contains(&character)
}

#[cfg(test)]
const HALF_WIDTH_MATH_SYMBOLS: &[char] = &[
    'α', 'β', 'γ', 'δ', '∑', '∫', '√', '∞', '⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹', 'ⁿ',
    'ˣ', '₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉', 'ₖ',
];
