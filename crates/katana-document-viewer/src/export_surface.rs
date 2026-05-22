use crate::export_html_ops::fenced_body;
use crate::export_surface_code::SurfaceCodeHighlighter;
use crate::export_surface_font::{SurfaceTextLayout, SurfaceTextPainter};
use crate::export_surface_helpers::{
    BODY_MAX_CHARS, LIST_INDENT, PAGE_PADDING, QUOTE_INDENT, SURFACE_CONTENT_WIDTH,
    SURFACE_PAGE_HEIGHT, SURFACE_WIDTH, WrappedText, draw_fallback_text, draw_quote_bars,
    fill_rect, is_nested_blockquote, nested_blockquote_lines, parse_color, paste_rgba, stroke_rect,
    surface_block_height,
};
use crate::export_surface_line::{
    LIST_MARKER_COLUMN_WIDTH, SurfaceLine, SurfaceLineMarker, SurfaceTaskMarker,
};
use crate::export_surface_span::{SurfaceInlineSpans, SurfaceTextSpan};
use crate::export_surface_svg::{SurfaceSvgImage, SurfaceSvgRasterizer};
use crate::export_surface_text::{
    decode_basic_entities, html_fragment_text, inline_markdown_text, inline_text,
};
use crate::forge::BuildGraph;
use crate::render_runtime::{KrrMathMode, StubKrrRenderRuntime};
use crate::theme::KdvThemeSnapshot;
use image::RgbaImage;
use katana_markdown_model::{
    CodeBlockRole, DiagramKind, HtmlBlockRole, KatanaMarkdownModel, KmmNode, KmmNodeKind,
    ListItemNode, MarkdownInput, TableAlignment, TableNode,
};

const CODE_HORIZONTAL_PADDING: u32 = 24;
const CODE_VERTICAL_PADDING: u32 = 6;
const CODE_BLOCK_MARGIN: u32 = 14;
const DIAGRAM_MAX_WIDTH: u32 = 860;
const DIAGRAM_VERTICAL_MARGIN: u32 = 18;
const TABLE_ROW_HEIGHT: u32 = 52;
const TABLE_LINE_HEIGHT: u32 = 34;
const TABLE_CELL_PADDING: u32 = 16;
const TABLE_ROW_VERTICAL_PADDING: u32 = 16;
const RULE_HEIGHT: u32 = 34;
const BADGE_HEIGHT: u32 = 26;
const BADGE_VERTICAL_MARGIN: u32 = 10;
const BADGE_HORIZONTAL_GAP: u32 = 10;
const BADGE_HORIZONTAL_PADDING: u32 = 12;
const CODE_EMPTY_BLOCK_MIN_HEIGHT: u32 = 56;
const TASK_MARKER_SIZE: u32 = 18;
const MATH_MAX_WIDTH: u32 = 760;
const MATH_VERTICAL_MARGIN: u32 = 18;
const MATH_FALLBACK_HEIGHT: u32 = 74;

pub(crate) struct DocumentSurface {
    pub(crate) image: RgbaImage,
    pub(crate) pages: Vec<RgbaImage>,
    pub(crate) link_annotations: Vec<SurfaceLinkAnnotation>,
    pub(crate) link_anchors: Vec<SurfaceLinkAnchor>,
}

pub(crate) struct SurfaceLinkAnnotation {
    pub(crate) page_index: usize,
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) target: String,
}

pub(crate) struct SurfaceLinkAnchor {
    pub(crate) id: String,
    pub(crate) page_index: usize,
    pub(crate) x: u32,
    pub(crate) y: u32,
}

pub(crate) struct DocumentSurfaceFactory;

impl DocumentSurfaceFactory {
    pub(crate) fn create(graph: &BuildGraph, theme: &KdvThemeSnapshot) -> DocumentSurface {
        let blocks = SurfaceBlockFactory::create(graph);
        let height = surface_block_height(blocks.iter().map(SurfaceBlock::height));
        let background = parse_color(&theme.background);
        let mut image = RgbaImage::from_pixel(SURFACE_WIDTH, height, background);
        SurfacePainter::paint(&mut image, &blocks, theme);
        let (pages, link_annotations, link_anchors) = SurfacePainter::paint_pages(&blocks, theme);
        DocumentSurface {
            image,
            pages,
            link_annotations,
            link_anchors,
        }
    }
}

struct SurfacePainter;

impl SurfacePainter {
    fn paint(image: &mut RgbaImage, blocks: &[SurfaceBlock], theme: &KdvThemeSnapshot) {
        let palette = SurfacePaintPalette::from_theme(theme);
        let mut painter = SurfaceTextPainter::from_system_fonts();
        let mut y = PAGE_PADDING;
        for block in blocks {
            Self::paint_block(image, block, y, &mut painter, &palette);
            y += block.height();
        }
    }

    fn paint_pages(
        blocks: &[SurfaceBlock],
        theme: &KdvThemeSnapshot,
    ) -> (
        Vec<RgbaImage>,
        Vec<SurfaceLinkAnnotation>,
        Vec<SurfaceLinkAnchor>,
    ) {
        let palette = SurfacePaintPalette::from_theme(theme);
        let mut painter = SurfaceTextPainter::from_system_fonts();
        let background = parse_color(&theme.background);
        let mut pages = Vec::new();
        let mut link_annotations = Vec::new();
        let mut link_anchors = Vec::new();
        let plan = SurfacePagePlan::from_blocks(blocks);
        for (page_index, block_indexes) in plan.pages.iter().enumerate() {
            let mut page = RgbaImage::from_pixel(SURFACE_WIDTH, SURFACE_PAGE_HEIGHT, background);
            let mut y = PAGE_PADDING;
            for block_index in block_indexes {
                let block = &blocks[*block_index];
                Self::paint_block(&mut page, block, y, &mut painter, &palette);
                Self::append_link_metadata(
                    &mut link_annotations,
                    &mut link_anchors,
                    block,
                    page_index,
                    y,
                );
                y += block.height();
            }
            pages.push(page);
        }
        (pages, link_annotations, link_anchors)
    }

    fn paint_block(
        image: &mut RgbaImage,
        block: &SurfaceBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        match block {
            SurfaceBlock::Line(line) => Self::paint_line(image, line, y, painter, palette),
            SurfaceBlock::Code(code) => Self::paint_code_block(image, code, y, painter, palette),
            SurfaceBlock::Math(math) => Self::paint_math_block(image, math, y, painter, palette),
            SurfaceBlock::Table(table) => Self::paint_table(image, table, y, painter, palette),
            SurfaceBlock::Diagram(diagram) => Self::paint_diagram(image, diagram, y, palette),
            SurfaceBlock::BadgeRow(row) => Self::paint_badge_row(image, row, y, painter, palette),
            SurfaceBlock::Alert(alert) => Self::paint_alert(image, alert, y, painter, palette),
            SurfaceBlock::Rule => Self::paint_rule(image, y, palette),
        }
    }

    fn paint_line(
        image: &mut RgbaImage,
        line: &SurfaceLine,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let size = line.font_size();
        let text_x = Self::line_text_x(line);
        if line.quote_depth() > 0 {
            draw_quote_bars(
                image,
                line.quote_depth(),
                y,
                line.line_height(),
                palette.quote,
            );
        }
        let text_y = line.text_y(y);
        if let Some(marker) = line.list_marker() {
            Self::paint_line_marker(image, &marker, text_x, text_y, size, painter, palette);
            Self::paint_line_text(
                image,
                line.content_spans(),
                text_x + LIST_MARKER_COLUMN_WIDTH,
                text_y,
                size,
                painter,
                palette,
            );
            return;
        }
        Self::paint_line_text(image, &line.spans, text_x, text_y, size, painter, palette);
    }

    fn paint_line_text(
        image: &mut RgbaImage,
        spans: &[SurfaceTextSpan],
        x: u32,
        y: u32,
        size: f32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        match painter {
            Some(it) => it.draw_spans(image, spans, x, y, size, palette.text),
            None => draw_fallback_text(image, x, y, &spans_text(spans), palette.text),
        }
    }

    fn paint_line_marker(
        image: &mut RgbaImage,
        marker: &SurfaceLineMarker,
        x: u32,
        y: u32,
        size: f32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        match marker {
            SurfaceLineMarker::Bullet => {
                Self::paint_text_marker(image, "•", x + 5, y, size, painter, palette)
            }
            SurfaceLineMarker::Ordered(value) => {
                Self::paint_text_marker(image, value, x, y, size, painter, palette)
            }
            SurfaceLineMarker::Task(task) => Self::paint_task_marker(image, *task, x, y, palette),
        }
    }

    fn paint_text_marker(
        image: &mut RgbaImage,
        text: &str,
        x: u32,
        y: u32,
        size: f32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        match painter {
            Some(it) => it.draw_text(
                image,
                text,
                SurfaceTextLayout {
                    x,
                    y,
                    size,
                    color: palette.text,
                    max_width: Some(LIST_MARKER_COLUMN_WIDTH as f32),
                },
            ),
            None => draw_fallback_text(image, x, y, text, palette.text),
        }
    }

    fn paint_task_marker(
        image: &mut RgbaImage,
        marker: SurfaceTaskMarker,
        x: u32,
        y: u32,
        palette: &SurfacePaintPalette,
    ) {
        let box_x = x + 4;
        let box_y = y + 8;
        let fill = match marker {
            SurfaceTaskMarker::Empty => palette.task_empty_background,
            SurfaceTaskMarker::Done
            | SurfaceTaskMarker::Blocked
            | SurfaceTaskMarker::InProgress => palette.task_active_background,
        };
        fill_rect(
            image,
            box_x,
            box_y,
            TASK_MARKER_SIZE,
            TASK_MARKER_SIZE,
            fill,
        );
        stroke_rect(
            image,
            box_x,
            box_y,
            TASK_MARKER_SIZE,
            TASK_MARKER_SIZE,
            palette.table_border,
        );
        match marker {
            SurfaceTaskMarker::Done => {
                draw_check_mark(image, box_x, box_y, palette.task_done_accent);
            }
            SurfaceTaskMarker::Blocked => {
                fill_rect(
                    image,
                    box_x + 4,
                    box_y + TASK_MARKER_SIZE / 2,
                    TASK_MARKER_SIZE - 8,
                    3,
                    palette.task_in_progress_accent,
                );
            }
            SurfaceTaskMarker::InProgress => {
                draw_diagonal_mark(image, box_x, box_y, palette.task_in_progress_accent);
            }
            SurfaceTaskMarker::Empty => {}
        }
    }

    fn paint_code_block(
        image: &mut RgbaImage,
        block: &SurfaceCodeBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        if block.quote_depth > 0 {
            draw_quote_bars(image, block.quote_depth, y, block.height(), palette.quote);
        }
        let box_x =
            PAGE_PADDING + block.quote_depth * QUOTE_INDENT + block.indent_depth * LIST_INDENT;
        let box_width = SURFACE_WIDTH.saturating_sub(box_x + PAGE_PADDING);
        let box_y = y + CODE_BLOCK_MARGIN;
        fill_rect(
            image,
            box_x,
            box_y,
            box_width,
            block.box_height(),
            palette.code_background,
        );
        stroke_rect(
            image,
            box_x,
            box_y,
            box_width,
            block.box_height(),
            palette.code_border,
        );
        let mut line_y = box_y + CODE_VERTICAL_PADDING;
        for line in &block.lines {
            let text_y = line.text_y(line_y);
            match painter {
                Some(it) => it.draw_spans(
                    image,
                    &line.spans,
                    box_x + CODE_HORIZONTAL_PADDING,
                    text_y,
                    line.font_size(),
                    palette.text,
                ),
                None => draw_fallback_text(
                    image,
                    box_x + CODE_HORIZONTAL_PADDING,
                    text_y,
                    &line.text,
                    palette.text,
                ),
            }
            line_y += line.line_height();
        }
    }

    fn paint_math_block(
        image: &mut RgbaImage,
        block: &SurfaceMathBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        if let Some(rendered) = &block.image {
            let x = PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(rendered.image.width()) / 2;
            paste_rgba(image, &rendered.image, x, y + MATH_VERTICAL_MARGIN);
            return;
        }

        match painter {
            Some(it) => it.draw_text(
                image,
                &block.fallback_text,
                SurfaceTextLayout {
                    x: PAGE_PADDING,
                    y: y + MATH_VERTICAL_MARGIN,
                    size: 28.0,
                    color: palette.text,
                    max_width: Some(SURFACE_CONTENT_WIDTH as f32),
                },
            ),
            None => draw_fallback_text(
                image,
                PAGE_PADDING,
                y + MATH_VERTICAL_MARGIN,
                &block.fallback_text,
                palette.text,
            ),
        }
    }

    fn line_text_x(line: &SurfaceLine) -> u32 {
        if line.is_code() {
            return line.x() + CODE_HORIZONTAL_PADDING;
        }
        if line.is_centered() {
            let estimated_width = (line.text.chars().count() as u32).saturating_mul(14);
            return PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(estimated_width) / 2;
        }
        line.x()
    }

    fn append_link_metadata(
        annotations: &mut Vec<SurfaceLinkAnnotation>,
        anchors: &mut Vec<SurfaceLinkAnchor>,
        block: &SurfaceBlock,
        page_index: usize,
        y: u32,
    ) {
        match block {
            SurfaceBlock::Line(line) => {
                Self::append_line_link_metadata(annotations, anchors, line, page_index, y);
            }
            SurfaceBlock::BadgeRow(row) => {
                Self::append_badge_link_annotations(annotations, row, page_index, y);
            }
            _ => {}
        }
    }

    fn append_line_link_metadata(
        annotations: &mut Vec<SurfaceLinkAnnotation>,
        anchors: &mut Vec<SurfaceLinkAnchor>,
        line: &SurfaceLine,
        page_index: usize,
        y: u32,
    ) {
        let mut x = Self::line_text_x(line);
        let spans = if line.list_marker().is_some() {
            x += LIST_MARKER_COLUMN_WIDTH;
            line.content_spans()
        } else {
            &line.spans
        };
        for span in spans {
            let width = estimated_span_width(&span.text, line.font_size());
            let text_y = line.text_y(y);
            if let Some(target) = &span.link_target
                && !target.is_empty()
            {
                annotations.push(SurfaceLinkAnnotation {
                    page_index,
                    x,
                    y: text_y,
                    width,
                    height: line.line_height(),
                    target: target.clone(),
                });
                if let Some(anchor) = inferred_link_anchor(span, page_index, x, text_y) {
                    anchors.push(anchor);
                }
            }
            x += width;
        }
    }

    fn append_badge_link_annotations(
        annotations: &mut Vec<SurfaceLinkAnnotation>,
        row: &SurfaceBadgeRowBlock,
        page_index: usize,
        y: u32,
    ) {
        let mut x = PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(row.total_width()) / 2;
        for badge in &row.badges {
            if let Some(target) = &badge.link_target {
                annotations.push(SurfaceLinkAnnotation {
                    page_index,
                    x,
                    y: y + BADGE_VERTICAL_MARGIN,
                    width: badge.width(),
                    height: BADGE_HEIGHT,
                    target: target.clone(),
                });
            }
            x += badge.width() + BADGE_HORIZONTAL_GAP;
        }
    }

    fn paint_table(
        image: &mut RgbaImage,
        table: &SurfaceTableBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let row_width = SURFACE_CONTENT_WIDTH;
        let column_width = row_width / table.column_count().max(1) as u32;
        let mut row_y = y;
        for (row_index, row) in table.rows.iter().enumerate() {
            let row_height = table.row_height(row_index, column_width);
            if let Some(fill) = table_row_fill(row_index, palette) {
                fill_rect(image, PAGE_PADDING, row_y, row_width, row_height, fill);
            }
            for (column_index, cell) in row.iter().enumerate() {
                let x = PAGE_PADDING + column_index as u32 * column_width;
                stroke_rect(
                    image,
                    x,
                    row_y,
                    column_width,
                    row_height,
                    palette.table_border,
                );
                Self::paint_table_cell(
                    image,
                    SurfaceTableCellPaint {
                        cell,
                        alignment: table.alignment(column_index),
                        x,
                        y: row_y,
                        width: column_width,
                        row_height,
                    },
                    painter,
                    palette,
                );
            }
            row_y += row_height;
        }
    }

    fn paint_table_cell(
        image: &mut RgbaImage,
        cell: SurfaceTableCellPaint<'_>,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let text_x = table_cell_text_x(cell.cell, cell.alignment.clone(), cell.x, cell.width);
        let max_chars = table_cell_max_chars(cell.width);
        let lines = WrappedText::new(cell.cell, max_chars).collect::<Vec<_>>();
        let mut text_y = cell.y + table_cell_text_y(cell.row_height, lines.len());
        match painter {
            Some(it) => {
                for line in &lines {
                    it.draw_text(
                        image,
                        line,
                        SurfaceTextLayout {
                            x: table_cell_text_x(line, cell.alignment.clone(), cell.x, cell.width),
                            y: text_y,
                            size: 22.0,
                            color: palette.text,
                            max_width: Some(
                                cell.width.saturating_sub(TABLE_CELL_PADDING * 2) as f32
                            ),
                        },
                    );
                    text_y += TABLE_LINE_HEIGHT;
                }
            }
            None => draw_fallback_text(image, text_x, text_y, cell.cell, palette.text),
        }
    }

    fn paint_diagram(
        image: &mut RgbaImage,
        diagram: &SurfaceDiagramBlock,
        y: u32,
        palette: &SurfacePaintPalette,
    ) {
        let Some(rendered) = &diagram.image else {
            draw_fallback_text(
                image,
                PAGE_PADDING,
                y + DIAGRAM_VERTICAL_MARGIN,
                &diagram.fallback_text,
                palette.text,
            );
            return;
        };
        let x = PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(rendered.image.width()) / 2;
        paste_rgba(image, &rendered.image, x, y + DIAGRAM_VERTICAL_MARGIN);
    }

    fn paint_rule(image: &mut RgbaImage, y: u32, palette: &SurfacePaintPalette) {
        let line_y = y + RULE_HEIGHT / 2;
        fill_rect(
            image,
            PAGE_PADDING,
            line_y,
            SURFACE_CONTENT_WIDTH,
            2,
            palette.table_border,
        );
    }

    fn paint_badge_row(
        image: &mut RgbaImage,
        row: &SurfaceBadgeRowBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let total_width = row.total_width();
        let mut x = PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(total_width) / 2;
        let badge_y = y + BADGE_VERTICAL_MARGIN;
        for badge in &row.badges {
            let label_width = badge.label_width();
            let message_width = badge.message_width();
            let width = badge.width();
            fill_rect(
                image,
                x,
                badge_y,
                label_width,
                BADGE_HEIGHT,
                image::Rgba([85, 85, 85, 255]),
            );
            if message_width > 0 {
                fill_rect(
                    image,
                    x + label_width,
                    badge_y,
                    message_width,
                    BADGE_HEIGHT,
                    badge.color,
                );
            }
            stroke_rect(image, x, badge_y, width, BADGE_HEIGHT, palette.table_border);
            if let Some(it) = painter {
                it.draw_text(
                    image,
                    &badge.label,
                    SurfaceTextLayout {
                        x: x + BADGE_HORIZONTAL_PADDING,
                        y: badge_y + 2,
                        size: 18.0,
                        color: image::Rgba([255, 255, 255, 255]),
                        max_width: Some(
                            label_width.saturating_sub(BADGE_HORIZONTAL_PADDING * 2) as f32
                        ),
                    },
                );
                if !badge.message.is_empty() {
                    it.draw_text(
                        image,
                        &badge.message,
                        SurfaceTextLayout {
                            x: x + label_width + BADGE_HORIZONTAL_PADDING,
                            y: badge_y + 2,
                            size: 18.0,
                            color: image::Rgba([255, 255, 255, 255]),
                            max_width: Some(
                                message_width.saturating_sub(BADGE_HORIZONTAL_PADDING * 2) as f32,
                            ),
                        },
                    );
                }
            } else {
                draw_fallback_text(
                    image,
                    x + BADGE_HORIZONTAL_PADDING,
                    badge_y + 2,
                    &badge.text(),
                    image::Rgba([255, 255, 255, 255]),
                );
            }
            x += width + BADGE_HORIZONTAL_GAP;
        }
    }

    fn paint_alert(
        image: &mut RgbaImage,
        alert: &SurfaceAlertBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let x = PAGE_PADDING + alert.quote_depth * QUOTE_INDENT;
        let width = SURFACE_WIDTH.saturating_sub(x + PAGE_PADDING);
        fill_rect(
            image,
            x,
            y + 8,
            width,
            alert.height().saturating_sub(16),
            palette.alert_background,
        );
        fill_rect(
            image,
            x,
            y + 8,
            5,
            alert.height().saturating_sub(16),
            alert_color(&alert.label),
        );
        let text_x = x + 28;
        let mut text_y = y + 16;
        Self::paint_alert_line(image, &alert.title, text_x, text_y, painter, palette);
        text_y += alert.title.line_height();
        for line in &alert.body {
            Self::paint_alert_line(image, line, text_x, text_y, painter, palette);
            text_y += line.line_height();
        }
    }

    fn paint_alert_line(
        image: &mut RgbaImage,
        line: &SurfaceLine,
        x: u32,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let text_y = line.text_y(y);
        match painter {
            Some(it) => it.draw_spans(
                image,
                &line.spans,
                x,
                text_y,
                line.font_size(),
                palette.text,
            ),
            None => draw_fallback_text(image, x, text_y, &line.text, palette.text),
        }
    }
}

struct SurfacePaintPalette {
    text: image::Rgba<u8>,
    quote: image::Rgba<u8>,
    code_background: image::Rgba<u8>,
    code_border: image::Rgba<u8>,
    table_border: image::Rgba<u8>,
    table_header: image::Rgba<u8>,
    table_even: image::Rgba<u8>,
    task_active_background: image::Rgba<u8>,
    task_empty_background: image::Rgba<u8>,
    task_done_accent: image::Rgba<u8>,
    task_in_progress_accent: image::Rgba<u8>,
    alert_background: image::Rgba<u8>,
}

struct SurfaceTableCellPaint<'a> {
    cell: &'a str,
    alignment: TableAlignment,
    x: u32,
    y: u32,
    width: u32,
    row_height: u32,
}

impl SurfacePaintPalette {
    fn from_theme(theme: &KdvThemeSnapshot) -> Self {
        Self {
            text: parse_color(&theme.text),
            quote: parse_color(&theme.quote_border),
            code_background: parse_color(&theme.code_background),
            code_border: parse_color(&theme.code_border),
            table_border: parse_color(&theme.table_border),
            table_header: parse_color(&theme.table_header_background),
            table_even: parse_color(&theme.table_even_row_background),
            task_active_background: parse_color(&theme.task_active_background),
            task_empty_background: parse_color(&theme.task_empty_background),
            task_done_accent: parse_color(&theme.task_done_accent),
            task_in_progress_accent: parse_color(&theme.task_in_progress_accent),
            alert_background: image::Rgba([246, 248, 250, 255]),
        }
    }
}

enum SurfaceBlock {
    Line(SurfaceLine),
    Code(SurfaceCodeBlock),
    Math(SurfaceMathBlock),
    Table(SurfaceTableBlock),
    Diagram(SurfaceDiagramBlock),
    BadgeRow(SurfaceBadgeRowBlock),
    Alert(SurfaceAlertBlock),
    Rule,
}

impl SurfaceBlock {
    fn height(&self) -> u32 {
        match self {
            SurfaceBlock::Line(line) => line.line_height(),
            SurfaceBlock::Code(code) => code.height(),
            SurfaceBlock::Math(math) => math.height(),
            SurfaceBlock::Table(table) => table.height(),
            SurfaceBlock::Diagram(diagram) => diagram.height(),
            SurfaceBlock::BadgeRow(row) => row.height(),
            SurfaceBlock::Alert(alert) => alert.height(),
            SurfaceBlock::Rule => RULE_HEIGHT,
        }
    }

    fn is_heading(&self) -> bool {
        matches!(self, SurfaceBlock::Line(line) if line.is_heading())
    }

    #[cfg(test)]
    fn text_for_tests(&self) -> String {
        match self {
            SurfaceBlock::Line(line) => line.text.clone(),
            SurfaceBlock::Code(code) => code
                .lines
                .iter()
                .map(|line| line.text.as_str())
                .collect::<Vec<_>>()
                .join("\n"),
            SurfaceBlock::Math(math) => math.text(),
            SurfaceBlock::Table(table) => table.text(),
            SurfaceBlock::Diagram(diagram) => diagram.fallback_text.clone(),
            SurfaceBlock::BadgeRow(row) => row.text(),
            SurfaceBlock::Alert(alert) => alert.text(),
            SurfaceBlock::Rule => String::new(),
        }
    }

    #[cfg(test)]
    fn debug_for_tests(&self) -> String {
        match self {
            SurfaceBlock::Line(line) => {
                format!("line:{}:{}", line.text, line.debug_style_tags().join("|"))
            }
            SurfaceBlock::Code(code) => format!(
                "code:{}",
                code.lines
                    .iter()
                    .flat_map(SurfaceLine::debug_style_tags)
                    .collect::<Vec<_>>()
                    .join("|")
            ),
            SurfaceBlock::Math(math) => format!("math:{}", math.text()),
            SurfaceBlock::Table(table) => format!(
                "table:{}x{}:{}",
                table.rows.len(),
                table.column_count(),
                table.text()
            ),
            SurfaceBlock::Diagram(diagram) => {
                let size = diagram
                    .image
                    .as_ref()
                    .map(|image| format!("{}x{}", image.image.width(), image.image.height()))
                    .unwrap_or_else(|| "missing".to_string());
                format!("diagram:{size}")
            }
            SurfaceBlock::BadgeRow(row) => format!("badges:{}:[\"centered\"]", row.text()),
            SurfaceBlock::Alert(alert) => format!("alert:{}:{}", alert.label, alert.title.text),
            SurfaceBlock::Rule => "rule".to_string(),
        }
    }
}

struct SurfacePagePlan {
    pages: Vec<Vec<usize>>,
}

impl SurfacePagePlan {
    fn from_blocks(blocks: &[SurfaceBlock]) -> Self {
        let mut pages = Vec::new();
        let mut current = Vec::new();
        let mut y = PAGE_PADDING;
        let mut index = 0;
        while let Some(block) = blocks.get(index) {
            if Self::should_move_to_next_page(blocks, index, y) {
                pages.push(current);
                current = Vec::new();
                y = PAGE_PADDING;
                continue;
            }
            if y > PAGE_PADDING && y + block.height() > Self::page_bottom() {
                pages.push(current);
                current = Vec::new();
                y = PAGE_PADDING;
                continue;
            }
            current.push(index);
            y += block.height();
            index += 1;
        }
        if !current.is_empty() {
            pages.push(current);
        }
        Self { pages }
    }

    fn should_move_to_next_page(blocks: &[SurfaceBlock], index: usize, y: u32) -> bool {
        if y == PAGE_PADDING {
            return false;
        }
        let Some(block) = blocks.get(index) else {
            return false;
        };
        if !block.is_heading() {
            return false;
        }
        let Some(next) = blocks.get(index + 1) else {
            return false;
        };
        let combined_height = block.height() + next.height();
        combined_height <= Self::page_content_height() && y + combined_height > Self::page_bottom()
    }

    fn page_bottom() -> u32 {
        SURFACE_PAGE_HEIGHT - PAGE_PADDING
    }

    fn page_content_height() -> u32 {
        SURFACE_PAGE_HEIGHT - PAGE_PADDING * 2
    }
}

struct SurfaceTableBlock {
    rows: Vec<Vec<String>>,
    alignments: Vec<TableAlignment>,
}

impl SurfaceTableBlock {
    fn new(table: &TableNode) -> Self {
        Self {
            rows: table
                .rows
                .iter()
                .filter(|row| !is_table_separator_row(row))
                .map(|row| {
                    row.cells
                        .iter()
                        .map(|cell| inline_markdown_text(&cell.text))
                        .collect()
                })
                .collect(),
            alignments: table.alignments.clone(),
        }
    }

    fn height(&self) -> u32 {
        let column_width = SURFACE_CONTENT_WIDTH / self.column_count().max(1) as u32;
        self.rows
            .iter()
            .enumerate()
            .map(|(index, _)| self.row_height(index, column_width))
            .sum()
    }

    fn column_count(&self) -> usize {
        self.rows.iter().map(Vec::len).max().unwrap_or(1)
    }

    fn alignment(&self, index: usize) -> TableAlignment {
        self.alignments
            .get(index)
            .cloned()
            .unwrap_or(TableAlignment::Unspecified)
    }

    fn text(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.join("  "))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn row_height(&self, row_index: usize, column_width: u32) -> u32 {
        let line_count = self
            .rows
            .get(row_index)
            .map(|row| {
                row.iter()
                    .map(|cell| WrappedText::new(cell, table_cell_max_chars(column_width)).count())
                    .max()
                    .unwrap_or(1)
            })
            .unwrap_or(1);
        let dynamic_height = line_count as u32 * TABLE_LINE_HEIGHT + TABLE_ROW_VERTICAL_PADDING * 2;
        dynamic_height.max(TABLE_ROW_HEIGHT)
    }
}

fn is_table_separator_row(row: &katana_markdown_model::TableRow) -> bool {
    row.cells.iter().all(|cell| {
        let trimmed = cell.text.trim();
        !trimmed.is_empty()
            && trimmed
                .chars()
                .all(|character| matches!(character, '-' | ':'))
    })
}

fn table_row_fill(row_index: usize, palette: &SurfacePaintPalette) -> Option<image::Rgba<u8>> {
    if row_index == 0 {
        return Some(palette.table_header);
    }
    if row_index.is_multiple_of(2) {
        return Some(palette.table_even);
    }
    None
}

fn table_cell_text_x(cell: &str, alignment: TableAlignment, x: u32, width: u32) -> u32 {
    let content_width = width.saturating_sub(TABLE_CELL_PADDING * 2);
    let text_width = estimated_cell_text_width(cell).min(content_width);
    let left = x + TABLE_CELL_PADDING;
    match alignment {
        TableAlignment::Center => left + content_width.saturating_sub(text_width) / 2,
        TableAlignment::Right => left + content_width.saturating_sub(text_width),
        TableAlignment::Left | TableAlignment::Unspecified => left,
    }
}

fn table_cell_text_y(row_height: u32, line_count: usize) -> u32 {
    let content_height = line_count.max(1) as u32 * TABLE_LINE_HEIGHT;
    row_height.saturating_sub(content_height) / 2
}

fn estimated_cell_text_width(cell: &str) -> u32 {
    cell.chars()
        .map(|character| if character.is_ascii() { 12 } else { 22 })
        .sum()
}

fn table_cell_max_chars(width: u32) -> usize {
    (width.saturating_sub(TABLE_CELL_PADDING * 2) / 22)
        .max(8)
        .try_into()
        .unwrap_or(8)
}

fn has_surface_table_contract(table: &TableNode) -> bool {
    table.rows.len() >= 2 && table.rows.get(1).is_some_and(is_table_separator_row)
}

struct SurfaceCodeBlock {
    lines: Vec<SurfaceLine>,
    quote_depth: u32,
    indent_depth: u32,
}

impl SurfaceCodeBlock {
    fn new(lines: Vec<SurfaceLine>, quote_depth: u32, indent_depth: u32) -> Self {
        Self {
            lines,
            quote_depth,
            indent_depth,
        }
    }

    fn height(&self) -> u32 {
        self.box_height() + CODE_BLOCK_MARGIN * 2
    }

    fn box_height(&self) -> u32 {
        let content_height = self.lines.iter().map(SurfaceLine::line_height).sum::<u32>()
            + CODE_VERTICAL_PADDING * 2;
        content_height.max(CODE_EMPTY_BLOCK_MIN_HEIGHT)
    }
}

struct SurfaceMathBlock {
    image: Option<SurfaceSvgImage>,
    fallback_text: String,
}

impl SurfaceMathBlock {
    fn new(expression: &str) -> Self {
        let output = StubKrrRenderRuntime::render_math_tex(expression, KrrMathMode::Display);
        let image = output
            .svg_payload()
            .and_then(|svg| SurfaceSvgRasterizer::rasterize(svg, MATH_MAX_WIDTH));
        Self {
            image,
            fallback_text: math_fallback_text(expression, &output),
        }
    }

    fn height(&self) -> u32 {
        self.image
            .as_ref()
            .map(|rendered| rendered.image.height() + MATH_VERTICAL_MARGIN * 2)
            .unwrap_or(MATH_FALLBACK_HEIGHT)
    }

    #[cfg(test)]
    fn text(&self) -> String {
        if self.image.is_some() {
            return "math-svg:rendered".to_string();
        }
        self.fallback_text.clone()
    }
}

fn math_fallback_text(expression: &str, output: &crate::render_runtime::KrrRenderOutput) -> String {
    if output.svg_payload().is_some() {
        return expression.trim().to_string();
    }
    output.raw_payload().to_string()
}

struct SurfaceDiagramBlock {
    image: Option<SurfaceSvgImage>,
    fallback_text: String,
}

impl SurfaceDiagramBlock {
    fn rendered(svg: &str) -> Self {
        Self {
            image: SurfaceSvgRasterizer::rasterize(svg, DIAGRAM_MAX_WIDTH),
            fallback_text: "Rendered diagram".to_string(),
        }
    }

    fn height(&self) -> u32 {
        let content_height = self
            .image
            .as_ref()
            .map(|rendered| rendered.image.height())
            .unwrap_or(38);
        content_height + DIAGRAM_VERTICAL_MARGIN * 2
    }
}

struct SurfaceBadgeRowBlock {
    badges: Vec<SurfaceBadge>,
}

impl SurfaceBadgeRowBlock {
    fn new(badges: Vec<SurfaceBadge>) -> Self {
        Self { badges }
    }

    fn height(&self) -> u32 {
        BADGE_HEIGHT + BADGE_VERTICAL_MARGIN * 2
    }

    #[cfg(test)]
    fn text(&self) -> String {
        self.badges
            .iter()
            .map(SurfaceBadge::text)
            .collect::<Vec<_>>()
            .join(" | ")
    }

    fn total_width(&self) -> u32 {
        let badge_widths = self.badges.iter().map(SurfaceBadge::width).sum::<u32>();
        let gap_count = self.badges.len().saturating_sub(1) as u32;
        badge_widths + gap_count * BADGE_HORIZONTAL_GAP
    }
}

struct SurfaceBadge {
    label: String,
    message: String,
    color: image::Rgba<u8>,
    link_target: Option<String>,
}

impl SurfaceBadge {
    fn linked(
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

    fn single(label: String) -> Self {
        Self {
            label,
            message: String::new(),
            color: image::Rgba([159, 159, 159, 255]),
            link_target: None,
        }
    }

    fn text(&self) -> String {
        if self.message.is_empty() {
            return self.label.clone();
        }
        format!("{}={}", self.label, self.message)
    }

    fn width(&self) -> u32 {
        self.label_width() + self.message_width()
    }

    fn label_width(&self) -> u32 {
        badge_segment_width(&self.label)
    }

    fn message_width(&self) -> u32 {
        if self.message.is_empty() {
            return 0;
        }
        badge_segment_width(&self.message)
    }
}

fn badge_segment_width(label: &str) -> u32 {
    (label.chars().count() as u32 * 10 + BADGE_HORIZONTAL_PADDING * 2).max(38)
}

struct SurfaceAlertBlock {
    label: String,
    title: SurfaceLine,
    body: Vec<SurfaceLine>,
    quote_depth: u32,
}

impl SurfaceAlertBlock {
    fn new(label: &str, body_lines: Vec<String>, quote_depth: u32) -> Self {
        let title = SurfaceLine::body_spans(
            vec![SurfaceTextSpan::styled(
                alert_label_text(label),
                crate::export_surface_span::SurfaceTextStyle::default()
                    .bold()
                    .with_color(alert_color(label)),
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

    fn height(&self) -> u32 {
        let body_height = self.body.iter().map(SurfaceLine::line_height).sum::<u32>();
        self.title.line_height() + body_height + 32
    }

    #[cfg(test)]
    fn text(&self) -> String {
        let mut parts = vec![self.title.text.clone()];
        parts.extend(self.body.iter().map(|line| line.text.clone()));
        parts.join("\n")
    }
}

fn estimated_span_width(text: &str, font_size: f32) -> u32 {
    (text.chars().count() as f32 * font_size * 0.58).ceil() as u32
}

struct SurfaceBlockFactory;

impl SurfaceBlockFactory {
    fn create(graph: &BuildGraph) -> Vec<SurfaceBlock> {
        let mut blocks = Vec::new();
        let mut footnotes = Vec::new();
        for node in &graph.snapshot.document.nodes {
            if let Some(line) = Self::footnote_line(node, 0) {
                footnotes.push(line);
                continue;
            }
            Self::append_node(&mut blocks, graph, node, 0, 0);
        }
        if !footnotes.is_empty() {
            blocks.push(SurfaceBlock::Rule);
            blocks.extend(footnotes.into_iter().map(SurfaceBlock::Line));
        }
        blocks
    }

    fn append_node(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
        quote_depth: u32,
        list_depth: u32,
    ) {
        match &node.kind {
            KmmNodeKind::Heading(heading) => blocks.push(SurfaceBlock::Line(SurfaceLine::heading(
                heading.level,
                heading.text.clone(),
            ))),
            KmmNodeKind::Paragraph => Self::append_rich_line(blocks, node, quote_depth, list_depth),
            KmmNodeKind::CodeBlock(role) => {
                Self::append_code(blocks, graph, node, role, quote_depth, list_depth)
            }
            KmmNodeKind::BlockQuote => {
                if let Some((title, body)) = legacy_note_children(&node.children) {
                    Self::append_wrapped(
                        blocks,
                        format!("{title} {body}"),
                        quote_depth + 1,
                        list_depth,
                    );
                } else if let Some((title, body)) = legacy_note_quote(&node.source.raw.text) {
                    Self::append_wrapped(
                        blocks,
                        format!("{title} {body}"),
                        quote_depth + 1,
                        list_depth,
                    );
                } else if is_nested_blockquote(&node.source.raw.text) {
                    for (text, depth) in nested_blockquote_lines(&node.source.raw.text, quote_depth)
                    {
                        Self::append_wrapped(blocks, text, depth, list_depth);
                    }
                } else {
                    Self::append_children(blocks, graph, node, quote_depth + 1, list_depth);
                }
            }
            KmmNodeKind::Alert { label } => {
                if let Some((title, body)) = legacy_note_children(&node.children) {
                    Self::append_wrapped(
                        blocks,
                        format!("{title} {body}"),
                        quote_depth + 1,
                        list_depth,
                    );
                    return;
                }
                blocks.push(SurfaceBlock::Alert(SurfaceAlertBlock::new(
                    label,
                    alert_body_lines(node),
                    quote_depth,
                )));
            }
            KmmNodeKind::Table(table) => Self::append_table(
                blocks,
                table,
                &node.source.raw.text,
                quote_depth,
                list_depth,
            ),
            KmmNodeKind::HtmlBlock(role) => {
                Self::append_html(blocks, graph, node, role, quote_depth, list_depth)
            }
            KmmNodeKind::DollarMathBlock(math) => {
                Self::append_math_lines(blocks, &math.expression, quote_depth);
            }
            KmmNodeKind::FootnoteDefinition(_) => {
                if let Some(line) = Self::footnote_line(node, quote_depth) {
                    blocks.push(SurfaceBlock::Line(line));
                }
            }
            KmmNodeKind::RawBlock { .. } => {
                Self::append_raw(blocks, &node.source.raw.text, quote_depth, list_depth)
            }
            KmmNodeKind::List(list) => {
                for item in &list.items {
                    Self::append_list_item(
                        blocks,
                        graph,
                        item,
                        list.ordered,
                        quote_depth,
                        list_depth,
                    );
                }
            }
            KmmNodeKind::ThematicBreak => blocks.push(SurfaceBlock::Rule),
            _ => Self::append_wrapped(blocks, inline_text(node), quote_depth, list_depth),
        }
    }

    fn append_children(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
        quote_depth: u32,
        list_depth: u32,
    ) {
        for child in &node.children {
            Self::append_node(blocks, graph, child, quote_depth, list_depth);
        }
    }

    fn append_list_item(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        item: &ListItemNode,
        ordered: bool,
        quote_depth: u32,
        list_depth: u32,
    ) {
        let mut spans = vec![SurfaceTextSpan::plain(list_marker_text(item, ordered))];
        spans.extend(SurfaceInlineSpans::from_nodes(&item.body));
        if !spans.iter().any(|span| !span.text.trim().is_empty()) {
            return;
        }
        blocks.push(SurfaceBlock::Line(SurfaceLine::body_spans_with_indent(
            spans,
            quote_depth,
            list_depth,
        )));
        for child in &item.children {
            Self::append_node(blocks, graph, child, quote_depth, list_depth + 1);
        }
    }

    fn append_html(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
        role: &HtmlBlockRole,
        quote_depth: u32,
        list_depth: u32,
    ) {
        if Self::append_details(
            blocks,
            graph,
            &node.source.raw.text,
            quote_depth,
            list_depth,
        ) {
            return;
        }
        if matches!(role, HtmlBlockRole::BadgeRow) {
            blocks.push(SurfaceBlock::BadgeRow(SurfaceBadgeRowBlock::new(
                badge_row_badges(&node.source.raw.text),
            )));
            return;
        }
        let text = normalize_surface_html_text(&html_fragment_text(&node.source.raw.text));
        if matches!(role, HtmlBlockRole::Centered) {
            let spans = centered_html_spans(&node.source.raw.text);
            if !spans.is_empty() && text.chars().count() <= BODY_MAX_CHARS {
                blocks.push(SurfaceBlock::Line(SurfaceLine::centered_spans(spans)));
            } else {
                for chunk in WrappedText::new(&text, BODY_MAX_CHARS) {
                    blocks.push(SurfaceBlock::Line(SurfaceLine::body_centered(chunk)));
                }
            }
            return;
        }
        Self::append_wrapped(blocks, text, quote_depth, list_depth);
    }

    fn append_details(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        fragment: &str,
        quote_depth: u32,
        list_depth: u32,
    ) -> bool {
        let Some(parts) = SurfaceDetailsParts::parse(fragment) else {
            return false;
        };
        Self::append_wrapped(
            blocks,
            normalize_surface_html_text(parts.summary),
            quote_depth,
            list_depth,
        );
        let parsed = KatanaMarkdownModel::parse(MarkdownInput::from_content(
            "surface-details.md",
            parts.body.trim().to_string(),
        ));
        let Ok(document) = parsed else {
            Self::append_wrapped(
                blocks,
                normalize_surface_html_text(parts.body),
                quote_depth,
                list_depth,
            );
            return true;
        };
        for node in &document.nodes {
            Self::append_node(blocks, graph, node, quote_depth, list_depth);
        }
        true
    }

    fn append_code(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
        role: &CodeBlockRole,
        quote_depth: u32,
        list_depth: u32,
    ) {
        match role {
            CodeBlockRole::Diagram { kind } => {
                Self::append_diagram(blocks, graph, node, kind);
            }
            CodeBlockRole::Math => {
                Self::append_math_lines(blocks, &fenced_body(&node.source.raw.text), quote_depth);
            }
            CodeBlockRole::Plain { language } => {
                let lines = SurfaceCodeHighlighter::highlight(
                    language.as_deref(),
                    &fenced_body(&node.source.raw.text),
                )
                .into_iter()
                .map(SurfaceLine::code_spans)
                .collect::<Vec<_>>();
                blocks.push(SurfaceBlock::Code(SurfaceCodeBlock::new(
                    lines,
                    quote_depth,
                    list_depth,
                )));
            }
        }
    }

    fn append_math_lines(blocks: &mut Vec<SurfaceBlock>, expression: &str, _quote_depth: u32) {
        blocks.push(SurfaceBlock::Math(SurfaceMathBlock::new(expression)));
    }

    fn footnote_line(node: &KmmNode, quote_depth: u32) -> Option<SurfaceLine> {
        let KmmNodeKind::FootnoteDefinition(definition) = &node.kind else {
            return None;
        };
        let mut spans = vec![SurfaceTextSpan::plain(format!("{}. ", definition.label))];
        if node.children.is_empty() {
            spans.push(SurfaceTextSpan::plain(definition.text.clone()));
        } else {
            spans.extend(SurfaceInlineSpans::from_nodes(&node.children));
        }
        spans.push(SurfaceTextSpan::plain(" "));
        spans.push(SurfaceTextSpan::linked(
            "↩",
            format!("#fnref-{}", definition.label),
            crate::export_surface_span::SurfaceTextStyle::default().link(),
        ));
        Some(SurfaceLine::body_spans(spans, quote_depth))
    }

    fn append_diagram(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
        kind: &DiagramKind,
    ) {
        if let Some(diagram) = graph
            .rendered_diagrams
            .iter()
            .find(|diagram| diagram.node_id == node.id.0)
        {
            blocks.push(SurfaceBlock::Diagram(SurfaceDiagramBlock::rendered(
                &diagram.svg,
            )));
            return;
        }
        Self::append_wrapped(
            blocks,
            format!("Diagram rendering unavailable: {kind:?}"),
            0,
            0,
        );
    }

    fn append_table(
        blocks: &mut Vec<SurfaceBlock>,
        table: &TableNode,
        fallback_text: &str,
        quote_depth: u32,
        list_depth: u32,
    ) {
        if !has_surface_table_contract(table) {
            Self::append_wrapped(
                blocks,
                decode_basic_entities(fallback_text),
                quote_depth,
                list_depth,
            );
            return;
        }
        if quote_depth > 0 {
            for line in SurfaceTableBlock::new(table).text().lines() {
                Self::append_wrapped(blocks, line.to_string(), quote_depth, list_depth);
            }
            return;
        }
        blocks.push(SurfaceBlock::Table(SurfaceTableBlock::new(table)));
    }

    fn append_raw(blocks: &mut Vec<SurfaceBlock>, raw: &str, quote_depth: u32, list_depth: u32) {
        for line in raw.lines() {
            Self::append_wrapped(blocks, line.to_string(), quote_depth, list_depth);
        }
    }

    fn append_wrapped(
        blocks: &mut Vec<SurfaceBlock>,
        text: String,
        quote_depth: u32,
        list_depth: u32,
    ) {
        for chunk in WrappedText::new(&text, BODY_MAX_CHARS) {
            if list_depth > 0 {
                blocks.push(SurfaceBlock::Line(SurfaceLine::body_spans_with_indent(
                    vec![SurfaceTextSpan::plain(chunk)],
                    quote_depth,
                    list_depth,
                )));
            } else {
                blocks.push(SurfaceBlock::Line(SurfaceLine::body_with_quote(
                    chunk,
                    quote_depth,
                )));
            }
        }
    }

    fn append_rich_line(
        blocks: &mut Vec<SurfaceBlock>,
        node: &KmmNode,
        quote_depth: u32,
        list_depth: u32,
    ) {
        let spans = SurfaceInlineSpans::from_node(node);
        if spans.is_empty() {
            return;
        }
        if list_depth > 0 {
            blocks.push(SurfaceBlock::Line(SurfaceLine::body_spans_with_indent(
                spans,
                quote_depth,
                list_depth,
            )));
        } else {
            blocks.push(SurfaceBlock::Line(SurfaceLine::body_spans(
                spans,
                quote_depth,
            )));
        }
    }
}

fn list_marker_text(item: &ListItemNode, ordered: bool) -> String {
    if let Some(marker) = &item.task_marker {
        return format!("{} ", task_marker_text(marker));
    }
    if ordered {
        let number = item
            .ordered_number
            .or_else(|| ordered_number_from_marker(&item.marker))
            .unwrap_or(1);
        return format!("{number}. ");
    }
    "• ".to_string()
}

fn ordered_number_from_marker(marker: &str) -> Option<usize> {
    marker
        .trim_end_matches('.')
        .trim_end_matches(')')
        .parse::<usize>()
        .ok()
}

fn task_marker_text(marker: &str) -> &'static str {
    match marker {
        "[x]" => "☑",
        "[ ]" => "☐",
        "[-]" => "⊟",
        "[/]" => "◩",
        _ => "☐",
    }
}

fn spans_text(spans: &[SurfaceTextSpan]) -> String {
    spans.iter().map(|span| span.text.as_str()).collect()
}

fn inferred_link_anchor(
    span: &SurfaceTextSpan,
    page_index: usize,
    x: u32,
    y: u32,
) -> Option<SurfaceLinkAnchor> {
    let target = span.link_target.as_deref()?;
    if let Some(label) = target.strip_prefix("#fn-") {
        return Some(SurfaceLinkAnchor {
            id: format!("fnref-{label}"),
            page_index,
            x,
            y,
        });
    }
    if let Some(label) = target.strip_prefix("#fnref-") {
        return Some(SurfaceLinkAnchor {
            id: format!("fn-{label}"),
            page_index,
            x,
            y,
        });
    }
    None
}

fn draw_check_mark(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    for offset in 0..4 {
        fill_rect(image, x + 4 + offset, y + 10 + offset, 2, 2, color);
        fill_rect(image, x + 8 + offset, y + 13 - offset, 2, 2, color);
        fill_rect(image, x + 12 + offset, y + 9 - offset, 2, 2, color);
    }
}

fn draw_diagonal_mark(image: &mut RgbaImage, x: u32, y: u32, color: image::Rgba<u8>) {
    for offset in 0..TASK_MARKER_SIZE - 4 {
        fill_rect(
            image,
            x + 3 + offset,
            y + TASK_MARKER_SIZE - 5 - offset,
            2,
            2,
            color,
        );
    }
}

fn alert_title(label: &str) -> &str {
    match label {
        "TIP" => "Tip",
        "IMPORTANT" => "Important",
        "WARNING" => "Warning",
        "CAUTION" => "Caution",
        _ => "Note",
    }
}

fn alert_label_text(label: &str) -> String {
    format!("{} {}", alert_icon(label), alert_title(label))
}

fn alert_icon(label: &str) -> &str {
    match label {
        "TIP" => "💡",
        "IMPORTANT" => "▣",
        "WARNING" => "△",
        "CAUTION" => "!",
        _ => "ⓘ",
    }
}

fn alert_body_lines(node: &KmmNode) -> Vec<String> {
    let lines = node
        .children
        .iter()
        .map(inline_text)
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>();
    if !lines.is_empty() {
        return lines;
    }
    node.source
        .raw
        .text
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix('>'))
        .map(str::trim)
        .filter(|line| !line.starts_with("[!"))
        .filter(|line| !line.is_empty())
        .map(inline_markdown_text)
        .collect()
}

fn alert_color(label: &str) -> image::Rgba<u8> {
    match label {
        "TIP" => image::Rgba([26, 127, 55, 255]),
        "IMPORTANT" => image::Rgba([130, 80, 223, 255]),
        "WARNING" => image::Rgba([209, 36, 47, 255]),
        "CAUTION" => image::Rgba([191, 135, 0, 255]),
        _ => image::Rgba([9, 105, 218, 255]),
    }
}

fn legacy_note_quote(raw: &str) -> Option<(String, String)> {
    let mut lines = raw
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix('>'));
    let title = lines
        .next()?
        .trim()
        .strip_prefix("**")?
        .strip_suffix("**")?
        .trim()
        .to_string();
    if !is_legacy_note_title(&title) {
        return None;
    }
    let body = lines
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(inline_markdown_text)
        .collect::<Vec<_>>()
        .join(" ");
    (!body.is_empty()).then_some((title, body))
}

fn legacy_note_children(children: &[KmmNode]) -> Option<(String, String)> {
    let (first, rest) = children.split_first()?;
    let title = inline_text(first).trim().to_string();
    if !is_legacy_note_title(&title) {
        return None;
    }
    let body = rest
        .iter()
        .map(inline_text)
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    (!body.is_empty()).then_some((title, body))
}

fn is_legacy_note_title(title: &str) -> bool {
    matches!(title, "Note" | "Tip" | "Important" | "Warning" | "Caution")
}

fn normalize_surface_html_text(text: &str) -> String {
    let decoded = decode_basic_entities(text);
    let compact = decoded.split_whitespace().collect::<Vec<_>>().join(" ");
    compact
        .replace(" | ", "|")
        .replace(" |", "|")
        .replace("| ", "|")
        .replace('|', " | ")
}

fn badge_row_badges(fragment: &str) -> Vec<SurfaceBadge> {
    let badges = extract_img_refs(fragment)
        .into_iter()
        .filter_map(|image| shields_badge(&image.src, image.link_target))
        .collect::<Vec<_>>();
    if !badges.is_empty() {
        return badges;
    }
    let alt_text = html_fragment_text(fragment);
    let normalized = normalize_surface_html_text(&alt_text);
    if normalized.is_empty() {
        Vec::new()
    } else {
        vec![SurfaceBadge::single(normalized)]
    }
}

struct SurfaceHtmlImageRef {
    src: String,
    link_target: Option<String>,
}

fn extract_img_refs(fragment: &str) -> Vec<SurfaceHtmlImageRef> {
    let mut images = Vec::new();
    let mut rest = fragment;
    while let Some(img_start) = rest.find("<img") {
        let link_target = enclosing_link_target(&rest[..img_start]);
        let after_img = &rest[img_start..];
        let Some(img_end) = after_img.find('>') else {
            break;
        };
        let tag = &after_img[..img_end];
        if let Some(src) = quoted_attribute_value(tag, "src") {
            images.push(SurfaceHtmlImageRef { src, link_target });
        }
        rest = &after_img[img_end + 1..];
    }
    images
}

fn enclosing_link_target(prefix: &str) -> Option<String> {
    let link_start = prefix.rfind("<a")?;
    quoted_attribute_value(&prefix[link_start..], "href")
}

fn quoted_attribute_value(tag: &str, name: &str) -> Option<String> {
    let pattern = format!("{name}=\"");
    let start = tag.find(&pattern)? + pattern.len();
    let rest = &tag[start..];
    let end = rest.find('"')?;
    Some(decode_basic_entities(&rest[..end]))
}

fn shields_badge(src: &str, link_target: Option<String>) -> Option<SurfaceBadge> {
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
    segment.replace('_', " ")
}

fn badge_color(color: &str) -> image::Rgba<u8> {
    match color.to_ascii_lowercase().as_str() {
        "blue" => image::Rgba([0, 123, 192, 255]),
        "brightgreen" => image::Rgba([68, 204, 17, 255]),
        "green" => image::Rgba([76, 175, 80, 255]),
        "red" => image::Rgba([224, 49, 49, 255]),
        "orange" => image::Rgba([245, 159, 0, 255]),
        "yellow" => image::Rgba([250, 176, 5, 255]),
        _ => image::Rgba([159, 159, 159, 255]),
    }
}

fn centered_html_spans(fragment: &str) -> Vec<SurfaceTextSpan> {
    let mut spans = Vec::new();
    let mut rest = fragment;
    while let Some(link_start) = rest.find("<a") {
        push_plain_html_text(&mut spans, &rest[..link_start]);
        let after_link = &rest[link_start..];
        let Some(open_end) = after_link.find('>') else {
            break;
        };
        let link_tag = &after_link[..open_end];
        let link_target = quoted_attribute_value(link_tag, "href").unwrap_or_default();
        let content_start = open_end + 1;
        let Some(close_start) = after_link[content_start..].find("</a>") else {
            break;
        };
        let content = &after_link[content_start..content_start + close_start];
        let text = normalize_surface_html_text(&html_fragment_text(content));
        if !text.is_empty() {
            spans.push(SurfaceTextSpan::linked(
                text,
                link_target,
                crate::export_surface_span::SurfaceTextStyle::default().link(),
            ));
        }
        rest = &after_link[content_start + close_start + "</a>".len()..];
    }
    push_plain_html_text(&mut spans, rest);
    spans
}

fn push_plain_html_text(spans: &mut Vec<SurfaceTextSpan>, fragment: &str) {
    let text = normalize_surface_html_text(&html_fragment_text(fragment));
    if !text.is_empty() {
        spans.push(SurfaceTextSpan::plain(text));
    }
}

struct SurfaceDetailsParts<'a> {
    summary: &'a str,
    body: &'a str,
}

impl<'a> SurfaceDetailsParts<'a> {
    fn parse(fragment: &'a str) -> Option<Self> {
        let trimmed = fragment.trim();
        if !trimmed.starts_with("<details") {
            return None;
        }
        let summary_start = trimmed.find("<summary>")? + "<summary>".len();
        let summary_end = trimmed.find("</summary>")?;
        let body_start = summary_end + "</summary>".len();
        let body_end = trimmed.rfind("</details>")?;
        let body = Self::strip_div(&trimmed[body_start..body_end]);
        Some(Self {
            summary: &trimmed[summary_start..summary_end],
            body,
        })
    }

    fn strip_div(value: &'a str) -> &'a str {
        let trimmed = value.trim();
        if let Some(body) = trimmed.strip_prefix("<div>") {
            return body.strip_suffix("</div>").unwrap_or(body);
        }
        trimmed
    }
}

#[cfg(test)]
#[path = "export_surface_test_modules.rs"]
mod test_modules;
