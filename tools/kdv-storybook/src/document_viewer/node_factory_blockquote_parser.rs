use super::QuoteLine;

pub(crate) struct QuoteLineParser;

impl QuoteLineParser {
    pub(crate) fn parse(raw: &str) -> Vec<QuoteLine> {
        if let Some(line) = Self::legacy_note_line(raw) {
            return vec![line];
        }
        Self::parse_lines_without_legacy(raw)
    }

    fn quote_line(line: &str) -> Option<(usize, &str)> {
        let mut rest = line.trim_start();
        let mut depth = 0;
        while let Some(next) = rest.strip_prefix('>') {
            depth += 1;
            rest = next.trim_start();
        }
        (depth > 0).then_some((depth, rest))
    }

    fn visible_line(body: &str, in_code: bool) -> Option<(String, bool)> {
        if in_code {
            return Some((body.to_string(), false));
        }
        let trimmed = body.trim();
        if trimmed.is_empty() {
            return None;
        }
        if let Some(text) = Self::bullet_body(trimmed) {
            return Some((text.to_string(), true));
        }
        Self::decorated_or_plain_line(trimmed)
    }

    fn decorated_or_plain_line(trimmed: &str) -> Option<(String, bool)> {
        if let Some(stripped) = trimmed
            .strip_prefix("**")
            .and_then(|value| value.strip_suffix("**"))
        {
            return Some((stripped.to_string(), false));
        }
        Some((trimmed.to_string(), false))
    }

    fn bullet_body(value: &str) -> Option<&str> {
        ["- ", "* ", "+ "]
            .iter()
            .find_map(|prefix| value.strip_prefix(prefix))
    }

    fn fence_language(value: &str) -> Option<String> {
        value
            .trim_start()
            .strip_prefix("```")
            .map(str::trim)
            .filter(|language| !language.is_empty())
            .map(ToString::to_string)
    }

    fn legacy_note_line(raw: &str) -> Option<QuoteLine> {
        let lines = Self::parse_lines_without_legacy(raw)
            .into_iter()
            .filter(|line| !line.text.trim().is_empty())
            .collect::<Vec<_>>();
        let [title, body @ ..] = lines.as_slice() else {
            return None;
        };
        if !Self::is_legacy_note_title(&title.text) || body.is_empty() {
            return None;
        }
        Some(QuoteLine {
            depth: title.depth,
            text: Self::legacy_note_text(title, body),
            code: false,
            bullet: false,
            language: None,
        })
    }

    fn legacy_note_text(title: &QuoteLine, body: &[QuoteLine]) -> String {
        format!(
            "{} {}",
            title.text,
            body.iter()
                .map(|line| line.text.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    fn parse_lines_without_legacy(raw: &str) -> Vec<QuoteLine> {
        let mut lines = Vec::new();
        let mut in_code = false;
        let mut code_language = None;
        for line in raw.lines() {
            let Some((depth, body)) = Self::quote_line(line) else {
                continue;
            };
            if Self::update_code_state(body, &mut in_code, &mut code_language) {
                continue;
            }
            Self::push_visible_line(&mut lines, depth, body, in_code, &code_language);
        }
        lines
    }

    fn update_code_state(
        body: &str,
        in_code: &mut bool,
        code_language: &mut Option<String>,
    ) -> bool {
        if !body.trim_start().starts_with("```") {
            return false;
        }
        Self::toggle_code_state(body, in_code, code_language);
        true
    }

    fn toggle_code_state(body: &str, in_code: &mut bool, code_language: &mut Option<String>) {
        if *in_code {
            *in_code = false;
            *code_language = None;
            return;
        }
        *in_code = true;
        *code_language = Self::fence_language(body);
    }

    fn push_visible_line(
        lines: &mut Vec<QuoteLine>,
        depth: usize,
        body: &str,
        in_code: bool,
        code_language: &Option<String>,
    ) {
        let Some((text, bullet)) = Self::visible_line(body, in_code) else {
            return;
        };
        lines.push(QuoteLine {
            depth,
            text,
            code: in_code,
            bullet,
            language: code_language.clone(),
        });
    }

    fn is_legacy_note_title(value: &str) -> bool {
        matches!(value, "Note" | "Tip" | "Important" | "Warning" | "Caution")
    }
}
