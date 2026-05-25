pub(crate) struct SurfaceMathText;

impl SurfaceMathText {
    pub(crate) fn render(expression: &str) -> String {
        MathRenderer::new(expression.trim()).render()
    }
}

struct MathRenderer<'a> {
    remaining: &'a str,
    output: String,
}

impl<'a> MathRenderer<'a> {
    fn new(expression: &'a str) -> Self {
        Self {
            remaining: expression,
            output: String::new(),
        }
    }

    fn render(mut self) -> String {
        while !self.remaining.is_empty() {
            if self.try_render_fraction() {
                continue;
            }
            if self.try_render_simple_token() {
                continue;
            }
            self.render_next_character();
        }
        self.output.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn try_render_fraction(&mut self) -> bool {
        let Some(rest) = self.remaining.strip_prefix(r"\frac") else {
            return false;
        };
        let Some((numerator, after_numerator)) = take_braced(rest) else {
            return false;
        };
        let Some((denominator, after_denominator)) = take_braced(after_numerator) else {
            return false;
        };

        self.output.push('(');
        self.output.push_str(&render_inline_text(numerator));
        self.output.push_str(")⁄(");
        self.output.push_str(&render_inline_text(denominator));
        self.output.push(')');
        self.remaining = after_denominator;
        true
    }

    fn try_render_simple_token(&mut self) -> bool {
        if self.try_render_static_token(r"\int", '∫') {
            return true;
        }
        if self.try_render_static_token(r"\sum", '∑') {
            return true;
        }
        if self.try_render_static_token(r"\,", ' ') {
            return true;
        }
        if self.try_render_script('^', true) {
            return true;
        }
        if self.try_render_script('_', false) {
            return true;
        }
        false
    }

    fn try_render_static_token(&mut self, token: &str, replacement: char) -> bool {
        let Some(rest) = self.remaining.strip_prefix(token) else {
            return false;
        };
        self.output.push(replacement);
        self.remaining = rest;
        true
    }

    fn try_render_script(&mut self, marker: char, is_superscript: bool) -> bool {
        let Some(rest) = self.remaining.strip_prefix(marker) else {
            return false;
        };
        let (script, next) = take_script(rest);
        let mapped = if is_superscript {
            script.chars().map(superscript).collect::<String>()
        } else {
            script.chars().map(subscript).collect::<String>()
        };
        self.output.push_str(&mapped);
        self.remaining = next;
        true
    }

    fn render_next_character(&mut self) {
        let mut characters = self.remaining.chars();
        let Some(character) = characters.next() else {
            self.remaining = "";
            return;
        };
        if character != '\\' {
            self.output.push(character);
        }
        self.remaining = characters.as_str();
    }
}

fn render_inline_text(expression: &str) -> String {
    MathRenderer::new(expression).render()
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
