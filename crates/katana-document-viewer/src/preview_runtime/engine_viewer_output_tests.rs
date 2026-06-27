use super::*;
use crate::PreviewConfig;

#[test]
fn viewer_output_route_does_not_create_vendor_surface() -> Result<(), Box<dyn std::error::Error>> {
    let source = MarkdownSource {
        content: ["# Neutral Viewer", "", "```rust", "fn main() {}", "```"].join("\n"),
        document_id: Some("neutral-viewer.md".to_string()),
    };

    let output = PreviewRenderEngine.render_viewer_output(&source, &PreviewConfig::default())?;

    assert!(output.surface.is_none());
    assert_eq!(
        "Neutral Viewer",
        output.input.snapshot.outline.items[0].text
    );
    assert!(output.content_height > 0.0);
    Ok(())
}
