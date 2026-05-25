use crate::export_surface_math::SurfaceMathText;
use katana_markdown_model::{KmmNode, KmmNodeKind};

pub(crate) struct SurfaceTextParser;

impl SurfaceTextParser {
    pub(crate) fn inline_text(node: &KmmNode) -> String {
        match &node.kind {
            KmmNodeKind::Text(text) => Self::decode_basic_entities(&text.text),
            KmmNodeKind::Strong(span)
            | KmmNodeKind::Emphasis(span)
            | KmmNodeKind::Strikethrough(span) => span.text.clone(),
            KmmNodeKind::InlineCode(code) => code.code.clone(),
            KmmNodeKind::InlineHtml(html) => Self::html_fragment_text(&html.html),
            KmmNodeKind::Link(link) => link.label.clone(),
            KmmNodeKind::Image(image) => image.alt.clone(),
            KmmNodeKind::FootnoteReference(reference) => format!("[{}]", reference.label),
            KmmNodeKind::InlineMath(math) => SurfaceMathText::render(&math.expression),
            KmmNodeKind::Emoji(emoji) => emoji.value.clone(),
            _ if node.children.is_empty() => Self::decode_basic_entities(&node.source.raw.text),
            _ => node
                .children
                .iter()
                .map(Self::inline_text)
                .collect::<String>(),
        }
    }

    pub(crate) fn inline_markdown_text(text: &str) -> String {
        let without_links = Self::remove_link_targets(text);
        Self::decode_basic_entities(&without_links)
            .replace("**", "")
            .replace("__", "")
            .replace("~~", "")
            .replace(['`', '*'], "")
    }

    pub(crate) fn html_fragment_text(fragment: &str) -> String {
        let alt_texts = Self::extract_attribute_values(fragment, "alt");
        if !alt_texts.is_empty() {
            return alt_texts.join(" ");
        }
        Self::strip_tags(fragment)
    }

    pub(crate) fn decode_basic_entities(text: &str) -> String {
        text.replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
    }

    fn strip_tags(fragment: &str) -> String {
        let mut text = String::new();
        let mut inside_tag = false;
        for character in fragment.chars() {
            match character {
                '<' => inside_tag = true,
                '>' => {
                    inside_tag = false;
                    text.push(' ');
                }
                _ if !inside_tag => text.push(character),
                _ => {}
            }
        }
        Self::decode_basic_entities(&text).trim().to_string()
    }

    fn remove_link_targets(text: &str) -> String {
        let mut output = String::new();
        let mut rest = text;
        while let Some(open) = rest.find('[') {
            let Some(close) = rest[open + 1..].find(']') else {
                break;
            };
            let label_end = open + 1 + close;
            let after_label = &rest[label_end + 1..];
            if let Some(target) = after_label.strip_prefix('(')
                && let Some(target_end) = target.find(')')
            {
                output.push_str(&rest[..open]);
                output.push_str(&rest[open + 1..label_end]);
                rest = &target[target_end + 1..];
                continue;
            }
            break;
        }
        output.push_str(rest);
        output
    }

    fn extract_attribute_values(fragment: &str, name: &str) -> Vec<String> {
        let mut values = Vec::new();
        let pattern = format!("{name}=\"");
        let mut rest = fragment;
        while let Some(start) = rest.find(&pattern) {
            let value_start = start + pattern.len();
            let value_rest = &rest[value_start..];
            let Some(end) = value_rest.find('"') else {
                break;
            };
            values.push(Self::decode_basic_entities(&value_rest[..end]));
            rest = &value_rest[end + 1..];
        }
        values
    }
}
