use crate::catalog::StorybookFixture;
use crate::preview::{PreviewAssetSceneEvent, PreviewBuilder};
use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use katana_document_viewer::{
    DiagramViewportState, ViewerInteractionConfig, ViewerMode, ViewerSearchState, ViewerTaskState,
    ViewerTypographyConfig, ViewerViewport,
};
use std::collections::{BTreeMap, BTreeSet};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::{Duration, Instant};

const DEFAULT_ASSET_JOB_TIMEOUT: Duration = Duration::from_secs(8);
const FIRST_FRAME_ASSET_JOB_DELAY: Duration = Duration::from_millis(16);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StorybookAssetJobKey {
    fixture_label: String,
    dark: bool,
    mode: ViewerMode,
    preview_font_size: u16,
    search_query: String,
    search_current_index: Option<usize>,
    diagram_viewport_key: String,
    image_viewport_key: String,
    task_state_key: String,
    accordion_state_key: String,
    viewport_width: u32,
    viewport_height: u32,
}

pub struct StorybookAssetJob {
    key: StorybookAssetJobKey,
    scope_key: String,
    receiver: Receiver<Result<PreviewAssetSceneEvent, String>>,
    cancel_token: Arc<AtomicBool>,
    started_at: Instant,
    timeout: Duration,
}

pub struct StorybookAssetJobRequest {
    pub key: StorybookAssetJobKey,
    pub scope_key: String,
    pub fixture: StorybookFixture,
    pub viewport: ViewerViewport,
    pub dark: bool,
    pub interaction: ViewerInteractionConfig,
    pub mode: ViewerMode,
    pub typography: ViewerTypographyConfig,
    pub search: ViewerSearchState,
    pub diagram_viewports: BTreeMap<String, DiagramViewportState>,
    pub image_viewports: BTreeMap<String, DiagramViewportState>,
    pub task_state_overrides: BTreeMap<String, ViewerTaskState>,
    pub accordion_open_overrides: BTreeMap<String, bool>,
    pub copied_code_node_ids: BTreeSet<String>,
}

impl StorybookAssetJobKey {
    #[must_use]
    pub fn new(input: StorybookAssetJobKeyInput<'_>) -> Self {
        Self {
            fixture_label: input.fixture_label,
            dark: input.dark,
            mode: input.mode,
            preview_font_size: input.typography.preview_font_size,
            search_query: input.search.query.clone(),
            search_current_index: input.search.current_index,
            diagram_viewport_key: Self::diagram_viewport_key(input.diagram_viewports),
            image_viewport_key: Self::diagram_viewport_key(input.image_viewports),
            task_state_key: Self::task_state_key(input.task_state_overrides),
            accordion_state_key: Self::accordion_state_key(input.accordion_open_overrides),
            viewport_width: input.viewport.width.round().max(0.0) as u32,
            viewport_height: input.viewport.height.round().max(0.0) as u32,
        }
    }

    fn accordion_state_key(accordion_open_overrides: &BTreeMap<String, bool>) -> String {
        accordion_open_overrides
            .iter()
            .map(|(node_id, open)| format!("{node_id}={open}"))
            .collect::<Vec<_>>()
            .join(";")
    }

    fn diagram_viewport_key(viewports: &BTreeMap<String, DiagramViewportState>) -> String {
        viewports
            .iter()
            .map(|(node_id, state)| {
                format!(
                    "{}:zoom={}:pan={},{}:fullscreen={}:help={}",
                    node_id,
                    state.zoom.to_bits(),
                    state.pan.x.to_bits(),
                    state.pan.y.to_bits(),
                    state.fullscreen_open,
                    state.help_requested
                )
            })
            .collect::<Vec<_>>()
            .join(";")
    }

    fn task_state_key(task_state_overrides: &BTreeMap<String, ViewerTaskState>) -> String {
        task_state_overrides
            .iter()
            .map(|(node_id, state)| format!("{node_id}={}", state.marker()))
            .collect::<Vec<_>>()
            .join(";")
    }
}

pub struct StorybookAssetJobKeyInput<'a> {
    pub fixture_label: String,
    pub dark: bool,
    pub mode: ViewerMode,
    pub typography: ViewerTypographyConfig,
    pub search: &'a ViewerSearchState,
    pub diagram_viewports: &'a BTreeMap<String, DiagramViewportState>,
    pub image_viewports: &'a BTreeMap<String, DiagramViewportState>,
    pub task_state_overrides: &'a BTreeMap<String, ViewerTaskState>,
    pub accordion_open_overrides: &'a BTreeMap<String, bool>,
    pub viewport: ViewerViewport,
}

impl StorybookAssetJob {
    #[must_use]
    pub fn spawn(preview: PreviewBuilder, request: StorybookAssetJobRequest) -> Self {
        let (sender, receiver) = channel();
        let key = request.key.clone();
        let scope_key = request.scope_key.clone();
        let cancel_token = Arc::new(AtomicBool::new(false));
        let worker_cancel_token = cancel_token.clone();
        std::thread::spawn(move || {
            let send_failure_cancel_token = worker_cancel_token.clone();
            let result = catch_unwind(AssertUnwindSafe(|| {
                std::thread::sleep(FIRST_FRAME_ASSET_JOB_DELAY);
                if worker_cancel_token.load(Ordering::Relaxed) {
                    return Ok(());
                }
                preview.build_incremental_asset_scenes(
                    PreviewBuildRequest {
                        fixture: &request.fixture,
                        viewport: request.viewport,
                        dark: request.dark,
                        theme: None,
                        interaction: request.interaction,
                        mode: request.mode,
                        typography: request.typography,
                        search: request.search,
                        diagram_viewports: request.diagram_viewports,
                        image_viewports: request.image_viewports,
                        task_state_overrides: request.task_state_overrides,
                        accordion_open_overrides: request.accordion_open_overrides,
                        copied_code_node_ids: request.copied_code_node_ids,
                        asset_mode: PreviewBuildAssetMode::VisibleAndNearViewport,
                        attach_surface: false,
                        export_surface: false,
                    },
                    worker_cancel_token,
                    |event| sender.send(Ok(event)).map_err(|error| error.to_string()),
                )
            }));
            let send_result = match result {
                Ok(Ok(())) => Ok(()),
                Ok(Err(error)) => {
                    Self::send_build_error(&sender, &send_failure_cancel_token, error.to_string())
                }
                Err(error) => sender
                    .send(Err(Self::panic_message(error)))
                    .map_err(|error| error.to_string()),
            };
            if let Err(error) = send_result
                && let Some(message) =
                    Self::send_failure_log_message(&send_failure_cancel_token, &error)
            {
                eprintln!("{message}");
            }
        });
        Self {
            key,
            scope_key,
            receiver,
            cancel_token,
            started_at: Instant::now(),
            timeout: DEFAULT_ASSET_JOB_TIMEOUT,
        }
    }

    #[must_use]
    pub fn key(&self) -> &StorybookAssetJobKey {
        &self.key
    }

    #[must_use]
    pub fn scope_key(&self) -> &str {
        &self.scope_key
    }

    #[must_use]
    pub fn try_recv(&self) -> Option<Result<PreviewAssetSceneEvent, String>> {
        if let Ok(result) = self.receiver.try_recv() {
            return Some(result);
        }
        if self.started_at.elapsed() <= self.timeout {
            return None;
        }
        self.cancel();
        Some(Err(format!(
            "asset job timed out after {}ms: scope={}",
            self.timeout.as_millis(),
            self.scope_key
        )))
    }

    pub fn cancel(&self) {
        self.cancel_token.store(true, Ordering::Relaxed);
    }

    #[must_use]
    pub fn covers_scope(&self, scope_key: &str) -> bool {
        self.scope_key == scope_key || scope_is_subset(scope_key, &self.scope_key)
    }

    fn panic_message(error: Box<dyn std::any::Any + Send>) -> String {
        if let Some(message) = error.downcast_ref::<&str>() {
            return format!("asset job panicked: {message}");
        }
        if let Some(message) = error.downcast_ref::<String>() {
            return format!("asset job panicked: {message}");
        }
        "asset job panicked".to_string()
    }

    #[must_use]
    pub(crate) fn should_log_send_failure(cancel_token: &AtomicBool) -> bool {
        !cancel_token.load(Ordering::Relaxed)
    }

    #[must_use]
    pub(crate) fn send_failure_log_message(
        cancel_token: &AtomicBool,
        error: &str,
    ) -> Option<String> {
        if !Self::should_log_send_failure(cancel_token) {
            return None;
        }
        Some(format!(
            "[kdv-storybook] asset job result send failed: {error}"
        ))
    }

    fn send_build_error(
        sender: &Sender<Result<PreviewAssetSceneEvent, String>>,
        cancel_token: &AtomicBool,
        error: String,
    ) -> Result<(), String> {
        if Self::is_channel_close_error(&error) {
            if let Some(message) = Self::send_failure_log_message(cancel_token, &error) {
                eprintln!("{message}");
            }
            return Ok(());
        }
        sender.send(Err(error)).map_err(|error| error.to_string())
    }

    #[must_use]
    pub(crate) fn is_channel_close_error(error: &str) -> bool {
        error.contains("sending on a closed channel")
            || error.contains("asset worker receiver disconnected")
    }

    #[cfg(test)]
    pub(crate) fn from_receiver_for_test(
        key: StorybookAssetJobKey,
        scope_key: String,
        receiver: Receiver<Result<PreviewAssetSceneEvent, String>>,
        started_at: Instant,
        timeout: Duration,
    ) -> Self {
        Self {
            key,
            scope_key,
            receiver,
            cancel_token: Arc::new(AtomicBool::new(false)),
            started_at,
            timeout,
        }
    }
}

impl Drop for StorybookAssetJob {
    fn drop(&mut self) {
        self.cancel();
    }
}

fn scope_is_subset(candidate: &str, current: &str) -> bool {
    let candidate_ids = scope_ids(candidate);
    if candidate_ids.is_empty() {
        return true;
    }
    let current_ids = scope_ids(current);
    candidate_ids
        .iter()
        .all(|candidate_id| current_ids.contains(candidate_id))
}

fn scope_ids(scope: &str) -> BTreeSet<&str> {
    scope
        .split(';')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .collect()
}

#[cfg(test)]
#[path = "window_asset_job_tests.rs"]
mod tests;
