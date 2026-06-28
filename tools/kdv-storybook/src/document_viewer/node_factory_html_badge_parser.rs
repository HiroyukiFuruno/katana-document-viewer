const PERCENT_ESCAPE_BYTE_LEN: usize = 3;
const HEX_HIGH_NIBBLE_MULTIPLIER: u8 = 16;
const ASCII_HEX_ALPHA_OFFSET: u8 = 10;

pub(super) struct HtmlBadgeRow {
    pub(super) badges: Vec<HtmlBadge>,
}

impl HtmlBadgeRow {
    pub(super) fn parse(raw: &str) -> Option<Self> {
        let badges = HtmlImageRef::parse_all(raw)
            .into_iter()
            .filter_map(|image| HtmlBadge::parse(&image.src))
            .collect::<Vec<_>>();
        if badges.is_empty() {
            return None;
        }
        Some(Self { badges })
    }
}

pub(super) struct HtmlBadge {
    pub(super) label: String,
    pub(super) message: String,
    pub(super) color: &'static str,
}

impl HtmlBadge {
    fn parse(src: &str) -> Option<Self> {
        let marker = "/badge/";
        let badge_start = src.find(marker)? + marker.len();
        let badge_path = &src[badge_start..];
        let without_extension = badge_path.split('.').next().unwrap_or(badge_path);
        let mut segments = without_extension.split('-');
        Some(Self {
            label: decode_badge_segment(segments.next()?),
            message: decode_badge_segment(segments.next()?),
            color: badge_color(segments.next().unwrap_or("lightgrey")),
        })
    }
}

struct HtmlImageRef {
    src: String,
}

impl HtmlImageRef {
    fn parse_all(raw: &str) -> Vec<Self> {
        let mut images = Vec::new();
        let mut rest = raw;
        while let Some(image_start) = rest.find("<img") {
            let after_image = &rest[image_start..];
            let Some(image_end) = after_image.find('>') else {
                break;
            };
            let tag = &after_image[..image_end];
            if let Some(src) = quoted_attribute_value(tag, "src") {
                images.push(Self { src });
            }
            rest = &after_image[image_end + 1..];
        }
        images
    }
}

fn quoted_attribute_value(tag: &str, name: &str) -> Option<String> {
    let pattern = format!("{name}=\"");
    let start = tag.find(&pattern)? + pattern.len();
    let rest = &tag[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn decode_badge_segment(segment: &str) -> String {
    percent_decode(segment).replace('_', " ")
}

fn percent_decode(value: &str) -> String {
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
    match String::from_utf8(output) {
        Ok(decoded) => decoded,
        Err(_) => value.to_string(),
    }
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

fn badge_color(color: &str) -> &'static str {
    match color.to_ascii_lowercase().as_str() {
        "blue" => "#007bc0",
        "brightgreen" => "#44cc11",
        "green" => "#4caf50",
        "red" => "#e03131",
        "orange" => "#f59f00",
        "yellow" => "#fab005",
        _ => "#9f9f9f",
    }
}

#[cfg(test)]
mod tests {
    use super::HtmlBadgeRow;

    #[test]
    fn parses_shields_badges_from_html_images() -> Result<(), Box<dyn std::error::Error>> {
        let row = HtmlBadgeRow::parse(
            r#"<p><img src="https://img.shields.io/badge/License-MIT-blue.svg"><img src="https://img.shields.io/badge/CI-passing-brightgreen.svg"></p>"#,
        )
        .ok_or("badge row")?;

        assert_eq!(2, row.badges.len());
        assert_eq!("License", row.badges[0].label);
        assert_eq!("MIT", row.badges[0].message);
        assert_eq!("#007bc0", row.badges[0].color);
        assert_eq!("#44cc11", row.badges[1].color);
        Ok(())
    }
}
