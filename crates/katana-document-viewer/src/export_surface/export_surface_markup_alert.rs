use crate::export_surface_text::SurfaceTextParser;
use katana_markdown_model::KmmNode;

const ALERT_COLOR_TIP: image::Rgba<u8> = image::Rgba([26, 127, 55, 255]);
const ALERT_COLOR_IMPORTANT: image::Rgba<u8> = image::Rgba([130, 80, 223, 255]);
const ALERT_COLOR_WARNING: image::Rgba<u8> = image::Rgba([191, 135, 0, 255]);
const ALERT_COLOR_CAUTION: image::Rgba<u8> = image::Rgba([209, 36, 47, 255]);
const ALERT_COLOR_DEFAULT: image::Rgba<u8> = image::Rgba([9, 105, 218, 255]);

pub(in crate::export_surface) fn alert_title(label: &str) -> &str {
    match label {
        "TIP" => "Tip",
        "IMPORTANT" => "Important",
        "WARNING" => "Warning",
        "CAUTION" => "Caution",
        _ => "Note",
    }
}

pub(in crate::export_surface) fn alert_label_text(label: &str) -> String {
    alert_title(label).to_string()
}

#[cfg(test)]
pub(in crate::export_surface) fn alert_icon_name(label: &str) -> &str {
    match label {
        "TIP" => "tip-bulb",
        "IMPORTANT" => "important-callout",
        "WARNING" => "warning-triangle",
        "CAUTION" => "caution-circle-slash",
        _ => "note-circle",
    }
}

pub(in crate::export_surface) fn alert_body_lines(node: &KmmNode, label: &str) -> Vec<String> {
    let child_lines = alert_child_body_lines(node, label);
    if child_lines.is_empty() {
        return alert_raw_body_lines(node, label);
    }
    child_lines
}

fn alert_child_body_lines(node: &KmmNode, label: &str) -> Vec<String> {
    let title = alert_title(label);
    node.children
        .iter()
        .map(SurfaceTextParser::inline_text)
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
        .enumerate()
        .filter_map(|(index, text)| alert_child_line(index, text, title))
        .collect()
}

fn alert_child_line(index: usize, text: String, title: &str) -> Option<String> {
    if index == 0 && text == title {
        None
    } else {
        Some(text)
    }
}

fn alert_raw_body_lines(node: &KmmNode, label: &str) -> Vec<String> {
    node.source
        .raw
        .text
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix('>'))
        .map(str::trim)
        .filter_map(|line| alert_body_line_from_quote(line, label))
        .filter(|line| !line.is_empty())
        .map(SurfaceTextParser::inline_markdown_text)
        .collect()
}

fn alert_body_line_from_quote<'a>(line: &'a str, label: &str) -> Option<&'a str> {
    let Some(marker) = line.strip_prefix("[!") else {
        return Some(line);
    };
    let Some((raw_label, rest)) = marker.split_once(']') else {
        return Some(line);
    };
    if !raw_label.eq_ignore_ascii_case(label) {
        return Some(line);
    }
    let body = rest.trim();
    (!body.is_empty()).then_some(body)
}

pub(in crate::export_surface) fn alert_color(label: &str) -> image::Rgba<u8> {
    match label {
        "TIP" => ALERT_COLOR_TIP,
        "IMPORTANT" => ALERT_COLOR_IMPORTANT,
        "WARNING" => ALERT_COLOR_WARNING,
        "CAUTION" => ALERT_COLOR_CAUTION,
        _ => ALERT_COLOR_DEFAULT,
    }
}

#[cfg(test)]
#[path = "export_surface_markup_alert_private_tests.rs"]
mod tests;
