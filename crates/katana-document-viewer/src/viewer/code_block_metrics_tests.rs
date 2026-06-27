use super::ViewerCodeBlockMetrics;
use crate::ViewerTypographyConfig;

#[test]
fn default_katana_code_metrics_keep_three_line_block_height() {
    let typography = ViewerTypographyConfig {
        preview_font_size: 24,
    };

    assert_eq!(34, ViewerCodeBlockMetrics::line_height_px(typography));
    assert_eq!(
        142,
        ViewerCodeBlockMetrics::block_height_from_line_count_px(3, typography)
    );
}

#[test]
fn compact_code_metrics_scale_from_preview_font_size() {
    let typography = ViewerTypographyConfig {
        preview_font_size: 14,
    };

    assert_eq!(19, ViewerCodeBlockMetrics::line_height_px(typography));
    assert_eq!(
        96,
        ViewerCodeBlockMetrics::block_height_from_line_count_px(3, typography)
    );
}
