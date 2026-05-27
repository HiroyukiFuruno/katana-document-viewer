use super::contract_test_support::HtmlContractTestSupport;
use crate::{
    BuildProfile, BuildRequest, CliApi, CliExportRequest, CliOutput, CliRequest, CliThemeMode,
    DocumentSnapshotFactory, DocumentSource, ExportFormat, KdvThemeSnapshot, ManifestOnlyBackend,
    RenderedDiagram, SourceKind, SourceRevision, SourceUri,
};
use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};

#[test]
fn app_export_requires_complete_theme_snapshot() -> Result<(), Box<dyn std::error::Error>> {
    let graph = build_graph("# Theme\n", KdvThemeSnapshot::katana_light())?;
    let html =
        HtmlContractTestSupport::export_html_with_graph(graph, KdvThemeSnapshot::katana_light())?;

    assert!(html.contains(r#"<html lang="ja" data-kdv-theme="katana-light">"#));
    assert!(html.contains("--kdv-text:#24292f;"));
    assert!(html.contains("--kdv-background:#ffffff;"));
    Ok(())
}

#[test]
fn cli_export_uses_katana_light_when_theme_mode_is_omitted()
-> Result<(), Box<dyn std::error::Error>> {
    let api = CliApi::new(ManifestOnlyBackend);
    let graph = build_graph("# Theme\n", KdvThemeSnapshot::katana_light())?;
    let output = api.handle(CliRequest::Export(CliExportRequest {
        graph,
        format: ExportFormat::Html,
        theme_mode: None,
    }))?;
    let CliOutput::Export { output, .. } = output else {
        return Err("expected export output".into());
    };
    let html = String::from_utf8(output.artifact.bytes.bytes)?;

    assert!(html.contains(r#"data-kdv-theme="katana-light""#));
    Ok(())
}

#[test]
fn cli_export_uses_katana_dark_when_dark_mode_is_selected() -> Result<(), Box<dyn std::error::Error>>
{
    let api = CliApi::new(ManifestOnlyBackend);
    let graph = build_graph("# Theme\n", KdvThemeSnapshot::katana_dark())?;
    let output = api.handle(CliRequest::Export(CliExportRequest {
        graph,
        format: ExportFormat::Html,
        theme_mode: Some(CliThemeMode::Dark),
    }))?;
    let CliOutput::Export { output, .. } = output else {
        return Err("expected export output".into());
    };
    let html = String::from_utf8(output.artifact.bytes.bytes)?;

    assert!(html.contains(r#"data-kdv-theme="katana-dark""#));
    assert!(html.contains("--kdv-background:#0d1117;"));
    Ok(())
}

#[test]
fn complete_theme_json_is_reflected_in_html_export() -> Result<(), Box<dyn std::error::Error>> {
    let mut theme = KdvThemeSnapshot::katana_dark();
    theme.name = "cli-json".to_string();
    theme.background = "#010203".to_string();
    let theme_json = serde_json::to_string(&theme)?;
    let cli_theme = serde_json::from_str::<KdvThemeSnapshot>(&theme_json)?;
    let graph = build_graph("# Theme\n", cli_theme.clone())?;

    let html = HtmlContractTestSupport::export_html_with_graph(graph, cli_theme)?;

    assert!(html.contains(r#"data-kdv-theme="cli-json""#));
    assert!(html.contains("--kdv-background:#010203;"));
    Ok(())
}

#[test]
fn app_dark_theme_marks_rendered_diagram_as_dark() -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = build_graph(
        "```mermaid\ngraph TD; A-->B\n```\n",
        KdvThemeSnapshot::katana_dark(),
    )?;
    let node_id = graph.snapshot.document.nodes[0].id.0.clone();
    graph = graph.with_rendered_diagrams(vec![RenderedDiagram {
        node_id,
        kind: "mermaid".to_string(),
        svg: r#"<svg data-test="diagram"></svg>"#.to_string(),
    }]);

    let html =
        HtmlContractTestSupport::export_html_with_graph(graph, KdvThemeSnapshot::katana_dark())?;

    assert!(html.contains(r#"data-kdv-diagram-theme="dark""#));
    Ok(())
}

#[test]
fn diagram_background_is_transparent_not_code_block_background()
-> Result<(), Box<dyn std::error::Error>> {
    let theme = KdvThemeSnapshot::katana_light();
    let mut graph = build_graph(
        "```mermaid\ngraph TD; A-->B\n```\n",
        KdvThemeSnapshot::katana_light(),
    )?;
    let node_id = graph.snapshot.document.nodes[0].id.0.clone();
    graph = graph.with_rendered_diagrams(vec![RenderedDiagram {
        node_id,
        kind: "mermaid".to_string(),
        svg: r#"<svg data-test="diagram"></svg>"#.to_string(),
    }]);

    let html = HtmlContractTestSupport::export_html_with_graph(graph, theme.clone())?;
    let diagram_theme = theme.krr_theme();

    assert_eq!(theme.diagram_background, "transparent");
    assert_ne!(theme.diagram_background, theme.code_background);
    assert_eq!(diagram_theme.background, "transparent");
    assert!(html.contains("--kdv-diagram-background:transparent;"));
    assert!(html.contains("figure[data-kdv-diagram]{background:var(--kdv-diagram-background);"));
    assert!(!html.contains("figure[data-kdv-diagram]{background:var(--kdv-code-bg)"));
    Ok(())
}

fn build_graph(
    markdown: &str,
    theme: KdvThemeSnapshot,
) -> Result<crate::BuildGraph, Box<dyn std::error::Error>> {
    let source = DocumentSource {
        uri: SourceUri("file:///theme.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("theme".to_string()),
        content: markdown.to_string(),
    };
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        "theme.md",
        markdown.to_string(),
    ))?;
    let snapshot = DocumentSnapshotFactory::from_kmm(source, document);
    Ok(crate::BuildGraph::from_request(&BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme,
    }))
}
