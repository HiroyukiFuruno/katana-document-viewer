use crate::{
    Artifact, ArtifactFormat, BuildGraph, BuildProfile, BuildRequest, KdvPreviewSurface,
    KdvPreviewSurfaceFactory, KdvThemeSnapshot, PreviewConfig, PreviewOutput, PreviewSurfaceImage,
    RenderedDiagram, ViewerDiagramKind, ViewerNode, ViewerNodeKind, ViewerNodePlanner,
    ViewerStateEngine,
};

pub(super) struct PreviewSurfaceExporter;

impl PreviewSurfaceExporter {
    pub(super) fn attach_surface(output: &mut PreviewOutput, config: &PreviewConfig) {
        let theme = Self::theme(config);
        let graph = Self::build_graph(output, &theme);
        let preview_surface = KdvPreviewSurfaceFactory::create_from_config(&graph, &theme, config);
        let document_surface_height = preview_surface.content_height;
        let mut image = Self::surface_image(preview_surface);
        image.fingerprint = Self::fingerprint(output, config, &image);
        output.scroll_offset = config.scroll_offset.max(0.0);
        output.content_height = Self::content_height(document_surface_height, &image, config);
        output.state =
            ViewerStateEngine::snapshot(&output.input, output.content_height, output.scroll_offset);
        output.surface = Some(image);
    }

    fn build_graph(output: &PreviewOutput, theme: &KdvThemeSnapshot) -> BuildGraph {
        let request = BuildRequest {
            snapshot: output.input.snapshot.clone(),
            profile: BuildProfile::markdown_export(),
            theme: theme.clone(),
        };
        BuildGraph::from_request(&request).with_rendered_diagrams(Self::rendered_diagrams(output))
    }

    fn rendered_diagrams(output: &PreviewOutput) -> Vec<RenderedDiagram> {
        if output.input.artifacts.is_empty() {
            return Vec::new();
        }
        let plan = ViewerNodePlanner::create(&output.input, output.scroll_offset);
        let mut diagrams = Vec::new();
        for node in plan
            .nodes
            .iter()
            .filter(|node| matches!(node.kind, ViewerNodeKind::Diagram { .. }))
        {
            Self::push_rendered_diagram(output, node, &mut diagrams);
        }
        diagrams
    }

    fn push_rendered_diagram(
        output: &PreviewOutput,
        node: &ViewerNode,
        diagrams: &mut Vec<RenderedDiagram>,
    ) {
        let Some(artifact) = node
            .artifact_id
            .as_ref()
            .and_then(|artifact_id| Self::artifact(output, artifact_id))
        else {
            return;
        };
        if let Some(diagram) = Self::rendered_diagram(node, artifact) {
            diagrams.push(diagram);
        }
    }

    fn artifact<'a>(
        output: &'a PreviewOutput,
        artifact_id: &crate::ArtifactId,
    ) -> Option<&'a Artifact> {
        output
            .input
            .artifacts
            .iter()
            .find(|artifact| artifact.manifest.id == *artifact_id)
    }

    fn rendered_diagram(node: &ViewerNode, artifact: &Artifact) -> Option<RenderedDiagram> {
        if artifact.manifest.format != ArtifactFormat::Svg
            || !artifact.manifest.diagnostics.entries.is_empty()
        {
            return None;
        }
        let svg = std::str::from_utf8(&artifact.bytes.bytes).ok()?.trim();
        if !svg.starts_with("<svg") {
            return None;
        }
        Some(RenderedDiagram {
            node_id: node.node_id.0.clone(),
            kind: Self::diagram_kind(&node.kind)?.to_string(),
            svg: svg.to_string(),
        })
    }

    fn diagram_kind(kind: &ViewerNodeKind) -> Option<&'static str> {
        let ViewerNodeKind::Diagram { kind } = kind else {
            return None;
        };
        Some(match kind {
            ViewerDiagramKind::Mermaid => "mermaid",
            ViewerDiagramKind::DrawIo => "drawio",
            ViewerDiagramKind::PlantUml => "plantuml",
        })
    }

    fn surface_image(surface: KdvPreviewSurface) -> PreviewSurfaceImage {
        PreviewSurfaceImage {
            fingerprint: String::new(),
            width: surface.width,
            height: surface.height,
            origin_y: surface.origin_y,
            content_height: surface.content_height,
            rgba: surface.rgba,
        }
    }

    fn content_height(
        document_surface_height: u32,
        image: &PreviewSurfaceImage,
        config: &PreviewConfig,
    ) -> f32 {
        let scaled_height = document_surface_height as f32 * Self::scale(image, config);
        Self::scrollable_height(scaled_height, config.viewport.height)
    }

    fn scrollable_height(content_height: f32, viewport_height: f32) -> f32 {
        let viewport_height = viewport_height.max(0.0);
        if content_height <= viewport_height {
            return content_height;
        }
        content_height + viewport_height
    }

    fn scale(image: &PreviewSurfaceImage, config: &PreviewConfig) -> f32 {
        if image.width == 0 || config.viewport.width <= 0.0 {
            return 1.0;
        }
        config.viewport.width / image.width as f32
    }

    pub(super) fn theme(config: &PreviewConfig) -> KdvThemeSnapshot {
        if config.theme.is_katana_export_reference() {
            return KdvThemeSnapshot::katana_export_reference();
        }
        if config.theme.is_dark() {
            return KdvThemeSnapshot::katana_dark();
        }
        KdvThemeSnapshot::katana_light()
    }

    fn fingerprint(
        output: &PreviewOutput,
        config: &PreviewConfig,
        image: &PreviewSurfaceImage,
    ) -> String {
        format!(
            "{}:{}:{}:{}x{}@{}",
            output.input.snapshot.revision.0,
            config.theme.name,
            config.theme.fingerprint,
            image.width,
            image.height,
            image.origin_y
        )
    }
}

#[cfg(test)]
#[path = "engine_surface_exporter_diagram_tests.rs"]
mod diagram_tests;

#[cfg(test)]
#[path = "engine_surface_exporter_node_tests.rs"]
mod node_tests;

#[cfg(test)]
#[path = "engine_surface_exporter_layout_tests.rs"]
mod layout_tests;
