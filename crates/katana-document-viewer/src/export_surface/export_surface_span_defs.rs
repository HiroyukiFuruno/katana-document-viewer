use crate::export_surface_svg::SurfaceSvgImage;
use image::Rgba;

use super::{SurfaceInlineImage, SurfaceTextSpan, SurfaceTextStyle};

const INLINE_IMAGE_LAYOUT_SPACE_FACTOR: f32 = 0.35;
const ASCII_TEXT_WIDTH_ESTIMATE_FACTOR: f32 = 0.58;
const SPACE_TEXT_WIDTH_ESTIMATE_FACTOR: f32 = 0.35;
const WIDE_TEXT_WIDTH_ESTIMATE_FACTOR: f32 = 1.0;
const LINK_COLOR_RED: u8 = 9;
const LINK_COLOR_GREEN: u8 = 105;
const LINK_COLOR_BLUE: u8 = 218;
const LINK_COLOR_ALPHA: u8 = 255;

impl SurfaceInlineImage {
    pub(crate) fn new(image: SurfaceSvgImage) -> Self {
        Self {
            image: std::sync::Arc::new(image.image),
        }
    }

    pub(crate) fn image(&self) -> &image::RgbaImage {
        &self.image
    }

    pub(crate) fn width(&self) -> u32 {
        self.image.width()
    }

    pub(crate) fn height(&self) -> u32 {
        self.image.height()
    }
}

impl SurfaceTextSpan {
    pub(crate) fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: SurfaceTextStyle::default(),
            link_target: None,
            inline_image: None,
        }
    }

    pub(crate) fn styled(text: impl Into<String>, style: SurfaceTextStyle) -> Self {
        Self {
            text: text.into(),
            style,
            link_target: None,
            inline_image: None,
        }
    }

    pub(crate) fn linked(
        text: impl Into<String>,
        target: impl Into<String>,
        style: SurfaceTextStyle,
    ) -> Self {
        Self {
            text: text.into(),
            style,
            link_target: Some(target.into()),
            inline_image: None,
        }
    }

    pub(crate) fn inline_image(
        text: impl Into<String>,
        image: SurfaceSvgImage,
        style: SurfaceTextStyle,
    ) -> Self {
        Self {
            text: text.into(),
            style,
            link_target: None,
            inline_image: Some(SurfaceInlineImage::new(image)),
        }
    }

    pub(crate) fn layout_text(&self, size: f32) -> String {
        let Some(image) = &self.inline_image else {
            return self.text.clone();
        };
        let space_width = (size * INLINE_IMAGE_LAYOUT_SPACE_FACTOR).max(1.0);
        let count = (image.width() as f32 / space_width).ceil() as usize;
        "\u{00a0}".repeat(count.max(1))
    }

    pub(crate) fn estimated_width(&self, font_size: f32) -> u32 {
        if let Some(image) = &self.inline_image {
            return image.width();
        }
        self.text
            .chars()
            .map(|character| character_width(character, font_size))
            .sum::<f32>()
            .ceil() as u32
    }

    pub(crate) fn is_plain(&self) -> bool {
        !self.style.bold
            && !self.style.italic
            && !self.style.monospace
            && !self.style.underline
            && !self.style.strikethrough
            && !self.style.highlight
            && !self.style.inline_code
            && !self.style.emoji
            && self.style.color.is_none()
            && self.link_target.is_none()
            && self.inline_image.is_none()
    }
}

fn character_width(character: char, font_size: f32) -> f32 {
    font_size * character_width_factor(character)
}

fn character_width_factor(character: char) -> f32 {
    if character.is_ascii_whitespace() {
        return SPACE_TEXT_WIDTH_ESTIMATE_FACTOR;
    }
    if is_east_asian_wide(character) {
        return WIDE_TEXT_WIDTH_ESTIMATE_FACTOR;
    }
    ASCII_TEXT_WIDTH_ESTIMATE_FACTOR
}

fn is_east_asian_wide(character: char) -> bool {
    matches!(
        character as u32,
        0x1100..=0x115F
            | 0x2329..=0x232A
            | 0x2E80..=0xA4CF
            | 0xAC00..=0xD7A3
            | 0xF900..=0xFAFF
            | 0xFE10..=0xFE19
            | 0xFE30..=0xFE6F
            | 0xFF00..=0xFF60
            | 0xFFE0..=0xFFE6
    )
}

impl SurfaceTextStyle {
    pub(crate) fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub(crate) fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub(crate) fn monospace(mut self) -> Self {
        self.monospace = true;
        self
    }

    pub(crate) fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub(crate) fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    pub(crate) fn highlight(mut self) -> Self {
        self.highlight = true;
        self
    }

    pub(crate) fn inline_code(mut self) -> Self {
        self.monospace = true;
        self.inline_code = true;
        self
    }

    pub(crate) fn emoji(mut self) -> Self {
        self.emoji = true;
        self
    }

    pub(crate) fn link(self) -> Self {
        self.underline().with_color(Rgba([
            LINK_COLOR_RED,
            LINK_COLOR_GREEN,
            LINK_COLOR_BLUE,
            LINK_COLOR_ALPHA,
        ]))
    }

    pub(crate) fn with_color(mut self, color: Rgba<u8>) -> Self {
        self.color = Some(color);
        self
    }
}
