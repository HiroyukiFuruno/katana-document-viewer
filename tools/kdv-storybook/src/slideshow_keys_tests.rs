use super::{SlideshowKeyPress, SlideshowSceneState, StorybookSlideshowKeys};
use crate::layout::{
    HEADER_HEIGHT, PREVIEW_CONTENT_INSET, STATUS_BAR_HEIGHT, preview_viewport_height,
};
use katana_document_viewer::ViewerMode;
use minifb::Key;

#[test]
fn slideshow_keymap_matches_katana_spec_inputs() {
    assert!(StorybookSlideshowKeys::next_keys().contains(&Key::Right));
    assert!(StorybookSlideshowKeys::next_keys().contains(&Key::PageDown));
    assert!(StorybookSlideshowKeys::next_keys().contains(&Key::Space));
    assert!(StorybookSlideshowKeys::previous_keys().contains(&Key::Left));
    assert!(StorybookSlideshowKeys::previous_keys().contains(&Key::PageUp));
    assert!(StorybookSlideshowKeys::close_keys().contains(&Key::Escape));
}

#[test]
fn toggle_mode_resets_scroll_position() {
    let mut mode = ViewerMode::Document;
    let mut scroll_y = 640.0;

    let changed = StorybookSlideshowKeys::apply_pressed(
        SlideshowKeyPress::ToggleMode,
        &mut mode,
        &mut scroll_y,
        None,
        480.0,
    );

    assert!(changed);
    assert_eq!(mode, ViewerMode::Slideshow);
    assert!((scroll_y - 0.0).abs() < f32::EPSILON);
}

#[test]
fn slideshow_next_and_previous_change_page_scroll() {
    let mut mode = ViewerMode::Slideshow;
    let mut scroll_y = 0.0;

    assert!(apply_next(&mut mode, &mut scroll_y, 0, 3, 400.0));
    assert!((scroll_y - 400.0).abs() < f32::EPSILON);

    let changed = StorybookSlideshowKeys::apply_pressed(
        SlideshowKeyPress::PreviousPage,
        &mut mode,
        &mut scroll_y,
        Some(SlideshowSceneState {
            current_page: 1,
            max_page: 3,
        }),
        400.0,
    );

    assert!(changed);
    assert!((scroll_y - 0.0).abs() < f32::EPSILON);
}

#[test]
fn slideshow_spec_keys_change_pages_and_close_mode() {
    let mut mode = ViewerMode::Slideshow;
    let mut scroll_y = 0.0;

    assert!(apply_next(&mut mode, &mut scroll_y, 0, 2, 360.0));
    assert!((scroll_y - 360.0).abs() < f32::EPSILON);

    let changed = StorybookSlideshowKeys::apply_pressed(
        SlideshowKeyPress::Close,
        &mut mode,
        &mut scroll_y,
        None,
        360.0,
    );

    assert!(changed);
    assert_eq!(mode, ViewerMode::Document);
    assert!((scroll_y - 0.0).abs() < f32::EPSILON);
}

#[test]
fn slideshow_page_height_uses_preview_viewport_not_header_only_window_height() {
    let window_height = 900;
    let expected = preview_viewport_height(window_height) as f32;
    let header_only = window_height.saturating_sub(HEADER_HEIGHT) as f32;

    assert_eq!(
        expected,
        StorybookSlideshowKeys::viewport_height_for_window(window_height)
    );
    assert_eq!(
        expected + (PREVIEW_CONTENT_INSET * 2 + STATUS_BAR_HEIGHT) as f32,
        header_only,
        "header-only height ignores preview insets and status bar reservation"
    );
}

#[test]
fn slideshow_next_page_scrolls_by_storybook_preview_viewport_height() {
    let mut mode = ViewerMode::Slideshow;
    let mut scroll_y = 0.0;
    let viewport_height = StorybookSlideshowKeys::viewport_height_for_window(900);

    assert!(apply_next(&mut mode, &mut scroll_y, 0, 3, viewport_height));
    assert!((scroll_y - preview_viewport_height(900) as f32).abs() < f32::EPSILON);
}

#[test]
fn document_mode_rejects_page_navigation() {
    let mut mode = ViewerMode::Document;
    let mut scroll_y = 120.0;

    let changed = apply_next(&mut mode, &mut scroll_y, 0, 3, 400.0);

    assert!(!changed);
    assert!((scroll_y - 120.0).abs() < f32::EPSILON);
}

fn apply_next(
    mode: &mut ViewerMode,
    scroll_y: &mut f32,
    current_page: usize,
    max_page: usize,
    viewport_height: f32,
) -> bool {
    StorybookSlideshowKeys::apply_pressed(
        SlideshowKeyPress::NextPage,
        mode,
        scroll_y,
        Some(SlideshowSceneState {
            current_page,
            max_page,
        }),
        viewport_height,
    )
}
