use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use crate::preview_scene::PreviewScene;
use katana_document_viewer::{
    Artifact, DiagramViewportState, MarkdownSource, PreviewConfig, PreviewError, PreviewOutput,
    PreviewOutputFactory, PreviewRenderEngine, ViewerMode, ViewerStateEngine, ViewerTaskState,
};
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};

const DEFAULT_LINE_HEIGHT: f32 = 20.0;
const MAX_CACHED_OUTPUTS: usize = 16;

#[derive(Default)]
pub(crate) struct PreviewBuilderCache {
    parsed_outputs: Vec<CachedParsedOutput>,
    artifact_outputs: Vec<CachedArtifactOutput>,
    lazy_scenes: Vec<CachedLazyScene>,
    parsed_hits: usize,
    parsed_misses: usize,
    artifact_hits: usize,
    artifact_misses: usize,
    lazy_scene_hits: usize,
    lazy_scene_misses: usize,
}

impl PreviewBuilderCache {
    pub(crate) fn parsed_output(
        &mut self,
        engine: &PreviewRenderEngine,
        source: &MarkdownSource,
        config: &PreviewConfig,
    ) -> Result<PreviewOutput, PreviewError> {
        if let Some(index) = self
            .parsed_outputs
            .iter()
            .position(|cached| cached.matches(source))
        {
            self.parsed_hits += 1;
            let cached = &self.parsed_outputs[index];
            return Ok(Self::reconfigure(&cached.output, source, config));
        }
        self.parsed_misses += 1;
        let output = engine.render_viewer_output(source, config)?;
        Self::push_bounded(
            &mut self.parsed_outputs,
            CachedParsedOutput::new(source, &output),
        );
        Ok(output)
    }

    pub(crate) fn output_with_artifacts(
        &mut self,
        source: &MarkdownSource,
        config: &PreviewConfig,
        dark: bool,
        fallback: PreviewOutput,
    ) -> PreviewOutput {
        let theme_fingerprint = &config.theme.fingerprint;
        let cached_outputs = self
            .artifact_outputs
            .iter()
            .filter(|cached| cached.matches(source, dark, theme_fingerprint))
            .collect::<Vec<_>>();
        if cached_outputs.is_empty() {
            self.artifact_misses += 1;
            return fallback;
        }
        self.artifact_hits += 1;
        Self::reconfigure_with_cached_artifacts(fallback, source, config, cached_outputs)
    }

    pub(crate) fn store_artifacts(
        &mut self,
        source: &MarkdownSource,
        config: &PreviewConfig,
        dark: bool,
        output: &PreviewOutput,
    ) {
        Self::push_bounded(
            &mut self.artifact_outputs,
            CachedArtifactOutput::new(source, config, dark, output),
        );
    }

    pub(crate) fn lazy_scene(
        &mut self,
        source: &MarkdownSource,
        request: &PreviewBuildRequest<'_>,
    ) -> Option<PreviewScene> {
        let key = LazySceneCacheKey::new(source, request)?;
        if let Some(index) = self.lazy_scenes.iter().position(|cached| cached.key == key) {
            self.lazy_scene_hits += 1;
            return Some(self.lazy_scenes[index].scene.clone());
        }
        self.lazy_scene_misses += 1;
        None
    }

    pub(crate) fn store_lazy_scene(
        &mut self,
        source: &MarkdownSource,
        request: &PreviewBuildRequest<'_>,
        scene: &PreviewScene,
    ) {
        let Some(key) = LazySceneCacheKey::new(source, request) else {
            return;
        };
        Self::push_bounded(
            &mut self.lazy_scenes,
            CachedLazyScene {
                key,
                scene: scene.clone(),
            },
        );
    }

    #[cfg(test)]
    pub(crate) fn stats(&self) -> PreviewBuilderCacheStats {
        PreviewBuilderCacheStats {
            parsed_hits: self.parsed_hits,
            parsed_misses: self.parsed_misses,
            artifact_hits: self.artifact_hits,
            artifact_misses: self.artifact_misses,
            cached_artifact_count: self.cached_artifact_count(),
            lazy_scene_hits: self.lazy_scene_hits,
            lazy_scene_misses: self.lazy_scene_misses,
        }
    }

    fn reconfigure(
        output: &PreviewOutput,
        source: &MarkdownSource,
        config: &PreviewConfig,
    ) -> PreviewOutput {
        let mut next = output.clone();
        next.surface = None;
        PreviewOutputFactory::apply_config(&mut next, config);
        next.content_height = Self::estimated_content_height(source, config);
        next.state =
            ViewerStateEngine::snapshot(&next.input, next.content_height, next.scroll_offset);
        next
    }

    fn reconfigure_with_cached_artifacts(
        fallback: PreviewOutput,
        source: &MarkdownSource,
        config: &PreviewConfig,
        cached_outputs: Vec<&CachedArtifactOutput>,
    ) -> PreviewOutput {
        let mut next = fallback;
        for artifact in Self::merged_artifacts(cached_outputs) {
            if !next
                .input
                .artifacts
                .iter()
                .any(|existing| existing.manifest.id == artifact.manifest.id)
            {
                next.input.artifacts.push(artifact);
            }
        }
        Self::reconfigure(&next, source, config)
    }

    fn merged_artifacts(cached_outputs: Vec<&CachedArtifactOutput>) -> Vec<Artifact> {
        let mut seen = BTreeSet::new();
        let mut artifacts = Vec::new();
        for cached in cached_outputs {
            for artifact in &cached.output.input.artifacts {
                if seen.insert(artifact.manifest.id.0.clone()) {
                    artifacts.push(artifact.clone());
                }
            }
        }
        artifacts
    }

    #[cfg(test)]
    fn cached_artifact_count(&self) -> usize {
        self.artifact_outputs
            .iter()
            .flat_map(|cached| {
                cached
                    .output
                    .input
                    .artifacts
                    .iter()
                    .map(|artifact| artifact.manifest.id.0.clone())
            })
            .collect::<BTreeSet<_>>()
            .len()
    }

    fn estimated_content_height(source: &MarkdownSource, config: &PreviewConfig) -> f32 {
        let line_height = config.line_height.unwrap_or(DEFAULT_LINE_HEIGHT);
        let line_count = source.content.lines().count().max(1) as f32;
        line_count * line_height + config.viewport.height.max(0.0)
    }

    fn push_bounded<T>(entries: &mut Vec<T>, entry: T) {
        entries.push(entry);
        while entries.len() > MAX_CACHED_OUTPUTS {
            entries.remove(0);
        }
    }
}

struct CachedParsedOutput {
    source: SourceCacheKey,
    output: PreviewOutput,
}

impl CachedParsedOutput {
    fn new(source: &MarkdownSource, output: &PreviewOutput) -> Self {
        let mut output = output.clone();
        output.surface = None;
        Self {
            source: SourceCacheKey::new(source),
            output,
        }
    }

    fn matches(&self, source: &MarkdownSource) -> bool {
        self.source == SourceCacheKey::new(source)
    }
}

struct CachedArtifactOutput {
    source: SourceCacheKey,
    dark: bool,
    theme_fingerprint: String,
    output: PreviewOutput,
}

impl CachedArtifactOutput {
    fn new(
        source: &MarkdownSource,
        config: &PreviewConfig,
        dark: bool,
        output: &PreviewOutput,
    ) -> Self {
        let mut output = output.clone();
        output.surface = None;
        Self {
            source: SourceCacheKey::new(source),
            dark,
            theme_fingerprint: config.theme.fingerprint.clone(),
            output,
        }
    }

    fn matches(&self, source: &MarkdownSource, dark: bool, theme_fingerprint: &str) -> bool {
        self.dark == dark
            && self.theme_fingerprint == theme_fingerprint
            && self.source == SourceCacheKey::new(source)
    }
}

struct CachedLazyScene {
    key: LazySceneCacheKey,
    scene: PreviewScene,
}

#[derive(Clone, PartialEq, Eq)]
struct SourceCacheKey {
    document_id: Option<String>,
    content_len: usize,
    content_hash: u64,
}

impl SourceCacheKey {
    fn new(source: &MarkdownSource) -> Self {
        Self {
            document_id: source.document_id.clone(),
            content_len: source.content.len(),
            content_hash: Self::content_hash(&source.content),
        }
    }

    fn content_hash(content: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct LazySceneCacheKey {
    source: SourceCacheKey,
    viewport_width_bits: u32,
    viewport_height_bits: u32,
    dark: bool,
    mode: ViewerMode,
    preview_font_size: u16,
    search_query: String,
    search_current_index: Option<usize>,
    hover_highlight_enabled: bool,
    selection_enabled: bool,
    image_controls_enabled: bool,
    diagram_controls_enabled: bool,
    code_controls_enabled: bool,
    diagram_viewports: Vec<(String, DiagramViewportStateKey)>,
    image_viewports: Vec<(String, DiagramViewportStateKey)>,
    task_state_overrides: BTreeMap<String, ViewerTaskState>,
    accordion_open_overrides: BTreeMap<String, bool>,
    copied_code_node_ids: BTreeSet<String>,
}

impl LazySceneCacheKey {
    fn new(source: &MarkdownSource, request: &PreviewBuildRequest<'_>) -> Option<Self> {
        if request.asset_mode != PreviewBuildAssetMode::Lazy || request.attach_surface {
            return None;
        }
        Some(Self {
            source: SourceCacheKey::new(source),
            viewport_width_bits: request.viewport.width.to_bits(),
            viewport_height_bits: request.viewport.height.to_bits(),
            dark: request.dark,
            mode: request.mode.clone(),
            preview_font_size: request.typography.preview_font_size,
            search_query: request.search.query.clone(),
            search_current_index: request.search.current_index,
            hover_highlight_enabled: request.interaction.hover_highlight_enabled,
            selection_enabled: request.interaction.selection_enabled,
            image_controls_enabled: request.interaction.image_controls_enabled,
            diagram_controls_enabled: request.interaction.diagram_controls_enabled,
            code_controls_enabled: request.interaction.code_controls_enabled,
            diagram_viewports: request
                .diagram_viewports
                .iter()
                .map(|(key, value)| (key.clone(), DiagramViewportStateKey::from(*value)))
                .collect(),
            image_viewports: request
                .image_viewports
                .iter()
                .map(|(key, value)| (key.clone(), DiagramViewportStateKey::from(*value)))
                .collect(),
            task_state_overrides: request.task_state_overrides.clone(),
            accordion_open_overrides: request.accordion_open_overrides.clone(),
            copied_code_node_ids: request.copied_code_node_ids.clone(),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct DiagramViewportStateKey {
    zoom_bits: u32,
    pan_x_bits: u32,
    pan_y_bits: u32,
    fullscreen_open: bool,
    help_requested: bool,
}

impl From<DiagramViewportState> for DiagramViewportStateKey {
    fn from(value: DiagramViewportState) -> Self {
        Self {
            zoom_bits: value.zoom.to_bits(),
            pan_x_bits: value.pan.x.to_bits(),
            pan_y_bits: value.pan.y.to_bits(),
            fullscreen_open: value.fullscreen_open,
            help_requested: value.help_requested,
        }
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PreviewBuilderCacheStats {
    pub(crate) parsed_hits: usize,
    pub(crate) parsed_misses: usize,
    pub(crate) artifact_hits: usize,
    pub(crate) artifact_misses: usize,
    pub(crate) cached_artifact_count: usize,
    pub(crate) lazy_scene_hits: usize,
    pub(crate) lazy_scene_misses: usize,
}
