pub(crate) struct MarkdownImageSource;

struct ReferenceImageScan<'a> {
    candidate: Option<ReferenceImageCandidate<'a>>,
    next_search_from: usize,
}

struct ReferenceImageCandidate<'a> {
    label_start: usize,
    label_end: usize,
    target: &'a str,
}

impl MarkdownImageSource {
    pub(crate) fn line_contains_markdown_image(line: &str, definitions: &[String]) -> bool {
        Self::line_contains_inline_image(line)
            || Self::line_contains_reference_image(line, definitions)
            || Self::line_contains_shortcut_reference_image(line, definitions)
    }

    fn line_contains_inline_image(line: &str) -> bool {
        let mut search_from = 0;
        while let Some(label_start) = line[search_from..].find("![") {
            let absolute_start = search_from + label_start;
            let search_from_label = absolute_start + 2;
            let Some(label_end_offset) = line[search_from_label..].find(']') else {
                return false;
            };
            let label_end = search_from_label + label_end_offset;
            let after_label = &line[label_end + 1..];
            if !after_label.starts_with('(') {
                search_from = label_end + 1;
                continue;
            }
            return true;
        }
        false
    }

    fn line_contains_reference_image(line: &str, definitions: &[String]) -> bool {
        let mut search_from = 0;
        while let Some(scan) = Self::reference_image_scan(line, search_from) {
            if let Some(candidate) = scan.candidate
                && Self::is_valid_markdown_image_reference(
                    line,
                    candidate.label_start,
                    candidate.label_end,
                    candidate.target,
                    definitions,
                )
            {
                return true;
            }
            search_from = scan.next_search_from;
        }
        false
    }

    fn reference_image_scan(line: &str, search_from: usize) -> Option<ReferenceImageScan<'_>> {
        let label_start = search_from + line[search_from..].find("![")?;
        let label_body_start = label_start + 2;
        let label_end = label_body_start + line[label_body_start..].find(']')?;
        let rest = &line[label_end + 1..];
        if !rest.starts_with('[') {
            return Some(ReferenceImageScan {
                candidate: None,
                next_search_from: label_end + 1,
            });
        }
        let target_end = rest[1..].find(']')?;
        Some(ReferenceImageScan {
            candidate: Some(ReferenceImageCandidate {
                label_start,
                label_end,
                target: &rest[1..1 + target_end],
            }),
            next_search_from: label_end + target_end + 2,
        })
    }

    fn line_contains_shortcut_reference_image(line: &str, definitions: &[String]) -> bool {
        let mut search_from = 0;
        while let Some(label_start) = line[search_from..].find("![") {
            let absolute_start = search_from + label_start;
            let search_from_label = absolute_start + 2;
            let Some(label_end_offset) = line[search_from_label..].find(']') else {
                return false;
            };
            let label_end = search_from_label + label_end_offset;
            let rest = &line[label_end + 1..];
            if !rest.trim().is_empty() {
                search_from = label_end + 1;
                continue;
            }
            let label = Self::normalize_label(&line[search_from_label..label_end]);
            if definitions.iter().any(|definition| definition == &label) {
                return true;
            }
            search_from = label_end + 1;
        }
        false
    }

    fn is_valid_markdown_image_reference(
        line: &str,
        label_start: usize,
        label_end: usize,
        target_label: &str,
        definitions: &[String],
    ) -> bool {
        if !Self::has_valid_label_at(line, label_start, label_end) {
            return false;
        }
        if target_label.is_empty() {
            let label = Self::normalize_label(&line[label_start + 2..label_end]);
            return definitions.iter().any(|definition| definition == &label);
        }
        let target = Self::normalize_label(target_label);
        definitions.iter().any(|definition| definition == &target)
    }

    fn has_valid_label_at(line: &str, label_start: usize, label_end: usize) -> bool {
        let label = &line[label_start + 2..label_end];
        !label.trim().is_empty() && !label.starts_with('^')
    }

    pub(crate) fn reference_definitions(source: &str) -> Vec<String> {
        Self::lines_outside_fences(source)
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
        label
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_ascii_lowercase()
    }
}
