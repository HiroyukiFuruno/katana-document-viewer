use katana_document_viewer::{
    DiagramViewportState, ViewerDiagramKind, ViewerImageSurface, ViewerNode, ViewerNodeKind,
    ViewerRect, ViewerVector,
};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};
use katana_ui_core::render_model::{UiImageSurfaceTransform, UiNode};
use std::collections::BTreeMap;

pub(super) const STORYBOOK_MEDIA_MAX_WIDTH: u32 = 120;
pub(super) const KATANA_VIEWER_ROW_MAX_WIDTH: u32 = 1168;
pub(super) const EXPORT_MEDIA_MAX_WIDTH: u32 = KATANA_VIEWER_ROW_MAX_WIDTH;
pub(super) const DIAGRAM_MEDIA_MAX_WIDTH: u32 = 860;
pub(super) const MATH_MEDIA_MAX_WIDTH: u32 = 760;
pub(super) const IMAGE_SURFACE_WIDTH: u32 = 2;
pub(super) const IMAGE_SURFACE_HEIGHT: u32 = 1;
pub(super) const IMAGE_SURFACE_CONTENT_SCALE: u32 = 100;
pub(super) const SCALED_IMAGE_SURFACE_WIDTH: u32 = 4;
pub(super) const SCALED_IMAGE_SURFACE_HEIGHT: u32 = 2;
pub(super) const SCALED_IMAGE_SURFACE_CONTENT_SCALE: u32 = 200;
pub(super) const OPAQUE_ALPHA: u8 = 255;
pub(super) const SEMI_TRANSPARENT_ALPHA: u8 = 128;
pub(super) const VIEWPORT_ZOOM: f32 = 1.75;
pub(super) const VIEWPORT_PAN_X: f32 = 3.2;
pub(super) const VIEWPORT_PAN_Y: f32 = -4.6;
pub(super) const EXPECTED_ZOOM_PERCENT: u32 = 175;
pub(super) const EXPECTED_PAN_X: i32 = 3;
pub(super) const EXPECTED_PAN_Y: i32 = -5;
pub(super) const VIEWER_NODE_WIDTH: f32 = 120.0;
pub(super) const VIEWER_NODE_HEIGHT: f32 = 32.0;
pub(super) const KATANA_MIN_CONTROL_CONTAINER_HEIGHT_PX: u16 = 145;

pub(super) fn viewport_states() -> BTreeMap<String, DiagramViewportState> {
    let mut viewports = BTreeMap::new();
    viewports.insert(
        "diagram".to_string(),
        DiagramViewportState {
            zoom: VIEWPORT_ZOOM,
            pan: ViewerVector {
                x: VIEWPORT_PAN_X,
                y: VIEWPORT_PAN_Y,
            },
            fullscreen_open: false,
            help_requested: false,
        },
    );
    viewports.insert(
        "image".to_string(),
        DiagramViewportState {
            zoom: VIEWPORT_ZOOM,
            pan: ViewerVector {
                x: VIEWPORT_PAN_X,
                y: VIEWPORT_PAN_Y,
            },
            fullscreen_open: false,
            help_requested: false,
        },
    );
    viewports
}

pub(super) fn diagram_node() -> ViewerNode {
    viewer_node(
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::Mermaid,
        },
        "diagram",
        "diagram",
    )
}

pub(super) fn image_surface() -> ViewerImageSurface {
    ViewerImageSurface {
        fingerprint: "fingerprint".to_string(),
        width: IMAGE_SURFACE_WIDTH,
        height: IMAGE_SURFACE_HEIGHT,
        display_width: (IMAGE_SURFACE_WIDTH * 100 / IMAGE_SURFACE_CONTENT_SCALE) as f32,
        display_height: (IMAGE_SURFACE_HEIGHT * 100 / IMAGE_SURFACE_CONTENT_SCALE) as f32,
        content_scale: IMAGE_SURFACE_CONTENT_SCALE,
        rgba: semi_transparent_rgba_surface(),
    }
}

pub(super) fn expected_transform() -> UiImageSurfaceTransform {
    UiImageSurfaceTransform::new(EXPECTED_ZOOM_PERCENT, EXPECTED_PAN_X, EXPECTED_PAN_Y)
}

pub(super) fn opaque_rgba_surface() -> Vec<u8> {
    vec![
        0,
        0,
        0,
        OPAQUE_ALPHA,
        OPAQUE_ALPHA,
        OPAQUE_ALPHA,
        OPAQUE_ALPHA,
        OPAQUE_ALPHA,
    ]
}

pub(super) fn semi_transparent_rgba_surface() -> Vec<u8> {
    vec![
        0,
        0,
        0,
        OPAQUE_ALPHA,
        OPAQUE_ALPHA,
        OPAQUE_ALPHA,
        OPAQUE_ALPHA,
        SEMI_TRANSPARENT_ALPHA,
    ]
}

pub(super) fn has_style_class(node: &UiNode, expected: &str) -> bool {
    node.props()
        .style_classes
        .iter()
        .any(|style_class| style_class == expected)
}

pub(super) fn viewer_node(kind: ViewerNodeKind, text: &str, node_id: &str) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId(node_id.to_string()),
        kind,
        source: source(text),
        text: text.to_string(),
        spans: Vec::new(),
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: VIEWER_NODE_WIDTH,
            height: VIEWER_NODE_HEIGHT,
        },
        artifact_id: None,
    }
}

fn source(raw: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: raw.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: raw.len() + 1,
            },
        },
        raw: RawSnippet {
            text: raw.to_string(),
        },
    }
}
