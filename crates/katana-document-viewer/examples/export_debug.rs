use katana_document_viewer::{
    BuildProfile, BuildRequest, DiagramRenderingBackend, DocumentSnapshotFactory, DocumentSource,
    EvaluationCoverageMatrix, ExportFormat, ExportOutput, ExportRequest, ForgePipeline,
    KdvThemeSnapshot, KrrDiagramRenderEngine, SourceKind, SourceRevision, SourceUri,
};
use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};
use serde::Serialize;
use std::error::Error;
use std::fs;
use std::path::Path;

#[path = "export_debug/args.rs"]
mod args;
use args::{CommandArgs, CommandArgsParser, EXPORT_FORMATS};

#[derive(Debug, Serialize)]
struct ExportDebugSummary {
    input_path: String,
    output_dir: String,
    document_id: String,
    node_count: usize,
    missing_implementation_count: usize,
    external_backend_required_count: usize,
    export_files: Vec<ExportDebugFile>,
}

#[derive(Debug, Serialize)]
struct ExportDebugFile {
    format: String,
    artifact_file: String,
    artifact_id: String,
    backend: String,
    byte_len: u64,
}

struct ExportDebugCommand;

impl ExportDebugCommand {
    fn run(args: CommandArgs) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(&args.input_path)?;
        let content = content.replace("\r\n", "\n").replace('\r', "\n");
        let source = Self::source(&args.input_path, &content);
        let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
            args.input_path.display().to_string(),
            content,
        ))?;
        let snapshot = DocumentSnapshotFactory::from_kmm(source, document);
        let request = BuildRequest {
            snapshot,
            profile: BuildProfile::markdown_export(),
            theme: args.theme.clone(),
        };
        let pipeline = ForgePipeline::new(DiagramRenderingBackend::new(KrrDiagramRenderEngine));
        let graph = pipeline.build(&request)?;
        let coverage = EvaluationCoverageMatrix::v0_1();
        prepare_output_dir(&args.output_dir)?;
        write_toml(&args.output_dir.join("build-graph.toml"), &graph)?;
        write_toml(&args.output_dir.join("coverage-matrix.toml"), &coverage)?;
        let export_files = Self::write_exports(&args.output_dir, &pipeline, &graph, &args.theme)?;
        let summary = ExportDebugSummary {
            input_path: args.input_path.display().to_string(),
            output_dir: args.output_dir.display().to_string(),
            document_id: graph.snapshot.id.0.clone(),
            node_count: graph.snapshot.document.nodes.len(),
            missing_implementation_count: coverage
                .status_count(katana_document_viewer::CoverageStatus::MissingImplementation),
            external_backend_required_count: coverage
                .status_count(katana_document_viewer::CoverageStatus::ExternalBackendRequired),
            export_files,
        };
        write_toml(&args.output_dir.join("summary.toml"), &summary)?;
        Ok(())
    }

    fn source(input_path: &Path, content: &str) -> DocumentSource {
        DocumentSource {
            uri: SourceUri(format!("file://{}", input_path.display())),
            kind: SourceKind::Markdown,
            revision: SourceRevision("debug".to_string()),
            content: content.to_string(),
        }
    }

    fn write_exports(
        output_dir: &Path,
        pipeline: &ForgePipeline<DiagramRenderingBackend<KrrDiagramRenderEngine>>,
        graph: &katana_document_viewer::BuildGraph,
        theme: &KdvThemeSnapshot,
    ) -> Result<Vec<ExportDebugFile>, Box<dyn Error>> {
        let mut files = Vec::new();
        for format in EXPORT_FORMATS {
            let output = pipeline.export(&ExportRequest {
                graph: graph.clone(),
                format,
                theme: theme.clone(),
            })?;
            files.push(write_export(output_dir, format, &output)?);
        }
        Ok(files)
    }
}

fn write_export(
    output_dir: &Path,
    format: ExportFormat,
    output: &ExportOutput,
) -> Result<ExportDebugFile, Box<dyn Error>> {
    let label = format_label(format);
    let artifact_path = export_artifact_path(output_dir, format);
    fs::write(&artifact_path, &output.artifact.bytes.bytes)?;
    let byte_len = fs::metadata(&artifact_path)?.len();
    if byte_len == 0 {
        return Err(invalid_input("export artifact file is empty").into());
    }
    Ok(ExportDebugFile {
        format: label.to_string(),
        artifact_file: artifact_path.display().to_string(),
        artifact_id: output.artifact.manifest.id.0.clone(),
        backend: output.artifact.manifest.backend.clone(),
        byte_len,
    })
}

fn export_artifact_path(output_dir: &Path, format: ExportFormat) -> std::path::PathBuf {
    output_dir
        .join("exports")
        .join(format!("sample.ja.{}", format_label(format)))
}

fn format_label(format: ExportFormat) -> &'static str {
    match format {
        ExportFormat::Html => "html",
        ExportFormat::Pdf => "pdf",
        ExportFormat::Png => "png",
        ExportFormat::Jpeg => "jpg",
    }
}

fn write_toml<T: Serialize>(path: &Path, value: &T) -> Result<(), Box<dyn Error>> {
    fs::write(path, toml::to_string_pretty(value)?)?;
    Ok(())
}

fn invalid_input(message: &'static str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, message)
}

fn prepare_output_dir(output_dir: &Path) -> Result<(), Box<dyn Error>> {
    if output_dir.exists() {
        ensure_empty_dir(output_dir)?;
    } else {
        fs::create_dir_all(output_dir)?;
    }
    fs::create_dir_all(output_dir.join("exports"))?;
    Ok(())
}

fn ensure_empty_dir(output_dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut entries = fs::read_dir(output_dir)?;
    if entries.next().transpose()?.is_some() {
        return Err(invalid_input("output directory must be empty").into());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    ExportDebugCommand::run(CommandArgsParser::parse()?)
}
