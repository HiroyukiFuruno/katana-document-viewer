use super::*;
use katana_document_viewer::{
    ArtifactId, ByteRange, LineColumn, LineColumnRange, RawSnippet, SourceSpan, ViewerRect,
    ViewerTarget,
};
use katana_ui_core::render_model::{UiTaskControlAction, UiTaskMarker};

#[test]
fn row_source_keeps_rendered_row_when_row_index_matches() {
    let action = task_action(1, UiTaskMarker::Done);
    let target = target_with_source("- [ ] first\n- [/] second");

    let source = row_source(&action, &target);

    assert_eq!("- [/] second", source.raw.text);
}

#[test]
fn row_source_uses_kuc_marker_when_row_index_is_stale() {
    let action = task_action(99, UiTaskMarker::Empty);
    let target = target_with_source("- [x] first\n- [/] second");

    let source = row_source(&action, &target);

    assert_eq!("[ ]", source.raw.text);
}

#[test]
fn viewer_target_keeps_base_artifact_and_task_target_carries_state_id() {
    let action = task_action(2, UiTaskMarker::Progress);
    let base = target_with_source("- [ ] first\n- [x] second\n- [/] third");
    let target = viewer_target(
        &action,
        &base,
        ViewerRect {
            x: 4.0,
            y: 8.0,
            width: 16.0,
            height: 20.0,
        },
    );
    let task_target = task_control_target(&action);

    assert_eq!(base.artifact_id, target.artifact_id);
    assert_eq!("task-state", task_target.state_id);
    assert_eq!(2, task_target.row_index);
    assert_eq!(KmmNodeId("task-node".to_string()), task_target.node_id);
}

fn task_action(row_index: usize, current_marker: UiTaskMarker) -> UiTaskControlAction {
    UiTaskControlAction {
        node_id: "task-node".to_string(),
        row_index,
        current_marker,
        state_id: "task-state".to_string(),
        menu_items: Vec::new(),
    }
}

fn target_with_source(raw: &str) -> ViewerTarget {
    ViewerTarget {
        node_id: KmmNodeId("task-node".to_string()),
        source: source_span(raw),
        artifact_id: ArtifactId("base-artifact".to_string()),
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: 24.0,
            height: 24.0,
        },
    }
}

fn source_span(text: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: text.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: text.lines().count().max(1),
                column: text.lines().last().map_or(1, |it| it.len() + 1),
            },
        },
        raw: RawSnippet {
            text: text.to_string(),
        },
    }
}
