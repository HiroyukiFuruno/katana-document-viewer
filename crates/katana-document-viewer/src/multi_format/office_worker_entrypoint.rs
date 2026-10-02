use super::OfficeDocumentFormat;
use super::office_worker_constraints::OfficeWorkerConstraints;
use super::office_worker_fonts::stage_deterministic_fonts;
use super::office_worker_protocol::{INPUT_NAME, OUTPUT_NAME, OfficeWorkerResponse};
use super::spreadsheet_worker_entrypoint::SpreadsheetWorkerEntrypoint;
use super::spreadsheet_worker_protocol::SPREADSHEET_MODE;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[path = "office_worker_failure.rs"]
mod failure;
use failure::{engine_failure, input_failure, output_failure};

#[path = "office_worker_response_writer.rs"]
mod response_writer;
use response_writer::write_response;
#[cfg(test)]
use response_writer::write_response_with;

#[path = "office_worker_format.rs"]
mod format;
use format::{conversion_options, engine_format};
#[path = "office_worker_runtime.rs"]
mod runtime;
use runtime::apply_runtime_constraints;
#[path = "office_worker_arguments.rs"]
mod arguments;
use arguments::parse_arguments;

const EXIT_USAGE: i32 = 64;
const EXIT_FAILURE: i32 = 70;

#[derive(Debug)]
struct WorkerArguments {
    workspace: PathBuf,
    format: OfficeDocumentFormat,
    max_memory_bytes: u64,
    max_cpu_seconds: u64,
    max_output_bytes: u64,
}

type OfficeExecutor = fn(&WorkerArguments) -> Result<OfficeWorkerResponse, (String, String)>;
type ConstraintApplier = fn(&Path, u64, u64) -> Result<(), (String, String)>;

pub struct OfficeWorkerEntrypoint;

impl OfficeWorkerEntrypoint {
    #[must_use]
    pub fn run_from_env() -> i32 {
        Self::run(std::env::args_os().collect())
    }

    #[must_use]
    pub fn run(arguments: Vec<OsString>) -> i32 {
        if arguments.get(2).and_then(|value| value.to_str())
            == Some(super::pdf_raster_worker_protocol::PDF_RASTER_MODE)
        {
            return super::pdf_raster_worker_entrypoint::PdfRasterWorkerEntrypoint::run(arguments);
        }
        if arguments.get(1).and_then(|value| value.to_str()) == Some(SPREADSHEET_MODE) {
            return SpreadsheetWorkerEntrypoint::run(arguments);
        }
        Self::run_office(arguments, execute)
    }

    fn run_office(arguments: Vec<OsString>, executor: OfficeExecutor) -> i32 {
        let arguments = match parse_arguments(arguments) {
            Ok(arguments) => arguments,
            Err(message) => {
                eprintln!("KDV office worker usage error: {message}");
                return EXIT_USAGE;
            }
        };
        let _trace_session = super::debug_trace::DebugTrace::session_from_environment_or_workspace(
            &arguments.workspace,
        );
        match executor(&arguments) {
            Ok(response) => write_response(&arguments.workspace, &response, 0),
            Err((stage, message)) => write_response(
                &arguments.workspace,
                &OfficeWorkerResponse::Failed { stage, message },
                EXIT_FAILURE,
            ),
        }
    }
}

fn execute(arguments: &WorkerArguments) -> Result<OfficeWorkerResponse, (String, String)> {
    execute_with_constraints(arguments, OfficeWorkerConstraints::apply)
}

fn execute_with_constraints(
    arguments: &WorkerArguments,
    apply_constraints: ConstraintApplier,
) -> Result<OfficeWorkerResponse, (String, String)> {
    let _worker = super::debug_trace::DebugTrace::start("office.worker_total");
    apply_runtime_constraints(arguments, apply_constraints)?;
    let input = {
        let _read = super::debug_trace::DebugTrace::start("office.worker_input");
        std::fs::read(arguments.workspace.join(INPUT_NAME)).map_err(input_failure)?
    };
    let font_path = {
        let _fonts = super::debug_trace::DebugTrace::start("office.worker_fonts");
        stage_deterministic_fonts(&arguments.workspace)?
    };
    let result = convert_document(arguments, &input, font_path)?;
    validate_output_size(arguments, result.pdf.len())?;
    write_pdf(arguments, result.pdf)?;
    Ok(OfficeWorkerResponse::Completed {
        warnings: result
            .warnings
            .into_iter()
            .map(|warning| warning.to_string())
            .collect(),
    })
}

fn write_pdf(arguments: &WorkerArguments, pdf: Vec<u8>) -> Result<(), (String, String)> {
    let _write = super::debug_trace::DebugTrace::start("office.worker_output_write");
    std::fs::write(arguments.workspace.join(OUTPUT_NAME), pdf).map_err(output_failure)
}

fn convert_document(
    arguments: &WorkerArguments,
    input: &[u8],
    font_path: PathBuf,
) -> Result<office2pdf::error::ConvertResult, (String, String)> {
    let _engine = super::debug_trace::DebugTrace::start("office.parse_layout");
    let options = conversion_options(font_path);
    office2pdf::convert_bytes(input, engine_format(arguments.format), &options)
        .map_err(engine_failure)
}

fn validate_output_size(
    arguments: &WorkerArguments,
    output_bytes: usize,
) -> Result<(), (String, String)> {
    if output_bytes as u64 > arguments.max_output_bytes {
        return Err((
            "output_limit".to_owned(),
            format!(
                "converted PDF is {} bytes and exceeds {} bytes",
                output_bytes, arguments.max_output_bytes
            ),
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "office_worker_entrypoint_tests.rs"]
mod tests;
