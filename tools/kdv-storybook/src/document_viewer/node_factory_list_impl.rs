use super::KucNodeFactory;
use super::task_state::{KdvTaskState, task_context_menu};
use katana_document_viewer::{ViewerNode, ViewerTextSpan};
use katana_ui_core::atom::{Checkbox, Text};
use katana_ui_core::layout::{Column, Row};
use katana_ui_core::render_model::{
    UI_TASK_STATE_ID_PREFIX, UiCommonProps, UiDimension, UiEdgeInsets, UiNode, UiStateId,
    UiTextWrapMode,
};
use list_row::{KdvListMarker, KdvListRow};
use list_spans::ListSpanRows;

const LIST_DEPTH_INDENT_PX: u16 = 40;
const MAX_LIST_DEPTH: usize = 8;

impl<'a> KucNodeFactory<'a> {
    pub(super) fn list_node(&self, node: &ViewerNode) -> UiNode {
        let mut column = Column::new();
        let span_rows = Self::list_rows(node);
        let row_height = self.body_line_height_px();
        for (index, line) in node.text.lines().enumerate() {
            let row_spans = span_rows
                .get(index)
                .cloned()
                .unwrap_or_else(|| vec![ViewerTextSpan::plain(line)]);
            column = column.child(self.list_row_node(
                &node.node_id.0,
                line,
                row_spans,
                index,
                row_height,
            ));
        }
        UiNode::from(column)
    }

    fn list_row_node(
        &self,
        node_id: &str,
        line: &str,
        row_spans: Vec<ViewerTextSpan>,
        index: usize,
        row_height: u16,
    ) -> UiNode {
        let row = KdvListRow::parse(line);
        let body = row.body.to_string();
        let body_spans = Self::body_spans_for_row(row_spans, &body);
        match row.marker {
            KdvListMarker::Task(state) => self.task_row_node(TaskRowNodeInput {
                node_id,
                body: &body,
                body_spans,
                state,
                index,
                depth: row.depth,
                row_height,
            }),
            KdvListMarker::Text(marker) => {
                self.marked_row_node(node_id, marker, &body, body_spans, row.depth)
            }
        }
    }

    fn marked_row_node(
        &self,
        node_id: &str,
        marker: &str,
        body: &str,
        body_spans: Vec<ViewerTextSpan>,
        depth: usize,
    ) -> UiNode {
        let row: UiNode = Row::new()
            .child(Self::list_marker_node(marker, depth))
            .child(self.list_body_text_node(node_id, body, body_spans))
            .into();
        row.common(Self::list_depth_common(depth))
    }

    fn list_marker_node(marker: &str, depth: usize) -> UiNode {
        let bullet = matches!(marker, "-" | "*" | "+");
        let label = if bullet { "  " } else { marker };
        let node: UiNode = Text::new(label).text_role("list-marker").into();
        node.common(Self::list_depth_common(depth))
    }

    fn task_row_node(&self, input: TaskRowNodeInput<'_>) -> UiNode {
        let state = self.task_state(input.node_id, input.index, input.state);
        let checkbox = UiNode::from(
            Checkbox::new("")
                .checked(state.is_active())
                .value(state.marker())
                .accessibility_label(state.accessibility_label())
                .task_control_action(state.accessibility_label(), input.node_id, input.index),
        )
        .stable_node_id(task_checkbox_node_id(input.node_id, input.index))
        .state_id(task_state_id(input.node_id, input.index))
        .style_class("kdv-task-checkbox")
        .style_class(state.style_class())
        .context_menu(task_context_menu(state, input.node_id, input.index))
        .height(UiDimension::Px(input.row_height));
        let text = self.list_body_text_node(input.node_id, input.body, input.body_spans);
        let row: UiNode = Row::new()
            .value(state.marker())
            .child(checkbox)
            .child(text)
            .into();
        row.common(Self::list_depth_common(input.depth))
            .stable_node_id(task_row_node_id(input.node_id, input.index))
            .task_control_action(state.accessibility_label(), input.node_id, input.index)
    }

    fn task_state(&self, node_id: &str, index: usize, source: KdvTaskState) -> KdvTaskState {
        let Some(overrides) = self.task_state_overrides else {
            return source;
        };
        overrides
            .get(task_state_id(node_id, index).as_str())
            .copied()
            .map(KdvTaskState::from_viewer)
            .unwrap_or(source)
    }

    fn list_body_text_node(
        &self,
        node_id: &str,
        body: &str,
        body_spans: Vec<ViewerTextSpan>,
    ) -> UiNode {
        let spans = ListSpanRows::body_text_spans(body, body_spans);
        let has_link = spans.iter().any(|span| !span.link_target.is_empty());
        let text = Text::new(body)
            .text_role("list-item")
            .wrap(UiTextWrapMode::Wrap)
            .text_spans(Self::text_spans(&spans));
        let rendered: UiNode = text.into();
        let common = rendered
            .props()
            .common
            .clone()
            .semantic_node_id(node_id.to_string());
        let rendered = rendered.common(common);
        if has_link {
            return rendered
                .stable_node_id(node_id.to_string())
                .stable_state_id(node_id.to_string());
        }
        rendered
    }

    fn list_rows(node: &ViewerNode) -> Vec<Vec<ViewerTextSpan>> {
        if node.spans.is_empty() {
            return node
                .text
                .lines()
                .map(ViewerTextSpan::plain)
                .map(|span| vec![span])
                .collect();
        }
        ListSpanRows::split_by_line(&node.spans)
    }

    fn body_spans_for_row(row_spans: Vec<ViewerTextSpan>, body: &str) -> Vec<ViewerTextSpan> {
        let span_text = ListSpanRows::text(&row_spans);
        let Some(offset) = span_text.find(body) else {
            return Vec::new();
        };
        ListSpanRows::after_offset(row_spans, offset)
    }

    fn list_depth_common(depth: usize) -> UiCommonProps {
        UiCommonProps::default().margin(UiEdgeInsets {
            left: UiDimension::Px(Self::list_depth_indent(depth)),
            ..UiEdgeInsets::default()
        })
    }

    fn list_depth_indent(depth: usize) -> u16 {
        let capped_depth = depth.min(MAX_LIST_DEPTH);
        let capped_depth = capped_depth as u16;
        capped_depth.saturating_mul(LIST_DEPTH_INDENT_PX)
    }
}

fn task_state_id(node_id: &str, index: usize) -> UiStateId {
    UiStateId::new(format!("{UI_TASK_STATE_ID_PREFIX}{node_id}:{index}"))
}

fn task_checkbox_node_id(node_id: &str, index: usize) -> String {
    format!("ui-task-checkbox:{node_id}:{index}")
}

fn task_row_node_id(node_id: &str, index: usize) -> String {
    format!("ui-task-row:{node_id}:{index}")
}

struct TaskRowNodeInput<'a> {
    node_id: &'a str,
    body: &'a str,
    body_spans: Vec<ViewerTextSpan>,
    state: KdvTaskState,
    index: usize,
    depth: usize,
    row_height: u16,
}

#[path = "node_factory_list_row.rs"]
mod list_row;
#[path = "node_factory_list_spans.rs"]
mod list_spans;
