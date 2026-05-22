use crate::export_surface_text::inline_markdown_text;
use image::{Rgba, RgbaImage};

pub(crate) struct WrappedText {
    chunks: Vec<String>,
    index: usize,
}

impl WrappedText {
    pub(crate) fn new(text: &str, max_chars: usize) -> Self {
        let characters: Vec<char> = text.chars().collect();
        let mut chunks = Vec::new();
        for chunk in characters.chunks(max_chars) {
            chunks.push(chunk.iter().collect());
        }
        if chunks.is_empty() {
            chunks.push(String::new());
        }
        Self { chunks, index: 0 }
    }
}

impl Iterator for WrappedText {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.chunks.get(self.index)?.clone();
        self.index += 1;
        Some(item)
    }
}

pub(crate) fn is_nested_blockquote(text: &str) -> bool {
    text.lines()
        .filter_map(blockquote_line_parts)
        .any(|line| line.depth > 1)
}

pub(crate) fn nested_blockquote_lines(text: &str, base_depth: u32) -> Vec<(String, u32)> {
    text.lines()
        .filter_map(blockquote_line_parts)
        .filter(|line| !line.text.trim().is_empty())
        .map(|line| (inline_markdown_text(line.text), base_depth + line.depth))
        .collect()
}

fn blockquote_line_parts(line: &str) -> Option<BlockquoteLine<'_>> {
    let mut rest = line.trim_start();
    let mut depth = 0;
    while let Some(stripped) = rest.strip_prefix('>') {
        depth += 1;
        rest = stripped.trim_start();
    }
    (depth > 0).then_some(BlockquoteLine { depth, text: rest })
}

struct BlockquoteLine<'a> {
    depth: u32,
    text: &'a str,
}

pub(crate) fn surface_block_height(block_heights: impl Iterator<Item = u32>) -> u32 {
    let content_height = block_heights.sum::<u32>() + PAGE_PADDING * 2;
    content_height.max(SURFACE_MIN_HEIGHT)
}

pub(crate) fn parse_color(value: &str) -> Rgba<u8> {
    let value = value.trim_start_matches('#');
    if value.len() != 6 {
        return Rgba([255, 255, 255, 255]);
    }
    let red = u8::from_str_radix(&value[0..2], 16).unwrap_or(255);
    let green = u8::from_str_radix(&value[2..4], 16).unwrap_or(255);
    let blue = u8::from_str_radix(&value[4..6], 16).unwrap_or(255);
    Rgba([red, green, blue, 255])
}

pub(crate) fn draw_quote_bars(
    image: &mut RgbaImage,
    depth: u32,
    y: u32,
    height: u32,
    color: Rgba<u8>,
) {
    for index in 0..depth {
        let x = PAGE_PADDING + index * QUOTE_INDENT;
        for dy in 0..height {
            for dx in 0..4 {
                put_pixel_if_inside(image, x + dx, y + dy, color);
            }
        }
    }
}

pub(crate) fn fill_rect(
    image: &mut RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: Rgba<u8>,
) {
    for dy in 0..height {
        for dx in 0..width {
            put_pixel_if_inside(image, x + dx, y + dy, color);
        }
    }
}

pub(crate) fn stroke_rect(
    image: &mut RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: Rgba<u8>,
) {
    if width == 0 || height == 0 {
        return;
    }
    for dx in 0..width {
        put_pixel_if_inside(image, x + dx, y, color);
        put_pixel_if_inside(image, x + dx, y + height - 1, color);
    }
    for dy in 0..height {
        put_pixel_if_inside(image, x, y + dy, color);
        put_pixel_if_inside(image, x + width - 1, y + dy, color);
    }
}

pub(crate) fn paste_rgba(image: &mut RgbaImage, source: &RgbaImage, x: u32, y: u32) {
    for source_y in 0..source.height() {
        for source_x in 0..source.width() {
            let target_x = x + source_x;
            let target_y = y + source_y;
            if target_x >= image.width() || target_y >= image.height() {
                continue;
            }
            let foreground = source.get_pixel(source_x, source_y);
            let alpha = foreground[3] as u16;
            if alpha == 0 {
                continue;
            }
            let background = image.get_pixel(target_x, target_y);
            let inverse_alpha = 255 - alpha;
            let red = blend_channel(foreground[0], background[0], alpha, inverse_alpha);
            let green = blend_channel(foreground[1], background[1], alpha, inverse_alpha);
            let blue = blend_channel(foreground[2], background[2], alpha, inverse_alpha);
            image.put_pixel(target_x, target_y, Rgba([red, green, blue, 255]));
        }
    }
}

fn blend_channel(foreground: u8, background: u8, alpha: u16, inverse_alpha: u16) -> u8 {
    let blended = foreground as u16 * alpha + background as u16 * inverse_alpha;
    (blended / 255) as u8
}

pub(crate) fn draw_fallback_text(
    image: &mut RgbaImage,
    x: u32,
    y: u32,
    text: &str,
    color: Rgba<u8>,
) {
    let width = (text.chars().count() as u32).saturating_mul(10).min(720);
    for dy in 0..18 {
        for dx in 0..width {
            put_pixel_if_inside(image, x + dx, y + dy, color);
        }
    }
}

fn put_pixel_if_inside(image: &mut RgbaImage, x: u32, y: u32, color: Rgba<u8>) {
    if x < image.width() && y < image.height() {
        image.put_pixel(x, y, color);
    }
}

pub(crate) const SURFACE_WIDTH: u32 = 1280;
pub(crate) const SURFACE_MIN_HEIGHT: u32 = 720;
pub(crate) const SURFACE_PAGE_HEIGHT: u32 = 1810;
pub(crate) const PAGE_PADDING: u32 = 56;
pub(crate) const BODY_MAX_CHARS: usize = 58;
pub(crate) const QUOTE_INDENT: u32 = 32;
pub(crate) const LIST_INDENT: u32 = 42;
pub(crate) const SURFACE_CONTENT_WIDTH: u32 = SURFACE_WIDTH - PAGE_PADDING * 2;
