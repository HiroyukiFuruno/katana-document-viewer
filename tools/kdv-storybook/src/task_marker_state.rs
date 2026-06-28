use katana_document_viewer::ViewerTaskState;
use katana_ui_core::render_model::UiTaskMarker;

pub(crate) const fn viewer_task_state(marker: UiTaskMarker) -> ViewerTaskState {
    match marker {
        UiTaskMarker::Empty => ViewerTaskState::Empty,
        UiTaskMarker::Done => ViewerTaskState::Done,
        UiTaskMarker::Progress => ViewerTaskState::Progress,
        UiTaskMarker::Blocked => ViewerTaskState::Blocked,
    }
}
