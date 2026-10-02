use super::{OfficeDocumentFormat, WorkerArguments};
use std::ffi::OsString;
use std::path::PathBuf;

pub(super) fn parse_arguments(
    arguments: impl IntoIterator<Item = OsString>,
) -> Result<WorkerArguments, String> {
    let mut values = arguments.into_iter();
    let _program = values.next();
    let workspace = values
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| "workspace argument is missing".to_owned())?;
    let format = parse_format(values.next())?;
    let max_memory_bytes = parse_u64(values.next(), "max memory")?;
    let max_cpu_seconds = parse_u64(values.next(), "max CPU seconds")?;
    let max_output_bytes = parse_u64(values.next(), "max output bytes")?;
    if values.next().is_some() {
        return Err("unexpected trailing arguments".to_owned());
    }
    if !workspace.is_absolute() {
        return Err("workspace must be absolute".to_owned());
    }
    Ok(WorkerArguments {
        workspace,
        format,
        max_memory_bytes,
        max_cpu_seconds,
        max_output_bytes,
    })
}

pub(super) fn parse_format(value: Option<OsString>) -> Result<OfficeDocumentFormat, String> {
    match value.as_deref().and_then(|value| value.to_str()) {
        Some("docx") => Ok(OfficeDocumentFormat::Docx),
        Some("pptx") => Ok(OfficeDocumentFormat::Pptx),
        Some(value) => Err(format!("unsupported worker format `{value}`")),
        None => Err("format argument is missing or invalid UTF-8".to_owned()),
    }
}

pub(super) fn parse_u64(value: Option<OsString>, name: &str) -> Result<u64, String> {
    let value = match value {
        Some(value) => match value.into_string() {
            Ok(value) => value,
            Err(_) => return Err(format!("{name} argument is missing or invalid UTF-8")),
        },
        None => return Err(format!("{name} argument is missing or invalid UTF-8")),
    };
    let parsed = match value.parse::<u64>() {
        Ok(parsed) => parsed,
        Err(_) => return Err(format!("{name} argument is not an unsigned integer")),
    };
    if parsed == 0 {
        return Err(format!("{name} must be greater than zero"));
    }
    Ok(parsed)
}
