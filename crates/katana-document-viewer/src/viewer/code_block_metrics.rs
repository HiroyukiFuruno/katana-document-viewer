use super::settings_update::ViewerTypographyConfig;

const BASE_CODE_FONT_SIZE: f32 = 22.0;
const BASE_CODE_LINE_HEIGHT: f32 = 34.0;
const BASE_CODE_MIN_BOX_HEIGHT: f32 = 56.0;
const CODE_VERTICAL_PADDING: f32 = 6.0;
const CODE_BLOCK_MARGIN: f32 = 14.0;

pub struct ViewerCodeBlockMetrics;

impl ViewerCodeBlockMetrics {
    pub const BASE_LINE_HEIGHT_PX: u32 = BASE_CODE_LINE_HEIGHT as u32;
    pub const BLOCK_MARGIN_PX: u32 = CODE_BLOCK_MARGIN as u32;
    pub const VERTICAL_PADDING_PX: u32 = CODE_VERTICAL_PADDING as u32;

    #[must_use]
    pub fn line_count(text: &str) -> usize {
        text.lines().count().max(1)
    }

    #[must_use]
    pub fn code_scale(typography: ViewerTypographyConfig) -> f32 {
        let code_font_size = typography.preview_font_size.saturating_sub(2).max(10);
        Self::code_scale_from_font_size(f32::from(code_font_size))
    }

    #[must_use]
    pub fn code_scale_from_font_size(code_font_size: f32) -> f32 {
        if !code_font_size.is_finite() || code_font_size <= 0.0 {
            return 1.0;
        }
        code_font_size / BASE_CODE_FONT_SIZE
    }

    #[must_use]
    pub fn line_height_px(typography: ViewerTypographyConfig) -> u32 {
        Self::line_height_from_scale_px(Self::code_scale(typography))
    }

    #[must_use]
    pub fn line_height_from_scale_px(scale: f32) -> u32 {
        Self::scale_to_u32(BASE_CODE_LINE_HEIGHT, scale)
    }

    #[must_use]
    pub fn block_height(text: &str, typography: ViewerTypographyConfig) -> f32 {
        Self::block_height_from_line_count(Self::line_count(text), typography)
    }

    #[must_use]
    pub fn block_height_px(text: &str, typography: ViewerTypographyConfig) -> u16 {
        Self::block_height_from_line_count_px(Self::line_count(text), typography)
    }

    #[must_use]
    pub fn block_height_from_line_count(
        line_count: usize,
        typography: ViewerTypographyConfig,
    ) -> f32 {
        Self::block_height_from_line_count_with_scale(line_count, Self::code_scale(typography))
    }

    #[must_use]
    pub fn block_height_from_line_count_px(
        line_count: usize,
        typography: ViewerTypographyConfig,
    ) -> u16 {
        Self::f32_to_u16(Self::block_height_from_line_count(line_count, typography))
    }

    #[must_use]
    pub fn box_height_from_line_count_with_scale_px(line_count: usize, scale: f32) -> u32 {
        Self::f32_to_u32(Self::box_height_from_line_count_with_scale(
            line_count, scale,
        ))
    }

    #[must_use]
    pub fn block_height_from_line_count_with_scale_px(line_count: usize, scale: f32) -> u32 {
        Self::f32_to_u32(Self::block_height_from_line_count_with_scale(
            line_count, scale,
        ))
    }

    fn block_height_from_line_count_with_scale(line_count: usize, scale: f32) -> f32 {
        Self::box_height_from_line_count_with_scale(line_count, scale) + CODE_BLOCK_MARGIN * 2.0
    }

    fn box_height_from_line_count_with_scale(line_count: usize, scale: f32) -> f32 {
        let normalized_count = line_count.max(1) as f32;
        let content_height =
            normalized_count * BASE_CODE_LINE_HEIGHT * Self::normalized_scale(scale)
                + CODE_VERTICAL_PADDING * 2.0;
        content_height.max(BASE_CODE_MIN_BOX_HEIGHT * Self::normalized_scale(scale))
    }

    fn normalized_scale(scale: f32) -> f32 {
        if scale.is_finite() && scale > 0.0 {
            return scale;
        }
        1.0
    }

    fn scale_to_u32(value: f32, scale: f32) -> u32 {
        Self::f32_to_u32(value * Self::normalized_scale(scale))
    }

    fn f32_to_u32(value: f32) -> u32 {
        value.round().clamp(1.0, u32::MAX as f32) as u32
    }

    fn f32_to_u16(value: f32) -> u16 {
        value.round().clamp(1.0, f32::from(u16::MAX)) as u16
    }
}

#[cfg(test)]
#[path = "code_block_metrics_tests.rs"]
mod tests;
