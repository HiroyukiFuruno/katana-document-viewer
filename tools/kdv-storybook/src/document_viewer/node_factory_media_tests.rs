use super::KucNodeFactory;
use katana_document_viewer::{
    Artifact, ArtifactBytes, ArtifactDiagnostic, ArtifactDiagnostics, ArtifactFactory,
    ArtifactFormat, ArtifactId, DiagnosticSeverity, DocumentId, SourceRevision, ViewerDiagramKind,
    ViewerImageSurface, ViewerNode, ViewerNodeKind, ViewerRect,
};
use katana_markdown_model::{
    ByteRange, KmmNodeId, LineColumn, LineColumnRange, RawSnippet, SourceSpan,
};

const NODE_WIDTH: f32 = 120.0;
const NODE_HEIGHT: f32 = 32.0;

#[test]
fn media_artifact_error_is_reported_as_media_error_node() {
    let artifact_id = ArtifactId("doc:error:Svg".to_string());
    let artifacts = vec![svg_artifact(artifact_id.clone(), error_diagnostics())];
    let factory = KucNodeFactory::new(&artifacts, 120);
    let node = viewer_node(
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::Mermaid,
        },
        "graph",
        Some(artifact_id),
    );

    let ui_node = factory.viewer_node(&node);

    assert_eq!("media-error", ui_node.props().text.role);
    assert_eq!("media-error", ui_node.props().label);
    assert_ne!("graph", ui_node.props().label);
}

#[test]
fn media_raster_error_is_reported_as_media_error_node() {
    let artifact_id = ArtifactId("doc:invalid:Svg".to_string());
    let artifacts = vec![ArtifactFactory::image_asset_with_id(
        artifact_id.clone(),
        ArtifactFormat::Svg,
        DocumentId("doc".to_string()),
        SourceRevision("rev".to_string()),
        ArtifactBytes {
            bytes: b"<svg>".to_vec(),
        },
        "test",
        empty_diagnostics(),
    )];
    let factory = KucNodeFactory::new(&artifacts, 120);
    let node = viewer_node(ViewerNodeKind::Math, "x+y", Some(artifact_id));

    let ui_node = factory.viewer_node(&node);

    assert_eq!("media-error", ui_node.props().text.role);
    assert_eq!("media-error", ui_node.props().label);
    assert_ne!("x+y", ui_node.props().label);
}

#[test]
fn invalid_image_surface_is_reported_as_media_error_node() {
    let factory = KucNodeFactory::new(&[], 120);
    let node = viewer_node(ViewerNodeKind::Image, "alt", None);

    let ui_node = factory.image_surface_node(
        &node,
        &ArtifactId("bad".to_string()),
        ViewerImageSurface {
            fingerprint: String::new(),
            width: 1,
            height: 1,
            display_width: 1.0,
            display_height: 1.0,
            content_scale: 100,
            rgba: vec![255, 255, 255, 255],
        },
    );

    assert_eq!("media-error", ui_node.props().text.role);
    assert_eq!("media-error", ui_node.props().label);
    assert_ne!("alt", ui_node.props().label);
}

fn svg_artifact(artifact_id: ArtifactId, diagnostics: ArtifactDiagnostics) -> Artifact {
    ArtifactFactory::image_asset_with_id(
        artifact_id,
        ArtifactFormat::Svg,
        DocumentId("doc".to_string()),
        SourceRevision("rev".to_string()),
        ArtifactBytes {
            bytes: svg().as_bytes().to_vec(),
        },
        "test",
        diagnostics,
    )
}

fn error_diagnostics() -> ArtifactDiagnostics {
    ArtifactDiagnostics {
        entries: vec![ArtifactDiagnostic {
            severity: DiagnosticSeverity::Error,
            code: "diagram.error".to_string(),
            message: "failed".to_string(),
        }],
    }
}

fn empty_diagnostics() -> ArtifactDiagnostics {
    ArtifactDiagnostics {
        entries: Vec::new(),
    }
}

fn viewer_node(kind: ViewerNodeKind, text: &str, artifact_id: Option<ArtifactId>) -> ViewerNode {
    ViewerNode {
        node_id: KmmNodeId("node".to_string()),
        kind,
        source: source(text),
        text: text.to_string(),
        spans: Vec::new(),
        html_margin_left_px: 0,
        rule_line_offset_px: 0,
        rect: ViewerRect {
            x: 0.0,
            y: 0.0,
            width: NODE_WIDTH,
            height: NODE_HEIGHT,
        },
        artifact_id,
    }
}

fn source(raw: &str) -> SourceSpan {
    SourceSpan {
        byte_range: ByteRange {
            start: 0,
            end: raw.len(),
        },
        line_column_range: LineColumnRange {
            start: LineColumn { line: 1, column: 1 },
            end: LineColumn {
                line: 1,
                column: raw.len() + 1,
            },
        },
        raw: RawSnippet {
            text: raw.to_string(),
        },
    }
}

fn svg() -> &'static str {
    r#"<svg xmlns="http://www.w3.org/2000/svg" width="40" height="20"><rect width="40" height="20" fill="red"/></svg>"#
}
