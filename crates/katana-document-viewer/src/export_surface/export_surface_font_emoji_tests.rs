use super::{SurfaceTextLayout, SurfaceTextPainter};
use crate::export_surface_span::{SurfaceTextSpan, SurfaceTextStyle};
use image::{Rgba, RgbaImage};

#[test]
fn emoji_characters_are_not_rendered_as_blank_advance() -> Result<(), Box<dyn std::error::Error>> {
    let mut painter = SurfaceTextPainter::from_system_fonts();
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
fn emoji_span_preserves_color_pixels() {
    let mut painter = SurfaceTextPainter::from_system_fonts();
    let background = Rgba([16, 16, 16, 255]);
    let mut image = RgbaImage::from_pixel(120, 96, background);
    let spans = vec![SurfaceTextSpan::styled(
        "🔥",
        SurfaceTextStyle::default().emoji(),
    )];

    painter.draw_spans(&mut image, &spans, 16, 12, 64.0, Rgba([245, 245, 245, 255]));

    let chromatic_pixels = image
        .pixels()
        .filter(|pixel| **pixel != background)
        .filter(|pixel| is_chromatic(**pixel))
        .count();
    assert!(chromatic_pixels > 32);
}

#[test]
fn issue_14_emoji_sequence_paints_visible_pixels() {
    let mut painter = SurfaceTextPainter::from_system_fonts();
    let background = Rgba([255, 255, 255, 255]);
    let mut image = RgbaImage::from_pixel(720, 120, background);
    let spans = vec![SurfaceTextSpan::styled(
        "🧪 ✨ ✅ ⚠️ 🛠️ 🧑‍💻",
        SurfaceTextStyle::default().emoji(),
    )];

    painter.draw_spans(&mut image, &spans, 12, 16, 32.0, Rgba([36, 36, 36, 255]));

    let painted_pixels = image.pixels().filter(|pixel| **pixel != background).count();
    assert!(
        painted_pixels > 64,
        "issue #14 emoji sequence must not render as missing blank glyphs"
    );
}

#[test]
fn star_variation_sequence_paints_visible_pixels() {
    let mut painter = SurfaceTextPainter::from_system_fonts();
    let background = Rgba([255, 255, 255, 255]);
    let mut image = RgbaImage::from_pixel(160, 96, background);
    let spans = vec![SurfaceTextSpan::styled(
        "⭐️",
        SurfaceTextStyle::default().emoji(),
    )];

    painter.draw_spans(&mut image, &spans, 16, 12, 48.0, Rgba([36, 36, 36, 255]));

    let painted_pixels = image.pixels().filter(|pixel| **pixel != background).count();
    assert!(
        painted_pixels > 32,
        "star variation emoji must not render as missing blank glyphs"
    );
}

fn is_chromatic(pixel: Rgba<u8>) -> bool {
    pixel[0] != pixel[1] || pixel[1] != pixel[2]
}
