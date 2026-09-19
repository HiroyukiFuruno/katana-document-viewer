use super::{SpreadsheetGridSurface, test_support::sample_sheet};
use crate::{DocumentGridCommand, DocumentViewport};
use katana_ui_core::molecule::GridCoordinate;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn filter_visibility_preserves_grid_state_and_source_rows() -> TestResult {
    let sheet = sample_sheet();
    let viewport = DocumentViewport::new(320, 120);
    let mut surface = SpreadsheetGridSurface::new_filtered(&sheet, &[1], viewport)?;
    select_and_scroll(&mut surface);

    surface.replace_sheet_filtered(&sheet, &[1, 4], viewport)?;

    assert_eq!(
        Some(GridCoordinate::new(5, 0)),
        surface.grid.active_coordinate()
    );
    let request = surface.materialization_request();
    assert!(!request.iter().any(|coordinate| coordinate.row == 1));
    assert!(!request.iter().any(|coordinate| coordinate.row == 4));
    let frame = surface.frame()?;
    assert!(frame.grid().ok_or("grid frame missing")?.viewport.scroll_y > 0);
    Ok(())
}

fn select_and_scroll(surface: &mut SpreadsheetGridSurface) {
    let _ = surface.apply_command(DocumentGridCommand::Select {
        row: 4,
        column: 0,
        extend: false,
    });
    let _ = surface.apply_command(DocumentGridCommand::ScrollTo { x: 0, y: 240 });
}
