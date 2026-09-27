use crate::{DocumentGridCell, DocumentGridHorizontalAlignment, DocumentGridVerticalAlignment};

pub(super) fn assert_materialized_appearance(cell: &DocumentGridCell) {
    assert_text_style(cell);
    assert_conditional_formatting(cell);
}

fn assert_text_style(cell: &DocumentGridCell) {
    assert_eq!(12, cell.appearance.font_size_px);
    assert_eq!(
        DocumentGridHorizontalAlignment::Center,
        cell.appearance.horizontal_alignment
    );
    assert_eq!(
        DocumentGridVerticalAlignment::Center,
        cell.appearance.vertical_alignment
    );
}

fn assert_conditional_formatting(cell: &DocumentGridCell) {
    assert_eq!(
        Some(6_250),
        cell.appearance
            .data_bar
            .as_ref()
            .map(|bar| bar.fill_ratio_basis_points)
    );
    assert_eq!(
        Some("arrow-up"),
        cell.appearance.icon.as_ref().map(|icon| icon.name.as_str())
    );
    assert_eq!(
        Some(4),
        cell.appearance.rating.as_ref().map(|rating| rating.count)
    );
}
