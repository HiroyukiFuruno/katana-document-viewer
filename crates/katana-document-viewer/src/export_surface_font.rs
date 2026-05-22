use crate::export_surface_span::SurfaceTextSpan;
use cosmic_text::{
    Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, Style, SwashCache, Weight,
};
use image::{Rgba, RgbaImage};

pub(crate) struct SurfaceTextLayout {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) size: f32,
    pub(crate) color: Rgba<u8>,
    pub(crate) max_width: Option<f32>,
}

pub(crate) struct SurfaceTextPainter {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl SurfaceTextPainter {
    pub(crate) fn from_system_fonts() -> Option<Self> {
        Some(Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
        })
    }

    pub(crate) fn draw_text(
        &mut self,
        image: &mut RgbaImage,
        text: &str,
        layout: SurfaceTextLayout,
    ) {
        let metrics = Metrics::new(layout.size, layout.size * 1.35);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        let max_width = layout
            .max_width
            .unwrap_or_else(|| image.width().saturating_sub(layout.x) as f32);
        buffer.set_size(Some(max_width), Some(layout.size * 1.8));
        buffer.set_text(text, &Attrs::new(), Shaping::Advanced, None);
        self.draw_buffer(image, &mut buffer, layout.x, layout.y, layout.color);
    }

    pub(crate) fn draw_spans(
        &mut self,
        image: &mut RgbaImage,
        spans: &[SurfaceTextSpan],
        x: u32,
        y: u32,
        size: f32,
        color: Rgba<u8>,
    ) {
        let metrics = Metrics::new(size, size * 1.35);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        let max_width = image.width().saturating_sub(x) as f32;
        buffer.set_size(Some(max_width), Some(size * 1.8));
        let default_attrs = Attrs::new();
        let rich = spans.iter().enumerate().map(|(index, span)| {
            (
                span.text.as_str(),
                attrs_for_span_with_metadata(span, index.saturating_add(1)),
            )
        });
        buffer.set_rich_text(rich, &default_attrs, Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut self.font_system, false);
        let ranges = span_visual_ranges(&buffer, spans.len());
        draw_span_backgrounds(image, spans, &ranges, x, y, size);
        self.draw_buffer(image, &mut buffer, x, y, color);
        draw_span_decorations(image, spans, &ranges, x, y, size);
    }

    fn draw_buffer(
        &mut self,
        image: &mut RgbaImage,
        buffer: &mut Buffer,
        x: u32,
        y: u32,
        color: Rgba<u8>,
    ) {
        let default_color = Color::rgba(color[0], color[1], color[2], color[3]);
        buffer.draw(
            &mut self.font_system,
            &mut self.swash_cache,
            default_color,
            |glyph_x, glyph_y, width, height, pixel| {
                fill_glyph_pixel(
                    image,
                    SurfaceGlyphPixel {
                        origin_x: x,
                        origin_y: y,
                        glyph_x,
                        glyph_y,
                        width,
                        height,
                        color: pixel,
                    },
                );
            },
        );
    }
}

fn attrs_for_span_with_metadata(span: &SurfaceTextSpan, metadata: usize) -> Attrs<'static> {
    let mut attrs = Attrs::new();
    let style = span.style;
    if style.bold {
        attrs = attrs.weight(Weight::BOLD);
    }
    if style.italic {
        attrs = attrs.style(Style::Italic);
    }
    if style.monospace && span.text.is_ascii() {
        attrs = attrs.family(Family::Monospace);
    }
    if let Some(color) = style.color {
        attrs = attrs.color(Color::rgba(color[0], color[1], color[2], color[3]));
    }
    attrs.metadata(metadata)
}

#[derive(Clone, Copy, Debug)]
struct SpanVisualRange {
    start_x: u32,
    end_x: u32,
}

impl SpanVisualRange {
    fn new(start_x: f32, end_x: f32) -> Self {
        Self {
            start_x: start_x.floor().max(0.0) as u32,
            end_x: end_x.ceil().max(0.0) as u32,
        }
    }

    fn merge(self, start_x: f32, end_x: f32) -> Self {
        Self::new(
            (self.start_x as f32).min(start_x),
            (self.end_x as f32).max(end_x),
        )
    }

    fn width(self) -> u32 {
        self.end_x.saturating_sub(self.start_x).max(1)
    }
}

fn span_visual_ranges(buffer: &Buffer, span_count: usize) -> Vec<Option<SpanVisualRange>> {
    let mut ranges: Vec<Option<SpanVisualRange>> = vec![None; span_count];
    for run in buffer.layout_runs() {
        for glyph in run.glyphs {
            if glyph.metadata == 0 {
                continue;
            }
            let span_index = glyph.metadata.saturating_sub(1);
            if span_index >= span_count {
                continue;
            }
            let start_x = glyph.x;
            let end_x = glyph.x + glyph.w;
            ranges[span_index] = Some(match ranges[span_index] {
                Some(range) => range.merge(start_x, end_x),
                None => SpanVisualRange::new(start_x, end_x),
            });
        }
    }
    ranges
}

fn draw_span_backgrounds(
    image: &mut RgbaImage,
    spans: &[SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
) {
    for (index, span) in spans.iter().enumerate() {
        let Some(range) = ranges.get(index).and_then(|range| *range) else {
            continue;
        };
        let cursor_x = x.saturating_add(range.start_x);
        let width = range.width();
        if span.style.highlight {
            fill_rect(
                image,
                cursor_x,
                y + (size * 0.12) as u32,
                width,
                (size * 1.18) as u32,
                Rgba([255, 235, 59, 255]),
            );
        }
        if span.style.inline_code {
            fill_rect(
                image,
                cursor_x.saturating_sub(4),
                y + (size * 0.08) as u32,
                width + 8,
                (size * 1.24) as u32,
                Rgba([239, 242, 246, 255]),
            );
        }
    }
}

fn draw_span_decorations(
    image: &mut RgbaImage,
    spans: &[SurfaceTextSpan],
    ranges: &[Option<SpanVisualRange>],
    x: u32,
    y: u32,
    size: f32,
) {
    for (index, span) in spans.iter().enumerate() {
        let Some(range) = ranges.get(index).and_then(|range| *range) else {
            continue;
        };
        let cursor_x = x.saturating_add(range.start_x);
        let width = range.width();
        if span.style.underline {
            let color = span.style.color.unwrap_or(Rgba([36, 41, 47, 255]));
            let underline_y = y + (size * 1.22) as u32;
            draw_horizontal_line(image, cursor_x, underline_y, width, color);
        }
        if span.style.strikethrough {
            let color = span.style.color.unwrap_or(Rgba([36, 41, 47, 255]));
            let strike_y = y + (size * 0.72) as u32;
            draw_horizontal_line(image, cursor_x, strike_y, width, color);
        }
    }
}

#[cfg(test)]
fn estimated_text_width(text: &str, size: f32) -> u32 {
    text.chars()
        .map(|character| character_width_factor(character) * size)
        .sum::<f32>()
        .ceil() as u32
}

#[cfg(test)]
fn character_width_factor(character: char) -> f32 {
    if character.is_ascii_whitespace() {
        return 0.35;
    }
    if character.is_ascii_punctuation() {
        return 0.43;
    }
    if character.is_ascii() {
        return 0.54;
    }
    if is_half_width_math_symbol(character) {
        return 0.65;
    }
    0.92
}

#[cfg(test)]
fn is_half_width_math_symbol(character: char) -> bool {
    matches!(
        character,
        'α' | 'β'
            | 'γ'
            | 'δ'
            | '∑'
            | '∫'
            | '√'
            | '∞'
            | '⁰'
            | '¹'
            | '²'
            | '³'
            | '⁴'
            | '⁵'
            | '⁶'
            | '⁷'
            | '⁸'
            | '⁹'
            | 'ⁿ'
            | 'ˣ'
            | '₀'
            | '₁'
            | '₂'
            | '₃'
            | '₄'
            | '₅'
            | '₆'
            | '₇'
            | '₈'
            | '₉'
            | 'ₖ'
    )
}

fn draw_horizontal_line(image: &mut RgbaImage, x: u32, y: u32, width: u32, color: Rgba<u8>) {
    for dy in 0..2 {
        for dx in 0..width {
            let target_x = x + dx;
            let target_y = y + dy;
            if target_x < image.width() && target_y < image.height() {
                image.put_pixel(target_x, target_y, color);
            }
        }
    }
}

fn fill_rect(image: &mut RgbaImage, x: u32, y: u32, width: u32, height: u32, color: Rgba<u8>) {
    for dy in 0..height {
        for dx in 0..width {
            let target_x = x + dx;
            let target_y = y + dy;
            if target_x < image.width() && target_y < image.height() {
                image.put_pixel(target_x, target_y, color);
            }
        }
    }
}

struct SurfaceGlyphPixel {
    origin_x: u32,
    origin_y: u32,
    glyph_x: i32,
    glyph_y: i32,
    width: u32,
    height: u32,
    color: Color,
}

fn fill_glyph_pixel(image: &mut RgbaImage, glyph: SurfaceGlyphPixel) {
    let [red, green, blue, alpha] = glyph.color.as_rgba();
    for dy in 0..glyph.height {
        for dx in 0..glyph.width {
            blend_pixel(
                image,
                glyph.origin_x as i32 + glyph.glyph_x + dx as i32,
                glyph.origin_y as i32 + glyph.glyph_y + dy as i32,
                Rgba([red, green, blue, alpha]),
            );
        }
    }
}

fn blend_pixel(image: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>) {
    if x < 0 || y < 0 || x >= image.width() as i32 || y >= image.height() as i32 {
        return;
    }
    let alpha = f32::from(color[3]) / 255.0;
    let pixel = image.get_pixel_mut(x as u32, y as u32);
    for index in 0..3 {
        let source = f32::from(color[index]);
        let target = f32::from(pixel[index]);
        pixel[index] = (source * alpha + target * (1.0 - alpha)) as u8;
    }
    pixel[3] = 255;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::export_surface_span::SurfaceTextStyle;

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

        let ranges = vec![Some(SpanVisualRange::new(0.0, 44.0))];
        draw_span_decorations(&mut image, &spans, &ranges, 8, 8, 24.0);

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

    fn actual_span_x_range(
        spans: &[SurfaceTextSpan],
        span_index: usize,
        size: f32,
    ) -> Option<(u32, u32)> {
        let mut font_system = cosmic_text::FontSystem::new();
        let metrics = cosmic_text::Metrics::new(size, size * 1.35);
        let mut buffer = cosmic_text::Buffer::new(&mut font_system, metrics);
        buffer.set_size(Some(2048.0), Some(size * 1.8));
        let default_attrs = cosmic_text::Attrs::new();
        let rich = spans.iter().enumerate().map(|(index, span)| {
            (
                span.text.as_str(),
                attrs_for_span_with_metadata(span, index.saturating_add(1)),
            )
        });
        buffer.set_rich_text(rich, &default_attrs, cosmic_text::Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut font_system, false);

        let target_metadata = span_index.saturating_add(1);
        let mut min_x: Option<f32> = None;
        let mut max_x: Option<f32> = None;
        for run in buffer.layout_runs() {
            for glyph in run.glyphs {
                if glyph.metadata != target_metadata {
                    continue;
                }
                min_x = Some(min_x.map_or(glyph.x, |current| current.min(glyph.x)));
                max_x =
                    Some(max_x.map_or(glyph.x + glyph.w, |current| current.max(glyph.x + glyph.w)));
            }
        }
        Some((min_x?.floor() as u32, max_x?.ceil() as u32))
    }

    fn painted_x_range(image: &RgbaImage, color: Rgba<u8>) -> Option<(u32, u32)> {
        let mut min_x = None;
        let mut max_x = None;
        for (x, _, pixel) in image.enumerate_pixels() {
            if *pixel != color {
                continue;
            }
            min_x = Some(min_x.map_or(x, |current: u32| current.min(x)));
            max_x = Some(max_x.map_or(x, |current: u32| current.max(x)));
        }
        Some((min_x?, max_x?))
    }
}
