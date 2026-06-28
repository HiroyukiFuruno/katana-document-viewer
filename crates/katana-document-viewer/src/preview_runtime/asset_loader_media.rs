use crate::preview_runtime::asset_loader::PreviewAssetLoader;
use crate::preview_runtime::asset_loader_media_types::{
    DiagramArtifactContext, PreviewAssetDiagnostics,
};
use crate::preview_runtime::asset_loader_support::PreviewAssetLoaderSupport;
use crate::{
    Artifact, ArtifactBytes, ArtifactDiagnostics, ArtifactFactory, ArtifactFormat, ArtifactId,
    DiagramRenderEngine, KdvThemeSnapshot, KrrMathRenderEngine, ViewerAssetLoadRequest,
};
use crate::{PreviewError, PreviewOutput};
use katana_markdown_model::KmmNode;
use std::path::PathBuf;

const BACKEND: &str = "katana-document-viewer";

impl<E: DiagramRenderEngine> PreviewAssetLoader<E> {
    pub(crate) fn load_request(
        &self,
        output: &PreviewOutput,
        request: &ViewerAssetLoadRequest,
        theme: &KdvThemeSnapshot,
    ) -> Result<Option<Artifact>, PreviewError> {
        if request.format == ArtifactFormat::Html {
            return Ok(None);
        }
        if let Some(path) = PreviewAssetLoaderSupport::file_path_from_uri(&request.uri.0) {
            return Ok(Some(self.load_file_artifact(output, request, path)));
        }
        self.load_diagram_artifact(output, request, theme).map(Some)
    }

    fn load_file_artifact(
        &self,
        output: &PreviewOutput,
        request: &ViewerAssetLoadRequest,
        path: PathBuf,
    ) -> Artifact {
        let bytes = match std::fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) => {
                return Self::error_artifact(
                    output,
                    request,
                    Vec::new(),
                    format!("asset file read failed: {}: {error}", path.display()),
                );
            }
        };
        Self::image_artifact(
            output,
            request.artifact_id.clone(),
            request.format,
            bytes,
            PreviewAssetDiagnostics::empty(),
        )
    }

    fn load_diagram_artifact(
        &self,
        output: &PreviewOutput,
        request: &ViewerAssetLoadRequest,
        theme: &KdvThemeSnapshot,
    ) -> Result<Artifact, PreviewError> {
        let Some(node) = PreviewAssetLoaderSupport::find_node(
            &output.input.snapshot.document.nodes,
            &request.node_id,
        ) else {
            return Ok(Self::error_artifact(
                output,
                request,
                Vec::new(),
                format!("asset node missing: {}", request.node_id.0),
            ));
        };
        let Some((kind, source)) = PreviewAssetLoaderSupport::diagram_source(node) else {
            return self.load_math_or_error(output, request, theme, node);
        };
        Ok(self.render_diagram_artifact(DiagramArtifactContext {
            output,
            request,
            theme,
            node,
            kind,
            source,
        }))
    }

    fn load_math_or_error(
        &self,
        output: &PreviewOutput,
        request: &ViewerAssetLoadRequest,
        theme: &KdvThemeSnapshot,
        node: &KmmNode,
    ) -> Result<Artifact, PreviewError> {
        let Some(source) = PreviewAssetLoaderSupport::math_source(node) else {
            return Ok(Self::error_artifact(
                output,
                request,
                node.source.raw.text.as_bytes().to_vec(),
                format!("unsupported lazy asset request: {}", request.node_id.0),
            ));
        };
        Ok(Self::render_math_artifact(output, request, theme, source))
    }

    fn render_math_artifact(
        output: &PreviewOutput,
        request: &ViewerAssetLoadRequest,
        theme: &KdvThemeSnapshot,
        source: String,
    ) -> Artifact {
        match KrrMathRenderEngine::render_display_svg(&source, theme) {
            Ok(svg) => Self::image_artifact(
                output,
                request.artifact_id.clone(),
                ArtifactFormat::Svg,
                svg.into_bytes(),
                PreviewAssetDiagnostics::empty(),
            ),
            Err(message) => Self::image_artifact(
                output,
                request.artifact_id.clone(),
                ArtifactFormat::Svg,
                source.into_bytes(),
                PreviewAssetLoaderSupport::error_diagnostics(message),
            ),
        }
    }

    pub(crate) fn image_artifact(
        output: &PreviewOutput,
        artifact_id: ArtifactId,
        format: ArtifactFormat,
        bytes: Vec<u8>,
        diagnostics: ArtifactDiagnostics,
    ) -> Artifact {
        ArtifactFactory::image_asset_with_id(
            artifact_id,
            format,
            output.input.snapshot.id.clone(),
            output.input.snapshot.revision.clone(),
            ArtifactBytes { bytes },
            BACKEND,
            diagnostics,
        )
    }

    fn error_artifact(
        output: &PreviewOutput,
        request: &ViewerAssetLoadRequest,
        bytes: Vec<u8>,
        message: String,
    ) -> Artifact {
        Self::image_artifact(
            output,
            request.artifact_id.clone(),
            request.format,
            bytes,
            PreviewAssetLoaderSupport::error_diagnostics(message),
        )
    }
}
