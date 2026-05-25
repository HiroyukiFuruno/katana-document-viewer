use super::PdfDocumentHelpers;

impl PdfDocumentHelpers {
    pub(crate) fn escape_pdf_string(value: &str) -> String {
        value
            .replace('\\', r"\\")
            .replace('(', r"\(")
            .replace(')', r"\)")
    }

    pub(crate) fn ascii_object(number: usize, body: &str) -> Vec<u8> {
        format!("{number} 0 obj\n{body}\nendobj\n").into_bytes()
    }

    pub(crate) fn stream_object(number: usize, dictionary: &str, stream: &[u8]) -> Vec<u8> {
        let mut object = format!(
            "{number} 0 obj\n{dictionary} /Length {} >>\nstream\n",
            stream.len()
        )
        .into_bytes();
        object.extend_from_slice(stream);
        object.extend_from_slice(b"\nendstream\nendobj\n");
        object
    }

    pub(crate) fn append_xref(output: &mut Vec<u8>, offsets: &[usize]) {
        let xref_start = output.len();
        output.extend_from_slice(format!("xref\n0 {}\n", offsets.len() + 1).as_bytes());
        output.extend_from_slice(b"0000000000 65535 f \n");
        for offset in offsets {
            output.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
        }
        output.extend_from_slice(
            format!(
                "trailer << /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
                offsets.len() + 1,
                xref_start
            )
            .as_bytes(),
        );
    }
}
