use super::{
    ByteSearch, PDF_DICTIONARY_START, PDF_HEIGHT_KEY, PDF_IMAGE_MARKER, PDF_STREAM_END,
    PDF_STREAM_START, PDF_WIDTH_KEY,
};
use flate2::read::ZlibDecoder;
use std::io::Read;

pub(super) struct PdfImageStreams;

impl PdfImageStreams {
    pub(super) fn extract(bytes: &[u8]) -> Result<Vec<PdfImagePage>, String> {
        let mut pages = Vec::new();
        let mut offset = 0;
        while let Some(marker) = ByteSearch::find(&bytes[offset..], PDF_IMAGE_MARKER) {
            let marker_start = offset + marker;
            let page = Self::extract_page(bytes, marker_start)?;
            offset = page.next_offset;
            pages.push(page.page);
        }
        if pages.is_empty() {
            return Err("pdf image stream is missing".to_string());
        }
        Ok(pages)
    }

    fn extract_page(bytes: &[u8], marker_start: usize) -> Result<PdfImageStream, String> {
        let dictionary_start = ByteSearch::rfind(&bytes[..marker_start], PDF_DICTIONARY_START)
            .ok_or("missing pdf image dictionary")?;
        let stream_start = ByteSearch::find(&bytes[marker_start..], PDF_STREAM_START)
            .ok_or("missing pdf image stream")?
            + marker_start;
        let stream_body_start = stream_start + PDF_STREAM_START.len();
        let stream_end = ByteSearch::find(&bytes[stream_body_start..], PDF_STREAM_END)
            .ok_or("missing pdf image stream end")?
            + stream_body_start;
        let dictionary = &bytes[dictionary_start..stream_start];
        let page = PdfImagePage {
            width: PdfDictionaryNumber::parse(dictionary, PDF_WIDTH_KEY)?,
            height: PdfDictionaryNumber::parse(dictionary, PDF_HEIGHT_KEY)?,
            rgb: Self::inflate(&bytes[stream_body_start..stream_end])?,
        };
        Ok(PdfImageStream {
            page,
            next_offset: stream_end + PDF_STREAM_END.len(),
        })
    }

    fn inflate(bytes: &[u8]) -> Result<Vec<u8>, String> {
        let mut decoder = ZlibDecoder::new(bytes);
        let mut decoded = Vec::new();
        decoder
            .read_to_end(&mut decoded)
            .map_err(|error| error.to_string())?;
        Ok(decoded)
    }
}

struct PdfImageStream {
    page: PdfImagePage,
    next_offset: usize,
}

pub(super) struct PdfImagePage {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) rgb: Vec<u8>,
}

struct PdfDictionaryNumber;

impl PdfDictionaryNumber {
    fn parse(dictionary: &[u8], key: &[u8]) -> Result<u32, String> {
        let start = ByteSearch::find(dictionary, key).ok_or("missing pdf image dimension")?;
        let digits = dictionary[start + key.len()..]
            .iter()
            .skip_while(|byte| byte.is_ascii_whitespace())
            .take_while(|byte| byte.is_ascii_digit())
            .copied()
            .collect::<Vec<_>>();
        let text = String::from_utf8_lossy(&digits);
        text.parse::<u32>().map_err(|error| error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::PdfImageStreams;

    #[test]
    fn extract_reports_invalid_compressed_image_stream() {
        let bytes = b"<< /Width 1 /Height 1 /Subtype /Image >>\nstream\nnot-zlib\nendstream";

        let result = PdfImageStreams::extract(bytes);

        assert!(result.is_err());
    }

    #[test]
    fn extract_reports_missing_image_dimension() {
        let bytes = b"<< /Width 1 /Subtype /Image >>\nstream\nnot-zlib\nendstream";

        let result = PdfImageStreams::extract(bytes);

        assert!(matches!(
            result,
            Err(error) if error == "missing pdf image dimension"
        ));
    }

    #[test]
    fn extract_reports_invalid_image_dimension_number() {
        let bytes = b"<< /Width x /Height 1 /Subtype /Image >>\nstream\nnot-zlib\nendstream";

        let result = PdfImageStreams::extract(bytes);

        assert!(result.is_err());
    }
}
