use super::{DecodedRgbSurface, PdfImagePage, PdfImageStreams, RGB_CHANNELS};

pub(super) struct PdfSurfaceDecoder;

impl PdfSurfaceDecoder {
    pub(super) fn decode(bytes: &[u8]) -> Result<DecodedRgbSurface, String> {
        let pages = PdfImageStreams::extract(bytes)?;
        let width = Self::page_width(&pages)?;
        let height = pages.iter().map(|page| page.height).sum();
        let mut rgb = Vec::new();
        for page in pages {
            Self::append_page(&mut rgb, page)?;
        }
        Ok(DecodedRgbSurface { width, height, rgb })
    }

    fn page_width(pages: &[PdfImagePage]) -> Result<u32, String> {
        let Some(first) = pages.first() else {
            return Err("pdf has no image page".to_string());
        };
        if pages.iter().all(|page| page.width == first.width) {
            return Ok(first.width);
        }
        Err("pdf image pages have mixed widths".to_string())
    }

    fn append_page(rgb: &mut Vec<u8>, page: PdfImagePage) -> Result<(), String> {
        let expected = page.width as usize * page.height as usize * RGB_CHANNELS;
        if page.rgb.len() != expected {
            return Err("pdf image stream byte length does not match dimensions".to_string());
        }
        rgb.extend_from_slice(&page.rgb);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_width_rejects_empty_pages() {
        assert_eq!(
            Err("pdf has no image page".to_string()),
            PdfSurfaceDecoder::page_width(&[])
        );
    }

    #[test]
    fn page_width_rejects_mixed_widths() {
        let pages = vec![page(2, 1, vec![0; 6]), page(3, 1, vec![0; 9])];

        assert_eq!(
            Err("pdf image pages have mixed widths".to_string()),
            PdfSurfaceDecoder::page_width(&pages)
        );
    }

    #[test]
    fn append_page_rejects_bad_rgb_length() {
        let mut rgb = Vec::new();

        assert_eq!(
            Err("pdf image stream byte length does not match dimensions".to_string()),
            PdfSurfaceDecoder::append_page(&mut rgb, page(2, 1, vec![0; 3]))
        );
    }

    fn page(width: u32, height: u32, rgb: Vec<u8>) -> PdfImagePage {
        PdfImagePage { width, height, rgb }
    }
}
