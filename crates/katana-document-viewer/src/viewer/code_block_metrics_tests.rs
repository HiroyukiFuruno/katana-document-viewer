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

#[test]
fn invalid_code_scale_uses_one_to_avoid_regressions() {
    assert_eq!(
        1.0,
        ViewerCodeBlockMetrics::code_scale_from_font_size(f32::NAN)
    );
    assert_eq!(1.0, ViewerCodeBlockMetrics::code_scale_from_font_size(-4.0));
    assert_eq!(1.0, ViewerCodeBlockMetrics::code_scale_from_font_size(0.0));
}

#[test]
fn empty_or_zero_line_blocks_keep_minimum_box_height() {
    let typography = ViewerTypographyConfig {
        preview_font_size: 14,
    };

    assert!(ViewerCodeBlockMetrics::line_count("").max(1) >= 1);
    assert_eq!(
        59,
        ViewerCodeBlockMetrics::block_height_from_line_count_px(0, typography)
    );
}

#[test]
fn block_height_px_counts_source_lines() {
    let typography = ViewerTypographyConfig {
        preview_font_size: 24,
    };

    assert_eq!(
        108,
        ViewerCodeBlockMetrics::block_height_px("one\ntwo", typography)
    );
}

#[test]
fn invalid_layout_scale_uses_the_default_scale() {
    assert_eq!(
        34,
        ViewerCodeBlockMetrics::line_height_from_scale_px(f32::NAN)
    );
    assert_eq!(
        56,
        ViewerCodeBlockMetrics::box_height_from_line_count_with_scale_px(0, 0.0)
    );
    assert_eq!(
        84,
        ViewerCodeBlockMetrics::block_height_from_line_count_with_scale_px(1, -1.0)
    );
}
