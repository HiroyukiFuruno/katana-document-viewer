use super::ViewerNodeMetrics;
use crate::{ViewerTableProjection, ViewerTypographyConfig};

impl ViewerNodeMetrics {
    pub(super) fn table_block_height(
        text: &str,
        typography: ViewerTypographyConfig,
        content_width: usize,
    ) -> f32 {
        ViewerTableProjection::from_text(text).map_or(0.0, |projection| {
            projection
                .row_heights(content_width.min(u32::MAX as usize) as u32, typography)
                .into_iter()
                .sum::<u32>() as f32
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ViewerNodeMetrics;

    #[test]
    fn table_block_height_uses_the_public_projection_geometry() {
        let text = [
            "Short | Long Column Test | Short",
            "--- | --- | ---",
            "ID | This text is a very long line to verify horizontal scrolling and word wrapping are working correctly. | Notes",
        ]
        .join("\n");

        assert_eq!(
            268.0,
            ViewerNodeMetrics::table_block_height(
                &text,
                ViewerNodeMetrics::default_typography(),
                500
            )
        );
    }

    #[test]
    fn empty_table_has_zero_height() {
        assert_eq!(
            0.0,
            ViewerNodeMetrics::table_block_height("", ViewerNodeMetrics::default_typography(), 100)
        );
    }
}
