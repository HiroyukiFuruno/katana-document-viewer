use super::KucNodeFactory;

const BASE_BODY_FONT_SIZE: f32 = 24.0;
const COMPACT_BODY_FONT_SIZE: f32 = 14.0;
const BASE_BODY_LINE_HEIGHT: usize = 34;
const COMPACT_BODY_LINE_HEIGHT: usize = 23;
const QUOTED_CODE_VERTICAL_EXTRA_PX: usize = 20;

impl KucNodeFactory<'_> {
    pub(super) fn body_line_height_px(&self) -> u16 {
        let font_size = f32::from(self.typography.preview_font_size);
        if font_size <= COMPACT_BODY_FONT_SIZE {
            return COMPACT_BODY_LINE_HEIGHT as u16;
        }
        if font_size >= BASE_BODY_FONT_SIZE {
            return BASE_BODY_LINE_HEIGHT as u16;
        }
        let progress =
            (font_size - COMPACT_BODY_FONT_SIZE) / (BASE_BODY_FONT_SIZE - COMPACT_BODY_FONT_SIZE);
        let height = COMPACT_BODY_LINE_HEIGHT as f32
            + (BASE_BODY_LINE_HEIGHT - COMPACT_BODY_LINE_HEIGHT) as f32 * progress;
        height.round().max(1.0) as u16
    }

    pub(super) fn quoted_code_block_height_from_line_count_px(&self, line_count: usize) -> u16 {
        let height = self
            .body_line_height_px()
            .saturating_mul(u16::try_from(line_count.max(1)).unwrap_or(u16::MAX))
            .saturating_add(QUOTED_CODE_VERTICAL_EXTRA_PX as u16);
        height.max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::KucNodeFactory;
    use katana_document_viewer::{Artifact, ViewerTypographyConfig};

    #[test]
    fn compact_body_line_height_matches_canvas_text_metrics() {
        let artifacts = Vec::<Artifact>::new();
        let factory = KucNodeFactory::new(&artifacts, 640).typography(ViewerTypographyConfig {
            preview_font_size: 14,
        });

        assert_eq!(23, factory.body_line_height_px());
    }

    #[test]
    fn quoted_code_block_height_uses_compact_document_metrics() {
        let artifacts = Vec::<Artifact>::new();
        let factory = KucNodeFactory::new(&artifacts, 640).typography(ViewerTypographyConfig {
            preview_font_size: 14,
        });

        assert_eq!(43, factory.quoted_code_block_height_from_line_count_px(1));
    }
}
