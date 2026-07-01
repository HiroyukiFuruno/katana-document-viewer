use crate::export_surface::{
    DocumentSurfaceFactory, test_modules::test_support::SurfaceTestSupport,
};
use crate::export_surface_helpers::SurfaceHelpers;

#[test]
fn pdf_surface_keeps_markdown_inline_semantics() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "inline.md",
        inline_markdown(),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "太字:[\"bold\"]",
            "斜体:[\"italic\"]",
            "取り消し線:[\"strikethrough\"]",
            "下線:[\"underline\"]",
            "code:[\"monospace\", \"inline-code\"]",
            "ハイライト:[\"highlight\"]",
            "太字と:[\"bold\"]",
            "イタリック:[\"bold\", \"italic\"]",
        ],
    );
    SurfaceTestSupport::assert_not_contains_any(&debug, &["**太字**", "*斜体*", "`code`"]);
    Ok(())
}

#[test]
fn pdf_surface_keeps_inline_markup_in_heading() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "heading-inline.md",
        [
            "# inline",
            "",
            "### 1.1 `<h1 align=\"center\">` — *中央*で**見出し**",
        ]
        .join("\n"),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "<h1 align=\"center\">:[\"monospace\", \"inline-code\"",
            "中央:[\"italic\"]",
            "見出し:[\"bold\"]",
        ],
    );
    SurfaceTestSupport::assert_not_contains_any(&debug, &["`", "*"]);
    Ok(())
}

#[test]
fn pdf_surface_keeps_inline_markup_in_list_item() -> Result<(), Box<dyn std::error::Error>> {
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        "list-inline.md",
        ["# inline list", "", "- `インラインコード` と *イタリック*"].join("\n"),
    )?);

    SurfaceTestSupport::assert_contains_all(
        &debug,
        &[
            "インラインコード:[\"monospace\", \"inline-code\", \"indent=0\"",
            "イタリック:[\"italic\", \"indent=0\"",
        ],
    );
    SurfaceTestSupport::assert_not_contains_any(&debug, &["`", "*"]);
    Ok(())
}

#[test]
fn pdf_surface_paints_issue_17_os_dependent_emoji_sequence()
-> Result<(), Box<dyn std::error::Error>> {
    let theme = crate::KdvThemeSnapshot::katana_light();
    let graph =
        SurfaceTestSupport::graph_from_markdown("issue-17-emoji.md", "⭐️ 🧑‍💻 ⚠️ ✅".to_string())?;
    let surface = DocumentSurfaceFactory::create(&graph, &theme);
    let background = SurfaceHelpers::parse_color(&theme.background);
    let painted_pixels = surface
        .image
        .pixels()
        .filter(|pixel| **pixel != background)
        .count();

    assert!(
        painted_pixels > 160,
        "issue #17 emoji sequence must not render as blank PDF/export surface glyphs"
    );
    Ok(())
}

fn inline_markdown() -> String {
    [
        "# inline",
        "",
        "**太字** *斜体* ~~取り消し線~~ <u>下線</u> `code` <mark>ハイライト</mark> **太字と*イタリック*の混在**",
    ]
    .join("\n")
}
