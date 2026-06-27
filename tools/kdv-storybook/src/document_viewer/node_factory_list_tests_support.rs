use katana_document_viewer::{ViewerNode, ViewerNodeKind, ViewerRect};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};
use katana_ui_core::render_model::{UiDimension, UiNode};
use katana_ui_core::theme::{Rgba, ThemeSnapshot};
use katana_ui_core_storybook::Canvas;

const NODE_WIDTH: f32 = 120.0;
const NODE_HEIGHT: f32 = 64.0;
const RGB_RED_SHIFT_BITS: u32 = 16;
const RGB_GREEN_SHIFT_BITS: u32 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VerticalBounds {
    top: usize,
    bottom: usize,
}

impl VerticalBounds {
    pub(super) const fn center_twice(self) -> usize {
        self.top + self.bottom
    }
}

pub(super) fn vertical_bounds_for_color_in_x_range(
    canvas: &Canvas,
    color: u32,
    start_x: usize,
    end_x: usize,
) -> Option<VerticalBounds> {
    let mut top = None;
    let mut bottom = None;
    let end_x = end_x.min(canvas.width());
    for y in 0..canvas.height() {
        let found = (start_x..end_x).any(|x| pixel_at(canvas, x, y) == Some(color));
        if found {
            top.get_or_insert(y);
            bottom = Some(y);
        }
    }
    Some(VerticalBounds {
        top: top?,
        bottom: bottom?,
    })
}

pub(super) fn theme_rgb(theme: &ThemeSnapshot, token: &str) -> u32 {
    theme.color(token).map_or(0, rgb)
}

pub(super) fn assert_margin_left_px(node: &UiNode, expected: u16) {
    assert_eq!(
        UiDimension::Px(expected),
        node.props().common.margin.left,
        "{:#?}",
        node.props().common
    );
}

pub(super) fn assert_no_kdv_list_class(node: &UiNode) {
    assert!(
        !node
            .props()
            .style_classes
            .iter()
            .any(|style_class| style_class.starts_with("kdv-list")),
        "{:#?}",
        node.props().style_classes
    );
    for child in node.children() {
        assert_no_kdv_list_class(child);
    }
}

pub(super) fn viewer_node(text: &str) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId("list".to_string()),
        kind: ViewerNodeKind::List,
        source: source(text),
        text: text.to_string(),
        spans: Vec::new(),
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: NODE_WIDTH,
            height: NODE_HEIGHT,
        },
        artifact_id: None,
    }
}

fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> Option<u32> {
    canvas
        .pixels()
        .get(y.saturating_mul(canvas.width()) + x)
        .copied()
}

fn rgb(rgba: Rgba) -> u32 {
    ((rgba[0] as u32) << RGB_RED_SHIFT_BITS)
        | ((rgba[1] as u32) << RGB_GREEN_SHIFT_BITS)
        | rgba[2] as u32
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
