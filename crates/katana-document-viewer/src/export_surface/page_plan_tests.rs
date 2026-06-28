use super::*;

#[test]
fn page_plan_splits_explicit_surface_blocks() {
    let blocks = std::iter::repeat_with(|| SurfaceBlock::Rule)
        .take(50)
        .collect::<Vec<_>>();
    let plan = SurfacePagePlan::from_blocks(&blocks);

    let expected_first_page_blocks =
        (SurfacePagePlan::page_content_height() / SurfaceBlock::Rule.height()) as usize;
    assert_eq!(expected_first_page_blocks, plan.pages[0].len());
}
