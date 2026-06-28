pub(crate) struct MarkdownLinkSource;

impl MarkdownLinkSource {
    pub(crate) fn contains_markdown_link(source: &str) -> bool {
        let lines = Self::lines_outside_fences(source);
        let definitions = Self::reference_definitions(&lines);
        lines
            .iter()
            .any(|line| Self::line_contains_markdown_link(line, &definitions))
    }

    fn line_contains_markdown_link(line: &str, definitions: &[String]) -> bool {
        Self::line_contains_inline_link(line)
            || Self::line_contains_reference_link(line, definitions)
            || Self::line_contains_shortcut_reference_link(line, definitions)
            || Self::line_contains_autolink(line)
    }

    fn line_contains_inline_link(line: &str) -> bool {
        let mut search_from = 0;
        while let Some(label_end) = line[search_from..].find("](") {
            let absolute_end = search_from + label_end;
            if Self::has_valid_label_start(line, absolute_end) {
                return true;
            }
            search_from = absolute_end + 2;
        }
        false
    }

    fn line_contains_reference_link(line: &str, definitions: &[String]) -> bool {
        let mut search_from = 0;
        while let Some(label_end) = line[search_from..].find("][") {
            let absolute_end = search_from + label_end;
            let Some(label_start) = line[..absolute_end].rfind('[') else {
                search_from = absolute_end + 2;
                continue;
            };
            if !Self::has_valid_label_at(line, label_start, absolute_end) {
                search_from = absolute_end + 2;
                continue;
            }
            let target_start = absolute_end + 2;
            let Some((label, target_end)) =
                Self::reference_label(line, label_start, absolute_end, target_start)
            else {
                return false;
            };
            if definitions.iter().any(|definition| definition == &label) {
                return true;
            }
            search_from = target_end + 1;
        }
        false
    }

    fn reference_label(
        line: &str,
        label_start: usize,
        label_end: usize,
        target_start: usize,
    ) -> Option<(String, usize)> {
        let target_end = target_start + line[target_start..].find(']')?;
        let target = &line[target_start..target_end];
        let reference = if target.is_empty() {
            &line[label_start + 1..label_end]
        } else {
            target
        };
        Some((Self::normalize_label(reference), target_end))
    }

    fn line_contains_shortcut_reference_link(line: &str, definitions: &[String]) -> bool {
        let mut search_from = 0;
        while let Some(label_start_offset) = line[search_from..].find('[') {
            let label_start = search_from + label_start_offset;
            if label_start > 0 && line[..label_start].ends_with(']') {
                search_from = label_start + 1;
                continue;
            }
            let Some(label_end_offset) = line[label_start + 1..].find(']') else {
                return false;
            };
            let label_end = label_start + 1 + label_end_offset;
            if Self::is_shortcut_reference(line, label_start, label_end, definitions) {
                return true;
            }
            search_from = label_end + 1;
        }
        false
    }

    fn line_contains_autolink(line: &str) -> bool {
        let mut search_from = 0;
        while let Some(start_offset) = line[search_from..].find('<') {
            let start = search_from + start_offset + 1;
            let Some(end_offset) = line[start..].find('>') else {
                return false;
            };
            let end = start + end_offset;
            let target = &line[start..end];
            if Self::is_uri_autolink(target) || Self::is_email_autolink(target) {
                return true;
            }
            search_from = end + 1;
        }
        false
    }

    fn is_uri_autolink(target: &str) -> bool {
        if target.chars().any(char::is_whitespace) {
            return false;
        }
        let Some((scheme, rest)) = target.split_once(':') else {
            return false;
        };
        (2..=32).contains(&scheme.len())
            && scheme.starts_with(|value: char| value.is_ascii_alphabetic())
            && scheme
                .chars()
                .all(|value| value.is_ascii_alphanumeric() || matches!(value, '+' | '.' | '-'))
            && !rest.is_empty()
    }

    fn is_email_autolink(target: &str) -> bool {
        if target.chars().any(char::is_whitespace) {
            return false;
        }
        let Some((local, domain)) = target.split_once('@') else {
            return false;
        };
        !local.is_empty() && domain.contains('.') && !domain.ends_with('.')
    }

    fn is_shortcut_reference(
        line: &str,
        label_start: usize,
        label_end: usize,
        definitions: &[String],
    ) -> bool {
        if !Self::has_valid_label_at(line, label_start, label_end) {
            return false;
        }
        let rest = &line[label_end + 1..];
        if rest.starts_with(':') || rest.starts_with('(') || rest.starts_with('[') {
            return false;
        }
        let label = Self::normalize_label(&line[label_start + 1..label_end]);
        definitions.iter().any(|definition| definition == &label)
    }

    fn has_valid_label_start(line: &str, label_end: usize) -> bool {
        let Some(label_start) = line[..label_end].rfind('[') else {
            return false;
        };
        Self::has_valid_label_at(line, label_start, label_end)
    }

    fn has_valid_label_at(line: &str, label_start: usize, label_end: usize) -> bool {
        let label = &line[label_start + 1..label_end];
        !label.trim().is_empty()
            && !label.starts_with('^')
            && (label_start == 0 || !line[..label_start].ends_with('!'))
    }

    fn reference_definitions(lines: &[&str]) -> Vec<String> {
        lines
            .iter()
            .filter_map(|line| {
                let trimmed = line.trim_start();
                if trimmed.starts_with("[^") {
                    return None;
                }
                let (label, _) = trimmed.strip_prefix('[')?.split_once("]:")?;
                Some(Self::normalize_label(label))
            })
            .filter(|label| !label.is_empty())
            .collect()
    }

    fn lines_outside_fences(source: &str) -> Vec<&str> {
        let mut inside_fence = false;
        let mut lines = Vec::new();
        for line in source.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                inside_fence = !inside_fence;
                continue;
            }
            if !inside_fence {
                lines.push(line);
            }
        }
        lines
    }
    fn normalize_label(label: &str) -> String {
        let words = label.split_whitespace().collect::<Vec<_>>();
        words.join(" ").to_ascii_lowercase()
    }
}
