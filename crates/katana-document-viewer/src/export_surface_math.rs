pub(crate) struct SurfaceMathText;

impl SurfaceMathText {
    pub(crate) fn render(expression: &str) -> String {
        render_inline_text(expression.trim())
    }
}

fn render_inline_text(expression: &str) -> String {
    let mut output = String::new();
    let mut remaining = expression;
    while !remaining.is_empty() {
        if let Some(rest) = remaining.strip_prefix(r"\frac")
            && let Some((numerator, after_numerator)) = take_braced(rest)
            && let Some((denominator, after_denominator)) = take_braced(after_numerator)
        {
            output.push('(');
            output.push_str(&render_inline_text(numerator));
            output.push_str(")⁄(");
            output.push_str(&render_inline_text(denominator));
            output.push(')');
            remaining = after_denominator;
            continue;
        }
        if let Some(rest) = remaining.strip_prefix(r"\int") {
            output.push('∫');
            remaining = rest;
            continue;
        }
        if let Some(rest) = remaining.strip_prefix(r"\sum") {
            output.push('∑');
            remaining = rest;
            continue;
        }
        if let Some(rest) = remaining.strip_prefix(r"\,") {
            output.push(' ');
            remaining = rest;
            continue;
        }
        if let Some(rest) = remaining.strip_prefix('^') {
            let (script, next) = take_script(rest);
            output.push_str(&script.chars().map(superscript).collect::<String>());
            remaining = next;
            continue;
        }
        if let Some(rest) = remaining.strip_prefix('_') {
            let (script, next) = take_script(rest);
            output.push_str(&script.chars().map(subscript).collect::<String>());
            remaining = next;
            continue;
        }
        let mut chars = remaining.chars();
        let Some(character) = chars.next() else {
            break;
        };
        if character != '\\' {
            output.push(character);
        }
        remaining = chars.as_str();
    }
    output.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn take_braced(text: &str) -> Option<(&str, &str)> {
    let rest = text.strip_prefix('{')?;
    let mut depth = 0usize;
    for (index, character) in rest.char_indices() {
        match character {
            '{' => depth += 1,
            '}' if depth == 0 => return Some((&rest[..index], &rest[index + 1..])),
            '}' => depth -= 1,
            _ => {}
        }
    }
    None
}

fn take_script(text: &str) -> (&str, &str) {
    if let Some((script, rest)) = take_braced(text) {
        return (script, rest);
    }
    let Some(character) = text.chars().next() else {
        return ("", text);
    };
    let end = character.len_utf8();
    (&text[..end], &text[end..])
}

fn superscript(character: char) -> char {
    match character {
        '0' => '⁰',
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4' => '⁴',
        '5' => '⁵',
        '6' => '⁶',
        '7' => '⁷',
        '8' => '⁸',
        '9' => '⁹',
        '+' => '⁺',
        '-' => '⁻',
        '=' => '⁼',
        'n' => 'ⁿ',
        'x' => 'ˣ',
        _ => character,
    }
}

fn subscript(character: char) -> char {
    match character {
        '0' => '₀',
        '1' => '₁',
        '2' => '₂',
        '3' => '₃',
        '4' => '₄',
        '5' => '₅',
        '6' => '₆',
        '7' => '₇',
        '8' => '₈',
        '9' => '₉',
        '+' => '₊',
        '-' => '₋',
        '=' => '₌',
        'k' => 'ₖ',
        _ => character,
    }
}

#[cfg(test)]
#[path = "export_surface_math_tests.rs"]
mod tests;
