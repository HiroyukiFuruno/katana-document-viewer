use crate::multi_format::OfficeDocumentFormat;
use office2pdf::config::{ConvertOptions, Format};
use std::path::PathBuf;

pub(super) fn conversion_options(font_path: PathBuf) -> ConvertOptions {
    ConvertOptions {
        font_paths: vec![font_path],
        ..ConvertOptions::default()
    }
}

pub(super) const fn engine_format(format: OfficeDocumentFormat) -> Format {
    match format {
        OfficeDocumentFormat::Docx => Format::Docx,
        OfficeDocumentFormat::Pptx => Format::Pptx,
        OfficeDocumentFormat::Xlsx => Format::Xlsx,
    }
}

#[cfg(test)]
mod tests {
    use super::conversion_options;
    use std::path::PathBuf;

    #[test]
    fn updated_engine_preserves_fonts_and_hidden_slide_exclusion() {
        let path = PathBuf::from("deterministic-fonts");
        let options = conversion_options(path.clone());
        assert_eq!(vec![path], options.font_paths);
        assert!(!options.include_hidden_slides);
    }
}
