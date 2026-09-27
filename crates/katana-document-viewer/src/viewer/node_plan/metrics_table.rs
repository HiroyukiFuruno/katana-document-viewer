use super::ViewerNodeMetrics;
use crate::{ViewerTableProjection, ViewerTypographyConfig};

impl ViewerNodeMetrics {
    pub(crate) fn table_block_height(
        projection: &ViewerTableProjection,
        typography: ViewerTypographyConfig,
        content_width: usize,
    ) -> f32 {
        projection
            .row_heights(content_width.min(u32::MAX as usize) as u32, typography)
            .into_iter()
            .sum::<u32>() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::ViewerNodeMetrics;
    use crate::{
        ViewerTableAlignment, ViewerTableCellProjection, ViewerTableProjection,
        ViewerTableRowProjection, ViewerTableVerticalAlignment,
    };

    #[test]
    fn table_block_height_uses_the_public_projection_geometry() {
        let projection = projection();

        assert_eq!(
            268.0,
            ViewerNodeMetrics::table_block_height(
                &projection,
                ViewerNodeMetrics::default_typography(),
                500
            )
        );
    }

    #[test]
    fn empty_table_has_zero_height() {
        assert_eq!(
            0.0,
            ViewerNodeMetrics::table_block_height(
                &ViewerTableProjection {
                    rows: Vec::new(),
                    column_count: 0,
                },
                ViewerNodeMetrics::default_typography(),
                100,
            )
        );
    }

    fn projection() -> ViewerTableProjection {
        let cell = |text: &str| ViewerTableCellProjection {
            text: text.to_string(),
            alignment: ViewerTableAlignment::Unspecified,
            vertical_alignment: ViewerTableVerticalAlignment::Center,
            row_span: 1,
            column_span: 1,
        };
        ViewerTableProjection {
            rows: vec![
                ViewerTableRowProjection {
                    cells: vec![cell("Short"), cell("Long Column Test"), cell("Short")],
                },
                ViewerTableRowProjection {
                    cells: vec![
                        cell("ID"),
                        cell(
                            "This text is a very long line to verify horizontal scrolling and word wrapping are working correctly.",
                        ),
                        cell("Notes"),
                    ],
                },
            ],
            column_count: 3,
        }
    }
}
