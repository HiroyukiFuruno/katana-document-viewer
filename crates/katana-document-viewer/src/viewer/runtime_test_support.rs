use super::*;
use crate::artifact::ArtifactFactory;
use crate::{
    Artifact, ArtifactBytes, ArtifactFormat, ArtifactId, ArtifactUri, DocumentId, DocumentKind,
    DocumentMetadataView, DocumentOutline, DocumentSnapshot, KdvThemeSnapshot, SourceRevision,
    SourceUri,
};
use katana_markdown_model::{
    ByteRange, KmmDocument, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
    TextFingerprint,
};

pub(super) const CONTENT_HEIGHT: f32 = 1_000.0;
pub(super) const LAST_ANCHOR_Y: f32 = 950.0;
const MATCH_RANGE_START: usize = 0;
const MATCH_RANGE_END: usize = 6;
const VIEWPORT_WIDTH: f32 = 640.0;
const VIEWPORT_HEIGHT: f32 = 300.0;
const RESIZED_VIEWPORT_WIDTH: f32 = 800.0;
const RESIZED_VIEWPORT_HEIGHT: f32 = 400.0;
const TARGET_RECT_X: f32 = 0.0;
const TARGET_RECT_WIDTH: f32 = 100.0;
const TARGET_RECT_HEIGHT: f32 = 24.0;
const ARTIFACT_TEST_BYTE: u8 = 42;

pub(super) struct RuntimeTestData;

impl RuntimeTestData {
    pub(super) fn viewer_input(revision: &str, viewport: ViewerViewport) -> ViewerInput {
        ViewerInput {
            snapshot: Self::snapshot(revision),
            artifacts: Vec::new(),
            theme: KdvThemeSnapshot::default(),
            mode: ViewerMode::Document,
            interaction: ViewerInteractionConfig::default(),
            typography: ViewerTypographyConfig::default(),
            viewport,
            search: ViewerSearchState::default(),
        }
    }

    pub(super) fn search_target(label: &str, index: usize, y: f32) -> ViewerSearchTarget {
        ViewerSearchTarget {
            index,
            matched: ViewerSearchMatch {
                id: ViewerSearchMatchId(label.to_string()),
                node_id: Self::node_id(label),
                source: Self::source_span(label),
                range: ViewerTextRange {
                    start: MATCH_RANGE_START,
                    end: MATCH_RANGE_END,
                },
                text: "needle".to_string(),
                artifact_id: None,
            },
            rect: Self::rect_at_y(y),
        }
    }

    pub(super) fn asset_reference(artifact_id: &ArtifactId) -> ViewerAssetReference {
        ViewerAssetReference {
            node_id: Self::node_id(&artifact_id.0),
            artifact_id: artifact_id.clone(),
            uri: ArtifactUri(format!("kdv://asset/{}", artifact_id.0)),
            format: ArtifactFormat::Png,
        }
    }

    pub(super) fn artifact(label: &str, format: ArtifactFormat) -> Artifact {
        ArtifactFactory::export(
            format,
            DocumentId(format!("document-{label}")),
            SourceRevision("rev-artifact".to_string()),
            ArtifactBytes {
                bytes: vec![ARTIFACT_TEST_BYTE],
            },
        )
    }

    pub(super) fn asset_reference_for_format(
        label: &str,
        format: ArtifactFormat,
    ) -> ViewerAssetReference {
        let artifact = Self::artifact(label, format);
        ViewerAssetPipeline::reference_for_artifact(Self::node_id(label), &artifact)
    }

    pub(super) fn viewer_target(label: &str, y: f32) -> ViewerTarget {
        ViewerTarget {
            node_id: Self::node_id(label),
            source: Self::source_span(label),
            artifact_id: ArtifactId(format!("artifact-{label}")),
            rect: Self::rect_at_y(y),
        }
    }

    pub(super) fn command_target(target: &ViewerSearchTarget) -> ViewerTarget {
        ViewerTarget {
            node_id: target.matched.node_id.clone(),
            source: target.matched.source.clone(),
            artifact_id: ArtifactId(format!("search:{}", target.index)),
            rect: target.rect,
        }
    }

    pub(super) fn viewport() -> ViewerViewport {
        ViewerViewport {
            width: VIEWPORT_WIDTH,
            height: VIEWPORT_HEIGHT,
        }
    }

    pub(super) fn resized_viewport() -> ViewerViewport {
        ViewerViewport {
            width: RESIZED_VIEWPORT_WIDTH,
            height: RESIZED_VIEWPORT_HEIGHT,
        }
    }

    fn snapshot(revision: &str) -> DocumentSnapshot {
        DocumentSnapshot {
            id: DocumentId("document".to_string()),
            kind: DocumentKind::Markdown,
            source_uri: SourceUri("preview://document.md".to_string()),
            revision: SourceRevision(revision.to_string()),
            source_path: "document.md".into(),
            document: Self::kmm_document(),
            outline: DocumentOutline { items: Vec::new() },
            metadata: DocumentMetadataView {
                unresolved_count: 0,
                diagnostic_keys: Vec::new(),
            },
        }
    }

    fn kmm_document() -> KmmDocument {
        KmmDocument {
            path: "document.md".into(),
            fingerprint: TextFingerprint {
                algorithm: "test".to_string(),
                value: "document".to_string(),
            },
            nodes: Vec::new(),
        }
    }

    fn rect_at_y(y: f32) -> ViewerRect {
        ViewerRect {
            x: TARGET_RECT_X,
            y,
            width: TARGET_RECT_WIDTH,
            height: TARGET_RECT_HEIGHT,
        }
    }

    fn node_id(label: &str) -> KmmNodeId {
        KmmNodeId(format!("node-{label}"))
    }

    fn source_span(text: &str) -> SourceSpan {
        SourceSpan {
            byte_range: ByteRange {
                start: 0,
                end: text.len(),
            },
            line_column_range: LineColumnRange {
                start: LineColumn { line: 1, column: 1 },
                end: LineColumn {
                    line: 1,
                    column: text.len() + 1,
                },
            },
            raw: RawSnippet {
                text: text.to_string(),
            },
        }
    }
}
