use crate::multi_format::office_worker_protocol::MAX_RESPONSE_BYTES;
use crate::multi_format::pdf_raster_worker_protocol::PdfRasterResponse;
use std::io::{self, Write};

struct BoundedResponseBuffer {
    bytes: Vec<u8>,
}

impl Write for BoundedResponseBuffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        // シリアライズ完了後の検査では、一時バッファの上限を守れない。
        if bytes.len() > MAX_RESPONSE_BYTES as usize - self.bytes.len() {
            return Err(io::Error::other(format!(
                "response.json exceeds {MAX_RESPONSE_BYTES} bytes"
            )));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(super) fn encode(response: &PdfRasterResponse) -> Result<Vec<u8>, serde_json::Error> {
    let mut buffer = BoundedResponseBuffer { bytes: Vec::new() };
    serde_json::to_writer(&mut buffer, response)?;
    Ok(buffer.bytes)
}

#[cfg(test)]
mod tests {
    use super::BoundedResponseBuffer;
    use crate::multi_format::office_worker_protocol::MAX_RESPONSE_BYTES;
    use std::io::{self, Write};

    #[test]
    fn buffer_accepts_exact_limit_and_rejects_next_byte_without_mutation() -> io::Result<()> {
        let mut buffer = BoundedResponseBuffer { bytes: Vec::new() };
        buffer.write_all(&vec![b'x'; MAX_RESPONSE_BYTES as usize])?;
        buffer.flush()?;
        assert_eq!(buffer.bytes.len(), MAX_RESPONSE_BYTES as usize);
        assert!(buffer.write_all(b"x").is_err());
        assert_eq!(buffer.bytes.len(), MAX_RESPONSE_BYTES as usize);
        Ok(())
    }
}
