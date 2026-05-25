use crate::export_surface::test_modules::test_support::SurfaceTestSupport;

#[test]
fn sample_fixture_surface_does_not_leak_raw_markup_or_diagram_source()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/rendering/sample.ja.md");
    let markdown = std::fs::read_to_string(&fixture)?;
    let joined = SurfaceTestSupport::surface_text(&SurfaceTestSupport::graph_from_markdown(
        &fixture.display().to_string(),
        markdown,
    )?);

    SurfaceTestSupport::assert_not_contains_any(
        &joined,
        &[
            "```",
            "[!NOTE]",
            "| 機能 |",
            "|---",
            "> 内側の引用",
            "@startuml",
            "@enduml",
        ],
    );
    Ok(())
}

#[test]
fn sample_fixture_surface_accepts_crlf_markdown_input() -> Result<(), Box<dyn std::error::Error>> {
    let fixture =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/rendering/sample.ja.md");
    let markdown = std::fs::read_to_string(&fixture)?.replace('\n', "\r\n");
    let joined = SurfaceTestSupport::surface_text(&SurfaceTestSupport::graph_from_markdown(
        &fixture.display().to_string(),
        markdown,
    )?);

    SurfaceTestSupport::assert_not_contains_any(&joined, &["| 機能 |", "[!NOTE]"]);
    Ok(())
}
