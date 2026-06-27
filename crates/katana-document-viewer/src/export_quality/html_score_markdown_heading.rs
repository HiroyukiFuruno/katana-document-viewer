pub(super) fn source_has_heading(lines: &[String]) -> bool {
    source_heading_count(lines) > 0
}

pub(super) fn source_heading_count(lines: &[String]) -> usize {
    source_atx_heading_count(lines) + source_setext_heading_count(lines)
}

pub(super) fn source_is_setext_marker_line(lines: &[String], index: usize) -> bool {
    index > 0
        && is_setext_marker(lines[index].trim())
        && looks_like_setext_heading_text(&lines[index - 1])
}

fn source_atx_heading_count(lines: &[String]) -> usize {
    lines
        .iter()
        .filter(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with('#')
                && trimmed.chars().take_while(|value| *value == '#').count() <= 6
        })
        .count()
}

fn source_setext_heading_count(lines: &[String]) -> usize {
    lines
        .iter()
        .enumerate()
        .filter(|(index, _line)| source_is_setext_marker_line(lines, *index))
        .count()
}

fn is_setext_marker(marker: &str) -> bool {
    !marker.is_empty()
        && (marker.chars().all(|value| value == '=') || marker.chars().all(|value| value == '-'))
}

fn looks_like_setext_heading_text(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() || is_setext_marker(trimmed) {
        return false;
    }
    if trimmed.starts_with('#') && trimmed.chars().take_while(|value| *value == '#').count() <= 6 {
        return false;
    }
    if starts_with_bullet_list(trimmed) || starts_with_ordered_list(trimmed) {
        return false;
    }
    true
}

fn starts_with_bullet_list(trimmed: &str) -> bool {
    ["- ", "* ", "+ "]
        .iter()
        .any(|prefix| trimmed.starts_with(prefix))
}

fn starts_with_ordered_list(trimmed: &str) -> bool {
    let Some((digits, rest)) = trimmed.split_once('.') else {
        return false;
    };
    !digits.is_empty()
        && digits.chars().all(|value| value.is_ascii_digit())
        && rest.starts_with(' ')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setext_marker_on_first_line_is_not_setext_heading_marker() {
        let lines = vec!["-----".to_string()];
        assert!(!source_is_setext_marker_line(&lines, 0));
    }

    #[test]
    fn marker_like_setext_without_heading_text_is_rejected() {
        let lines = vec!["text".to_string(), "-----x".to_string()];
        assert!(!source_is_setext_marker_line(&lines, 1));
    }
}
