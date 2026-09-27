use super::{
    DocumentSessionConfig, DocumentSessionError, DocumentSessionInfo, DocumentSessionRuntime,
    OfficeDocumentFormat, PagedDocumentSession, SpreadsheetDocumentSession, ViewerDocumentFormat,
    ViewerSource,
};
use crate::{OfficeWorkerConfig, ViewerCapabilities, ViewerDiagnostic, ViewerSourceIdentity};

pub(super) fn open_runtime(
    source: ViewerSource,
    config: &DocumentSessionConfig,
) -> Result<DocumentSessionRuntime, DocumentSessionError> {
    match source {
        ViewerSource::Pdf(source) => Ok(DocumentSessionRuntime::Paged(Box::new(
            PagedDocumentSession::open_pdf(source, config.viewport)?,
        ))),
        ViewerSource::Office(source) if source.format == OfficeDocumentFormat::Xlsx => {
            let worker = required_worker(config, source.format)?;
            Ok(DocumentSessionRuntime::Spreadsheet(Box::new(
                SpreadsheetDocumentSession::open(source, worker, config.viewport)?,
            )))
        }
        ViewerSource::Office(source) => {
            let worker = required_worker(config, source.format)?;
            Ok(DocumentSessionRuntime::Paged(Box::new(
                PagedDocumentSession::open_office(source, worker, config.viewport)?,
            )))
        }
    }
}

pub(super) fn session_info(
    identity: ViewerSourceIdentity,
    mime: String,
    runtime: &DocumentSessionRuntime,
) -> DocumentSessionInfo {
    DocumentSessionInfo {
        identity,
        mime,
        format: runtime.format(),
        capabilities: runtime.capabilities().clone(),
        diagnostics: runtime.diagnostics().to_vec(),
    }
}

impl DocumentSessionRuntime {
    fn format(&self) -> ViewerDocumentFormat {
        match self {
            Self::Paged(runtime) => runtime.info_parts().0,
            Self::Spreadsheet(runtime) => runtime.info_parts().0,
        }
    }

    fn capabilities(&self) -> &ViewerCapabilities {
        match self {
            Self::Paged(runtime) => runtime.info_parts().1,
            Self::Spreadsheet(runtime) => runtime.info_parts().1,
        }
    }

    fn diagnostics(&self) -> &[ViewerDiagnostic] {
        match self {
            Self::Paged(runtime) => runtime.info_parts().2,
            Self::Spreadsheet(runtime) => runtime.info_parts().2,
        }
    }
}

fn required_worker(
    config: &DocumentSessionConfig,
    format: OfficeDocumentFormat,
) -> Result<OfficeWorkerConfig, DocumentSessionError> {
    config
        .office_worker
        .clone()
        .ok_or(DocumentSessionError::MissingOfficeWorker { format })
}
