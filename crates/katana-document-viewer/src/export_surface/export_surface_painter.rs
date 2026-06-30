use crate::export_surface_font::{SurfaceTextLayout, SurfaceTextPainter};
use crate::export_surface_helpers::{
    PAGE_PADDING, QUOTE_INDENT, SURFACE_CONTENT_WIDTH, SURFACE_PAGE_HEIGHT, SURFACE_WIDTH,
    SurfaceHelpers,
};
use crate::export_surface_line::{
    LIST_MARKER_COLUMN_WIDTH, SurfaceLine, SurfaceLineMarker, SurfaceTaskMarker,
};
use crate::export_surface_span::SurfaceTextSpan;
use crate::theme::KdvThemeSnapshot;
use crate::viewer::ViewerCodeBlockMetrics;
use image::RgbaImage;

use super::icons::{
    alert_title_icon_y, draw_caution_icon, draw_check_mark, draw_diagonal_mark, draw_filled_circle,
    draw_important_icon, draw_note_icon, draw_stroked_circle, draw_tip_icon, draw_warning_icon,
};
use super::markup::alert_color;
use super::page_plan::SurfacePagePlan;
use super::{
    SurfaceAlertBlock, SurfaceBadge, SurfaceBadgeRowBlock, SurfaceBlock, SurfaceCodeBlock,
    SurfaceDiagramBlock, SurfaceImageBlock, SurfaceLinkAnchor, SurfaceLinkAnnotation,
    SurfaceMathBlock, SurfaceSpanMetrics, SurfaceTableBlock, SurfaceTableCellPaint,
    SurfaceTableLayout,
};
use crate::export_surface_svg::SurfaceSvgImage;

const CODE_HORIZONTAL_PADDING: u32 = 24;
const CODE_VERTICAL_PADDING: u32 = ViewerCodeBlockMetrics::VERTICAL_PADDING_PX;
const CODE_BLOCK_MARGIN: u32 = ViewerCodeBlockMetrics::BLOCK_MARGIN_PX;
const CODE_LINE_BULLET_X_OFFSET: u32 = 14;
const CODE_LINE_BULLET_FILLED_Y_OFFSET: u32 = 17;
const CODE_LINE_BULLET_FILLED_RADIUS: u32 = 4;
const CODE_LINE_BULLET_STROKED_Y_OFFSET: u32 = 17;
const CODE_LINE_BULLET_STROKED_RADIUS: u32 = 5;
const CODE_LINE_BULLET_RECT_X_OFFSET: u32 = 10;
const CODE_LINE_BULLET_RECT_Y_OFFSET: u32 = 13;
const CODE_LINE_BULLET_RECT_SIZE: u32 = 8;

const DIAGRAM_VERTICAL_MARGIN: u32 = 18;
const TABLE_CELL_PADDING: u32 = 16;
const RULE_HEIGHT: u32 = 34;

const BADGE_HEIGHT: u32 = 26;
const BADGE_VERTICAL_MARGIN: u32 = 10;
const BADGE_HORIZONTAL_GAP: u32 = 10;
const BADGE_HORIZONTAL_PADDING: u32 = 12;
const BADGE_TEXT_FONT_SIZE: f32 = 18.0;
const BADGE_TEXT_Y_OFFSET: u32 = 2;
const BADGE_LABEL_BACKGROUND: image::Rgba<u8> = image::Rgba([85, 85, 85, 255]);
const BADGE_TEXT_COLOR: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);

const IMAGE_VERTICAL_MARGIN: u32 = 18;

const MATH_VERTICAL_MARGIN: u32 = 18;
const TASK_MARKER_SIZE: u32 = 18;
const TASK_MARKER_BOX_OFFSET: u32 = 4;
const TASK_MARKER_INLINE_OFFSET: u32 = 4;
const TASK_MARKER_PROGRESS_STROKE: u32 = 3;

const ALERT_PANEL_PADDING_X: u32 = 28;
const ALERT_PANEL_PADDING_Y: u32 = 16;
const ALERT_PANEL_BORDER_WIDTH: u32 = 5;
const ALERT_PANEL_TITLE_X_OFFSET: u32 = 30;
const ALERT_PANEL_BODY_X_OFFSET: u32 = 28;
const ALERT_PANEL_TEXT_Y_STEP: u32 = 4;
const LINE_CENTERED_TEXT_GUESS_CHAR_WIDTH: u32 = 14;

const RULE_STROKE_WIDTH: u32 = 2;
pub(crate) struct SurfacePaintPalette {
    pub(crate) text: image::Rgba<u8>,
    pub(crate) quote: image::Rgba<u8>,
    pub(crate) code_background: image::Rgba<u8>,
    pub(crate) code_border: image::Rgba<u8>,
    pub(crate) table_border: image::Rgba<u8>,
    pub(crate) table_header: image::Rgba<u8>,
    pub(crate) table_even: image::Rgba<u8>,
    pub(crate) task_active_background: image::Rgba<u8>,
    pub(crate) task_empty_background: image::Rgba<u8>,
    pub(crate) task_done_accent: image::Rgba<u8>,
    pub(crate) task_in_progress_accent: image::Rgba<u8>,
}

impl SurfacePaintPalette {
    fn from_theme(theme: &KdvThemeSnapshot) -> Self {
        Self {
            text: SurfaceHelpers::parse_color(&theme.text),
            quote: SurfaceHelpers::parse_color(&theme.quote_border),
            code_background: SurfaceHelpers::parse_color(&theme.code_background),
            code_border: SurfaceHelpers::parse_color(&theme.code_border),
            table_border: SurfaceHelpers::parse_color(theme.export_table_border()),
            table_header: SurfaceHelpers::parse_color(theme.export_table_header_background()),
            table_even: SurfaceHelpers::parse_color(theme.export_table_even_row_background()),
            task_active_background: SurfaceHelpers::parse_color(&theme.task_active_background),
            task_empty_background: SurfaceHelpers::parse_color(&theme.task_empty_background),
            task_done_accent: SurfaceHelpers::parse_color(&theme.task_done_accent),
            task_in_progress_accent: SurfaceHelpers::parse_color(&theme.task_in_progress_accent),
        }
    }
}

struct SurfaceMarkerPaintRequest<'a> {
    marker: &'a SurfaceLineMarker,
    x: u32,
    y: u32,
    indent_depth: u32,
    size: f32,
}

struct SurfaceListLinePaintRequest<'a> {
    line: &'a SurfaceLine,
    text_x: u32,
    text_y: u32,
    size: f32,
    marker: &'a SurfaceLineMarker,
}

struct SurfacePageLinkMetadata<'a> {
    annotations: &'a mut Vec<super::SurfaceLinkAnnotation>,
    anchors: &'a mut Vec<super::SurfaceLinkAnchor>,
}

struct SurfacePagePaintRequest<'a> {
    blocks: &'a [SurfaceBlock],
    block_indexes: &'a [usize],
    page_index: usize,
    painter: &'a mut SurfaceTextPainter,
    palette: &'a SurfacePaintPalette,
    links: SurfacePageLinkMetadata<'a>,
}

struct SurfaceTableRowPaintRequest<'a> {
    table: &'a SurfaceTableBlock,
    row: &'a [String],
    row_index: usize,
    row_y: u32,
    row_height: u32,
    column_widths: &'a [u32],
    row_width: u32,
}

struct SurfaceSpanMetadataRequest<'a> {
    span: &'a SurfaceTextSpan,
    font_size: f32,
    line_height: u32,
    page_index: usize,
    text_y: u32,
}

type SurfacePaintPages = (
    Vec<RgbaImage>,
    Vec<super::SurfaceLinkAnnotation>,
    Vec<super::SurfaceLinkAnchor>,
);

pub(crate) struct SurfacePainter;

#[path = "export_surface_painter_alert.rs"]
mod alert;
#[path = "export_surface_painter_badge.rs"]
mod badge;
#[path = "export_surface_painter_code_media.rs"]
mod code_media;
#[path = "export_surface_painter_core.rs"]
mod core;
#[path = "export_surface_painter_line.rs"]
mod line;
#[path = "export_surface_painter_links.rs"]
mod links;
#[path = "export_surface_painter_markers.rs"]
mod markers;
#[path = "export_surface_painter_media.rs"]
mod media;
#[path = "export_surface_painter_rule.rs"]
mod rule;
#[path = "export_surface_painter_table.rs"]
mod table;
#[path = "export_surface_painter_table_links.rs"]
mod table_links;
#[path = "export_surface_painter_task_markers.rs"]
mod task_markers;
