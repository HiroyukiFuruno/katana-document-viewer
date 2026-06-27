use crate::preview_runtime::direct_html_table_normalizer::DirectHtmlTableNormalizer;

pub(crate) struct DirectHtmlNormalizer;

impl DirectHtmlNormalizer {
    pub(crate) fn normalize(content: &str) -> String {
        let lines = content.lines().collect::<Vec<_>>();
        let mut blocks = Vec::new();
        let mut index = 0;
        while index < lines.len() {
            let line = lines[index].trim();
            if line.is_empty() || Self::is_structural_wrapper(line) {
                index += 1;
                continue;
            }
            let (block, next_index) = Self::next_block(&lines, index);
            blocks.push(block);
            index = next_index;
        }
        blocks.join("\n\n")
    }

    fn next_block(lines: &[&str], index: usize) -> (String, usize) {
        let line = lines[index].trim();
        if Self::starts_tag(line, "details") {
            let (block, next_index) = Self::collect_until(lines, index, "</details>");
            return (Self::single_line_html_block(&block), next_index);
        }
        if Self::starts_tag(line, "table") {
            let (block, next_index) = Self::collect_until(lines, index, "</table>");
            return (DirectHtmlTableNormalizer::normalize(&block), next_index);
        }
        (Self::normalize_tag_case(line), index + 1)
    }

    fn collect_until(lines: &[&str], start: usize, closing_tag: &str) -> (String, usize) {
        let mut block = Vec::new();
        let mut index = start;
        while index < lines.len() {
            let line = lines[index].trim();
            if !line.is_empty() && !Self::is_structural_wrapper(line) {
                block.push(Self::normalize_tag_case(line));
            }
            index += 1;
            if line.to_ascii_lowercase().contains(closing_tag) {
                break;
            }
        }
        (block.join("\n"), index)
    }

    fn is_structural_wrapper(line: &str) -> bool {
        let lower = line.to_ascii_lowercase();
        lower == "<main>" || lower.starts_with("<main ") || lower == "</main>"
    }

    fn single_line_html_block(block: &str) -> String {
        block.lines().map(str::trim).collect::<Vec<_>>().join("")
    }

    fn starts_tag(line: &str, tag: &str) -> bool {
        let lower = line.to_ascii_lowercase();
        lower.starts_with(&format!("<{tag}>")) || lower.starts_with(&format!("<{tag} "))
    }

    fn normalize_tag_case(line: &str) -> String {
        let mut normalized = String::with_capacity(line.len());
        let mut characters = line.chars().peekable();
        while let Some(character) = characters.next() {
            normalized.push(character);
            if character != '<' {
                continue;
            }
            if characters.next_if_eq(&'/').is_some() {
                normalized.push('/');
            }
            while let Some(next) = characters.next_if(char::is_ascii_alphanumeric) {
                normalized.push(next.to_ascii_lowercase());
            }
        }
        normalized
    }
}

#[cfg(test)]
#[path = "direct_html_normalizer_tests.rs"]
mod tests;
