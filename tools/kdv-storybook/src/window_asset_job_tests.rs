use super::{StorybookAssetJobKey, StorybookAssetJobKeyInput};
use katana_document_viewer::{
    DiagramViewportState, ViewerInteractionConfig, ViewerMode, ViewerSearchEngine, ViewerTaskState,
    ViewerTypographyConfig, ViewerVector, ViewerViewport,
};
use std::collections::BTreeMap;
use std::sync::mpsc;
use std::time::{Duration, Instant};

#[test]
fn asset_job_key_is_independent_from_scroll_offsets() {
    let first = key_for_scroll(0.0);
    let middle = key_for_scroll(240.0);
    let far_scrolled = key_for_scroll(1200.0);

    assert_eq!(first, middle);
    assert_eq!(first, far_scrolled);
}

#[test]
fn asset_job_key_keeps_search_task_and_viewport_asset_state() {
    let baseline = key_with_state(StateInput::default());
    let mut diagram_viewports = BTreeMap::new();
    diagram_viewports.insert(
        "diagram-1".to_string(),
        DiagramViewportState {
            zoom: 2.0,
            pan: ViewerVector { x: 40.0, y: 12.0 },
            fullscreen_open: true,
            help_requested: true,
        },
    );
    let mut task_states = BTreeMap::new();
    task_states.insert("task-1".to_string(), ViewerTaskState::Done);
    let search = ViewerSearchEngine::state("needle", Vec::new(), Some(0));

    let changed = key_with_state(StateInput {
        search,
        diagram_viewports,
        image_viewports: BTreeMap::new(),
        task_state_overrides: task_states,
        accordion_open_overrides: BTreeMap::new(),
    });

    assert_ne!(baseline, changed);
}

#[test]
fn asset_job_key_ignores_interaction_display_state() {
    let baseline = key_with_state(StateInput::default());
    let _interaction = ViewerInteractionConfig {
        hover_highlight_enabled: false,
        selection_enabled: false,
        image_controls_enabled: false,
        diagram_controls_enabled: false,
        code_controls_enabled: false,
    };

    let changed = key_with_state(StateInput::default());

    assert_eq!(baseline, changed);
}

#[test]
fn asset_job_key_keeps_image_viewport_state() {
    let baseline = key_with_state(StateInput::default());
    let mut image_viewports = BTreeMap::new();
    image_viewports.insert(
        "image-1".to_string(),
        DiagramViewportState {
            zoom: 2.0,
            pan: ViewerVector { x: 0.0, y: 0.0 },
            fullscreen_open: false,
            help_requested: false,
        },
    );

    let changed = key_with_state(StateInput {
        image_viewports,
        ..StateInput::default()
    });

    assert_ne!(baseline, changed);
}

#[test]
fn asset_job_key_keeps_accordion_state_because_visible_assets_can_change() {
    let baseline = key_with_state(StateInput::default());
    let mut accordion_open_overrides = BTreeMap::new();
    accordion_open_overrides.insert("details-1".to_string(), true);

    let changed = key_with_state(StateInput {
        accordion_open_overrides,
        ..StateInput::default()
    });

    assert_ne!(baseline, changed);
}

#[test]
fn asset_job_scope_cover_accepts_shrunk_pending_scope() {
    let scope = "diagram-1;diagram-2;image-1".to_string();
    let job = storybook_asset_job_with_scope(scope);

    assert!(job.covers_scope("diagram-1;image-1"));
    assert!(job.covers_scope("diagram-1;diagram-2;image-1"));
    assert!(!job.covers_scope("diagram-3"));
}

#[test]
fn asset_job_scope_cover_normalizes_order_and_empty_segments() {
    let job = storybook_asset_job_with_scope("diagram-2;;image-1;diagram-1".to_string());

    assert!(job.covers_scope("image-1;diagram-1"));
    assert!(job.covers_scope(";diagram-1;"));
    assert!(!job.covers_scope("diagram-3;diagram-1"));
}

#[test]
fn asset_job_try_recv_fails_after_timeout_and_cancels_worker()
-> Result<(), Box<dyn std::error::Error>> {
    let (_sender, receiver) = mpsc::channel();
    let job = super::StorybookAssetJob::from_receiver_for_test(
        key_with_state(StateInput::default()),
        "diagram-1".to_string(),
        receiver,
        Instant::now() - Duration::from_millis(20),
        Duration::from_millis(1),
    );

    let result = match job.try_recv() {
        Some(Ok(_)) => {
            return Err(std::io::Error::other("timed out job must not look successful").into());
        }
        Some(Err(error)) => error,
        None => return Err(std::io::Error::other("timed out job must emit an error").into()),
    };

    assert!(result.contains("asset job timed out"));
    assert!(job.cancel_token.load(std::sync::atomic::Ordering::Relaxed));
    Ok(())
}

#[test]
fn asset_job_timeout_uses_default_for_missing_invalid_or_zero_env_value() {
    assert_eq!(
        Duration::from_secs(8),
        super::asset_job_timeout_from_env(None)
    );
    assert_eq!(
        Duration::from_secs(8),
        super::asset_job_timeout_from_env(Some("not-a-number"))
    );
    assert_eq!(
        Duration::from_secs(8),
        super::asset_job_timeout_from_env(Some("0"))
    );
}

#[test]
fn asset_job_timeout_accepts_env_value_and_caps_it() {
    assert_eq!(
        Duration::from_secs(30),
        super::asset_job_timeout_from_env(Some("30"))
    );
    assert_eq!(
        Duration::from_secs(60),
        super::asset_job_timeout_from_env(Some("3600"))
    );
}

#[test]
fn cancelled_asset_job_send_failure_is_normal_discard() {
    let cancel_token = std::sync::atomic::AtomicBool::new(true);

    assert!(!super::StorybookAssetJob::should_log_send_failure(
        &cancel_token
    ));
    assert_eq!(
        None,
        super::StorybookAssetJob::send_failure_log_message(
            &cancel_token,
            "sending on a closed channel"
        )
    );
}

#[test]
fn active_asset_job_send_failure_is_reported() {
    let cancel_token = std::sync::atomic::AtomicBool::new(false);

    assert!(super::StorybookAssetJob::should_log_send_failure(
        &cancel_token
    ));
    assert_eq!(
        Some(
            "[kdv-storybook] asset job result send failed: sending on a closed channel".to_string()
        ),
        super::StorybookAssetJob::send_failure_log_message(
            &cancel_token,
            "sending on a closed channel"
        )
    );
}

#[test]
fn cancelled_asset_job_discards_closed_channel_build_error() {
    let cancel_token = std::sync::atomic::AtomicBool::new(true);
    let (sender, receiver) = mpsc::channel();
    drop(receiver);

    let result = super::StorybookAssetJob::send_build_error(
        &sender,
        &cancel_token,
        "sending on a closed channel".to_string(),
    );

    assert!(result.is_ok());
    assert_eq!(
        None,
        super::StorybookAssetJob::send_failure_log_message(
            &cancel_token,
            "sending on a closed channel"
        )
    );
}

#[test]
fn active_asset_job_reports_closed_channel_build_error() {
    let cancel_token = std::sync::atomic::AtomicBool::new(false);
    let (sender, receiver) = mpsc::channel();
    drop(receiver);

    let result = super::StorybookAssetJob::send_build_error(
        &sender,
        &cancel_token,
        "sending on a closed channel".to_string(),
    );

    assert!(result.is_ok());
    assert_eq!(
        Some(
            "[kdv-storybook] asset job result send failed: sending on a closed channel".to_string()
        ),
        super::StorybookAssetJob::send_failure_log_message(
            &cancel_token,
            "sending on a closed channel"
        )
    );
}

#[test]
fn asset_job_keeps_non_channel_build_error_for_receiver() {
    let cancel_token = std::sync::atomic::AtomicBool::new(false);
    let (sender, receiver) = mpsc::channel();

    let send_result = super::StorybookAssetJob::send_build_error(
        &sender,
        &cancel_token,
        "failed to render svg".to_string(),
    );
    assert!(
        send_result.is_ok(),
        "non channel build errors must be sent to the host"
    );

    let received = receiver.recv();
    assert!(
        received.is_ok(),
        "host must receive non channel build errors"
    );
    let Ok(received) = received else {
        return;
    };

    assert!(
        received.is_err(),
        "non channel build errors must not look successful"
    );
    if let Err(error) = received {
        assert_eq!("failed to render svg", error);
    }
}

#[test]
fn asset_job_classifies_closed_storybook_channel_as_channel_close() {
    assert!(super::StorybookAssetJob::is_channel_close_error(
        "sending on a closed channel"
    ));
}

#[test]
fn asset_job_classifies_closed_asset_worker_channel_as_channel_close() {
    assert!(super::StorybookAssetJob::is_channel_close_error(
        "asset worker receiver disconnected"
    ));
}

#[test]
fn asset_job_keeps_render_error_as_reportable_error() {
    assert!(!super::StorybookAssetJob::is_channel_close_error(
        "failed to render svg"
    ));
}

fn key_for_scroll(_scroll_y: f32) -> StorybookAssetJobKey {
    key_with_state(StateInput::default())
}

fn key_with_state(input: StateInput) -> StorybookAssetJobKey {
    StorybookAssetJobKey::new(StorybookAssetJobKeyInput {
        fixture_label: "katana/sample_diagrams.md".to_string(),
        dark: true,
        mode: ViewerMode::Document,
        typography: ViewerTypographyConfig::default(),
        search: &input.search,
        diagram_viewports: &input.diagram_viewports,
        image_viewports: &input.image_viewports,
        task_state_overrides: &input.task_state_overrides,
        accordion_open_overrides: &input.accordion_open_overrides,
        viewport: ViewerViewport {
            width: 900.0,
            height: 600.0,
        },
    })
}

struct StateInput {
    search: katana_document_viewer::ViewerSearchState,
    diagram_viewports: BTreeMap<String, DiagramViewportState>,
    image_viewports: BTreeMap<String, DiagramViewportState>,
    task_state_overrides: BTreeMap<String, ViewerTaskState>,
    accordion_open_overrides: BTreeMap<String, bool>,
}

impl Default for StateInput {
    fn default() -> Self {
        Self {
            search: ViewerSearchEngine::state("", Vec::new(), None),
            diagram_viewports: BTreeMap::new(),
            image_viewports: BTreeMap::new(),
            task_state_overrides: BTreeMap::new(),
            accordion_open_overrides: BTreeMap::new(),
        }
    }
}

fn storybook_asset_job_with_scope(scope_key: String) -> super::StorybookAssetJob {
    let (_sender, receiver) = std::sync::mpsc::channel();
    super::StorybookAssetJob::from_receiver_for_test(
        key_with_state(StateInput::default()),
        scope_key,
        receiver,
        Instant::now(),
        Duration::from_secs(60),
    )
}
