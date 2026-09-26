use super::KucNodeFactory;
use super::html_badge_parser::{HtmlBadge, HtmlBadgeRow};
use katana_document_viewer::{
    ViewerHtmlRole, ViewerImageSurface, ViewerImageSurfaceFactory, ViewerNode, ViewerNodeKind,
};
use katana_ui_core::atom::ImageSurface;
use katana_ui_core::layout::{Alignment, Row};
use katana_ui_core::render_model::{UiDimension, UiNode};

const INTERACTIVE_BADGE_ROW_HEIGHT: u32 = 28;
const INTERACTIVE_BADGE_DRAW_HEIGHT: u32 = 20;
const EXPORT_BADGE_ROW_HEIGHT: u32 = 46;
const SHIELDS_BADGE_HEIGHT: u32 = 20;
const SHIELDS_BADGE_HORIZONTAL_PADDING: u32 = 5;
const SHIELDS_BADGE_TEXT_BASELINE: u32 = 14;
const SHIELDS_BADGE_TEXT_SCALE: u32 = 10;
const BADGE_LABEL_BACKGROUND: &str = "#555555";
const BADGE_TEXT_COLOR: &str = "#ffffff";
const BADGE_CORNER_RADIUS: u32 = 3;
const COMPACT_FONT_SIZE: u16 = 14;
const FULL_FONT_SIZE: u16 = 24;
const COMPACT_BADGE_METRICS: BadgeRenderMetrics = BadgeRenderMetrics {
    row_height: INTERACTIVE_BADGE_ROW_HEIGHT,
    surface_height: INTERACTIVE_BADGE_DRAW_HEIGHT,
    height: INTERACTIVE_BADGE_DRAW_HEIGHT,
    vertical_margin: 0,
    horizontal_gap: 4,
};
const FULL_BADGE_METRICS: BadgeRenderMetrics = BadgeRenderMetrics {
    row_height: EXPORT_BADGE_ROW_HEIGHT,
    surface_height: EXPORT_BADGE_ROW_HEIGHT,
    height: 26,
    vertical_margin: 10,
    horizontal_gap: 10,
};

#[derive(Clone, Copy)]
struct BadgeRenderMetrics {
    row_height: u32,
    surface_height: u32,
    height: u32,
    vertical_margin: u32,
    horizontal_gap: u32,
}

impl BadgeRenderMetrics {
    fn from_preview_font_size(font_size: u16) -> Self {
        if font_size <= COMPACT_FONT_SIZE {
            return Self::compact();
        }
        if font_size >= FULL_FONT_SIZE {
            return Self::full();
        }
        Self::interpolate(Self::compact(), Self::full(), font_size)
    }

    fn compact() -> Self {
        COMPACT_BADGE_METRICS
    }

    fn full() -> Self {
        FULL_BADGE_METRICS
    }

    fn interpolate(compact: Self, full: Self, font_size: u16) -> Self {
        let span = (FULL_FONT_SIZE - COMPACT_FONT_SIZE) as f32;
        let ratio = (font_size - COMPACT_FONT_SIZE) as f32 / span;
        Self {
            row_height: interpolate_u32(compact.row_height, full.row_height, ratio),
            height: interpolate_u32(compact.height, full.height, ratio),
            surface_height: interpolate_u32(compact.surface_height, full.surface_height, ratio),
            vertical_margin: interpolate_u32(compact.vertical_margin, full.vertical_margin, ratio),
            horizontal_gap: interpolate_u32(compact.horizontal_gap, full.horizontal_gap, ratio),
        }
    }

    fn shields_scale(self) -> f32 {
        self.height as f32 / SHIELDS_BADGE_HEIGHT as f32
    }
}

impl<'a> KucNodeFactory<'a> {
    pub(super) fn html_badge_row_node(&self, node: &ViewerNode) -> Option<UiNode> {
        if !matches!(
            node.kind,
            ViewerNodeKind::Html {
                role: ViewerHtmlRole::BadgeRow
            }
        ) {
            return None;
        }
        let row = HtmlBadgeRow::parse(&node.source.raw.text)?;
        let metrics = self.badge_render_metrics();
        let width = self.content_width.max(row_width(&row, metrics));
        let svg = badge_row_svg(&row, width, metrics);
        let surface = ViewerImageSurfaceFactory::from_svg_str(
            format!("html-badge-row:{}", node.node_id.0),
            &svg,
            width,
        )
        .ok()?;
        html_badge_surface_node(node, surface, metrics.row_height)
    }

    fn badge_render_metrics(&self) -> BadgeRenderMetrics {
        if self.export_surface {
            return BadgeRenderMetrics::full();
        }
        BadgeRenderMetrics::from_preview_font_size(self.typography.preview_font_size)
    }
}

fn html_badge_surface_node(
    node: &ViewerNode,
    surface: ViewerImageSurface,
    surface_height: u32,
) -> Option<UiNode> {
    let image = ImageSurface::from_rgba(
        "html badge row",
        surface.fingerprint,
        surface.width,
        surface.height,
        surface.rgba,
    )
    .ok()?
    .content_scale(surface.content_scale)
    .accessibility_label(node.text.clone());
    Some(
        UiNode::from(Row::new().align(Alignment::Center).child(image))
            .height(UiDimension::Px(surface_height as u16)),
    )
}

fn badge_row_svg(row: &HtmlBadgeRow, surface_width: u32, metrics: BadgeRenderMetrics) -> String {
    let mut x = surface_width.saturating_sub(row_width(row, metrics)) / 2;
    let mut badges = String::new();
    for badge in &row.badges {
        badges.push_str(&badge_svg(badge, x, metrics));
        x += badge_width(badge, metrics) + metrics.horizontal_gap;
    }
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{surface_width}" height="{surface_height}" viewBox="0 0 {surface_width} {surface_height}">{badges}</svg>"##,
        surface_height = metrics.surface_height,
    )
}

fn badge_svg(badge: &HtmlBadge, x: u32, metrics: BadgeRenderMetrics) -> String {
    let y = metrics.vertical_margin;
    let scale = metrics.shields_scale();
    let label_width = badge_label_width(badge);
    let message_width = badge_message_width(badge);
    let native_width = label_width + message_width;
    let clip_id = format!("html-badge-clip-{x}");
    format!(
        r##"<g transform="translate({x} {y}) scale({scale})"><clipPath id="{clip_id}"><rect width="{native_width}" height="{height}" rx="{BADGE_CORNER_RADIUS}" ry="{BADGE_CORNER_RADIUS}"/></clipPath><g clip-path="url(#{clip_id})"><rect width="{label_width}" height="{height}" fill="{BADGE_LABEL_BACKGROUND}"/><rect x="{label_width}" width="{message_width}" height="{height}" fill="{color}"/></g>{label}{message}</g>"##,
        height = SHIELDS_BADGE_HEIGHT,
        color = badge.color,
        label = badge_text(&badge.label, 1, label_width),
        message = badge_text(&badge.message, label_width.saturating_sub(1), message_width),
    )
}

fn badge_text(text: &str, margin: u32, text_width: u32) -> String {
    if text.is_empty() {
        return String::new();
    }
    let x = SHIELDS_BADGE_TEXT_SCALE / 2 * (margin * 2 + text_width);
    let text_length = (text_width.saturating_sub(SHIELDS_BADGE_HORIZONTAL_PADDING * 2))
        * SHIELDS_BADGE_TEXT_SCALE;
    format!(
        r#"<g fill="{BADGE_TEXT_COLOR}" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110"><g transform="scale(.1)"><text x="{x}" y="{text_y}" textLength="{text_length}">{}</text></g></g>"#,
        escape_text(text),
        text_y = SHIELDS_BADGE_TEXT_BASELINE * SHIELDS_BADGE_TEXT_SCALE,
    )
}

fn row_width(row: &HtmlBadgeRow, metrics: BadgeRenderMetrics) -> u32 {
    let badge_widths = row
        .badges
        .iter()
        .map(|badge| badge_width(badge, metrics))
        .sum::<u32>();
    let gap_count = row.badges.len().saturating_sub(1) as u32;
    badge_widths + gap_count * metrics.horizontal_gap
}

fn badge_width(badge: &HtmlBadge, metrics: BadgeRenderMetrics) -> u32 {
    scale_shields_width(
        badge_label_width(badge) + badge_message_width(badge),
        metrics,
    )
}

fn badge_label_width(badge: &HtmlBadge) -> u32 {
    if badge.label.is_empty() {
        return 0;
    }
    badge_segment_width(&badge.label)
}

fn badge_message_width(badge: &HtmlBadge) -> u32 {
    badge_segment_width(&badge.message)
}

fn badge_segment_width(label: &str) -> u32 {
    shields_text_width(label).saturating_add(SHIELDS_BADGE_HORIZONTAL_PADDING * 2)
}

fn scale_shields_width(width: u32, metrics: BadgeRenderMetrics) -> u32 {
    (width as f32 * metrics.shields_scale()).round() as u32
}

fn shields_text_width(text: &str) -> u32 {
    let width = text.chars().map(shields_glyph_width).sum::<f32>().floor() as u32;
    if width.is_multiple_of(2) {
        width.saturating_add(1)
    } else {
        width
    }
}

fn shields_glyph_width(character: char) -> f32 {
    match character {
        ' ' => 3.87,
        '!' => 4.33,
        '"' => 5.05,
        '#' => 9.0,
        '$' => 6.99,
        '%' => 11.84,
        '&' => 7.99,
        '\'' => 2.95,
        '(' | ')' | '[' | '\\' | ']' | '{' | '|' | '}' => 5.0,
        '*' => 6.99,
        '+' | '<' | '=' | '>' | '^' | '~' => 9.0,
        ',' | '.' => 4.0,
        '-' => 5.0,
        '/' => 5.0,
        '0'..='9' => 6.99,
        ':' | ';' => 5.0,
        '?' => 6.0,
        '@' => 11.0,
        'A' | 'S' => 7.52,
        'B' | 'X' => 7.54,
        'C' => 7.68,
        'D' => 8.48,
        'E' => 6.96,
        'F' => 6.32,
        'G' => 8.53,
        'H' => 8.27,
        'I' => 4.63,
        'J' => 5.0,
        'K' => 7.62,
        'L' => 6.12,
        'M' => 9.27,
        'N' => 8.23,
        'O' | 'Q' => 8.66,
        'P' => 6.63,
        'R' => 7.65,
        'T' => 6.78,
        'U' => 8.05,
        'V' => 7.52,
        'W' => 10.88,
        'Y' => 6.77,
        'Z' => 7.54,
        '_' | '`' => 6.99,
        'a' => 6.61,
        'b' | 'd' | 'g' | 'p' | 'q' => 6.85,
        'c' | 's' => 5.73,
        'e' => 6.55,
        'f' => 3.87,
        'h' | 'n' | 'u' => 6.96,
        'i' | 'l' => 3.02,
        'j' => 3.79,
        'k' | 'v' | 'x' | 'y' => 6.51,
        'm' => 10.7,
        'o' => 6.68,
        'r' => 4.69,
        't' => 4.33,
        'w' => 9.0,
        'z' => 5.78,
        _ => 10.7,
    }
}

fn interpolate_u32(start: u32, end: u32, ratio: f32) -> u32 {
    (start as f32 + (end as f32 - start as f32) * ratio).round() as u32
}

fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::super::html_badge_parser::HtmlBadgeRow;
    use super::{
        BadgeRenderMetrics, EXPORT_BADGE_ROW_HEIGHT, KucNodeFactory, badge_row_svg, row_width,
        shields_text_width,
    };
    use katana_document_viewer::{ViewerImageSurface, ViewerImageSurfaceFactory};

    #[test]
    fn renders_badge_row_as_svg() -> Result<(), Box<dyn std::error::Error>> {
        let row =
            HtmlBadgeRow::parse(r#"<img src="https://img.shields.io/badge/License-MIT-blue.svg">"#)
                .ok_or("badge row")?;
        let metrics = BadgeRenderMetrics::from_preview_font_size(24);

        let svg = badge_row_svg(&row, row_width(&row, metrics), metrics);

        assert!(svg.contains("License"));
        assert!(svg.contains("#007bc0"));
        assert!(svg.contains("MIT"));
        assert!(svg.contains(r#"height="46""#));
        assert!(svg.contains(r#"scale(1.3)"#));
        assert!(svg.contains(r#"font-size="110""#));
        assert!(svg.contains(r#"textLength="410""#));
        assert!(svg.contains("<clipPath"));
        assert!(svg.contains(r#"rx="3""#));
        assert!(!svg.contains("stroke="));
        Ok(())
    }

    #[test]
    fn badge_surface_uses_shields_like_dimensions() -> Result<(), Box<dyn std::error::Error>> {
        let row =
            HtmlBadgeRow::parse(r#"<img src="https://img.shields.io/badge/License-MIT-blue.svg">"#)
                .ok_or("badge row")?;
        let metrics = BadgeRenderMetrics::from_preview_font_size(24);

        assert_eq!(46, EXPORT_BADGE_ROW_HEIGHT);
        assert_eq!(26, metrics.height);
        assert_eq!(107, row_width(&row, metrics));
        Ok(())
    }

    #[test]
    fn sample_fixture_badge_row_matches_katana_export_width()
    -> Result<(), Box<dyn std::error::Error>> {
        let row = HtmlBadgeRow::parse(
            r##"<a href="#"><img src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
<a href="#"><img src="https://img.shields.io/badge/CI-passing-brightgreen.svg"></a>
<a href="#"><img src="https://img.shields.io/badge/platform-macOS-lightgrey"></a>"##,
        )
        .ok_or("badge row")?;
        let metrics = BadgeRenderMetrics::from_preview_font_size(24);

        assert_eq!(361, row_width(&row, metrics));
        Ok(())
    }

    #[test]
    fn export_surface_badge_metrics_match_kdv_export_surface() {
        let factory = KucNodeFactory::new(&[], 860).export_surface(true);
        let metrics = factory.badge_render_metrics();

        assert_eq!(26, metrics.height);
        assert_eq!(10, metrics.vertical_margin);
    }

    #[test]
    fn sample_fixture_badge_row_matches_katana_preview_width()
    -> Result<(), Box<dyn std::error::Error>> {
        let row = HtmlBadgeRow::parse(
            r##"<a href="#"><img src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
<a href="#"><img src="https://img.shields.io/badge/CI-passing-brightgreen.svg"></a>
<a href="#"><img src="https://img.shields.io/badge/platform-macOS-lightgrey"></a>"##,
        )
        .ok_or("badge row")?;
        let metrics = BadgeRenderMetrics::from_preview_font_size(14);

        assert_eq!(270, row_width(&row, metrics));
        Ok(())
    }

    #[test]
    fn preview_badge_metrics_match_katana_reference_vertical_band() {
        let metrics = BadgeRenderMetrics::from_preview_font_size(14);

        assert_eq!(20, metrics.height);
        assert_eq!(0, metrics.vertical_margin);
        assert_eq!(20, metrics.height);
    }

    #[test]
    fn shields_width_metrics_match_the_source_svg_for_mixed_glyphs()
    -> Result<(), Box<dyn std::error::Error>> {
        let row = HtmlBadgeRow::parse(
            r##"<img src="https://img.shields.io/badge/License-MIT-blue.svg">
<img src="https://img.shields.io/badge/CI-passing-brightgreen.svg">
<img src="https://img.shields.io/badge/platform-macOS-lightgrey.svg">"##,
        )
        .ok_or("badge row")?;
        let metrics = BadgeRenderMetrics::from_preview_font_size(14);
        let svg = badge_row_svg(&row, row_width(&row, metrics), metrics);

        assert!(svg.contains(r##"<rect width="51" height="20" fill="#555555"/>"##));
        assert!(svg.contains(r##"<rect x="51" width="31" height="20" fill="#007bc0"/>"##));
        assert!(svg.contains(r##"<rect width="23" height="20" fill="#555555"/>"##));
        assert!(svg.contains(r##"<rect x="23" width="51" height="20" fill="#44cc11"/>"##));
        assert!(svg.contains(r##"<rect width="57" height="20" fill="#555555"/>"##));
        assert!(svg.contains(r##"<rect x="57" width="49" height="20" fill="#9f9f9f"/>"##));
        assert_eq!(11, shields_text_width("\u{10ffff}"));
        Ok(())
    }

    #[test]
    fn badge_svg_escapes_text_while_preserving_shields_text_length()
    -> Result<(), Box<dyn std::error::Error>> {
        let row = HtmlBadgeRow::parse(
            r#"<img src="https://img.shields.io/badge/R%26D-1.0%25-blue.svg">"#,
        )
        .ok_or("badge row")?;
        let metrics = BadgeRenderMetrics::from_preview_font_size(14);
        let svg = badge_row_svg(&row, row_width(&row, metrics), metrics);

        assert!(svg.contains("R&amp;D"));
        assert!(svg.contains("1.0%"));
        assert!(svg.contains("textLength"));
        Ok(())
    }

    #[test]
    fn badge_segment_presence_matches_badge_maker_for_empty_text()
    -> Result<(), Box<dyn std::error::Error>> {
        let message_only =
            HtmlBadgeRow::parse(r#"<img src="https://img.shields.io/badge/-message-blue.svg">"#)
                .ok_or("message-only badge row")?;
        let empty_message =
            HtmlBadgeRow::parse(r#"<img src="https://img.shields.io/badge/label--blue.svg">"#)
                .ok_or("empty-message badge row")?;
        let metrics = BadgeRenderMetrics::from_preview_font_size(14);
        let svg = badge_row_svg(&empty_message, row_width(&empty_message, metrics), metrics);

        assert_eq!(59, row_width(&message_only, metrics));
        assert_eq!(48, row_width(&empty_message, metrics));
        assert!(!svg.contains(r#"textLength="10"></text>"#));
        Ok(())
    }

    #[test]
    fn badge_surface_has_rounded_transparent_corners() -> Result<(), Box<dyn std::error::Error>> {
        let row =
            HtmlBadgeRow::parse(r#"<img src="https://img.shields.io/badge/License-MIT-blue.svg">"#)
                .ok_or("badge row")?;
        let metrics = BadgeRenderMetrics::from_preview_font_size(14);
        let width = row_width(&row, metrics);
        let svg = badge_row_svg(&row, width, metrics);
        let surface = ViewerImageSurfaceFactory::from_svg_str("badge-test", &svg, width)?;

        assert!(
            alpha_at(&surface, 0, metrics.vertical_margin) < 128,
            "badge outer corner should be transparent after rounded clipping"
        );
        assert!(
            alpha_at(&surface, 4, metrics.vertical_margin + metrics.height / 2) > 200,
            "badge body should remain opaque after rounded clipping"
        );
        Ok(())
    }

    fn alpha_at(surface: &ViewerImageSurface, x: u32, y: u32) -> u8 {
        let scaled_x = logical_to_surface_pixel(x, surface.content_scale);
        let scaled_y = logical_to_surface_pixel(y, surface.content_scale);
        let offset = ((scaled_y * surface.width + scaled_x) * 4 + 3) as usize;
        surface.rgba[offset]
    }

    fn logical_to_surface_pixel(value: u32, content_scale: u32) -> u32 {
        value.saturating_mul(content_scale.max(1)) / 100
    }
}
