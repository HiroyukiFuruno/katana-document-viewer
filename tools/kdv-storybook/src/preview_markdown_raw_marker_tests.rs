use crate::catalog::StorybookFixture;
use crate::preview::PreviewBuilder;
use katana_document_viewer::{ViewerInteractionConfig, ViewerViewport};
use katana_ui_core::render_model::UiNode;
use std::path::PathBuf;

const SAMPLE_BASIC: &str = "katana/sample_basic.md";

#[test]
fn katana_basic_markdown_does_not_leak_raw_block_markers_to_kuc_labels()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = PreviewBuilder::default().build(
        &StorybookFixture {
            label: SAMPLE_BASIC.to_string(),
            path: fixture_path(SAMPLE_BASIC),
        },
        ViewerViewport {
            width: 900.0,
            height: 20_000.0,
        },
        true,
        ViewerInteractionConfig::default(),
    )?;
    let text = visible_text(scene.tree.root());

    for marker in raw_markers() {
        assert!(
            !text.contains(marker),
            "raw marker `{marker}` leaked into KUC visible text"
        );
    }
    Ok(())
}

fn raw_markers() -> &'static [&'static str] {
    &[
        "```",
        "| --- |",
        "| :--- |",
        "| :---: |",
        "[!NOTE]",
        "[!TIP]",
        "[!IMPORTANT]",
        "[!WARNING]",
        "[!CAUTION]",
        "<details",
        "</details>",
        "<summary",
        "</summary>",
    ]
}

fn visible_text(node: &UiNode) -> String {
    let mut text = String::new();
    push_visible_text(&mut text, node);
    text
}

fn push_visible_text(text: &mut String, node: &UiNode) {
    if !node.props().label.is_empty() {
        text.push_str(&node.props().label);
        text.push('\n');
    }
    for span in &node.props().text.spans {
        text.push_str(&span.text);
        text.push('\n');
    }
    for child in node.children() {
        push_visible_text(text, child);
    }
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../assets/fixtures/{path}"))
}
