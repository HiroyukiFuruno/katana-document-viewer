use katana_document_viewer::{ViewerTarget, ViewerTaskState};

pub(crate) struct StorybookTaskStateChangeInput<'a> {
    pub(crate) document_id: &'a str,
    pub(crate) target: &'a ViewerTarget,
    pub(crate) previous_state: Option<ViewerTaskState>,
    pub(crate) next_state: ViewerTaskState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StorybookTaskStateChange {
    document_id: String,
    artifact_id: String,
    node_id: String,
    byte_start: usize,
    byte_end: usize,
    line_start: usize,
    column_start: usize,
    line_end: usize,
    column_end: usize,
    previous_marker: String,
    next_marker: String,
    source_preview: String,
}

impl StorybookTaskStateChange {
    pub(crate) fn new(change: StorybookTaskStateChangeInput<'_>) -> Self {
        let source = &change.target.source;
        Self {
            document_id: change.document_id.to_string(),
            artifact_id: change.target.artifact_id.0.clone(),
            node_id: change.target.node_id.0.clone(),
            byte_start: source.byte_range.start,
            byte_end: source.byte_range.end,
            line_start: source.line_column_range.start.line,
            column_start: source.line_column_range.start.column,
            line_end: source.line_column_range.end.line,
            column_end: source.line_column_range.end.column,
            previous_marker: marker_label(change.previous_state),
            next_marker: change.next_state.marker().to_string(),
            source_preview: compact_source_preview(&source.raw.text),
        }
    }

    pub(crate) fn label(&self) -> String {
        format!(
            "{} {}:{} {} -> {}",
            self.artifact_id,
            self.line_start,
            self.column_start,
            self.previous_marker,
            self.next_marker
        )
    }

    pub(crate) fn location_label(&self) -> String {
        format!(
            "{}:{}:{}-{}:{} bytes {}..{} artifact={} node={}",
            self.document_id,
            self.line_start,
            self.column_start,
            self.line_end,
            self.column_end,
            self.byte_start,
            self.byte_end,
            self.artifact_id,
            self.node_id
        )
    }

    pub(crate) fn target_label(&self) -> String {
        format!(
            "document={} artifact={} node={}",
            self.document_id, self.artifact_id, self.node_id
        )
    }

    pub(crate) fn span_label(&self) -> String {
        format!(
            "line {}:{}-{}:{} bytes {}..{}",
            self.line_start,
            self.column_start,
            self.line_end,
            self.column_end,
            self.byte_start,
            self.byte_end
        )
    }

    pub(crate) fn source_label(&self) -> String {
        format!(
            "{} {} {}",
            self.document_id, self.node_id, self.source_preview
        )
    }
}

fn marker_label(value: Option<ViewerTaskState>) -> String {
    value.map_or_else(|| "unknown".to_string(), |state| state.marker().to_string())
}

fn compact_source_preview(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.chars().count() <= 48 {
        return trimmed.to_string();
    }
    format!("{}...", trimmed.chars().take(48).collect::<String>())
}
