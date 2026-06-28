use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::{Result, Write};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

pub(super) fn draw_text_like_marks(rgb: &mut [u8]) {
    for y in 80..96 {
        for x in 72..260 {
            let offset = ((y * WIDTH + x) * 3) as usize;
            rgb[offset..offset + 3].copy_from_slice(&[24, 24, 24]);
        }
    }
}

pub(super) fn draw_single_dot(rgb: &mut [u8]) {
    let offset = ((80 * WIDTH + 72) * 3) as usize;
    rgb[offset..offset + 3].copy_from_slice(&[24, 24, 24]);
}

pub(super) fn pdf_rgb_len() -> usize {
    (WIDTH * HEIGHT * 3) as usize
}

pub(super) fn pdf_with_rgb(rgb: &[u8]) -> Result<Vec<u8>> {
    let compressed = zlib_bytes(rgb)?;
    let mut bytes = format!(
        "%PDF-1.4\n1 0 obj\n<< /Type /Pages >>\nendobj\n2 0 obj\n<< /Type /Page >>\nendobj\n5 0 obj\n<< /Type /XObject /Subtype /Image /Width {WIDTH} /Height {HEIGHT} /Filter /FlateDecode >>\nstream\n"
    )
    .into_bytes();
    bytes.extend_from_slice(&compressed);
    bytes.extend_from_slice(b"\nendstream\nendobj\n");
    Ok(bytes)
}

fn zlib_bytes(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(bytes)?;
    encoder.finish()
}
