use super::KucNodeFactory;
use super::html_badge_parser::{HtmlBadge, HtmlBadgeRow};
use katana_document_viewer::{
    ViewerHtmlRole, ViewerImageSurface, ViewerImageSurfaceFactory, ViewerNode, ViewerNodeKind,
};
use katana_ui_core::atom::ImageSurface;
use katana_ui_core::layout::AlignCenter;
use katana_ui_core::render_model::UiNode;

const BADGE_ROW_HEIGHT: u32 = 46;
const BADGE_SEGMENT_MIN_WIDTH: u32 = 38;
const BADGE_LABEL_BACKGROUND: &str = "#555555";
const BADGE_TEXT_COLOR: &str = "#ffffff";
const BADGE_CORNER_RADIUS: u32 = 3;
const COMPACT_FONT_SIZE: u16 = 14;
const FULL_FONT_SIZE: u16 = 24;
const COMPACT_BADGE_METRICS: BadgeRenderMetrics = BadgeRenderMetrics {
    height: 19,
    vertical_margin: 13,
    horizontal_gap: 10,
    horizontal_padding: 4,
    char_width: 7,
    text_font_size: 12,
    text_y: 25,
};
const FULL_BADGE_METRICS: BadgeRenderMetrics = BadgeRenderMetrics {
    height: 26,
    vertical_margin: 10,
    horizontal_gap: 10,
    horizontal_padding: 12,
    char_width: 10,
    text_font_size: 18,
    text_y: 30,
};

#[derive(Clone, Copy)]
struct BadgeRenderMetrics {
    height: u32,
    vertical_margin: u32,
    horizontal_gap: u32,
    horizontal_padding: u32,
    char_width: u32,
    text_font_size: u32,
    text_y: u32,
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
            height: interpolate_u32(compact.height, full.height, ratio),
            vertical_margin: interpolate_u32(compact.vertical_margin, full.vertical_margin, ratio),
            horizontal_gap: interpolate_u32(compact.horizontal_gap, full.horizontal_gap, ratio),
            horizontal_padding: interpolate_u32(
                compact.horizontal_padding,
                full.horizontal_padding,
                ratio,
            ),
            char_width: interpolate_u32(compact.char_width, full.char_width, ratio),
            text_font_size: interpolate_u32(compact.text_font_size, full.text_font_size, ratio),
            text_y: interpolate_u32(compact.text_y, full.text_y, ratio),
        }
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
        html_badge_surface_node(node, surface)
    }

    fn badge_render_metrics(&self) -> BadgeRenderMetrics {
        if self.export_surface {
            return BadgeRenderMetrics::full();
        }
        BadgeRenderMetrics::from_preview_font_size(self.typography.preview_font_size)
    }
}

fn html_badge_surface_node(node: &ViewerNode, surface: ViewerImageSurface) -> Option<UiNode> {
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
    Some(AlignCenter::new().child(image).into())
}

fn badge_row_svg(row: &HtmlBadgeRow, surface_width: u32, metrics: BadgeRenderMetrics) -> String {
    let mut x = surface_width.saturating_sub(row_width(row, metrics)) / 2;
    let mut badges = String::new();
    for badge in &row.badges {
        badges.push_str(&badge_svg(badge, x, metrics));
        x += badge_width(badge, metrics) + metrics.horizontal_gap;
    }
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{surface_width}" height="{BADGE_ROW_HEIGHT}" viewBox="0 0 {surface_width} {BADGE_ROW_HEIGHT}">{badges}</svg>"##
    )
}

fn badge_svg(badge: &HtmlBadge, x: u32, metrics: BadgeRenderMetrics) -> String {
    let y = metrics.vertical_margin;
    let label_width = badge_label_width(badge, metrics);
    let message_width = badge_message_width(badge, metrics);
    let width = badge_width(badge, metrics);
    let clip_id = format!("html-badge-clip-{x}");
    format!(
        r##"<g><clipPath id="{clip_id}"><rect x="{x}" y="{y}" width="{width}" height="{height}" rx="{BADGE_CORNER_RADIUS}" ry="{BADGE_CORNER_RADIUS}"/></clipPath><g clip-path="url(#{clip_id})"><rect x="{x}" y="{y}" width="{label_width}" height="{height}" fill="{BADGE_LABEL_BACKGROUND}"/><rect x="{message_x}" y="{y}" width="{message_width}" height="{height}" fill="{color}"/></g>{label}{message}</g>"##,
        message_x = x + label_width,
        height = metrics.height,
        color = badge.color,
        label = badge_text(&badge.label, x + metrics.horizontal_padding, metrics),
        message = badge_text(
            &badge.message,
            x + label_width + metrics.horizontal_padding,
            metrics
        ),
    )
}

fn badge_text(text: &str, x: u32, metrics: BadgeRenderMetrics) -> String {
    format!(
        r#"<text x="{x}" y="{text_y}" font-family="Verdana,-apple-system,BlinkMacSystemFont,Segoe UI,sans-serif" font-size="{font_size}" fill="{BADGE_TEXT_COLOR}">{}</text>"#,
        escape_text(text),
        text_y = metrics.text_y,
        font_size = metrics.text_font_size,
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
    badge_label_width(badge, metrics) + badge_message_width(badge, metrics)
}

fn badge_label_width(badge: &HtmlBadge, metrics: BadgeRenderMetrics) -> u32 {
    badge_segment_width(&badge.label, metrics)
}

fn badge_message_width(badge: &HtmlBadge, metrics: BadgeRenderMetrics) -> u32 {
    if badge.message.is_empty() {
        return 0;
    }
    badge_segment_width(&badge.message, metrics)
}

fn badge_segment_width(label: &str, metrics: BadgeRenderMetrics) -> u32 {
    (label.chars().count() as u32 * metrics.char_width + metrics.horizontal_padding * 2)
        .max(BADGE_SEGMENT_MIN_WIDTH)
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
    use super::{BADGE_ROW_HEIGHT, BadgeRenderMetrics, KucNodeFactory, badge_row_svg, row_width};
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
        assert!(svg.contains(r#"height="26""#));
        assert!(svg.contains(r#"font-size="18""#));
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

        assert_eq!(46, BADGE_ROW_HEIGHT);
        assert_eq!(26, metrics.height);
        assert_eq!(148, row_width(&row, metrics));
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

        assert_eq!(484, row_width(&row, metrics));
        Ok(())
    }

    #[test]
    fn export_surface_badge_metrics_match_kdv_export_surface() {
        let factory = KucNodeFactory::new(&[], 860).export_surface(true);
        let metrics = factory.badge_render_metrics();

        assert_eq!(26, metrics.height);
        assert_eq!(10, metrics.vertical_margin);
        assert_eq!(12, metrics.horizontal_padding);
        assert_eq!(10, metrics.char_width);
        assert_eq!(18, metrics.text_font_size);
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

        assert_eq!(317, row_width(&row, metrics));
        Ok(())
    }

    #[test]
    fn preview_badge_metrics_match_katana_reference_vertical_band() {
        let metrics = BadgeRenderMetrics::from_preview_font_size(14);

        assert_eq!(19, metrics.height);
        assert_eq!(13, metrics.vertical_margin);
        assert_eq!(25, metrics.text_y);
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
