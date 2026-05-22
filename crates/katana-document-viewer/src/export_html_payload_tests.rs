use super::*;
use crate::HtmlExportContractMatrix;
use crate::{BuildProfile, BuildRequest, DocumentSnapshotFactory, DocumentSource};
use crate::{RenderedDiagram, SourceKind, SourceRevision, SourceUri};
use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};

#[test]
fn html_export_renders_links_from_kmm_v0_2_dto() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html("- [通常のリンク](https://github.com)\n")?;
    let matrix = HtmlExportContractMatrix::v0_1();

    assert!(matrix.contains_feature("commonmark-link", crate::HtmlExportReadiness::Implemented));
    assert!(html.contains(r#"<a href="https://github.com">通常のリンク</a>"#));
    assert!(!html.contains("[通常のリンク](https://github.com)"));
    Ok(())
}

#[test]
fn html_export_renders_inline_decoration_from_kmm_v0_2_dto()
-> Result<(), Box<dyn std::error::Error>> {
    let html = export_html("- **太字**\n- *斜体*\n- `code`\n")?;
    let matrix = HtmlExportContractMatrix::v0_1();

    for feature in [
        "commonmark-strong",
        "commonmark-emphasis",
        "commonmark-inline-code",
    ] {
        assert!(matrix.contains_feature(feature, crate::HtmlExportReadiness::Implemented));
    }
    assert!(html.contains("<strong>太字</strong>"));
    assert!(html.contains("<em>斜体</em>"));
    assert!(html.contains("<code>code</code>"));
    Ok(())
}

#[test]
fn html_export_strips_markdown_heading_markers() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html("# H1 見出し\n\n### `code` 見出し\n")?;

    assert!(html.contains("<h1>H1 見出し</h1>"));
    assert!(html.contains("<h3><code>code</code> 見出し</h3>"));
    assert!(!html.contains("<h1># "));
    assert!(!html.contains("<h3>###"));
    Ok(())
}

#[test]
fn html_export_strips_fenced_code_markers() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html("```rust\nfn main() {}\n```\n")?;

    assert!(html.contains(
        r#"<pre data-kdv-code-role="plain" data-kdv-code-language="rust" data-kdv-code-highlighter="syntect" data-kdv-syntax-theme="InspiredGitHub">"#
    ));
    assert!(html.contains(r#"<code class="language-rust">"#));
    assert!(html.contains("<span style="));
    assert!(html.contains("main"));
    assert!(!html.contains("```rust"));
    Ok(())
}

#[test]
fn html_export_renders_alert_body_without_marker() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html("> [!NOTE]\n> 本文\n")?;

    assert!(html.contains(r#"<aside data-github-alert="NOTE" data-kdv-blockquote="alert">"#));
    assert!(html.contains(
        r#"<p data-kdv-alert-title="NOTE"><span data-kdv-alert-icon="NOTE" aria-hidden="true"><svg data-kdv-alert-icon-svg="NOTE""#
    ));
    assert!(html.contains("<p>本文</p>"));
    assert!(!html.contains("[!NOTE]"));
    Ok(())
}

#[test]
fn html_export_decodes_html_entities_in_text() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html("- HTML エンティティ: &amp; &lt; &gt; &quot;\n")?;

    assert!(html.contains("HTML エンティティ: &amp; &lt; &gt; &quot;"));
    assert!(!html.contains("&amp;amp;"));
    assert!(!html.contains("&amp;lt;"));
    assert!(!html.contains("&amp;gt;"));
    assert!(!html.contains("&amp;quot;"));
    Ok(())
}

#[test]
fn html_export_preserves_katana_html_block() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html("<p align=\"center\">\n  中央\n</p>\n")?;

    assert!(html.contains(r#"<p align="center">"#));
    assert!(html.contains("中央"));
    Ok(())
}

#[test]
fn html_export_uses_table_header_and_body() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html("| 左 | 右 |\n| :--- | ---: |\n| A | B |\n")?;

    assert!(html.contains("<thead><tr>"));
    assert!(html.contains(r#"<th data-align="left" data-kdv-column-size="short">左</th>"#));
    assert!(html.contains(r#"<td data-align="right" data-kdv-column-size="short">B</td>"#));
    assert!(!html.contains(":---"));
    Ok(())
}

#[test]
fn html_export_renders_fenced_math_with_shared_svg_renderer()
-> Result<(), Box<dyn std::error::Error>> {
    let html = export_html("```math\nE = mc^2\n```\n")?;

    assert!(html.contains(r#"data-kdv-math="block""#));
    assert!(html.contains(r#"data-kdv-render-runtime="katana-render-runtime-stub""#));
    assert!(!html.contains(r#"data-kdv-math-renderer="mathjax-svg""#));
    assert!(html.contains("<svg"));
    assert!(!html.contains(r#"data-kdv-export-readiness="external-backend-required""#));
    assert!(!html.contains("```math"));
    Ok(())
}

#[test]
fn html_export_embeds_rendered_diagram_svg() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "```mermaid\ngraph TD; A-->B\n```\n";
    let mut graph = build_graph(markdown)?;
    let node_id = graph.snapshot.document.nodes[0].id.0.clone();
    graph = graph.with_rendered_diagrams(vec![RenderedDiagram {
        node_id,
        kind: "mermaid".to_string(),
        svg: "<svg data-test=\"diagram\"></svg>".to_string(),
    }]);
    let bytes = HtmlExportPayloadFactory::create(&graph, &crate::KdvThemeSnapshot::katana_light());
    let html = String::from_utf8(bytes)?;

    assert!(
        html.contains(r#"<figure data-kdv-diagram="mermaid" data-kdv-diagram-theme="light"><svg"#)
    );
    assert!(!html.contains("requires-kdr-render"));
    Ok(())
}

#[test]
fn html_export_embeds_rendered_drawio_svg() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "```drawio\n<mxGraphModel />\n```\n";
    let mut graph = build_graph(markdown)?;
    let node_id = graph.snapshot.document.nodes[0].id.0.clone();
    graph = graph.with_rendered_diagrams(vec![RenderedDiagram {
        node_id,
        kind: "drawio".to_string(),
        svg: "<svg data-test=\"drawio\"></svg>".to_string(),
    }]);
    let bytes = HtmlExportPayloadFactory::create(&graph, &crate::KdvThemeSnapshot::katana_light());
    let html = String::from_utf8(bytes)?;

    assert!(
        html.contains(r#"<figure data-kdv-diagram="drawio" data-kdv-diagram-theme="light"><svg"#)
    );
    assert!(!html.contains("requires-kdr-render"));
    Ok(())
}

#[test]
fn html_export_marks_diagram_without_svg_as_kdr_required() -> Result<(), Box<dyn std::error::Error>>
{
    let html = export_html("```mermaid\ngraph TD; A-->B\n```\n")?;

    assert!(html.contains(r#"data-kdv-diagram="mermaid""#));
    assert!(html.contains(r#"data-kdv-export-readiness="requires-kdr-render""#));
    Ok(())
}

#[test]
fn html_export_preserves_badge_row_html() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        "<p align=\"center\">\n  <a href=\"#\"><img src=\"badge.svg\" alt=\"badge\"></a>\n</p>\n",
    )?;

    assert!(html.contains(r##"<a href="#"><img src="badge.svg" alt="badge"></a>"##));
    Ok(())
}

#[test]
fn html_export_normalizes_katana_svg_data_uri() -> Result<(), Box<dyn std::error::Error>> {
    let html = export_html(
        r#"<img src="data:image/svg+xml,%3Csvg xmlns=%22<http://www.w3.org/2000/svg%22>%3C/svg%3E" alt="アイコン">
"#,
    )?;

    assert!(html.contains("data:image/svg+xml"));
    assert!(html.contains("xmlns=%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20"));
    assert!(!html.contains("<http://www.w3.org/2000/svg"));
    Ok(())
}

#[test]
fn html_export_keeps_drawio_and_xml_file_refs_as_links() -> Result<(), Box<dyn std::error::Error>> {
    let html =
        export_html("See [diagram.drawio](./diagram.drawio) and [diagram.xml](./diagram.xml).\n")?;

    assert!(html.contains(r#"<a href="./diagram.drawio">diagram.drawio</a>"#));
    assert!(html.contains(r#"<a href="./diagram.xml">diagram.xml</a>"#));
    assert!(!html.contains("RequiresKdvImplementation"));
    Ok(())
}

fn export_html(markdown: &str) -> Result<String, Box<dyn std::error::Error>> {
    let graph = build_graph(markdown)?;
    let bytes = HtmlExportPayloadFactory::create(&graph, &crate::KdvThemeSnapshot::katana_light());
    Ok(String::from_utf8(bytes)?)
}

fn build_graph(markdown: &str) -> Result<crate::BuildGraph, Box<dyn std::error::Error>> {
    let source = DocumentSource {
        uri: SourceUri("file:///test.md".to_string()),
        kind: SourceKind::Markdown,
        revision: SourceRevision("test".to_string()),
        content: markdown.to_string(),
    };
    let document =
        KatanaMarkdownModel::parse(MarkdownInput::from_content("test.md", markdown.to_string()))?;
    let snapshot = DocumentSnapshotFactory::from_kmm(source, document);
    Ok(crate::BuildGraph::from_request(&BuildRequest {
        snapshot,
        profile: BuildProfile::markdown_export(),
        theme: crate::KdvThemeSnapshot::katana_light(),
    }))
}
