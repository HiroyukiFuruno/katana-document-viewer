use super::super::test_support::node;
use crate::artifact::{
    ArtifactBytes, ArtifactDiagnostic, ArtifactDiagnostics, ArtifactFactory, ArtifactFormat,
    ArtifactId, DiagnosticSeverity,
};
use crate::document::{DocumentId, SourceRevision};
use crate::viewer::types::ViewerInput;
use katana_markdown_model::{CodeBlockRole, DiagramKind, KmmNodeId, KmmNodeKind};

pub(super) fn rejected_svg_and_error_artifacts(input: &ViewerInput) -> Vec<crate::Artifact> {
    vec![
        diagram_artifact_raw(
            input,
            "diagram-node",
            "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>",
            Vec::new(),
        ),
        diagram_artifact_raw(
            input,
            "bad-diagnostics",
            "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg",
            vec![ArtifactDiagnostic {
                severity: DiagnosticSeverity::Warning,
                code: "w001".to_string(),
                message: "warn".to_string(),
            }],
        ),
        diagram_artifact_like(input, ArtifactFormat::Png, "non-svg"),
    ]
}

pub(super) fn diagram_node(raw: &str) -> katana_markdown_model::KmmNode {
    let mut value = node(
        KmmNodeKind::CodeBlock(CodeBlockRole::Diagram {
            kind: DiagramKind::Mermaid,
        }),
        &format!("```mermaid\n{raw}\n```"),
        Vec::new(),
    );
    value.id = KmmNodeId(format!("node-{raw}"));
    value
}

pub(super) fn diagram_artifact(input: &ViewerInput, node_id_suffix: &str) -> crate::Artifact {
    diagram_artifact_like(
        input,
        ArtifactFormat::Svg,
        &format!("{}:node-{}", input.snapshot.id.0, node_id_suffix),
    )
}

pub(super) fn diagram_artifact_raw(
    input: &ViewerInput,
    node_id_suffix: &str,
    svg: &str,
    diagnostics: Vec<ArtifactDiagnostic>,
) -> crate::Artifact {
    let artifact_id = ArtifactId(format!(
        "{}:node-{}:Svg",
        input.snapshot.id.0, node_id_suffix
    ));
    ArtifactFactory::image_asset_with_id(
        artifact_id,
        ArtifactFormat::Svg,
        DocumentId(input.snapshot.id.0.clone()),
        SourceRevision(input.snapshot.revision.0.clone()),
        ArtifactBytes {
            bytes: svg.as_bytes().to_vec(),
        },
        "ktest",
        ArtifactDiagnostics {
            entries: diagnostics,
        },
    )
}

pub(super) fn diagram_artifact_like(
    input: &ViewerInput,
    format: ArtifactFormat,
    suffix: &str,
) -> crate::Artifact {
    ArtifactFactory::image_asset_with_id(
        ArtifactId(format!("{suffix}:Svg")),
        format,
        DocumentId(input.snapshot.id.0.clone()),
        SourceRevision(input.snapshot.revision.0.clone()),
        ArtifactBytes {
            bytes: b"<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_vec(),
        },
        "ktest",
        ArtifactDiagnostics {
            entries: Vec::new(),
        },
    )
}
