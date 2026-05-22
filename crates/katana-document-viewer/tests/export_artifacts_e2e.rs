use katana_document_viewer::{
    BuildProfile, BuildRequest, DiagramRenderEngine, DiagramRenderRequest, DiagramRenderingBackend,
    DocumentSnapshotFactory, DocumentSource, ExportFormat, ExportRequest, ForgePipeline,
    KdvThemeSnapshot, RenderedDiagram, SourceKind, SourceRevision, SourceUri,
};
use katana_markdown_model::{DiagramKind, KatanaMarkdownModel, MarkdownInput};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn e2e_export_writes_evaluated_html_pdf_png_and_jpeg_without_sidecars() -> Result<(), Box<dyn Error>>
{
    let pipeline = ForgePipeline::new(DiagramRenderingBackend::new(StaticDiagramEngine));
    let theme = KdvThemeSnapshot::katana_light();
    let graph = pipeline.build(&BuildRequest {
        snapshot: snapshot_from_markdown(contract_markdown())?,
        profile: BuildProfile::markdown_export(),
        theme: theme.clone(),
    })?;
    let exports_dir = unique_output_dir()?.join("exports");
    fs::create_dir_all(&exports_dir)?;

    let html = write_export(&pipeline, &graph, &theme, &exports_dir, ExportFormat::Html)?;
    let pdf = write_export(&pipeline, &graph, &theme, &exports_dir, ExportFormat::Pdf)?;
    let png = write_export(&pipeline, &graph, &theme, &exports_dir, ExportFormat::Png)?;
    let jpeg = write_export(&pipeline, &graph, &theme, &exports_dir, ExportFormat::Jpeg)?;

    assert_evaluated_html(&String::from_utf8(fs::read(html)?)?);
    assert!(fs::read(pdf)?.starts_with(b"%PDF-1.4"));
    assert!(fs::read(png)?.starts_with(b"\x89PNG\r\n\x1a\n"));
    assert!(fs::read(jpeg)?.starts_with(b"\xff\xd8\xff"));
    assert_eq!(sidecar_count(&exports_dir)?, 0);
    Ok(())
}

fn snapshot_from_markdown(
    markdown: String,
) -> Result<katana_document_viewer::DocumentSnapshot, Box<dyn Error>> {
    let source = DocumentSource {
        uri: SourceUri("file://e2e-export.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("e2e".to_string()),
        content: markdown.clone(),
    };
    let document =
        KatanaMarkdownModel::parse(MarkdownInput::from_content("e2e-export.md", markdown))?;
    Ok(DocumentSnapshotFactory::from_kmm(source, document))
}

fn write_export(
    pipeline: &ForgePipeline<DiagramRenderingBackend<StaticDiagramEngine>>,
    graph: &katana_document_viewer::BuildGraph,
    theme: &KdvThemeSnapshot,
    exports_dir: &Path,
    format: ExportFormat,
) -> Result<PathBuf, Box<dyn Error>> {
    let output = pipeline.export(&ExportRequest {
        graph: graph.clone(),
        format,
        theme: theme.clone(),
    })?;
    let path = exports_dir.join(format!("sample.ja.{}", format_label(format)));
    fs::write(&path, &output.artifact.bytes.bytes)?;
    assert!(fs::metadata(&path)?.len() > 0);
    Ok(path)
}

fn assert_evaluated_html(html: &str) {
    for needle in [
        "<strong>太字</strong>",
        r#"data-kdv-blockquote="alert""#,
        r#"data-kdv-task-state="in-progress""#,
        r#"data-kdv-render-runtime="katana-render-runtime-stub""#,
        r#"data-kdv-diagram="mermaid""#,
    ] {
        assert!(
            html.contains(needle),
            "missing evaluated HTML fragment: {needle}"
        );
    }
    for needle in [
        "**太字**",
        "[!WARNING]",
        "[/] 進行中",
        "```math",
        "```mermaid",
    ] {
        assert!(
            !html.contains(needle),
            "raw markdown leaked into HTML: {needle}"
        );
    }
}

fn sidecar_count(exports_dir: &Path) -> Result<usize, Box<dyn Error>> {
    Ok(fs::read_dir(exports_dir)?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .ends_with(".manifest.toml")
        })
        .count())
}

fn unique_output_dir() -> Result<PathBuf, Box<dyn Error>> {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    Ok(std::env::temp_dir().join(format!("kdv-export-artifacts-e2e-{nanos}")))
}

fn format_label(format: ExportFormat) -> &'static str {
    match format {
        ExportFormat::Html => "html",
        ExportFormat::Pdf => "pdf",
        ExportFormat::Png => "png",
        ExportFormat::Jpeg => "jpg",
    }
}

fn contract_markdown() -> String {
    [
        "# 契約",
        "",
        "**太字**",
        "",
        "> [!WARNING]",
        "> 危険です。",
        "",
        "- [/] 進行中",
        "",
        "inline math: $a^2 + b^2 = c^2$",
        "",
        "```math",
        "a^2 + b^2 = c^2",
        "```",
        "",
        "```mermaid",
        "graph TD",
        "  A --> B",
        "```",
    ]
    .join("\n")
}

struct StaticDiagramEngine;

impl DiagramRenderEngine for StaticDiagramEngine {
    fn render(&self, request: DiagramRenderRequest<'_>) -> Result<RenderedDiagram, String> {
        let kind = match request.kind {
            DiagramKind::Mermaid => "mermaid",
            DiagramKind::DrawIo => "drawio",
            DiagramKind::PlantUml => return Err("PlantUML is external-backend-required".into()),
        };
        Ok(RenderedDiagram {
            node_id: request.node_id.to_string(),
            kind: kind.to_string(),
            svg: format!("<svg data-test=\"{}\"></svg>", request.node_id),
        })
    }
}
