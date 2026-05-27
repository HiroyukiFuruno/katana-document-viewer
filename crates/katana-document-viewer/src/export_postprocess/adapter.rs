#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfPostprocessInput<'a> {
    pub pdf: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfPostprocessOutput {
    pub pdf: Vec<u8>,
    pub elapsed_millis: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfPostprocessError {
    pub message: String,
}

impl PdfPostprocessError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

pub trait PdfPostprocessAdapter {
    fn name(&self) -> &str;
    fn postprocess_pdf(
        &self,
        input: &PdfPostprocessInput<'_>,
    ) -> Result<PdfPostprocessOutput, PdfPostprocessError>;
}

pub struct KaruiPdfPostprocessAdapter;

impl PdfPostprocessAdapter for KaruiPdfPostprocessAdapter {
    fn name(&self) -> &str {
        "karui"
    }

    fn postprocess_pdf(
        &self,
        _input: &PdfPostprocessInput<'_>,
    ) -> Result<PdfPostprocessOutput, PdfPostprocessError> {
        Err(PdfPostprocessError::new(
            "Karui PDF engine is not exposed as a stable library or CLI for KDV integration",
        ))
    }
}
