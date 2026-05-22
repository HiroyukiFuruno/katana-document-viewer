use crate::backend::diagram::KdrDiagramInputFactory;
use crate::forge::{
    BuildGraph, BuildProfile, BuildRequest, ExportFormat, ExportOutput, ExportRequest,
    ForgeBackend, ForgeError, ForgePipeline,
};
use crate::{DocumentSnapshot, KdvThemeSnapshot};
use katana_diagram_renderer::{RenderContext, RenderInput};
use katana_markdown_model::DiagramKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliDiagnostics {
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CliThemeMode {
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliBuildRequest {
    pub snapshot: DocumentSnapshot,
    pub profile: BuildProfile,
    pub theme_mode: Option<CliThemeMode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliExportRequest {
    pub graph: BuildGraph,
    pub format: ExportFormat,
    pub theme_mode: Option<CliThemeMode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliExportDebugRequest {
    pub graph: BuildGraph,
    pub formats: Vec<ExportFormat>,
    pub theme_mode: Option<CliThemeMode>,
}

pub enum CliRequest {
    Build(CliBuildRequest),
    Export(CliExportRequest),
    ExportDebug(CliExportDebugRequest),
    Diagram {
        kind: DiagramKind,
        source: String,
        context: RenderContext,
    },
}

pub enum CliOutput {
    Build {
        graph: Box<BuildGraph>,
        diagnostics: CliDiagnostics,
    },
    Export {
        output: Box<ExportOutput>,
        diagnostics: CliDiagnostics,
    },
    ExportDebug {
        outputs: Vec<ExportOutput>,
        diagnostics: CliDiagnostics,
    },
    Diagram {
        input: Box<RenderInput>,
        diagnostics: CliDiagnostics,
    },
}

pub struct CliApi<B> {
    pipeline: ForgePipeline<B>,
}

impl<B: ForgeBackend> CliApi<B> {
    pub fn new(backend: B) -> Self {
        Self {
            pipeline: ForgePipeline::new(backend),
        }
    }

    pub fn handle(&self, request: CliRequest) -> Result<CliOutput, ForgeError> {
        match request {
            CliRequest::Build(request) => self.handle_build(&request),
            CliRequest::Export(request) => self.handle_export(&request),
            CliRequest::ExportDebug(request) => self.handle_export_debug(&request),
            CliRequest::Diagram {
                kind,
                source,
                context,
            } => self.handle_diagram(kind, source, context),
        }
    }

    fn handle_build(&self, request: &CliBuildRequest) -> Result<CliOutput, ForgeError> {
        let build_request = BuildRequest {
            snapshot: request.snapshot.clone(),
            profile: request.profile.clone(),
            theme: Self::theme_snapshot(request.theme_mode),
        };
        let graph = self.pipeline.build(&build_request)?;
        Ok(CliOutput::Build {
            graph: Box::new(graph),
            diagnostics: CliDiagnostics {
                messages: Vec::new(),
            },
        })
    }

    fn handle_export(&self, request: &CliExportRequest) -> Result<CliOutput, ForgeError> {
        let export_request = ExportRequest {
            graph: request.graph.clone(),
            format: request.format,
            theme: Self::theme_snapshot(request.theme_mode),
        };
        let output = self.pipeline.export(&export_request)?;
        Ok(CliOutput::Export {
            output: Box::new(output),
            diagnostics: CliDiagnostics {
                messages: Vec::new(),
            },
        })
    }

    fn handle_export_debug(
        &self,
        request: &CliExportDebugRequest,
    ) -> Result<CliOutput, ForgeError> {
        let mut outputs = Vec::with_capacity(request.formats.len());
        for format in &request.formats {
            let export_request = ExportRequest {
                graph: request.graph.clone(),
                format: *format,
                theme: Self::theme_snapshot(request.theme_mode),
            };
            outputs.push(self.pipeline.export(&export_request)?);
        }
        Ok(CliOutput::ExportDebug {
            outputs,
            diagnostics: CliDiagnostics {
                messages: Vec::new(),
            },
        })
    }

    fn handle_diagram(
        &self,
        kind: DiagramKind,
        source: String,
        context: RenderContext,
    ) -> Result<CliOutput, ForgeError> {
        let input = KdrDiagramInputFactory::create(kind, source, context);
        Ok(CliOutput::Diagram {
            input: Box::new(input),
            diagnostics: CliDiagnostics {
                messages: Vec::new(),
            },
        })
    }

    fn theme_snapshot(mode: Option<CliThemeMode>) -> KdvThemeSnapshot {
        match mode.unwrap_or(CliThemeMode::Light) {
            CliThemeMode::Light => KdvThemeSnapshot::katana_light(),
            CliThemeMode::Dark => KdvThemeSnapshot::katana_dark(),
        }
    }
}

#[cfg(test)]
#[path = "cli_api_tests.rs"]
mod tests;
