use super::{
    BADGE_HEIGHT, BADGE_HORIZONTAL_GAP, BADGE_HORIZONTAL_PADDING, BADGE_SEGMENT_MIN_WIDTH,
    BADGE_VERTICAL_MARGIN,
};
use crate::export_surface_helpers::{BODY_MAX_CHARS, SurfaceHelpers, WrappedText};
use crate::export_surface_line::{SurfaceLine, SurfaceTypographyConfig};
use crate::export_surface_span::{SurfaceTextSpan, SurfaceTextStyle};

const BADGE_TEXT_APPROX_CHAR_WIDTH: u32 = 10;
const ALERT_VERTICAL_PADDING: u32 = 32;

pub(crate) struct SurfaceBadgeRowBlock {
    badges: Vec<SurfaceBadge>,
}

impl SurfaceBadgeRowBlock {
    pub(crate) fn new(badges: Vec<SurfaceBadge>) -> Self {
        Self { badges }
    }

    pub(crate) fn height(&self) -> u32 {
        BADGE_HEIGHT + BADGE_VERTICAL_MARGIN * 2
    }

    #[cfg(test)]
    pub(crate) fn text(&self) -> String {
        self.badges
            .iter()
            .map(SurfaceBadge::text)
            .collect::<Vec<_>>()
            .join(" | ")
    }

    pub(crate) fn total_width(&self) -> u32 {
        let badge_widths = self.badges.iter().map(SurfaceBadge::width).sum::<u32>();
        let gap_count = self.badges.len().saturating_sub(1) as u32;
        badge_widths + gap_count * BADGE_HORIZONTAL_GAP
    }

    pub(crate) fn badges(&self) -> &[SurfaceBadge] {
        &self.badges
    }
}

pub(crate) struct SurfaceBadge {
    pub(crate) label: String,
    pub(crate) message: String,
    pub(crate) color: image::Rgba<u8>,
    pub(crate) link_target: Option<String>,
}

impl SurfaceBadge {
    pub(crate) fn linked(
        label: String,
        message: String,
        color: image::Rgba<u8>,
        link_target: Option<String>,
    ) -> Self {
        Self {
            label,
            message,
            color,
            link_target,
        }
    }

    pub(crate) fn single(label: String) -> Self {
        Self {
            label,
            message: String::new(),
            color: SurfaceHelpers::parse_color("#9f9f9f"),
            link_target: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn text(&self) -> String {
        if self.message.is_empty() {
            return self.label.clone();
        }
        format!("{}={}", self.label, self.message)
    }

    pub(crate) fn width(&self) -> u32 {
        self.label_width() + self.message_width()
    }

    pub(crate) fn label_width(&self) -> u32 {
        badge_segment_width(&self.label)
    }

    pub(crate) fn message_width(&self) -> u32 {
        if self.message.is_empty() {
            return 0;
        }
        badge_segment_width(&self.message)
    }
}

fn badge_segment_width(label: &str) -> u32 {
    (label.chars().count() as u32 * BADGE_TEXT_APPROX_CHAR_WIDTH + BADGE_HORIZONTAL_PADDING * 2)
        .max(BADGE_SEGMENT_MIN_WIDTH)
}

pub(crate) struct SurfaceAlertBlock {
    pub(crate) label: String,
    pub(crate) title: SurfaceLine,
    pub(crate) body: Vec<SurfaceLine>,
    pub(crate) quote_depth: u32,
}

#[cfg(test)]
#[path = "export_surface_blocks_badge_alert_tests.rs"]
mod tests;

impl SurfaceAlertBlock {
    pub(crate) fn new(label: &str, body_lines: Vec<String>, quote_depth: u32) -> Self {
        let title = SurfaceLine::body_spans(
            vec![SurfaceTextSpan::styled(
                super::super::markup::alert_label_text(label),
                SurfaceTextStyle::default()
                    .bold()
                    .with_color(super::super::markup::alert_color(label)),
            )],
            0,
        );
        let body = body_lines
            .into_iter()
            .flat_map(|line| WrappedText::new(&line, BODY_MAX_CHARS))
            .map(|line| SurfaceLine::body_with_quote(line, 0))
            .collect();
        Self {
            label: label.to_string(),
            title,
            body,
            quote_depth,
        }
    }

    pub(crate) fn height(&self) -> u32 {
        let body_height = self.body.iter().map(SurfaceLine::line_height).sum::<u32>();
        self.title.line_height() + body_height + ALERT_VERTICAL_PADDING
    }

    pub(crate) fn apply_typography(&mut self, typography: SurfaceTypographyConfig) {
        self.title.apply_typography(typography);
        for line in &mut self.body {
            line.apply_typography(typography);
        }
    }

    #[cfg(test)]
    pub(crate) fn text(&self) -> String {
        let mut parts = vec![self.title.text.clone()];
        parts.extend(self.body.iter().map(|line| line.text.clone()));
        parts.join("\n")
    }
}
