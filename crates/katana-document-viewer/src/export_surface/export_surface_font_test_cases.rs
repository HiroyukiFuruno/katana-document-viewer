#[cfg(test)]
mod tests {
    use super::super::export_surface_font_test_support::{
        actual_span_x_range, estimated_text_width, painted_x_range,
    };
    use super::super::{SurfaceTextLayout, SurfaceTextPainter, rendering};
    use crate::export_surface_span::{SurfaceTextSpan, SurfaceTextStyle};
    use image::{Rgba, RgbaImage};

    #[test]
    fn emoji_characters_are_not_rendered_as_blank_advance() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut painter =
            SurfaceTextPainter::from_system_fonts().ok_or("system font must be available")?;
        let background = Rgba([255, 255, 255, 255]);
        let mut image = RgbaImage::from_pixel(96, 64, background);

        painter.draw_text(
            &mut image,
            "🌍",
            SurfaceTextLayout {
                x: 8,
                y: 8,
                size: 32.0,
                color: Rgba([0, 0, 0, 255]),
                max_width: None,
            },
        );

        assert!(image.pixels().any(|pixel| *pixel != background));
        Ok(())
    }

    #[test]
    fn cjk_underline_width_tracks_full_width_text() {
        let width = estimated_text_width("下線", 24.0);

        assert!(
            width >= 44,
            "CJK underline must cover the rendered text width: {width}"
        );
    }

    #[test]
    fn url_underline_width_does_not_overshoot_ascii_text() {
        let width = estimated_text_width("https://github.com", 24.0);

        assert!(
            (190..=225).contains(&width),
            "URL underline should stay close to the rendered text width: {width}"
        );
    }

    #[test]
    fn plain_underline_uses_text_color_not_link_blue() {
        let mut image = RgbaImage::from_pixel(96, 64, Rgba([255, 255, 255, 255]));
        let spans = vec![SurfaceTextSpan::styled(
            "下線",
            SurfaceTextStyle::default().underline(),
        )];

        let ranges = vec![Some(rendering::SpanVisualRange::new(0.0, 44.0))];
        rendering::draw_span_decorations(&mut image, &spans, &ranges, 8, 8, 24.0);

        let underline_y = 8 + (24.0 * 1.22) as u32;
        let pixel = image.get_pixel(8, underline_y);
        assert_eq!(*pixel, Rgba([36, 41, 47, 255]));
    }

    #[test]
    fn link_underline_starts_at_actual_glyph_range_after_cjk_text()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut painter =
            SurfaceTextPainter::from_system_fonts().ok_or("system font must be available")?;
        let mut image = RgbaImage::from_pixel(720, 96, Rgba([255, 255, 255, 255]));
        let spans = vec![
            SurfaceTextSpan::plain("これは脚注付きのテキストです"),
            SurfaceTextSpan::linked("[1]", "#fn-1", SurfaceTextStyle::default().link()),
        ];

        painter.draw_spans(&mut image, &spans, 8, 16, 24.0, Rgba([36, 41, 47, 255]));

        let actual_link = actual_span_x_range(&spans, 1, 24.0).ok_or("link span must layout")?;
        let drawn_link =
            painted_x_range(&image, link_color()).ok_or("link underline must exist")?;
        assert!(
            drawn_link.0 >= 8 + actual_link.0.saturating_sub(1),
            "link underline must not start before actual glyph range: actual={actual_link:?}, drawn={drawn_link:?}"
        );
        assert!(
            drawn_link.1 <= 8 + actual_link.1 + 1,
            "link underline must not extend beyond actual glyph range: actual={actual_link:?}, drawn={drawn_link:?}"
        );
        Ok(())
    }

    #[test]
    fn backlink_underline_stays_on_backlink_marker_only() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut painter =
            SurfaceTextPainter::from_system_fonts().ok_or("system font must be available")?;
        let mut image = RgbaImage::from_pixel(720, 96, Rgba([255, 255, 255, 255]));
        let spans = vec![
            SurfaceTextSpan::linked("[1] ↩", "#fnref-1", SurfaceTextStyle::default().link()),
            SurfaceTextSpan::plain(" 最初の脚注の内容。"),
        ];

        painter.draw_spans(&mut image, &spans, 8, 16, 24.0, Rgba([36, 41, 47, 255]));

        let actual_link =
            actual_span_x_range(&spans, 0, 24.0).ok_or("backlink span must layout")?;
        let drawn_link =
            painted_x_range(&image, link_color()).ok_or("backlink underline must exist")?;
        assert!(
            drawn_link.0 >= 8 + actual_link.0.saturating_sub(1),
            "backlink underline must start at actual glyph range: actual={actual_link:?}, drawn={drawn_link:?}"
        );
        assert!(
            drawn_link.1 <= 8 + actual_link.1 + 1,
            "backlink underline must not enter following footnote text: actual={actual_link:?}, drawn={drawn_link:?}"
        );
        Ok(())
    }

    #[test]
    fn monospace_japanese_span_uses_readable_fallback_font()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut painter =
            SurfaceTextPainter::from_system_fonts().ok_or("system font must be available")?;
        let background = Rgba([255, 255, 255, 255]);
        let mut image = RgbaImage::from_pixel(480, 96, background);
        let spans = vec![SurfaceTextSpan::styled(
            "これは言語指定なしのコードブロックです。",
            SurfaceTextStyle::default().monospace(),
        )];

        painter.draw_spans(&mut image, &spans, 8, 16, 24.0, Rgba([36, 41, 47, 255]));

        let right_side_pixels = image
            .enumerate_pixels()
            .filter(|(x, _, pixel)| *x > 180 && **pixel != background)
            .count();
        assert!(
            right_side_pixels > 20,
            "Japanese code text must not disappear after the first few glyphs"
        );
        Ok(())
    }

    fn link_color() -> Rgba<u8> {
        Rgba([9, 105, 218, 255])
    }
}
