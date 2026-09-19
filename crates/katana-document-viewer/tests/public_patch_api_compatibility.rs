#![allow(dead_code)]

use katana_document_viewer::{
    DocumentFrame, DocumentGridCellAppearance, DocumentGridDataBar,
    DocumentGridHorizontalAlignment, DocumentGridIcon, DocumentGridRating,
    DocumentGridVerticalAlignment, DocumentSession, DocumentSurfaceFrame, DocumentViewerState,
    SpreadsheetCellStyleArtifact, SpreadsheetHorizontalAlignment, SpreadsheetMergedCellArtifact,
    SpreadsheetSheetArtifact, SpreadsheetTrackArtifact, SpreadsheetVerticalAlignment,
    ViewerCapabilities, ViewerDiagnostic, ViewerDocumentFormat,
};

fn v0_5_5_spreadsheet_cell_style(
    horizontal_alignment: SpreadsheetHorizontalAlignment,
    vertical_alignment: SpreadsheetVerticalAlignment,
) -> SpreadsheetCellStyleArtifact {
    SpreadsheetCellStyleArtifact {
        font_name: String::new(),
        font_size: 0.0,
        font_color: None,
        fill_color: None,
        bold: false,
        italic: false,
        underline: false,
        strike: false,
        horizontal_alignment,
        vertical_alignment,
        wrap_text: false,
        number_format: String::new(),
    }
}

fn v0_5_5_spreadsheet_sheet(
    row_tracks: Vec<SpreadsheetTrackArtifact>,
    column_tracks: Vec<SpreadsheetTrackArtifact>,
    merged_cells: Vec<SpreadsheetMergedCellArtifact>,
) -> SpreadsheetSheetArtifact {
    SpreadsheetSheetArtifact {
        index: 0,
        name: String::new(),
        row_count: 0,
        column_count: 0,
        row_tracks,
        column_tracks,
        frozen_rows: 0,
        frozen_columns: 0,
        merged_cells,
        show_grid_lines: true,
    }
}

fn v0_5_5_document_frame(
    surface: DocumentSurfaceFrame,
    state: DocumentViewerState,
    capabilities: ViewerCapabilities,
    diagnostics: Vec<ViewerDiagnostic>,
    format: ViewerDocumentFormat,
) -> DocumentFrame {
    DocumentFrame {
        surface,
        state,
        capabilities,
        diagnostics,
        format,
    }
}

fn v0_5_5_grid_cell_appearance(
    horizontal_alignment: DocumentGridHorizontalAlignment,
    vertical_alignment: DocumentGridVerticalAlignment,
    data_bar: Option<DocumentGridDataBar>,
    icon: Option<DocumentGridIcon>,
    rating: Option<DocumentGridRating>,
) -> DocumentGridCellAppearance {
    DocumentGridCellAppearance {
        font_family: String::new(),
        font_size_px: 0,
        text_color: None,
        fill_color: None,
        bold: false,
        italic: false,
        underline: false,
        strike: false,
        horizontal_alignment,
        vertical_alignment,
        wrap_text: false,
        data_bar,
        icon,
        rating,
    }
}

fn v0_5_5_document_session_close(session: DocumentSession) {
    session.close();
}
