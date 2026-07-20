use crate::PreviewAssetLoader;
use crate::{
    ArtifactFormat, ArtifactId, ArtifactUri, KdvThemeSnapshot, PreviewOutput,
    ViewerAssetLoadPriority, ViewerAssetLoadRequest, ViewerNodePlanner,
};
use katana_markdown_model::KmmNodeId;

use super::test_support::{FakeDiagramEngine, output_for};

#[test]
fn loader_clone_with_custom_engine() {
    let loader = PreviewAssetLoader::new(12u16);
    let cloned = loader.clone();

    assert_eq!(loader.engine, cloned.engine);
    assert_eq!(loader.diagram_cache_root(), cloned.diagram_cache_root());
}

#[test]
fn loader_skips_existing_artifacts_on_repeated_request() -> Result<(), Box<dyn std::error::Error>> {
    let output = output_for("```mermaid\ngraph TD\n  A --> B\n```")?;
    let theme = KdvThemeSnapshot::katana_light();
    let loader = PreviewAssetLoader::new(FakeDiagramEngine);

    let (materialized, _report) = loader.load_requested(&output, &theme)?;
    assert_eq!(1, materialized.input.artifacts.len());

    let (loaded, report) = loader.load_requested(&materialized, &theme)?;

    assert_eq!(1, loaded.input.artifacts.len());
    assert_eq!(0, report.loaded_artifact_count);
    assert_eq!(0, report.failed_artifact_count);
    Ok(())
}

#[test]
fn loader_missing_asset_node_reports_error_artifact() -> Result<(), Box<dyn std::error::Error>> {
    let output = output_for("```mermaid\ngraph TD\n  A --> B\n```")?;
    let theme = KdvThemeSnapshot::katana_light();
    let request = unsupported_request_from_output(
        &output,
        "missing-node",
        "missing-artifact",
        "artifact://missing",
        ArtifactFormat::Svg,
        ViewerAssetLoadPriority::Visible,
    );

    let artifact = PreviewAssetLoader::new(FakeDiagramEngine)
        .load_request(&output, &request, &theme)?
        .ok_or("expected artifact")?;

    assert!(
        artifact
            .manifest
            .diagnostics
            .entries
            .iter()
            .any(|entry| entry.message.contains("asset node missing"))
    );
    Ok(())
}

#[test]
fn loader_unsupported_request_node_reports_error_artifact() -> Result<(), Box<dyn std::error::Error>>
{
    let output = output_for("plain text paragraph")?;
    let node_id = unsupported_request_node_id(&output)?;
    let theme = KdvThemeSnapshot::katana_light();
    let request = unsupported_request_from_output(
        &output,
        &node_id,
        "unsupported-request",
        "artifact://unsupported",
        ArtifactFormat::Svg,
        ViewerAssetLoadPriority::Visible,
    );

    let artifact = PreviewAssetLoader::new(FakeDiagramEngine)
        .load_request(&output, &request, &theme)?
        .ok_or("expected artifact")?;

    assert!(
        artifact
            .manifest
            .diagnostics
            .entries
            .iter()
            .any(|entry| entry.message.contains("unsupported lazy asset request"))
    );
    Ok(())
}

#[test]
fn loader_load_asset_request_skips_html() -> Result<(), Box<dyn std::error::Error>> {
    let output = output_for(r#"<p align="center">Centered HTML</p>"#)?;
    let request = ViewerNodePlanner::create(&output.input, output.scroll_offset)
        .asset_requests
        .into_iter()
        .find(|request| request.format == ArtifactFormat::Html)
        .ok_or("missing html request")?;

    let artifact = PreviewAssetLoader::new(FakeDiagramEngine).load_asset_request(
        &output,
        &request,
        &KdvThemeSnapshot::katana_light(),
    )?;

    assert!(artifact.is_none());
    Ok(())
}

fn unsupported_request_node_id(
    output: &PreviewOutput,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(output
        .input
        .snapshot
        .document
        .nodes
        .first()
        .ok_or("missing node")?
        .id
        .0
        .clone())
}

fn unsupported_request_from_output(
    output: &PreviewOutput,
    node_id: &str,
    artifact_id: &str,
    uri: &str,
    format: ArtifactFormat,
    priority: ViewerAssetLoadPriority,
) -> ViewerAssetLoadRequest {
    ViewerAssetLoadRequest {
        document_revision: output.input.snapshot.revision.clone(),
        node_id: KmmNodeId(node_id.to_string()),
        artifact_id: ArtifactId(artifact_id.to_string()),
        uri: ArtifactUri(uri.to_string()),
        format,
        priority,
    }
}
