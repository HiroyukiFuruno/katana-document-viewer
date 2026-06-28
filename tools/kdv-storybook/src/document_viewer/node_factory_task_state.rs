use katana_document_viewer::ViewerTaskState;
use katana_ui_core::render_model::{
    UiContextMenuItem, UiContextMenuItemKind, UiContextMenuProps, UiTaskMarker,
};

#[derive(Clone, Copy)]
pub(super) struct KdvTaskState {
    marker: UiTaskMarker,
}

impl KdvTaskState {
    pub(super) fn from_viewer(value: ViewerTaskState) -> Self {
        Self::new(match value {
            ViewerTaskState::Empty => UiTaskMarker::Empty,
            ViewerTaskState::Done => UiTaskMarker::Done,
            ViewerTaskState::Progress => UiTaskMarker::Progress,
            ViewerTaskState::Blocked => UiTaskMarker::Blocked,
        })
    }

    pub(super) fn from_marker(value: &str) -> Option<Self> {
        UiTaskMarker::from_marker(value).map(Self::new)
    }

    const fn new(marker: UiTaskMarker) -> Self {
        Self { marker }
    }

    pub(super) fn marker(self) -> &'static str {
        self.marker.marker()
    }

    pub(super) fn style_class(self) -> &'static str {
        match self.marker {
            UiTaskMarker::Empty => "kdv-task-empty",
            UiTaskMarker::Done => "kdv-task-done",
            UiTaskMarker::Progress => "kdv-task-progress",
            UiTaskMarker::Blocked => "kdv-task-blocked",
        }
    }

    pub(super) fn accessibility_label(self) -> &'static str {
        match self.marker {
            UiTaskMarker::Empty => "未実施",
            UiTaskMarker::Done => "完了",
            UiTaskMarker::Progress => "実施中",
            UiTaskMarker::Blocked => "保留",
        }
    }

    pub(super) fn is_active(self) -> bool {
        !matches!(self.marker, UiTaskMarker::Empty)
    }
}

pub(super) fn task_context_menu(
    state: KdvTaskState,
    node_id: &str,
    row_index: usize,
) -> UiContextMenuProps {
    UiContextMenuProps {
        items: UiTaskMarker::ALL
            .into_iter()
            .map(KdvTaskState::new)
            .map(|it| {
                UiContextMenuItem::new(
                    it.marker.context_menu_item_id(),
                    it.accessibility_label(),
                    UiContextMenuItemKind::Radio,
                )
                .checked(it.marker() == state.marker())
                .radio_group("ui-task-state")
                .task_control_state_action(
                    it.accessibility_label(),
                    node_id,
                    row_index,
                    it.marker(),
                )
            })
            .collect(),
        ..UiContextMenuProps::default()
    }
}
