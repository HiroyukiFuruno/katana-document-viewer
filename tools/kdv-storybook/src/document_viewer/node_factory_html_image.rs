use super::KucNodeFactory;
use katana_document_viewer::{
    ArtifactId, HtmlFragmentNormalizer, ViewerImageSurfaceFactory, ViewerNode, ViewerNodeKind,
};
use katana_ui_core::render_model::UiNode;

const SVG_DATA_PREFIXES: [&str; 3] = [
    "data:image/svg+xml,",
    "data:image/svg+xml;charset=utf-8,",
    "data:image/svg+xml;utf8,",
];
const PERCENT_ESCAPE_BYTE_LEN: usize = 3;
const HEX_HIGH_NIBBLE_MULTIPLIER: u8 = 16;
const ASCII_HEX_ALPHA_OFFSET: u8 = 10;

impl<'a> KucNodeFactory<'a> {
    pub(super) fn html_image_node(&self, node: &ViewerNode) -> Option<UiNode> {
        if !matches!(node.kind, ViewerNodeKind::Html { .. }) {
            return None;
        }
        let fragment = HtmlFragmentNormalizer::normalize(&node.source.raw.text);
        let image = HtmlImageRef::parse(&fragment)?;
        let svg = svg_payload(&image.src)?;
        let surface = ViewerImageSurfaceFactory::from_svg_str(
            format!("html-data-image:{}", node.node_id.0),
            &svg,
            image.width.unwrap_or(self.content_width),
        )
        .ok()?;
        Some(self.image_surface_node(
            node,
            &ArtifactId(format!("html-data-image:{}", node.node_id.0)),
            surface,
        ))
    }
}

struct HtmlImageRef {
    src: String,
    width: Option<u32>,
}

impl HtmlImageRef {
    fn parse(raw: &str) -> Option<Self> {
        let image_start = raw.find("<img")?;
        let tag = &raw[image_start..];
        let image_end = tag.find('>')?;
        let image_tag = &tag[..image_end];
        Some(Self {
            src: quoted_attribute_value(image_tag, "src")?,
            width: quoted_attribute_value(image_tag, "width").and_then(|value| value.parse().ok()),
        })
    }
}

fn quoted_attribute_value(tag: &str, name: &str) -> Option<String> {
    let pattern = format!("{name}=\"");
    let start = tag.find(&pattern)? + pattern.len();
    let rest = &tag[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn svg_payload(src: &str) -> Option<String> {
    for prefix in SVG_DATA_PREFIXES {
        if src.len() >= prefix.len() && src[..prefix.len()].eq_ignore_ascii_case(prefix) {
            return percent_decode(&src[prefix.len()..]);
        }
    }
    None
}

fn percent_decode(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && index + 2 < bytes.len()
            && let Some(decoded) = hex_byte(bytes[index + 1], bytes[index + 2])
        {
            output.push(decoded);
            index += PERCENT_ESCAPE_BYTE_LEN;
            continue;
        }
        output.push(bytes[index]);
        index += 1;
    }
    String::from_utf8(output).ok()
}

fn hex_byte(high: u8, low: u8) -> Option<u8> {
    Some(hex_digit(high)? * HEX_HIGH_NIBBLE_MULTIPLIER + hex_digit(low)?)
}

fn hex_digit(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + ASCII_HEX_ALPHA_OFFSET),
        b'A'..=b'F' => Some(value - b'A' + ASCII_HEX_ALPHA_OFFSET),
        _ => None,
    }
}

#[cfg(test)]
#[path = "node_factory_html_image_tests.rs"]
mod tests;
