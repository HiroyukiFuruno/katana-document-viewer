use super::{
    BADGE_HEIGHT, BADGE_HORIZONTAL_GAP, BADGE_HORIZONTAL_PADDING, BADGE_LABEL_BACKGROUND,
    BADGE_TEXT_COLOR, BADGE_TEXT_FONT_SIZE, BADGE_TEXT_Y_OFFSET, BADGE_VERTICAL_MARGIN,
    PAGE_PADDING, SURFACE_CONTENT_WIDTH, SurfaceBadge, SurfaceBadgeRowBlock, SurfaceHelpers,
    SurfacePaintPalette, SurfacePainter, SurfaceTextLayout, SurfaceTextPainter,
};
use image::RgbaImage;

impl SurfacePainter {
    pub(super) fn paint_badge_row(
        image: &mut RgbaImage,
        row: &SurfaceBadgeRowBlock,
        y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) {
        let mut x = Self::badge_row_start_x(row);
        let badge_y = y + BADGE_VERTICAL_MARGIN;
        for badge in row.badges() {
            x = Self::paint_badge(image, badge, x, badge_y, painter, palette);
        }
    }

    pub(super) fn badge_row_start_x(row: &SurfaceBadgeRowBlock) -> u32 {
        PAGE_PADDING + SURFACE_CONTENT_WIDTH.saturating_sub(row.total_width()) / 2
    }

    pub(super) fn paint_badge(
        image: &mut RgbaImage,
        badge: &SurfaceBadge,
        x: u32,
        badge_y: u32,
        painter: &mut Option<SurfaceTextPainter>,
        palette: &SurfacePaintPalette,
    ) -> u32 {
        let label_width = badge.label_width();
        let message_width = badge.message_width();
        let width = badge.width();
        Self::paint_badge_backgrounds(image, badge, x, badge_y, label_width, message_width);
        SurfaceHelpers::stroke_rect(image, x, badge_y, width, BADGE_HEIGHT, palette.table_border);
        Self::paint_badge_label(image, badge, x, badge_y, painter);
        x + width + BADGE_HORIZONTAL_GAP
    }

    pub(super) fn paint_badge_backgrounds(
        image: &mut RgbaImage,
        badge: &SurfaceBadge,
        x: u32,
        badge_y: u32,
        label_width: u32,
        message_width: u32,
    ) {
        SurfaceHelpers::fill_rect(
            image,
            x,
            badge_y,
            label_width,
            BADGE_HEIGHT,
            BADGE_LABEL_BACKGROUND,
        );
        if message_width > 0 {
            SurfaceHelpers::fill_rect(
                image,
                x + label_width,
                badge_y,
                message_width,
                BADGE_HEIGHT,
                badge.color,
            );
        }
    }

    pub(super) fn paint_badge_label(
        image: &mut RgbaImage,
        badge: &SurfaceBadge,
        x: u32,
        badge_y: u32,
        painter: &mut Option<SurfaceTextPainter>,
    ) {
        if let Some(it) = painter {
            Self::paint_badge_label_texts(it, image, badge, x, badge_y);
            return;
        }
        Self::paint_fallback_badge_label(image, badge, x, badge_y);
    }

    pub(super) fn paint_badge_label_texts(
        painter: &mut SurfaceTextPainter,
        image: &mut RgbaImage,
        badge: &SurfaceBadge,
        x: u32,
        badge_y: u32,
    ) {
        Self::paint_badge_text(
            painter,
            image,
            &badge.label,
            x + BADGE_HORIZONTAL_PADDING,
            badge_y,
            badge.label_width(),
        );
        if !badge.message.is_empty() {
            Self::paint_badge_text(
                painter,
                image,
                &badge.message,
                x + badge.label_width() + BADGE_HORIZONTAL_PADDING,
                badge_y,
                badge.message_width().max(BADGE_HORIZONTAL_PADDING),
            );
        }
    }

    pub(super) fn paint_fallback_badge_label(
        image: &mut RgbaImage,
        badge: &SurfaceBadge,
        x: u32,
        badge_y: u32,
    ) {
        SurfaceHelpers::draw_fallback_text(
            image,
            x + BADGE_HORIZONTAL_PADDING,
            badge_y + BADGE_TEXT_Y_OFFSET,
            &badge.text(),
            BADGE_TEXT_COLOR,
        );
    }

    pub(super) fn paint_badge_text(
        painter: &mut SurfaceTextPainter,
        image: &mut RgbaImage,
        text: &str,
        x: u32,
        badge_y: u32,
        width: u32,
    ) {
        painter.draw_text(
            image,
            text,
            SurfaceTextLayout {
                x,
                y: badge_y + BADGE_TEXT_Y_OFFSET,
                size: BADGE_TEXT_FONT_SIZE,
                color: BADGE_TEXT_COLOR,
                max_width: Some(width.saturating_sub(BADGE_HORIZONTAL_PADDING * 2) as f32),
            },
        );
    }
}

#[cfg(test)]
#[path = "export_surface_painter_badge_tests.rs"]
mod tests;
