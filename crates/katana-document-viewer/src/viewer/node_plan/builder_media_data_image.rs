use super::super::planned_node::PlannedNode;
use super::super::types::ViewerNodeKind;
use super::{MEDIA_VERTICAL_MARGIN, ViewerMediaHeight};
use crate::ViewerImageSurfaceFactory;
use crate::html_sanitizer::HtmlFragmentNormalizer;

const SVG_DATA_PREFIXES: [&str; 3] = [
    "data:image/svg+xml,",
    "data:image/svg+xml;charset=utf-8,",
    "data:image/svg+xml;utf8,",
];
const PERCENT_ESCAPE_LEN: usize = 3;
const HEX_RADIX: u32 = 16;

pub(super) struct HtmlDataImageHeight;

impl HtmlDataImageHeight {
    pub(super) fn height(planned: &PlannedNode, content_width: u32) -> Option<f32> {
        if !matches!(planned.kind, ViewerNodeKind::Html { .. }) {
            return None;
        }
        if Self::is_broken_katana_svg_data_uri(&planned.source.raw.text) {
            return None;
        }
        let fragment = HtmlFragmentNormalizer::normalize(&planned.source.raw.text);
        let image_tag = Self::first_image_tag(&fragment)?;
        let src = Self::quoted_attribute_value(image_tag, "src")?;
        let svg = Self::svg_payload(&src)?;
        let requested_width = Self::quoted_attribute_value(image_tag, "width")
            .and_then(|value| value.parse::<u32>().ok());
        let max_width = requested_width.unwrap_or(content_width);
        let surface =
            ViewerImageSurfaceFactory::from_svg_str("html-data-image", &svg, max_width).ok()?;
        Some(
            ViewerMediaHeight::scaled_height_to_max_width(
                surface.logical_width(),
                surface.logical_height(),
                max_width,
            ) + MEDIA_VERTICAL_MARGIN,
        )
    }

    fn first_image_tag(fragment: &str) -> Option<&str> {
        let image_start = fragment.find("<img")?;
        let rest = &fragment[image_start..];
        let image_end = rest.find('>')?;
        Some(&rest[..image_end])
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
                return Self::percent_decode(&src[prefix.len()..]);
            }
        }
        None
    }

    fn percent_decode(value: &str) -> Option<String> {
        let mut bytes = Vec::with_capacity(value.len());
        let mut index = 0;
        let source = value.as_bytes();
        while index < source.len() {
            if source[index] == b'%' {
                let encoded = value.get(index + 1..index + PERCENT_ESCAPE_LEN)?;
                let decoded = u8::from_str_radix(encoded, HEX_RADIX).ok()?;
                bytes.push(decoded);
                index += PERCENT_ESCAPE_LEN;
            } else {
                bytes.push(source[index]);
                index += 1;
            }
        }
        String::from_utf8(bytes).ok()
    }

    fn is_broken_katana_svg_data_uri(raw: &str) -> bool {
        raw.contains("data:image/svg+xml") && raw.contains("xmlns=%22<http")
    }
}
