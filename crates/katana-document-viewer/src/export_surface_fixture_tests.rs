use super::test_support::{assert_not_contains_any, graph_from_markdown, surface_text};

#[test]
fn sample_fixture_surface_does_not_leak_raw_markup_or_diagram_source()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/rendering/sample.ja.md");
    let markdown = std::fs::read_to_string(&fixture)?;
    let joined = surface_text(&graph_from_markdown(
        &fixture.display().to_string(),
        markdown,
    )?);

    assert_not_contains_any(
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
