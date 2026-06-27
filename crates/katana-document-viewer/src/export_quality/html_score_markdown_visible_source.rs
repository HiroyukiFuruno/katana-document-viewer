use super::html_score_markdown_blocks_helpers::{
    is_external_block_fence, starts_with_bullet_list, starts_with_ordered_list,
};
use super::html_score_markdown_heading::{
    source_has_heading, source_heading_count, source_is_setext_marker_line,
};

pub(super) struct VisibleMarkdownSource {
    lines: Vec<String>,
    code_block_count: usize,
    syntax_code_block_count: usize,
}

impl VisibleMarkdownSource {
    pub(super) fn from(source: &str) -> Self {
        let mut lines = Vec::new();
        let mut inside_fence = false;
        let mut code_block_count = 0;
        let mut syntax_code_block_count = 0;
        for line in source.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                if !inside_fence && !is_external_block_fence(trimmed) {
                    code_block_count += 1;
                    syntax_code_block_count += usize::from(fence_language_requires_syntax(trimmed));
                }
                inside_fence = !inside_fence;
                continue;
            }
            if !inside_fence {
                lines.push(line.to_string());
            }
        }
        Self {
            lines,
            code_block_count,
            syntax_code_block_count,
        }
    }

    pub(super) fn has_heading(&self) -> bool {
        source_has_heading(&self.lines)
    }

    pub(super) fn heading_count(&self) -> usize {
        source_heading_count(&self.lines)
    }

    pub(super) fn has_list(&self) -> bool {
        self.list_item_count() > 0
    }

    pub(super) fn list_item_count(&self) -> usize {
        self.lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim_start();
                starts_with_bullet_list(trimmed) || starts_with_ordered_list(trimmed)
            })
            .count()
    }

    pub(super) fn has_nested_list(&self) -> bool {
        self.lines.iter().any(|line| {
            let trimmed = line.trim_start();
            line.len() > trimmed.len()
                && (starts_with_bullet_list(trimmed) || starts_with_ordered_list(trimmed))
        })
    }

    pub(super) fn has_blockquote(&self) -> bool {
        let mut inside_alert = false;
        for line in &self.lines {
            let trimmed = line.trim_start();
            if trimmed.is_empty() || !trimmed.starts_with('>') {
                inside_alert = false;
                continue;
            }
            let quoted = trimmed.trim_start_matches('>').trim_start();
            if quoted.starts_with("[!") {
                inside_alert = true;
                continue;
            }
            if !inside_alert {
                return true;
            }
        }
        false
    }

    pub(super) fn has_table(&self) -> bool {
        self.table_count() > 0
    }

    pub(super) fn table_count(&self) -> usize {
        self.lines
            .windows(2)
            .filter(|window| {
                window[0].contains('|')
                    && window[1].contains('|')
                    && window[1].chars().any(|character| character == '-')
            })
            .count()
    }

    pub(super) fn has_code_block(&self) -> bool {
        self.code_block_count > 0
    }

    pub(super) fn code_block_count(&self) -> usize {
        self.code_block_count
    }

    pub(super) fn has_syntax_code_block(&self) -> bool {
        self.syntax_code_block_count > 0
    }

    pub(super) fn has_rule(&self) -> bool {
        self.lines.iter().enumerate().any(|(index, line)| {
            let trimmed = line.trim();
            !source_is_setext_marker_line(&self.lines, index)
                && trimmed.len() >= 3
                && (trimmed.chars().all(|character| character == '-')
                    || trimmed.chars().all(|character| character == '*')
                    || trimmed.chars().all(|character| character == '_'))
        })
    }

    pub(super) fn has_footnote(&self) -> bool {
        self.lines.iter().any(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with("[^") && trimmed.contains("]:")
        })
    }

    pub(super) fn has_details_html(&self) -> bool {
        self.lines
            .iter()
            .any(|line| line.to_ascii_lowercase().contains("<details"))
    }
}

fn fence_language_requires_syntax(trimmed: &str) -> bool {
    let language = trimmed
        .trim_start_matches('`')
        .trim_start_matches('~')
        .trim()
        .to_ascii_lowercase();
    !language.is_empty() && !matches!(language.as_str(), "text" | "txt" | "plain")
}
