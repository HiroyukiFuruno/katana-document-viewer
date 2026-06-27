pub(super) use super::{FrameRenderRequest, StorybookFrameRenderer};

#[path = "frame_surface_dump.rs"]
mod frame_surface_dump;
#[path = "frame_surface_similarity.rs"]
mod frame_surface_similarity;
#[path = "frame_hover/matrix_tests.rs"]
mod hover_matrix_tests;
#[path = "frame_html_alignment_tests.rs"]
mod html_alignment_tests;
#[path = "frame_interaction_tests.rs"]
mod interaction_tests;
#[path = "frame_media_control_tests.rs"]
mod media_control_tests;
#[path = "frame_performance_tests.rs"]
mod performance_tests;
#[path = "frame_role_pixel_tests.rs"]
mod role_pixel_tests;
#[path = "frame_score_preview_crop_tests.rs"]
mod score_preview_crop_tests;
#[path = "frame_score_visual_tests.rs"]
mod score_visual_tests;
#[path = "frame_scroll_tests.rs"]
mod scroll_tests;
#[path = "frame_sidebar_tests.rs"]
mod sidebar_tests;
#[path = "frame_sidebar/tree_visual_tests.rs"]
mod sidebar_tree_visual_tests;
#[path = "frame_surface_parity_tests.rs"]
mod surface_parity_tests;
#[path = "frame_task_state_tests.rs"]
mod task_state_tests;
#[path = "frame_tests.rs"]
mod tests;
