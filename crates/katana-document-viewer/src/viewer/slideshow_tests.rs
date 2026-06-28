use super::runtime_test_support::RuntimeTestData as Data;
use super::*;

const VIEWPORT_WIDTH: f32 = 800.0;
const VIEWPORT_HEIGHT: f32 = 500.0;
const TALL_CONTENT_HEIGHT: f32 = 1200.0;
const EXACT_TWO_PAGE_CONTENT_HEIGHT: f32 = 1000.0;
const OUT_OF_RANGE_PAGE_INDEX: usize = 9;
const EXPECTED_LAST_PAGE_INDEX: usize = 2;
const EXPECTED_PREVIOUS_PAGE_INDEX: usize = 1;
const EXPECTED_EXACT_TWO_PAGE_MAX_INDEX: usize = 2;

#[test]
fn slideshow_navigation_clamps_to_virtual_pages() {
    let viewport = sample_viewport();
    let mut state = ViewerStateEngine::slideshow_state(
        viewport,
        TALL_CONTENT_HEIGHT,
        OUT_OF_RANGE_PAGE_INDEX,
        false,
        false,
    );

    assert_eq!(state.current_page_index, EXPECTED_LAST_PAGE_INDEX);
    assert_exact_two_page_max_index_matches_katana(viewport);

    state = ViewerStateEngine::apply_slideshow_command(state, SlideshowCommand::PreviousPage);
    assert_eq!(state.current_page_index, EXPECTED_PREVIOUS_PAGE_INDEX);

    state = ViewerStateEngine::apply_slideshow_command(state, SlideshowCommand::NextPage);
    state = ViewerStateEngine::apply_slideshow_command(state, SlideshowCommand::NextPage);
    state = ViewerStateEngine::apply_slideshow_command(state, SlideshowCommand::NextPage);
    assert_eq!(state.current_page_index, EXPECTED_LAST_PAGE_INDEX);
}

fn assert_exact_two_page_max_index_matches_katana(viewport: ViewerViewport) {
    let state = ViewerStateEngine::slideshow_state(
        viewport,
        EXACT_TWO_PAGE_CONTENT_HEIGHT,
        OUT_OF_RANGE_PAGE_INDEX,
        false,
        false,
    );
    assert_eq!(state.max_page_index, EXPECTED_EXACT_TWO_PAGE_MAX_INDEX);
}

#[test]
fn slideshow_settings_and_control_visibility_are_stateful() {
    let mut state =
        ViewerStateEngine::slideshow_state(sample_viewport(), TALL_CONTENT_HEIGHT, 0, false, false);
    state = ViewerStateEngine::apply_slideshow_command(
        state,
        SlideshowCommand::UpdateSettings(SlideshowSettingsUpdate {
            hover_highlight_enabled: true,
            diagram_controls_enabled: true,
        }),
    );

    assert!(state.hover_highlight_enabled);
    assert!(state.diagram_controls_enabled);

    state = ViewerStateEngine::hide_slideshow_controls(state);
    assert!(!state.controls_visible);
    state = ViewerStateEngine::apply_slideshow_command(state, SlideshowCommand::NextPage);
    assert!(state.controls_visible);
}

#[test]
fn slideshow_snapshot_uses_scroll_position_as_current_page() {
    let mut input = Data::viewer_input("slideshow-rev", sample_viewport());
    input.mode = ViewerMode::Slideshow;

    let state = ViewerStateEngine::snapshot(
        &input,
        TALL_CONTENT_HEIGHT,
        VIEWPORT_HEIGHT * EXPECTED_PREVIOUS_PAGE_INDEX as f32,
    );

    assert_eq!(
        EXPECTED_PREVIOUS_PAGE_INDEX,
        state.slideshow.current_page_index
    );
}

#[test]
fn slideshow_page_index_for_scroll_uses_floor_pages() {
    assert_eq!(
        0,
        ViewerStateEngine::page_index_for_scroll(0.0, VIEWPORT_HEIGHT)
    );
    assert_eq!(
        0,
        ViewerStateEngine::page_index_for_scroll(VIEWPORT_HEIGHT - 1.0, VIEWPORT_HEIGHT)
    );
    assert_eq!(
        EXPECTED_PREVIOUS_PAGE_INDEX,
        ViewerStateEngine::page_index_for_scroll(VIEWPORT_HEIGHT, VIEWPORT_HEIGHT)
    );
}

fn sample_viewport() -> ViewerViewport {
    ViewerViewport {
        width: VIEWPORT_WIDTH,
        height: VIEWPORT_HEIGHT,
    }
}
