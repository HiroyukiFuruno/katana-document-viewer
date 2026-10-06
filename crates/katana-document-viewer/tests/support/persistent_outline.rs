pub fn bytes() -> Vec<u8> {
    let objects = [
        "<< /Type /Catalog /Pages 2 0 R /Outlines 5 0 R >>",
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources <<>> /Contents 4 0 R >>",
        "<< /Length 0 >>\nstream\n\nendstream",
        "<< /Type /Outlines /First 6 0 R /Last 6 0 R /Count 1 >>",
        "<< /Title (Chapter) /Parent 5 0 R /Dest [3 0 R /Fit] >>",
    ];
    let mut bytes = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (index, object) in objects.iter().enumerate() {
        offsets.push(bytes.len());
        bytes.extend_from_slice(format!("{} 0 obj\n{object}\nendobj\n", index + 1).as_bytes());
    }
    append_xref(&mut bytes, &offsets);
    bytes
}

fn append_xref(bytes: &mut Vec<u8>, offsets: &[usize]) {
    let xref = bytes.len();
    bytes.extend_from_slice(
        format!("xref\n0 {}\n0000000000 65535 f \n", offsets.len() + 1).as_bytes(),
    );
    for offset in offsets {
        bytes.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    bytes.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
            offsets.len() + 1,
        )
        .as_bytes(),
    );
}
