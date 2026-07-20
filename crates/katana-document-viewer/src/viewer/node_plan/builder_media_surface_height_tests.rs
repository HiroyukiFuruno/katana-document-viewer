use super::tests_support;
use super::{
    super::super::metrics::ViewerNodeMetrics, super::super::types::ViewerNodeKind,
    ViewerHeightMode, ViewerMediaHeight,
};
use crate::viewer::settings_update::ViewerTypographyConfig;

#[test]
fn preferred_surface_height_returns_none_when_graph_is_none() {
    let node = tests_support::paragraph_kind_node();
    let planned = tests_support::planned_node(ViewerNodeKind::Paragraph, "paragraph", &node);

    assert!(
        ViewerMediaHeight::preferred_surface_height(
            None,
            &node,
            &planned,
            ViewerTypographyConfig {
                preview_font_size: 24,
            },
        )
        .is_none()
    );
}

#[test]
fn preferred_surface_height_uses_graph_height_when_enabled() {
    let graph = tests_support::fake_graph();
    let node = tests_support::table_node();
    let planned = tests_support::planned_node(ViewerNodeKind::Paragraph, "A|B", &node);
    assert!(
        ViewerMediaHeight::preferred_surface_height(
            Some(&graph),
            &node,
            &planned,
            ViewerTypographyConfig {
                preview_font_size: 24,
            },
        )
        .is_some()
    );
}

#[test]
fn fallback_node_height_uses_surface_height_for_export_surface_mode() {
    let graph = tests_support::fake_graph();
    let node = tests_support::paragraph_kind_node();
    let planned = tests_support::planned_node(ViewerNodeKind::Paragraph, "", &node);
    let height = ViewerMediaHeight::fallback_node_height(
        Some(&graph),
        &node,
        &planned,
        ViewerTypographyConfig {
            preview_font_size: 24,
        },
        100,
        ViewerHeightMode::ExportSurface,
        &ViewerNodeKind::Paragraph,
    );
    assert!(height > 0.0);
}

#[test]
fn fallback_node_height_uses_node_metrics_without_export_surface_mode() {
    let graph = tests_support::fake_graph();
    let node = tests_support::paragraph_kind_node();
    let planned = tests_support::planned_node(ViewerNodeKind::Paragraph, "text", &node);
    let typography = ViewerTypographyConfig {
        preview_font_size: 16,
    };
    let actual = ViewerMediaHeight::fallback_node_height(
        Some(&graph),
        &node,
        &planned,
        typography,
        120,
        ViewerHeightMode::InteractivePreview,
        &ViewerNodeKind::Paragraph,
    );
    let expected =
        ViewerNodeMetrics::block_height_with_width(&planned.kind, &planned.text, typography, 120);
    assert_eq!(expected, actual);
}

#[test]
fn preferred_surface_height_is_skipped_for_non_surface_node_kinds() {
    let graph = tests_support::fake_graph();
    let node = tests_support::paragraph_with_image();
    let planned = tests_support::planned_node(ViewerNodeKind::Image, "", &node);
    assert!(
        ViewerMediaHeight::preferred_surface_height(
            Some(&graph),
            &node,
            &planned,
            ViewerTypographyConfig {
                preview_font_size: 16,
            },
        )
        .is_none()
    );
}

#[test]
fn fallback_node_height_uses_node_metrics_if_surface_height_unavailable() {
    let node = tests_support::paragraph_kind_node();
    let planned = tests_support::planned_node(
        ViewerNodeKind::Alert {
            label: "NOTE".to_string(),
        },
        "text",
        &node,
    );
    let typography = ViewerTypographyConfig {
        preview_font_size: 16,
    };
    let actual = ViewerMediaHeight::fallback_node_height(
        None,
        &node,
        &planned,
        typography,
        120,
        ViewerHeightMode::ExportSurface,
        &planned.kind,
    );
    let expected =
        ViewerNodeMetrics::block_height_with_width(&planned.kind, &planned.text, typography, 120);
    assert_eq!(expected, actual);
}
