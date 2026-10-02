use serde::{Deserialize, Serialize};

pub(super) const PDF_RASTER_MODE: &str = "pdf-raster";
pub(super) const REQUEST_NAME: &str = "request.json";
pub(super) const RGBA_NAME: &str = "frame.rgba";
pub(super) const MAX_REQUEST_BYTES: u64 = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PdfRasterRequest {
    pub(super) page_index: usize,
    pub(super) scale_bits: u32,
    pub(super) max_rgba_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum PdfRasterResponse {
    Completed {
        page_index: usize,
        scale_bits: u32,
        width: u32,
        height: u32,
        rgba_bytes: u64,
    },
    Failed {
        stage: String,
        message: String,
    },
}
