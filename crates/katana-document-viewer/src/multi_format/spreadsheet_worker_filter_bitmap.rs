pub(super) fn filtered_out_row_bitmap(rows: impl IntoIterator<Item = usize>) -> Vec<u8> {
    let mut bitmap = Vec::new();
    for row in rows {
        let byte_index = row / 8;
        if byte_index >= bitmap.len() {
            bitmap.resize(byte_index.saturating_add(1), 0);
        }
        bitmap[byte_index] |= 1_u8 << (row % 8);
    }
    bitmap
}
