use super::SurfaceBadge;

const BADGE_COLOR_BLUE: image::Rgba<u8> = image::Rgba([0, 123, 192, 255]);
const BADGE_COLOR_BRIGHT_GREEN: image::Rgba<u8> = image::Rgba([68, 204, 17, 255]);
const BADGE_COLOR_GREEN: image::Rgba<u8> = image::Rgba([76, 175, 80, 255]);
const BADGE_COLOR_RED: image::Rgba<u8> = image::Rgba([224, 49, 49, 255]);
const BADGE_COLOR_ORANGE: image::Rgba<u8> = image::Rgba([245, 159, 0, 255]);
const BADGE_COLOR_YELLOW: image::Rgba<u8> = image::Rgba([250, 176, 5, 255]);
const BADGE_COLOR_DEFAULT: image::Rgba<u8> = image::Rgba([159, 159, 159, 255]);
const PERCENT_ESCAPE_BYTE_LEN: usize = 3;
const HEX_HIGH_NIBBLE_MULTIPLIER: u8 = 16;
const ASCII_HEX_ALPHA_OFFSET: u8 = 10;

pub(super) fn shields_badge(src: &str, link_target: Option<String>) -> Option<SurfaceBadge> {
    let marker = "/badge/";
    let badge_start = src.find(marker)? + marker.len();
    let badge_path = &src[badge_start..];
    let without_extension = badge_path.split('.').next().unwrap_or(badge_path);
    let mut segments = without_extension.split('-');
    let label = decode_badge_segment(segments.next()?);
    let message = decode_badge_segment(segments.next()?);
    let color = badge_color(segments.next().unwrap_or("lightgrey"));
    Some(SurfaceBadge::linked(label, message, color, link_target))
}

fn decode_badge_segment(segment: &str) -> String {
    let decoded = percent_decode(segment);
    decoded.replace('_', " ")
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
    String::from_utf8(output).unwrap_or_else(|_| value.to_string())
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

fn badge_color(color: &str) -> image::Rgba<u8> {
    match color.to_ascii_lowercase().as_str() {
        "blue" => BADGE_COLOR_BLUE,
        "brightgreen" => BADGE_COLOR_BRIGHT_GREEN,
        "green" => BADGE_COLOR_GREEN,
        "red" => BADGE_COLOR_RED,
        "orange" => BADGE_COLOR_ORANGE,
        "yellow" => BADGE_COLOR_YELLOW,
        _ => BADGE_COLOR_DEFAULT,
    }
}

#[cfg(test)]
#[path = "export_surface_markup_badge_tests.rs"]
mod tests;
