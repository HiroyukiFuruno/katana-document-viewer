use crate::catalog::StorybookFixture;
use crate::sidebar::{StorybookSidebar, StorybookSidebarRequest, StorybookSidebarScroll};
use crate::sidebar_settings_state::StorybookSettingsState;
use crate::sidebar_settings_task_change::StorybookTaskStateChangeInput;
use katana_document_viewer::{
    ArtifactId, ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
    ViewerInteractionConfig, ViewerRect, ViewerTarget, ViewerTaskState,
};
use katana_ui_core::render_model::{UiNode, UiNodeKind};
use std::path::PathBuf;

#[test]
fn sidebar_state_shows_task_session_changes() {
    let fixtures = vec![fixture("direct/sample.md")];
    let mut settings_state = StorybookSettingsState::default();
    let target = target("ui-task-state:list:0", "- [ ] Pending task");
    settings_state.record_task_change(StorybookTaskStateChangeInput {
        document_id: "direct/sample.md",
        target: &target,
        previous_state: Some(ViewerTaskState::Empty),
        next_state: ViewerTaskState::Progress,
    });

    let tree = StorybookSidebar::render(StorybookSidebarRequest {
        fixtures: &fixtures,
        selected_index: 0,
        scene: None,
        dark: true,
        interaction: &ViewerInteractionConfig::default(),
        typography: Default::default(),
        file_tree_state: Default::default(),
        settings_state: &settings_state,
        width: 300,
        height: 900,
        preview_width: 300,
        preview_height: 900,
        scroll: StorybookSidebarScroll::default(),
    });

    assert!(contains_label(tree.root(), "Task changes"));
    assert!(contains_input_value(tree.root(), "1"));
    assert!(contains_label(tree.root(), "Last task"));
    assert!(contains_input_value(
        tree.root(),
        "ui-task-state:list:0 7:3 [ ] -> [/]"
    ));
    assert!(contains_label(tree.root(), "Changed location"));
    assert!(contains_input_value(
        tree.root(),
        "direct/sample.md:7:3-7:21 bytes 0..18 artifact=ui-task-state:list:0 node=list-node"
    ));
    assert!(contains_label(tree.root(), "Changed target"));
    assert!(contains_input_value(
        tree.root(),
        "document=direct/sample.md artifact=ui-task-state:list:0 node=list-node"
    ));
    assert!(contains_label(tree.root(), "Changed span"));
    assert!(contains_input_value(
        tree.root(),
        "line 7:3-7:21 bytes 0..18"
    ));
    assert!(contains_label(tree.root(), "Changed history"));
    assert!(contains_input_value(
        tree.root(),
        "#1 direct/sample.md:7:3-7:21 bytes 0..18 artifact=ui-task-state:list:0 node=list-node"
    ));
    assert!(contains_label(tree.root(), "Task source"));
    assert!(contains_input_value(
        tree.root(),
        "direct/sample.md list-node - [ ] Pending task"
    ));
}

#[test]
fn sidebar_state_keeps_multiple_task_change_locations() {
    let fixtures = vec![fixture("direct/sample.md")];
    let mut settings_state = StorybookSettingsState::default();
    let first_target = target_at(
        "ui-task-state:list:0",
        "- [ ] Pending task",
        7,
        "first-node",
    );
    settings_state.record_task_change(StorybookTaskStateChangeInput {
        document_id: "direct/sample.md",
        target: &first_target,
        previous_state: Some(ViewerTaskState::Empty),
        next_state: ViewerTaskState::Done,
    });
    let second_target = target_at(
        "ui-task-state:list:1",
        "- [-] Blocked task",
        9,
        "second-node",
    );
    settings_state.record_task_change(StorybookTaskStateChangeInput {
        document_id: "direct/sample.md",
        target: &second_target,
        previous_state: Some(ViewerTaskState::Empty),
        next_state: ViewerTaskState::Blocked,
    });

    let tree = StorybookSidebar::render(StorybookSidebarRequest {
        fixtures: &fixtures,
        selected_index: 0,
        scene: None,
        dark: true,
        interaction: &ViewerInteractionConfig::default(),
        typography: Default::default(),
        file_tree_state: Default::default(),
        settings_state: &settings_state,
        width: 300,
        height: 900,
        preview_width: 300,
        preview_height: 900,
        scroll: StorybookSidebarScroll::default(),
    });

    assert!(contains_label(tree.root(), "Changed history"));
    assert!(contains_input_value_parts(
        tree.root(),
        &[
            "#1 direct/sample.md:7:3-7:",
            "artifact=ui-task-state:list:0 node=first-node",
            "#2 direct/sample.md:9:3-9:",
            "artifact=ui-task-state:list:1 node=second-node",
        ],
    ));
}

fn contains_input_value(node: &UiNode, value: &str) -> bool {
    if node.kind() == UiNodeKind::Input && node.props().interaction.value == value {
        return true;
    }
    node.children()
        .iter()
        .any(|child| contains_input_value(child, value))
}

fn contains_input_value_parts(node: &UiNode, parts: &[&str]) -> bool {
    if node.kind() == UiNodeKind::Input
        && parts
            .iter()
            .all(|part| node.props().interaction.value.contains(part))
    {
        return true;
    }
    node.children()
        .iter()
        .any(|child| contains_input_value_parts(child, parts))
}

fn contains_label(node: &UiNode, value: &str) -> bool {
    if node.props().label == value {
        return true;
    }
    node.children()
        .iter()
        .any(|child| contains_label(child, value))
}

fn fixture(label: &str) -> StorybookFixture {
    StorybookFixture {
        label: label.to_string(),
        path: PathBuf::from(label),
    }
}

fn target(state_id: &str, source_text: &str) -> ViewerTarget {
    target_at(state_id, source_text, 7, "list-node")
}

fn target_at(state_id: &str, source_text: &str, line: usize, node_id: &str) -> ViewerTarget {
    ViewerTarget {
        node_id: KmmNodeId(node_id.to_string()),
        source: source_span_at(source_text, line),
        artifact_id: ArtifactId(state_id.to_string()),
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: 24.0,
            height: 24.0,
        },
    }
}

fn source_span_at(text: &str, line: usize) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: text.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line, column: 3 },
            end: LineColumn {
                line,
                column: text.len() + 3,
            },
        },
        raw: RawSnippet {
            text: text.to_string(),
        },
    }
}
