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
fn katana_sample_viewer_plan_height_matches_export_surface_stack()
-> Result<(), Box<dyn std::error::Error>> {
    let case = SurfaceHeightCase::load()?;

    assert_eq!(
        case.expected_content_height() as f32,
        case.plan.content_height,
        "{}",
        case.plan_height_failure_message()
    );
    Ok(())
}

#[test]
fn katana_sample_consecutive_table_y_matches_export_surface_block()
-> Result<(), Box<dyn std::error::Error>> {
    let case = SurfaceHeightCase::load()?;
    let plan_y = case
        .plan_y_for_source("| Header |\n| --- |\n| Table after list |")
        .ok_or("table after list must be planned")?;
    let surface_y = case
        .surface_y_for_text("Table after list")
        .ok_or("table after list must reach export surface")?;

    assert_eq!(
        surface_y, plan_y,
        "viewer plan y must stay aligned to export surface block y"
    );
    Ok(())
}

#[test]
fn katana_sample_decorated_blockquote_y_matches_export_surface_block()
-> Result<(), Box<dyn std::error::Error>> {
    let case = SurfaceHeightCase::load()?;
    let plan_y = case
        .plan_y_for_source("> **Bold quote**")
        .ok_or("decorated blockquote must be planned")?;
    let surface_y = case
        .surface_y_for_text("Bold quote")
        .ok_or("decorated blockquote must reach export surface")?;

    assert_eq!(
        surface_y, plan_y,
        "decorated blockquote y must stay aligned to export surface block y"
    );
    Ok(())
}

#[test]
fn katana_sample_note_block_y_matches_export_surface_block()
-> Result<(), Box<dyn std::error::Error>> {
    let case = SurfaceHeightCase::load()?;
    let plan_y = case
        .plan_y_for_source("> **Note**")
        .ok_or("legacy note blockquote must be planned")?;
    let surface_y = case
        .surface_y_for_text("Note GitHub")
        .ok_or("legacy note blockquote must reach export surface")?;

    assert_eq!(
        surface_y, plan_y,
        "legacy note blockquote y must stay aligned to export surface block y"
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
