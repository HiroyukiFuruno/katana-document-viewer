use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

#[test]
fn pdf_surface_uses_material_list_markers_inside_details() -> Result<(), Box<dyn std::error::Error>>
{
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "details.md",
        details_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "刀:[\"indent=0\", \"list-marker=bullet\", \"marker-column=36\", \"marker-paint=material-dot\"]",
            "孫六兼元:[\"indent=1\", \"list-marker=bullet\", \"marker-column=36\", \"marker-paint=material-circle\"]",
        ],
    );
    Ok(())
}

#[test]
fn pdf_surface_uses_material_list_markers_when_item_contains_code_block()
-> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "list-code.md",
        list_with_code_block_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "最初の手順::[\"indent=0\", \"list-marker=bullet\", \"marker-column=36\", \"marker-paint=material-dot\"]",
            "cargo:[\"monospace\", \"color\"]",
        ],
    );
    Ok(())
}

#[test]
fn pdf_surface_marks_links_with_visual_link_style() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "link.md",
        link_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "通常のリンク:[\"underline\", \"color\"]",
            "https://github.com:[\"underline\", \"color\"]",
            "[1]:[\"underline\", \"color\"]",
        ],
    );
    Ok(())
}

fn details_markdown() -> String {
    [
        "<details><summary>詳細を見る</summary>",
        "",
        "- 刀",
        "  - 孫六兼元",
        "  - 菊一文字則宗",
        "",
        "</details>",
    ]
    .join("\n")
}

fn list_with_code_block_markdown() -> String {
    [
        "# list code",
        "",
        "- 最初の手順:",
        "",
        "  ```sh",
        "  cargo build",
        "  ```",
    ]
    .join("\n")
}

fn link_markdown() -> String {
    [
        "# links",
        "",
        "[通常のリンク](https://github.com)",
        "",
        "自動リンク: <https://github.com>",
        "",
        "脚注です[^1]。",
        "",
        "[^1]: 脚注本文。",
    ]
    .join("\n")
}
