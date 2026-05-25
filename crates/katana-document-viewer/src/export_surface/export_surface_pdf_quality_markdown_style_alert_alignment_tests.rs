use crate::KdvThemeSnapshot;
use crate::export_surface::ALERT_ICON_SIZE;
use crate::export_surface::DocumentSurfaceFactory;
use crate::export_surface::markup::alert_color;
use crate::export_surface::test_modules::test_support::SurfaceTestSupport;
use crate::export_surface::{SurfaceAlertBlock, SurfaceBlock, SurfaceBlockFactory};
use crate::export_surface_helpers::PAGE_PADDING;

const ALERT_ICON_SEARCH_X_OFFSET: u32 = 28;
const ALERT_TITLE_SEARCH_X_OFFSET: u32 = 58;
const ALERT_TITLE_SEARCH_WIDTH: u32 = 180;
const ALERT_TITLE_TEXT_Y_OFFSET: u32 = 16;

#[test]
fn pdf_surface_alert_title_icon_is_visually_centered_with_title_label()
-> Result<(), Box<dyn std::error::Error>> {
    AlertTitleAlignmentCase::assert_centered()
}

struct AlertTitleAlignmentCase;

impl AlertTitleAlignmentCase {
    fn assert_centered() -> Result<(), Box<dyn std::error::Error>> {
        for fixture in [
            ("[!WARNING]", "WARNING", "Warning body"),
            ("[!CAUTION]", "CAUTION", "Caution body"),
        ] {
            let (raw, label, body) = fixture;
            Self::assert_label(raw, label, body)?;
        }
        Ok(())
    }

    fn assert_label(raw: &str, label: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        let graph = SurfaceTestSupport::graph_from_markdown(
            "alert-icon-title.md",
            format!("> {raw}\n> {body}"),
        )?;
        let surface = DocumentSurfaceFactory::create(&graph, &KdvThemeSnapshot::katana_light());
        let blocks = SurfaceBlockFactory::create(&graph, &graph.theme);
        let alert_y = Self::alert_label_y(&blocks, label)
            .ok_or_else(|| std::io::Error::other("missing expected alert y"))?;
        let alert = Self::find_alert(&blocks, label)
            .ok_or_else(|| std::io::Error::other("missing expected alert"))?;

        Self::assert_icon_and_title_are_centered(&surface.image, alert, alert_y, label)?;
        Ok(())
    }

    fn alert_label_y(blocks: &[SurfaceBlock], label: &str) -> Option<u32> {
        let mut y = PAGE_PADDING;
        for block in blocks {
            if let SurfaceBlock::Alert(alert) = block
                && alert.label == label
            {
                return Some(y);
            }
            y = y.saturating_add(block.height());
        }
        None
    }

    fn find_alert<'a>(blocks: &'a [SurfaceBlock], label: &str) -> Option<&'a SurfaceAlertBlock> {
        for block in blocks {
            if let SurfaceBlock::Alert(alert) = block
                && alert.label == label
            {
                return Some(alert);
            }
        }
        None
    }

    fn assert_icon_and_title_are_centered(
        image: &image::RgbaImage,
        alert: &SurfaceAlertBlock,
        alert_y: u32,
        label: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let text_y = Self::alert_title_text_y(alert_y);
        let marker = alert_color(label);
        let title_line_height = alert.title.line_height();
        let icon_area = Self::bbox_for_image_segment(
            image,
            PAGE_PADDING + ALERT_ICON_SEARCH_X_OFFSET,
            PAGE_PADDING + ALERT_ICON_SEARCH_X_OFFSET + ALERT_ICON_SIZE,
            text_y,
            text_y + title_line_height,
            marker,
            "missing alert icon bbox",
        )?;
        let title_area = Self::bbox_for_image_segment(
            image,
            PAGE_PADDING + ALERT_TITLE_SEARCH_X_OFFSET,
            PAGE_PADDING + ALERT_TITLE_SEARCH_X_OFFSET + ALERT_TITLE_SEARCH_WIDTH,
            text_y,
            text_y + title_line_height,
            marker,
            "missing alert title bbox",
        )?;
        Self::assert_centers_are_aligned(icon_area, title_area, label);
        Ok(())
    }

    fn alert_title_text_y(alert_y: u32) -> u32 {
        alert_y + ALERT_TITLE_TEXT_Y_OFFSET
    }

    fn assert_centers_are_aligned(icon_area: (u32, u32), title_area: (u32, u32), label: &str) {
        let icon_center = (icon_area.0 as f32 + icon_area.1 as f32) / 2.0;
        let title_center = (title_area.0 as f32 + title_area.1 as f32) / 2.0;
        let center_delta = icon_center - title_center;
        assert!(
            center_delta.abs() <= 4.0,
            "alert title icon/text center y mismatch for {label}: delta={center_delta}, icon_y={:?}, title_y={:?}",
            icon_area.0,
            title_area.0,
        );
    }

    fn bbox_for_image_segment(
        image: &image::RgbaImage,
        x0: u32,
        x1: u32,
        y0: u32,
        y1: u32,
        marker: image::Rgba<u8>,
        missing_message: &str,
    ) -> Result<(u32, u32), Box<dyn std::error::Error>> {
        let area = Self::bbox_in_area(image, x0, x1, y0, y1, marker)
            .ok_or_else(|| std::io::Error::other(missing_message))?;
        Ok(area)
    }

    fn bbox_in_area(
        image: &image::RgbaImage,
        x0: u32,
        x1: u32,
        y0: u32,
        y1: u32,
        marker: image::Rgba<u8>,
    ) -> Option<(u32, u32)> {
        let mut top = y1;
        let mut bottom = 0;
        for y in y0..y1 {
            for x in x0..x1 {
                if image.get_pixel(x, y).0 != marker.0 {
                    continue;
                }
                if y < top {
                    top = y;
                }
                if y > bottom {
                    bottom = y;
                }
            }
        }
        if bottom == 0 && top == y1 {
            return None;
        }
        Some((top, bottom))
    }
}
