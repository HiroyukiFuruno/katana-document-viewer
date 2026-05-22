use super::*;
use crate::ManifestOnlyBackend;
use crate::test_support::SampleSnapshotFactory;

#[test]
fn build_entry_returns_graph() -> Result<(), Box<dyn std::error::Error>> {
    let api = CliApi::new(ManifestOnlyBackend);
    let request = CliBuildRequest {
        snapshot: SampleSnapshotFactory::create()?,
        profile: BuildProfile::markdown_export(),
        theme_mode: None,
    };
    let output = api.handle(CliRequest::Build(request))?;

    assert!(matches!(output, CliOutput::Build { .. }));
    Ok(())
}

#[test]
fn export_entry_returns_artifact() -> Result<(), Box<dyn std::error::Error>> {
    let api = CliApi::new(ManifestOnlyBackend);
    let graph = BuildGraph::from_request(&BuildRequest {
        snapshot: SampleSnapshotFactory::create()?,
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    });
    let request = CliExportRequest {
        graph,
        format: ExportFormat::Html,
        theme_mode: None,
    };
    let output = api.handle(CliRequest::Export(request))?;

    assert!(matches!(output, CliOutput::Export { .. }));
    Ok(())
}

#[test]
fn diagram_entry_returns_kdr_input() -> Result<(), Box<dyn std::error::Error>> {
    let api = CliApi::new(ManifestOnlyBackend);
    let output = api.handle(CliRequest::Diagram {
        kind: DiagramKind::Mermaid,
        source: "graph TD; A-->B".to_string(),
        context: RenderContext::default(),
    })?;

    assert!(matches!(output, CliOutput::Diagram { .. }));
    Ok(())
}

#[test]
fn export_debug_entry_returns_all_formats() -> Result<(), Box<dyn std::error::Error>> {
    let api = CliApi::new(ManifestOnlyBackend);
    let graph = BuildGraph::from_request(&BuildRequest {
        snapshot: SampleSnapshotFactory::create()?,
        profile: BuildProfile::markdown_export(),
        theme: KdvThemeSnapshot::katana_light(),
    });
    let output = api.handle(CliRequest::ExportDebug(CliExportDebugRequest {
        graph,
        formats: vec![ExportFormat::Html, ExportFormat::Pdf],
        theme_mode: None,
    }))?;

    let output_count = match output {
        CliOutput::ExportDebug { outputs, .. } => outputs.len(),
        _ => 0,
    };
    assert_eq!(output_count, 2);
    Ok(())
}
