use super::StorybookWindow;
use super::window_cursor::StorybookCursorStyle;
use crate::args::StorybookArgs;
use crate::catalog::{FixtureCatalog, StorybookFixture};
use crate::layout::sidebar_content_x;
use crate::preview::PreviewBuilder;
use katana_ui_core::render_model::UiCursor;
use std::path::PathBuf;

#[test]
fn sidebar_hover_uses_pointer_cursor_for_kuc_tree_hit() {
    let storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![
                fixture("direct/sample.md"),
                fixture("katana/html-alignment.htm"),
            ],
        },
        PreviewBuilder::default(),
    );

    let cursor =
        storybook.cursor_for_canvas_point(sidebar_content_x() as f32 + 24.0, 33.0, 1000, 900);

    assert_eq!(UiCursor::Pointer, cursor);
}

#[test]
fn kuc_pointer_cursor_maps_to_pointing_hand_not_open_hand() {
    assert_eq!(
        StorybookCursorStyle::PointingHand,
        StorybookCursorStyle::from_ui_cursor(UiCursor::Pointer)
    );
    assert_eq!(
        StorybookCursorStyle::OpenHand,
        StorybookCursorStyle::from_ui_cursor(UiCursor::Grab)
    );
}

fn fixture(label: &str) -> StorybookFixture {
    StorybookFixture {
        label: label.to_string(),
        path: PathBuf::from(label),
    }
}
