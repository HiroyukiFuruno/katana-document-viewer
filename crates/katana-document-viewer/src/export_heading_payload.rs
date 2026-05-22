use crate::export_html_ops::render_text;
use crate::export_inline_payload::InlineHtmlWriter;
use katana_markdown_model::{KmmNode, KmmNodeKind};

const MIN_HTML_HEADING_LEVEL: u8 = 1;
const MAX_HTML_HEADING_LEVEL: u8 = 6;

pub(crate) struct HeadingHtmlWriter;

impl HeadingHtmlWriter {
    pub(crate) fn append(html: &mut String, node: &KmmNode, level: u8, fallback_text: &str) {
        let tag_level = level.clamp(MIN_HTML_HEADING_LEVEL, MAX_HTML_HEADING_LEVEL);
        html.push_str(&format!("<h{tag_level}>"));
        if !Self::append_children_without_marker(html, node) {
            html.push_str(&render_text(fallback_text));
        }
        html.push_str(&format!("</h{tag_level}>\n"));
    }

    fn append_children_without_marker(html: &mut String, node: &KmmNode) -> bool {
        if node.children.is_empty() {
            return false;
        }
        let mut wrote = false;
        let mut should_strip_marker = true;
        for child in &node.children {
            if should_strip_marker {
                should_strip_marker = false;
                if let KmmNodeKind::Text(text) = &child.kind
                    && let Some(stripped) = Self::strip_heading_marker(&text.text)
                {
                    if !stripped.is_empty() {
                        html.push_str(&render_text(stripped));
                        wrote = true;
                    }
                    continue;
                }
            }
            InlineHtmlWriter::append_node(html, child);
            wrote = true;
        }
        wrote
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
