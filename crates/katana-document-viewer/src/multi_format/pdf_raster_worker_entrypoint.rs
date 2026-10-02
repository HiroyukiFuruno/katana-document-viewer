use super::office_worker_constraints::OfficeWorkerConstraints;
use super::office_worker_input::INPUT_NAME;
use super::office_worker_parent::OfficeWorkerError;
use super::office_worker_protocol::RESPONSE_NAME;
use super::pdf_raster_worker_protocol::{
    MAX_REQUEST_BYTES, PDF_RASTER_MODE, PdfRasterRequest, PdfRasterResponse, REQUEST_NAME,
};
use super::pdf_raster_worker_reader::read_regular_bounded;
use super::{BinaryDocumentSource, PdfViewerLimits};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const EXIT_USAGE: i32 = 64;
const EXIT_FAILURE: i32 = 70;
const FIXED_IDENTITY: &str = "kdv:private-pdf-raster/input.office";

#[derive(Debug, PartialEq, Eq)]
struct WorkerArguments {
    workspace: PathBuf,
    max_memory_bytes: u64,
    max_cpu_seconds: u64,
    max_output_bytes: u64,
}

pub(super) struct PdfRasterWorkerEntrypoint;

impl PdfRasterWorkerEntrypoint {
    pub(super) fn run(arguments: Vec<OsString>) -> i32 {
        let arguments = match parse_arguments(arguments) {
            Ok(arguments) => arguments,
            Err(message) => {
                eprintln!("KDV PDF raster worker usage error: {message}");
                return EXIT_USAGE;
            }
        };
        let _trace_session = super::debug_trace::DebugTrace::session_from_environment_or_workspace(
            &arguments.workspace,
        );
        match execute(&arguments) {
            Ok(response) => write_response(&arguments.workspace, &response, 0),
            Err((stage, message)) => write_response(
                &arguments.workspace,
                &PdfRasterResponse::Failed { stage, message },
                EXIT_FAILURE,
            ),
        }
    }
}

fn execute(arguments: &WorkerArguments) -> Result<PdfRasterResponse, (String, String)> {
    let _worker = super::debug_trace::DebugTrace::start("pdf_raster.worker_total");
    OfficeWorkerConstraints::apply(
        &arguments.workspace,
        arguments.max_memory_bytes,
        arguments.max_cpu_seconds,
    )?;
    let source = read_pdf_source(&arguments.workspace, arguments.max_output_bytes)?;
    let request = read_request(&arguments.workspace)?;
    render_and_write(arguments, source, request)
}

fn read_pdf_source(
    workspace: &Path,
    max_output_bytes: u64,
) -> Result<BinaryDocumentSource, Failure> {
    let input_limit = max_output_bytes.min(PdfViewerLimits::strict().max_source_bytes as u64);
    let bytes = read_regular_file(&workspace.join(INPUT_NAME), input_limit, "input")?;
    Ok(BinaryDocumentSource::new(
        super::ViewerSourceIdentity::new(FIXED_IDENTITY, "input.office"),
        "application/pdf",
        bytes,
    ))
}

fn read_regular_file(path: &Path, limit: u64, stage: &str) -> Result<Vec<u8>, Failure> {
    read_regular_bounded(path, limit).map_err(|error| match error {
        error @ OfficeWorkerError::OutputLimitExceeded { .. } => {
            (format!("{stage}_limit"), error.to_string())
        }
        error => (stage.to_owned(), error.to_string()),
    })
}

fn read_request(workspace: &Path) -> Result<PdfRasterRequest, Failure> {
    let bytes = read_regular_file(&workspace.join(REQUEST_NAME), MAX_REQUEST_BYTES, "request")?;
    serde_json::from_slice(&bytes).map_err(|error| failure("request", error))
}

fn render_and_write(
    arguments: &WorkerArguments,
    source: BinaryDocumentSource,
    request: PdfRasterRequest,
) -> Result<PdfRasterResponse, Failure> {
    pdf_raster_worker_render::render_and_write(arguments, source, request)
}

fn write_response(workspace: &Path, response: &PdfRasterResponse, exit_code: i32) -> i32 {
    let result = pdf_raster_worker_response::encode(response)
        .map_err(|error| format!("response encoding failed: {error}"))
        .and_then(|bytes| {
            std::fs::write(workspace.join(RESPONSE_NAME), bytes)
                .map_err(|error| format!("response.json write failed: {error}"))
        });
    if let Err(message) = result {
        eprintln!("KDV PDF raster worker response error: {message}");
        return EXIT_FAILURE;
    }
    exit_code
}

type Failure = (String, String);

fn failure(stage: &str, error: impl std::fmt::Display) -> Failure {
    (stage.to_owned(), error.to_string())
}

fn parse_arguments(arguments: Vec<OsString>) -> Result<WorkerArguments, String> {
    let mut values = arguments.into_iter();
    let _binary = values.next();
    let workspace = values
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| "workspace argument is missing".to_owned())?;
    if values.next().as_deref().and_then(|value| value.to_str()) != Some(PDF_RASTER_MODE) {
        return Err("worker mode must be pdf-raster".to_owned());
    }
    let max_memory_bytes = parse_positive_u64(values.next(), "max memory")?;
    let max_cpu_seconds = parse_positive_u64(values.next(), "max CPU seconds")?;
    let max_output_bytes = parse_positive_u64(values.next(), "max output bytes")?;
    if values.next().is_some() {
        return Err("unexpected trailing arguments".to_owned());
    }
    if !workspace.is_absolute() {
        return Err("workspace must be absolute".to_owned());
    }
    Ok(WorkerArguments {
        workspace,
        max_memory_bytes,
        max_cpu_seconds,
        max_output_bytes,
    })
}

fn parse_positive_u64(value: Option<OsString>, name: &str) -> Result<u64, String> {
    let text = match value {
        Some(value) => value
            .into_string()
            .map_err(|_| format!("{name} argument is missing or invalid UTF-8"))?,
        None => return Err(format!("{name} argument is missing or invalid UTF-8")),
    };
    let parsed = text
        .parse::<u64>()
        .map_err(|_| format!("{name} is not an unsigned integer"))?;
    if parsed == 0 {
        return Err(format!("{name} must be greater than zero"));
    }
    Ok(parsed)
}

#[cfg(test)]
#[path = "pdf_raster_worker_entrypoint_tests.rs"]
mod tests;

#[path = "pdf_raster_worker_render.rs"]
mod pdf_raster_worker_render;

#[path = "pdf_raster_worker_response.rs"]
mod pdf_raster_worker_response;
