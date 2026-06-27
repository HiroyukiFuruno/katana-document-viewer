use super::types::{ViewerHtmlRole, ViewerNodeKind};
use crate::viewer::code_block_metrics::ViewerCodeBlockMetrics;
use crate::viewer::settings_update::ViewerTypographyConfig;

const BASE_BODY_FONT_SIZE: f32 = 24.0;
const HEADING_1_BLOCK_HEIGHT: f32 = 92.0;
const HEADING_2_BLOCK_HEIGHT: f32 = 78.0;
const HEADING_DEFAULT_BLOCK_HEIGHT: f32 = 66.0;
const DIAGRAM_BLOCK_HEIGHT: f32 = 180.0;
const MEDIA_BLOCK_HEIGHT: f32 = 160.0;
const BADGE_ROW_BLOCK_HEIGHT: f32 = 46.0;
const SURFACE_CONTENT_WIDTH: usize = 1168;
const RULE_BLOCK_HEIGHT: f32 = 34.0;
const BODY_BLOCK_HEIGHT: f32 = 46.0;
const BODY_LINE_HEIGHT: f32 = 46.0;
const ALERT_VERTICAL_PADDING: f32 = 32.0;
const COMPACT_BODY_FONT_SIZE: f32 = 14.0;
const COMPACT_HEADING_1_BLOCK_HEIGHT: f32 = 43.0;
const COMPACT_HEADING_2_BLOCK_HEIGHT: f32 = 34.0;
const COMPACT_HEADING_DEFAULT_BLOCK_HEIGHT: f32 = 30.0;
const COMPACT_LONG_HEADING_2_EXTRA_HEIGHT: f32 = 13.0;
const LONG_HEADING_2_MIN_CHARS: usize = 64;
const COMPACT_BODY_BLOCK_HEIGHT: f32 = 20.0;
const COMPACT_BODY_LINE_HEIGHT: f32 = 23.0;
const BODY_MAX_CHARS: usize = 58;
const TEXT_BLOCK_PADDING: f32 = 0.0;

pub(super) struct ViewerNodeMetrics;

impl ViewerNodeMetrics {
    pub(super) fn block_height(
        kind: &ViewerNodeKind,
        text: &str,
        typography: ViewerTypographyConfig,
    ) -> f32 {
        Self::block_height_with_width(kind, text, typography, SURFACE_CONTENT_WIDTH)
    }

    pub(super) fn block_height_with_width(
        kind: &ViewerNodeKind,
        text: &str,
        typography: ViewerTypographyConfig,
        content_width: usize,
    ) -> f32 {
        match kind {
            ViewerNodeKind::Heading { level } => {
                Self::markdown_heading_height(*level, text, typography)
            }
            ViewerNodeKind::Code { .. } => Self::code_block_height(text, typography),
            ViewerNodeKind::Math => Self::code_block_height(text, typography),
            ViewerNodeKind::Diagram { .. } => DIAGRAM_BLOCK_HEIGHT,
            ViewerNodeKind::Table => Self::table_block_height(text, typography, content_width),
            ViewerNodeKind::Image => MEDIA_BLOCK_HEIGHT,
            ViewerNodeKind::Html { role } => {
                Self::html_block_height(*role, text, typography, content_width)
            }
            ViewerNodeKind::Alert { label } => {
                Self::alert_block_height(label, text, typography, content_width)
            }
            ViewerNodeKind::Rule => RULE_BLOCK_HEIGHT,
            _ => Self::wrapped_body_height(text, typography, content_width),
        }
    }

    pub(super) fn body_line_height(typography: ViewerTypographyConfig) -> f32 {
        Self::scaled_text_height(BODY_LINE_HEIGHT, COMPACT_BODY_LINE_HEIGHT, typography)
    }

    #[cfg(test)]
    pub(super) fn default_typography() -> ViewerTypographyConfig {
        ViewerTypographyConfig {
            preview_font_size: BASE_BODY_FONT_SIZE as u16,
        }
    }

    fn html_block_height(
        role: ViewerHtmlRole,
        text: &str,
        typography: ViewerTypographyConfig,
        content_width: usize,
    ) -> f32 {
        match role {
            ViewerHtmlRole::Heading { level, .. } => Self::heading_height(level, typography),
            ViewerHtmlRole::BadgeRow => BADGE_ROW_BLOCK_HEIGHT,
            ViewerHtmlRole::Accordion => Self::body_line_height(typography) * 2.0,
            _ => Self::wrapped_body_height(text, typography, content_width),
        }
    }

    fn code_block_height(text: &str, typography: ViewerTypographyConfig) -> f32 {
        ViewerCodeBlockMetrics::block_height(text, typography)
    }

    fn heading_height(level: u8, typography: ViewerTypographyConfig) -> f32 {
        let (default_height, compact_height) = match level {
            1 => (HEADING_1_BLOCK_HEIGHT, COMPACT_HEADING_1_BLOCK_HEIGHT),
            2 => (HEADING_2_BLOCK_HEIGHT, COMPACT_HEADING_2_BLOCK_HEIGHT),
            _ => (
                HEADING_DEFAULT_BLOCK_HEIGHT,
                COMPACT_HEADING_DEFAULT_BLOCK_HEIGHT,
            ),
        };
        Self::scaled_text_height(default_height, compact_height, typography)
    }

    fn markdown_heading_height(level: u8, text: &str, typography: ViewerTypographyConfig) -> f32 {
        let base_height = Self::heading_height(level, typography);
        if level != 2 || text.chars().count() < LONG_HEADING_2_MIN_CHARS {
            return base_height;
        }
        base_height
            + COMPACT_LONG_HEADING_2_EXTRA_HEIGHT
                * (f32::from(typography.preview_font_size) / COMPACT_BODY_FONT_SIZE)
    }

    fn wrapped_body_height(
        text: &str,
        typography: ViewerTypographyConfig,
        content_width: usize,
    ) -> f32 {
        let max_chars = Self::body_max_chars(content_width);
        let line_count = text
            .lines()
            .map(|line| Self::wrapped_body_line_count(line, max_chars))
            .sum::<usize>()
            .max(1) as f32;
        let body_block_height =
            Self::scaled_text_height(BODY_BLOCK_HEIGHT, COMPACT_BODY_BLOCK_HEIGHT, typography);
        let line_height = Self::body_line_height(typography);
        (line_count * line_height + TEXT_BLOCK_PADDING).max(body_block_height)
    }

    fn alert_block_height(
        label: &str,
        text: &str,
        typography: ViewerTypographyConfig,
        content_width: usize,
    ) -> f32 {
        let max_chars = Self::body_max_chars(content_width);
        let body = Self::alert_body_text(label, text);
        let body_line_count = Self::alert_body_line_count(body, max_chars);
        let line_count = body_line_count + 1;
        line_count as f32 * Self::body_line_height(typography) + ALERT_VERTICAL_PADDING
    }

    fn alert_body_line_count(body: &str, max_chars: usize) -> usize {
        if body.trim().is_empty() {
            return 0;
        }
        body.lines()
            .map(|line| Self::wrapped_body_line_count(line, max_chars))
            .sum()
    }

    fn alert_body_text<'a>(label: &str, text: &'a str) -> &'a str {
        let prefix = text.get(..label.len());
        let suffix = text.get(label.len()..);
        if let (Some(prefix), Some(suffix)) = (prefix, suffix)
            && prefix.eq_ignore_ascii_case(label)
            && let Some(body) = suffix.strip_prefix(':')
        {
            return body.trim_start();
        }
        if let Some((_, body)) = text.split_once('\n') {
            return body;
        }
        text
    }

    fn wrapped_body_line_count(line: &str, max_chars: usize) -> usize {
        line.chars().count().div_ceil(max_chars).max(1)
    }

    fn body_max_chars(content_width: usize) -> usize {
        content_width
            .saturating_mul(BODY_MAX_CHARS)
            .checked_div(SURFACE_CONTENT_WIDTH)
            .unwrap_or(BODY_MAX_CHARS)
            .max(1)
    }

    fn body_scale(typography: ViewerTypographyConfig) -> f32 {
        f32::from(typography.preview_font_size) / BASE_BODY_FONT_SIZE
    }

    fn scaled_text_height(
        default_height: f32,
        compact_height: f32,
        typography: ViewerTypographyConfig,
    ) -> f32 {
        let font_size = f32::from(typography.preview_font_size);
        if font_size <= COMPACT_BODY_FONT_SIZE {
            return compact_height;
        }
        if font_size >= BASE_BODY_FONT_SIZE {
            return default_height * Self::body_scale(typography);
        }
        let t =
            (font_size - COMPACT_BODY_FONT_SIZE) / (BASE_BODY_FONT_SIZE - COMPACT_BODY_FONT_SIZE);
        compact_height + (default_height - compact_height) * t
    }

    fn code_scale(typography: ViewerTypographyConfig) -> f32 {
        ViewerCodeBlockMetrics::code_scale(typography)
    }
}

#[path = "metrics_table.rs"]
mod table;
