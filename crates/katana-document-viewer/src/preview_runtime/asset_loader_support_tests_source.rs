use super::support::AssetLoaderSupportTestFixtures;
use crate::preview_runtime::asset_loader_support::PreviewAssetLoaderSupport;
use katana_markdown_model::{DiagramKind, DollarMathBlockNode, KmmNode, KmmNodeId, KmmNodeKind};

#[test]
fn diagram_source_keeps_non_fenced_content() {
    let node = KmmNode {
        id: KmmNodeId("diagram-non-fence".to_string()),
        kind: KmmNodeKind::CodeBlock(katana_markdown_model::CodeBlockRole::Diagram {
            kind: katana_markdown_model::DiagramKind::Mermaid,
        }),
        source: AssetLoaderSupportTestFixtures::source_span("diagram\nA --> B"),
        children: Vec::new(),
    };

    assert_eq!(
        Some((
            katana_markdown_model::DiagramKind::Mermaid,
            "diagram\nA --> B".to_string()
        )),
        PreviewAssetLoaderSupport::diagram_source(&node)
    );
}

#[test]
fn diagram_source_strips_fence_wrappers() {
    let node = KmmNode {
        id: KmmNodeId("diagram-fenced".to_string()),
        kind: KmmNodeKind::CodeBlock(katana_markdown_model::CodeBlockRole::Diagram {
            kind: DiagramKind::Mermaid,
        }),
        source: AssetLoaderSupportTestFixtures::source_span("```mermaid\ngraph TD\nA --> B\n```"),
        children: Vec::new(),
    };

    assert_eq!(
        Some((DiagramKind::Mermaid, "graph TD\nA --> B".to_string())),
        PreviewAssetLoaderSupport::diagram_source(&node)
    );
}

#[test]
fn diagram_source_keeps_unclosed_fence_body() {
    let node = KmmNode {
        id: KmmNodeId("diagram-unclosed-fence".to_string()),
        kind: KmmNodeKind::CodeBlock(katana_markdown_model::CodeBlockRole::Diagram {
            kind: DiagramKind::Mermaid,
        }),
        source: AssetLoaderSupportTestFixtures::source_span("```mermaid\ngraph TD\nA --> B"),
        children: Vec::new(),
    };

    assert_eq!(
        Some((
            DiagramKind::Mermaid,
            "```mermaid\ngraph TD\nA --> B".to_string()
        )),
        PreviewAssetLoaderSupport::diagram_source(&node)
    );
}

#[test]
fn math_source_supports_dollar_math_block() {
    let node = KmmNode {
        id: KmmNodeId("dollar-math".to_string()),
        kind: KmmNodeKind::DollarMathBlock(DollarMathBlockNode {
            expression: "x^2 + y^2 = z^2".to_string(),
        }),
        source: AssetLoaderSupportTestFixtures::source_span("E=mc^2"),
        children: Vec::new(),
    };

    assert_eq!(
        Some("x^2 + y^2 = z^2".to_string()),
        PreviewAssetLoaderSupport::math_source(&node)
    );
}
