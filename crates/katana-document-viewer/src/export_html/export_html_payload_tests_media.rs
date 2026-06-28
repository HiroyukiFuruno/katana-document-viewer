use crate::export_html_payload::HtmlExportPayloadFactory;

#[test]
fn html_export_embeds_rendered_diagram_svg() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "```mermaid\ngraph TD; A-->B\n```\n";
    let mut graph = super::support::build_graph(markdown)?;
    let node_id = graph.snapshot.document.nodes[0].id.0.clone();
    graph = graph.with_rendered_diagrams(vec![crate::RenderedDiagram {
        node_id,
        kind: "mermaid".to_string(),
        svg: "<svg data-test=\"diagram\"></svg>".to_string(),
    }]);
    let bytes = HtmlExportPayloadFactory::create(&graph, &crate::KdvThemeSnapshot::katana_light());
    let html = String::from_utf8(bytes)?;

    assert!(
        html.contains(r#"<figure data-kdv-diagram="mermaid" data-kdv-diagram-theme="light"><svg"#)
    );
    assert!(!html.contains("requires-krr-render"));
    Ok(())
}

#[test]
fn html_export_embeds_rendered_drawio_svg() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "```drawio\n<mxGraphModel />\n```\n";
    let mut graph = super::support::build_graph(markdown)?;
    let node_id = graph.snapshot.document.nodes[0].id.0.clone();
    graph = graph.with_rendered_diagrams(vec![crate::RenderedDiagram {
        node_id,
        kind: "drawio".to_string(),
        svg: "<svg data-test=\"drawio\"></svg>".to_string(),
    }]);
    let bytes = HtmlExportPayloadFactory::create(&graph, &crate::KdvThemeSnapshot::katana_light());
    let html = String::from_utf8(bytes)?;

    assert!(
        html.contains(r#"<figure data-kdv-diagram="drawio" data-kdv-diagram-theme="light"><svg"#)
    );
    assert!(!html.contains("requires-krr-render"));
    Ok(())
}

#[test]
fn html_export_marks_diagram_without_svg_as_runtime_error_raw()
-> Result<(), Box<dyn std::error::Error>> {
    let html = super::support::export_html("```mermaid\ngraph TD; A-->B\n```\n")?;

    assert!(html.contains(r#"data-kdv-diagram="mermaid""#));
    assert!(html.contains(r#"data-kdv-render-runtime="katana-render-runtime""#));
    assert!(html.contains(r#"data-kdv-render-error="diagram-render-missing""#));
    assert!(html.contains("graph TD; A--&gt;B"));
    assert!(!html.contains("requires-krr-render"));
    Ok(())
}

#[test]
fn html_export_preserves_badge_row_html() -> Result<(), Box<dyn std::error::Error>> {
    let markdown =
        "<p align=\"center\">\n  <a href=\"#\"><img src=\"badge.svg\" alt=\"badge\"></a>\n</p>\n";
    let html = super::support::export_html(markdown)?;

    assert!(html.contains(r##"<a href="#"><img src="file:///badge.svg" alt="badge"></a>"##));
    Ok(())
}

#[test]
fn html_export_resolves_relative_image_paths_from_source_markdown_directory()
-> Result<(), Box<dyn std::error::Error>> {
    let markdown = "![icon](assets/icon.png)\n";
    let source_uri = crate::SourceUri("file:///workspace/project/README.md".to_string());
    let graph = super::support::build_graph_with_uri(markdown, source_uri)?;
    let bytes = HtmlExportPayloadFactory::create(&graph, &crate::KdvThemeSnapshot::katana_light());
    let html = String::from_utf8(bytes)?;

    assert!(html.contains(r#"<img src="file:///workspace/project/assets/icon.png" alt="icon">"#));
    Ok(())
}

#[test]
fn html_export_normalizes_katana_svg_data_uri() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = r#"<img src="data:image/svg+xml,%3Csvg xmlns=%22<http://www.w3.org/2000/svg%22>%3C/svg%3E" alt="アイコン">
"#;
    let html = super::support::export_html(markdown)?;

    assert!(html.contains("data:image/svg+xml"));
    assert!(html.contains("xmlns=%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20"));
    assert!(!html.contains("<http://www.w3.org/2000/svg"));
    Ok(())
}

#[test]
fn html_export_keeps_drawio_and_xml_file_refs_as_links() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "See [diagram.drawio](./diagram.drawio) and [diagram.xml](./diagram.xml).\n";
    let html = super::support::export_html(markdown)?;

    assert!(html.contains(r#"<a href="./diagram.drawio">diagram.drawio</a>"#));
    assert!(html.contains(r#"<a href="./diagram.xml">diagram.xml</a>"#));
    assert!(!html.contains("RequiresKdvImplementation"));
    Ok(())
}
