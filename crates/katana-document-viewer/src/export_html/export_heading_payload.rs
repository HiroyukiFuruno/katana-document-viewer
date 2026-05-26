use crate::export_inline_payload::InlineHtmlWriter;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{KmmNode, KmmNodeKind};

const MIN_HTML_HEADING_LEVEL: u8 = 1;
const MAX_HTML_HEADING_LEVEL: u8 = 6;

pub(crate) struct HeadingHtmlWriter;

impl HeadingHtmlWriter {
    pub(crate) fn append(
        html: &mut String,
        node: &KmmNode,
        level: u8,
        fallback_text: &str,
        theme: &KdvThemeSnapshot,
    ) {
        let tag_level = level.clamp(MIN_HTML_HEADING_LEVEL, MAX_HTML_HEADING_LEVEL);
        html.push_str(&format!("<h{tag_level}>"));
        if !Self::append_children_without_marker(html, node, theme) {
            InlineHtmlWriter::append_text(html, fallback_text, theme);
        }
        html.push_str(&format!("</h{tag_level}>\n"));
    }

    fn append_children_without_marker(
        html: &mut String,
        node: &KmmNode,
        theme: &KdvThemeSnapshot,
    ) -> bool {
        if node.children.is_empty() {
            return false;
        }
        let mut wrote = false;
        let mut should_strip_marker = true;
        for child in &node.children {
            if should_strip_marker {
                should_strip_marker = false;
                if Self::append_stripped_heading_marker(html, child, theme, &mut wrote) {
                    continue;
                }
            }
            InlineHtmlWriter::append_node(html, child, theme);
            wrote = true;
        }
        wrote
    }

    fn try_strip_heading_marker(node: &KmmNode) -> Option<&str> {
        match &node.kind {
            KmmNodeKind::Text(text) => Self::strip_heading_marker(&text.text),
            _ => None,
        }
    }

    fn append_stripped_heading_marker(
        html: &mut String,
        node: &KmmNode,
        theme: &KdvThemeSnapshot,
        wrote: &mut bool,
    ) -> bool {
        let Some(stripped) = Self::try_strip_heading_marker(node) else {
            return false;
        };
        if !stripped.is_empty() {
            InlineHtmlWriter::append_text(html, stripped, theme);
            *wrote = true;
        }
        true
    }

    fn strip_heading_marker(text: &str) -> Option<&str> {
        let trimmed = text.trim_start();
        let hash_count = trimmed.bytes().take_while(|it| *it == b'#').count();
        if !(MIN_HTML_HEADING_LEVEL as usize..=MAX_HTML_HEADING_LEVEL as usize)
            .contains(&hash_count)
        {
            return None;
        }
        let after_marker = &trimmed[hash_count..];
        Some(after_marker.strip_prefix(' ').unwrap_or(after_marker))
    }
}

#[cfg(test)]
#[path = "export_heading_payload_tests.rs"]
mod tests;
