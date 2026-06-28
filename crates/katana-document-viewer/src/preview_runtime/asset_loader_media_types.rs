use crate::PreviewOutput;
use crate::{ArtifactDiagnostics, KdvThemeSnapshot, ViewerAssetLoadRequest};
use katana_markdown_model::{DiagramKind, KmmNode};

pub(crate) struct DiagramArtifactContext<'a> {
    pub(crate) output: &'a PreviewOutput,
    pub(crate) request: &'a ViewerAssetLoadRequest,
    pub(crate) theme: &'a KdvThemeSnapshot,
    pub(crate) node: &'a KmmNode,
    pub(crate) kind: DiagramKind,
    pub(crate) source: String,
}

pub(crate) struct PreviewAssetDiagnostics;

impl PreviewAssetDiagnostics {
    pub(crate) fn empty() -> ArtifactDiagnostics {
        ArtifactDiagnostics {
            entries: Vec::new(),
        }
    }
}
