use crate::export_surface_line::LIST_MARKER_COLUMN_WIDTH;

#[path = "builder_surface_height_test_support.rs"]
mod test_support;
use test_support::SurfaceHeightCase;

#[test]
fn katana_sample_viewer_node_heights_match_export_surface_nodes()
-> Result<(), Box<dyn std::error::Error>> {
    let case = SurfaceHeightCase::load()?;
    let failures = case.node_height_failures();

    assert!(
        failures.is_empty(),
        "viewer node height must match export surface height:\n{}",
        failures.join("\n")
    );
    Ok(())
}

#[test]
fn direct_sample_viewer_node_heights_match_export_surface_nodes()
-> Result<(), Box<dyn std::error::Error>> {
    let case = SurfaceHeightCase::load_direct_sample()?;
    let failures = case.node_height_failures();

    assert!(
        failures.is_empty(),
        "direct sample viewer node height must match export surface height:\n{}",
        failures.join("\n")
    );
    Ok(())
}

#[test]
fn katana_sample_viewer_plan_height_uses_softbreak_semantic_stack()
-> Result<(), Box<dyn std::error::Error>> {
    let case = SurfaceHeightCase::load()?;

    assert_ne!(
        case.expected_content_height() as f32,
        case.plan.content_height,
        "the PDF-oriented physical source-row stack must not define viewer softbreak layout"
    );
    assert_eq!(
        10_540.0,
        case.plan.content_height,
        "soft Markdown rows must not reserve physical source-line height in the viewer plan\n{}",
        case.plan_height_failure_message()
    );
    Ok(())
}

#[test]
fn katana_sample_softbreak_stack_keeps_table_anchor_stable()
-> Result<(), Box<dyn std::error::Error>> {
    let case = SurfaceHeightCase::load()?;
    let plan_y = case
        .plan_y_for_source("| Header |\n| --- |\n| Table after list |")
        .ok_or("table after list must be planned")?;
    let surface_y = case
        .surface_y_for_text("Table after list")
        .ok_or("table after list must reach export surface")?;

    assert_eq!(
        9_660, plan_y,
        "soft Markdown rows must not add phantom source-line height before the table"
    );
    assert!(
        plan_y < surface_y,
        "export block rows are not viewer layout rows"
    );
    Ok(())
}

#[test]
fn katana_sample_softbreak_stack_keeps_blockquote_anchor_stable()
-> Result<(), Box<dyn std::error::Error>> {
    let case = SurfaceHeightCase::load()?;
    let plan_y = case
        .plan_y_for_source("> **Bold quote**")
        .ok_or("decorated blockquote must be planned")?;
    let surface_y = case
        .surface_y_for_text("Bold quote")
        .ok_or("decorated blockquote must reach export surface")?;

    assert_eq!(
        5_074, plan_y,
        "soft Markdown rows must not add phantom source-line height before the blockquote"
    );
    assert!(
        plan_y < surface_y,
        "export block rows are not viewer layout rows"
    );
    Ok(())
}

#[test]
fn katana_sample_softbreak_stack_keeps_note_anchor_stable() -> Result<(), Box<dyn std::error::Error>>
{
    let case = SurfaceHeightCase::load()?;
    let plan_y = case
        .plan_y_for_source("> **Note**")
        .ok_or("legacy note blockquote must be planned")?;
    let surface_y = case
        .surface_y_for_text("Note GitHub")
        .ok_or("legacy note blockquote must reach export surface")?;

    assert_eq!(
        5_300, plan_y,
        "soft Markdown rows must not add phantom source-line height before the note"
    );
    assert!(
        plan_y < surface_y,
        "export block rows are not viewer layout rows"
    );
    Ok(())
}

#[test]
fn katana_sample_consecutive_code_block_uses_export_surface_height()
-> Result<(), Box<dyn std::error::Error>> {
    let case = SurfaceHeightCase::load()?;
    let height = case
        .plan_height_for_source("let code = \"directly after quote\";")
        .ok_or("code before table must be planned")?;

    assert_eq!(59, height);
    Ok(())
}

#[test]
fn katana_sample_empty_code_block_uses_export_surface_height()
-> Result<(), Box<dyn std::error::Error>> {
    let case = SurfaceHeightCase::load()?;
    let height = case
        .plan_height_for_source("```empty\n```")
        .ok_or("empty code block must be planned")?;

    assert_eq!(84, height);
    Ok(())
}

#[test]
fn katana_sample_consecutive_list_to_table_gap_is_planned() -> Result<(), Box<dyn std::error::Error>>
{
    let case = SurfaceHeightCase::load()?;
    let list_y = case
        .plan_y_for_source("- A list item directly after code block")
        .ok_or("list before table must be planned")?;
    let list_height = case
        .plan_height_for_source("- A list item directly after code block")
        .ok_or("list before table height must be planned")?;
    let table_y = case
        .plan_y_for_source("| Header |\n| --- |\n| Table after list |")
        .ok_or("table after list must be planned")?;

    assert_eq!(0, table_y - (list_y + list_height));
    Ok(())
}

#[test]
fn katana_sample_code_blocks_inside_lists_keep_export_surface_indent()
-> Result<(), Box<dyn std::error::Error>> {
    let case = SurfaceHeightCase::load()?;
    let nested_code = case.nested_code_node()?;

    assert_eq!(
        LIST_MARKER_COLUMN_WIDTH as f32, nested_code.rect.x,
        "source={:?} line_range={:?}",
        nested_code.source.raw.text, nested_code.source.line_column_range
    );
    assert_eq!(
        case.surface_width() - LIST_MARKER_COLUMN_WIDTH as f32,
        nested_code.rect.width
    );
    Ok(())
}
