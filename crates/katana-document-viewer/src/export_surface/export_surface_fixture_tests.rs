use crate::RenderedDiagram;
use crate::export_surface::test_modules::test_support::SurfaceTestSupport;
use katana_markdown_model::{CodeBlockRole, KmmNode, KmmNodeKind};

#[test]
fn sample_fixture_surface_does_not_leak_raw_markup_or_diagram_source()
-> Result<(), Box<dyn std::error::Error>> {
    let (fixture, markdown) = sample_fixture()?;
    let mut graph =
        SurfaceTestSupport::graph_from_markdown(&fixture.display().to_string(), markdown)?;
    let rendered_diagrams = rendered_diagrams_for(&graph.snapshot.document.nodes);
    graph = graph.with_rendered_diagrams(rendered_diagrams);
    let joined = SurfaceTestSupport::surface_text(&graph);

    SurfaceTestSupport::assert_not_contains_any(
        &joined,
        &[
            "```",
            "[!NOTE]",
            "| 機能 |",
            "|---",
            "> 内側の引用",
            "<img",
            "data:image/svg+xml",
            "http://www.w3.org/2000/svg",
            "@startuml",
            "@enduml",
        ],
    );
    Ok(())
}

#[test]
fn sample_fixture_surface_renders_readme_header_data_svg_as_image()
-> Result<(), Box<dyn std::error::Error>> {
    let (fixture, markdown) = sample_fixture()?;
    let debug = SurfaceTestSupport::surface_debug(&SurfaceTestSupport::graph_from_markdown(
        &fixture.display().to_string(),
        markdown,
    )?);

    SurfaceTestSupport::assert_contains_all(&debug, &["image:128x128@256x256:アイコン"]);
    SurfaceTestSupport::assert_not_contains_any(
        &debug,
        &["data:image/svg+xml", "<img", "http://www.w3.org/2000/svg"],
    );
    Ok(())
}

#[test]
fn sample_fixture_surface_accepts_crlf_markdown_input() -> Result<(), Box<dyn std::error::Error>> {
    let (fixture, markdown) = sample_fixture()?;
    let markdown = markdown
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace('\n', "\r\n");
    let joined = SurfaceTestSupport::surface_text(&SurfaceTestSupport::graph_from_markdown(
        &fixture.display().to_string(),
        markdown,
    )?);

    SurfaceTestSupport::assert_not_contains_any(&joined, &["| 機能 |", "[!NOTE]"]);
    Ok(())
}

fn sample_fixture() -> Result<(std::path::PathBuf, String), Box<dyn std::error::Error>> {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fixtures/katana/sample.ja.md");
    let markdown = std::fs::read_to_string(&fixture)?;
    Ok((fixture, markdown))
}

fn rendered_diagrams_for(nodes: &[KmmNode]) -> Vec<RenderedDiagram> {
    let mut diagrams = Vec::new();
    collect_rendered_diagrams(nodes, &mut diagrams);
    diagrams
}

fn collect_rendered_diagrams(nodes: &[KmmNode], diagrams: &mut Vec<RenderedDiagram>) {
    for node in nodes {
        if matches!(
            node.kind,
            KmmNodeKind::CodeBlock(CodeBlockRole::Diagram { .. })
        ) {
            diagrams.push(RenderedDiagram {
                node_id: node.id.0.clone(),
                kind: "fixture".to_string(),
                svg: "<svg><text>Rendered diagram</text></svg>".to_string(),
            });
        }
        if let KmmNodeKind::List(list) = &node.kind {
            for item in &list.items {
                collect_rendered_diagrams(&item.children, diagrams);
            }
        }
        collect_rendered_diagrams(&node.children, diagrams);
    }
}
