use super::{
    ALERT_PANEL_BODY_X_OFFSET, ALERT_PANEL_BORDER_WIDTH, ALERT_PANEL_PADDING_X,
    ALERT_PANEL_PADDING_Y, ALERT_PANEL_TEXT_Y_STEP, ALERT_PANEL_TITLE_X_OFFSET, PAGE_PADDING,
    QUOTE_INDENT, SURFACE_WIDTH, SurfaceAlertBlock, SurfaceHelpers, SurfaceLine,
    SurfacePaintPalette, SurfacePainter, SurfaceTextPainter, alert_color, alert_title_icon_y,
    draw_caution_icon, draw_important_icon, draw_note_icon, draw_tip_icon, draw_warning_icon,
};
use image::RgbaImage;

impl SurfacePainter {
    pub(super) fn paint_alert(
        image: &mut RgbaImage,
        alert: &SurfaceAlertBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        Self::paint_alert_background(image, alert, y, palette);
        Self::paint_alert_text(image, alert, y, painter, palette);
    }

    pub(super) fn paint_alert_background(
        image: &mut RgbaImage,
        alert: &SurfaceAlertBlock,
        y: u32,
        palette: &SurfacePaintPalette,
    ) {
        let x = PAGE_PADDING + alert.quote_depth * QUOTE_INDENT;
        let width = SURFACE_WIDTH.saturating_sub(x + PAGE_PADDING);
        let panel_height = alert.height().saturating_sub(ALERT_PANEL_PADDING_Y * 2);
        let panel_y = y + ALERT_PANEL_PADDING_Y;
        SurfaceHelpers::fill_rect(
            image,
            x,
            panel_y,
            width,
            panel_height,
            palette.alert_background,
        );
        SurfaceHelpers::fill_rect(
            image,
            x,
            panel_y,
            ALERT_PANEL_BORDER_WIDTH,
            panel_height,
            alert_color(&alert.label),
        );
    }

    pub(super) fn paint_alert_text(
        image: &mut RgbaImage,
        alert: &SurfaceAlertBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let x = PAGE_PADDING + alert.quote_depth * QUOTE_INDENT;
        let icon_x = x + ALERT_PANEL_PADDING_X;
        let mut text_y = y + ALERT_PANEL_PADDING_Y + ALERT_PANEL_TEXT_Y_STEP;
        let title_x = icon_x + ALERT_PANEL_TITLE_X_OFFSET;
        let body_x = x + ALERT_PANEL_BODY_X_OFFSET;
        Self::paint_alert_icon(
            image,
            &alert.label,
            icon_x,
            alert_title_icon_y(text_y, alert.title.line_height()),
        );
        Self::paint_alert_line(image, &alert.title, title_x, text_y, painter, palette);
        text_y += alert.title.line_height();
        for line in &alert.body {
            Self::paint_alert_line(image, line, body_x, text_y, painter, palette);
            text_y += line.line_height();
        }
    }

    pub(super) fn paint_alert_icon(image: &mut RgbaImage, label: &str, x: u32, y: u32) {
        let color = alert_color(label);
        match label {
            "TIP" => draw_tip_icon(image, x, y, color),
            "IMPORTANT" => draw_important_icon(image, x, y, color),
            "WARNING" => draw_warning_icon(image, x, y, color),
            "CAUTION" => draw_caution_icon(image, x, y, color),
            _ => draw_note_icon(image, x, y, color),
        }
    }

    pub(super) fn paint_alert_line(
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
            None => SurfaceHelpers::draw_fallback_text(image, x, text_y, &line.text, palette.text),
        }
    }
}
