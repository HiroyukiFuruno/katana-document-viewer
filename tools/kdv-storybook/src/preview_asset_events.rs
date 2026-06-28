use super::PreviewBuilder;
use super::preview_asset_request_scope::PreviewAssetRequestScope;
use crate::preview_build_request::PreviewBuildRequest;
use crate::preview_build_support::PreviewBuildSupport;
use crate::preview_scene::PreviewScene;
use katana_document_viewer::{
    Artifact, PreviewAssetLoadReport, PreviewOutput, ViewerAssetLoadRequest,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};

pub(super) const MAX_INCREMENTAL_ASSET_WORKERS: usize = 16;

pub(crate) struct PreviewAssetSceneEvent {
    pub(crate) scene: PreviewScene,
    pub(crate) complete: bool,
}

pub(super) struct AssetWorkerMessage {
    artifact: Option<Artifact>,
}

impl PreviewBuilder {
    pub(crate) fn build_incremental_asset_scenes(
        &self,
        request: PreviewBuildRequest<'_>,
        cancel_token: Arc<AtomicBool>,
        mut emit: impl FnMut(PreviewAssetSceneEvent) -> Result<(), String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let source = self.source_for_fixture(request.fixture)?;
        let config = PreviewBuildSupport::preview_config(
            request.viewport,
            request.scene_scroll_y(),
            request.dark,
            request.interaction.clone(),
            request.mode.clone(),
            request.typography,
            request.search.clone(),
        );
        let output = self.render_output(&source, &config)?;
        let mut output =
            self.output_with_cached_artifacts(&source, &config, request.dark, output)?;
        let requests = PreviewAssetRequestScope::pending_asset_requests(&output);
        let mut report = PreviewAssetLoadReport::default();
        if requests.is_empty() {
            let scene = self.scene_from_output(&source, &request, output, report)?;
            emit(PreviewAssetSceneEvent {
                scene,
                complete: true,
            })
            .map_err(std::io::Error::other)?;
            return Ok(());
        }
        let worker_output = output.clone();
        let received = self.render_assets_incrementally(
            &worker_output,
            &requests,
            cancel_token.clone(),
            |messages, complete| {
                if cancel_token.load(Ordering::Relaxed) {
                    return Ok(());
                }
                for message in messages {
                    if let Some(artifact) = message.artifact {
                        PreviewAssetRequestScope::append_artifact(
                            &mut output,
                            artifact,
                            &mut report,
                        );
                    }
                }
                self.store_artifacts(&source, &config, request.dark, &output)
                    .map_err(|error| error.to_string())?;
                let scene = self
                    .scene_from_output(&source, &request, output.clone(), report)
                    .map_err(|error| error.to_string())?;
                emit(PreviewAssetSceneEvent { scene, complete })
            },
        )?;
        if received == 0 && !cancel_token.load(Ordering::Relaxed) {
            let scene = self.scene_from_output(&source, &request, output, report)?;
            emit(PreviewAssetSceneEvent {
                scene,
                complete: true,
            })
            .map_err(std::io::Error::other)?;
        }
        Ok(())
    }

    fn render_assets_incrementally(
        &self,
        output: &PreviewOutput,
        requests: &[ViewerAssetLoadRequest],
        cancel_token: Arc<AtomicBool>,
        mut accept: impl FnMut(Vec<AssetWorkerMessage>, bool) -> Result<(), String>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let worker_output = output.clone();
        let (sender, receiver) = mpsc::channel();
        let jobs = Arc::new(Mutex::new(requests.iter().cloned()));
        let theme = output.input.theme.clone();
        let worker_count = Self::incremental_asset_worker_count(requests.len());
        std::thread::scope(|scope| {
            for _ in 0..worker_count {
                let sender = sender.clone();
                let jobs = jobs.clone();
                let cancel_token = cancel_token.clone();
                let worker_output = &worker_output;
                let theme = &theme;
                scope.spawn(move || {
                    self.run_asset_worker(worker_output, theme, jobs, sender, cancel_token);
                });
            }
            drop(sender);
            Self::receive_asset_worker_messages(
                receiver,
                requests.len(),
                &cancel_token,
                &mut accept,
            )
        })
    }

    fn run_asset_worker(
        &self,
        output: &PreviewOutput,
        theme: &katana_document_viewer::KdvThemeSnapshot,
        jobs: Arc<Mutex<std::iter::Cloned<std::slice::Iter<'_, ViewerAssetLoadRequest>>>>,
        sender: mpsc::Sender<Result<AssetWorkerMessage, String>>,
        cancel_token: Arc<AtomicBool>,
    ) {
        while !cancel_token.load(Ordering::Relaxed) {
            let Some(request) = Self::next_asset_request(&jobs) else {
                break;
            };
            let result = self
                .loader
                .load_asset_request(output, &request, theme)
                .map_err(|error| error.to_string())
                .map(|artifact| AssetWorkerMessage { artifact });
            if let Err(error) = Self::send_asset_result(&sender, result) {
                cancel_token.store(true, Ordering::Relaxed);
                let _ = sender.send(Err(error));
                break;
            }
        }
    }

    fn next_asset_request(
        jobs: &Arc<Mutex<std::iter::Cloned<std::slice::Iter<'_, ViewerAssetLoadRequest>>>>,
    ) -> Option<ViewerAssetLoadRequest> {
        jobs.lock().ok()?.next()
    }

    fn send_asset_result(
        sender: &mpsc::Sender<Result<AssetWorkerMessage, String>>,
        result: Result<AssetWorkerMessage, String>,
    ) -> Result<(), String> {
        match result {
            Ok(message) => sender
                .send(Ok(message))
                .map_err(|_| "asset worker receiver disconnected".to_string()),
            Err(error) => sender
                .send(Err(error))
                .map_err(|_| "asset worker receiver disconnected".to_string()),
        }
    }
}

#[path = "preview_asset_event_receiver.rs"]
mod event_receiver;

#[cfg(test)]
#[path = "preview_asset_events_tests.rs"]
mod tests;
