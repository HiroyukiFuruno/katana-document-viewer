use super::super::{ViewerDiagramKind, ViewerHtmlRole, ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::image;
use katana_markdown_model::{
    CodeBlockRole, DescriptionItem, DiagramKind, DollarMathBlockNode, FootnoteDefinitionNode,
    HtmlBlockRole, KmmNodeKind, TextSpan,
};

#[test]
fn node_kind_covers_block_variants() {
    let cases = block_variant_cases();
    for (kind, expected) in cases {
        assert_eq!(expected, ViewerNodeClassifier::node_kind(&kind));
    }
}

fn block_variant_cases() -> Vec<(KmmNodeKind, Option<ViewerNodeKind>)> {
    vec![
        (dollar_math_kind(), Some(ViewerNodeKind::Math)),
        (KmmNodeKind::BlockQuote, Some(ViewerNodeKind::BlockQuote)),
        (
            KmmNodeKind::Alert {
                label: "NOTE".to_string(),
            },
            Some(ViewerNodeKind::Alert {
                label: "NOTE".to_string(),
            }),
        ),
        (description_list_kind(), Some(ViewerNodeKind::List)),
        (
            KmmNodeKind::Image(image("alt")),
            Some(ViewerNodeKind::Image),
        ),
        (raw_kind(), Some(ViewerNodeKind::Raw)),
        (
            footnote_definition_kind(),
            Some(ViewerNodeKind::FootnoteDefinition {
                label: "a".to_string(),
            }),
        ),
        (KmmNodeKind::ThematicBreak, Some(ViewerNodeKind::Rule)),
        (text_kind(), None),
    ]
}

fn description_list_kind() -> KmmNodeKind {
    KmmNodeKind::DescriptionList {
        items: vec![DescriptionItem {
            term: "Term".to_string(),
            description: "Definition".to_string(),
        }],
    }
}

#[test]
fn node_kind_maps_code_diagram_and_html_roles() {
    let cases = vec![
        (
            diagram_code_kind(DiagramKind::DrawIo),
            diagram_node(ViewerDiagramKind::DrawIo),
        ),
        (
            diagram_code_kind(DiagramKind::PlantUml),
            diagram_node(ViewerDiagramKind::PlantUml),
        ),
        (
            KmmNodeKind::CodeBlock(CodeBlockRole::Math),
            Some(ViewerNodeKind::Math),
        ),
    ];

    for (kind, expected) in cases {
        assert_eq!(expected, ViewerNodeClassifier::node_kind(&kind));
    }
}

#[test]
fn node_kind_maps_html_roles() {
    let cases = vec![
        (
            html_kind(HtmlBlockRole::Centered),
            html_node(ViewerHtmlRole::Centered),
        ),
        (
            html_kind(HtmlBlockRole::Generic),
            html_node(ViewerHtmlRole::Generic),
        ),
        (
            html_kind(HtmlBlockRole::BadgeRow),
            html_node(ViewerHtmlRole::BadgeRow),
        ),
    ];

    for (kind, expected) in cases {
        assert_eq!(expected, ViewerNodeClassifier::node_kind(&kind));
    }
}

fn dollar_math_kind() -> KmmNodeKind {
    KmmNodeKind::DollarMathBlock(DollarMathBlockNode {
        expression: "x".to_string(),
    })
}

fn raw_kind() -> KmmNodeKind {
    KmmNodeKind::RawBlock {
        reason: "unsupported".to_string(),
    }
}

fn footnote_definition_kind() -> KmmNodeKind {
    KmmNodeKind::FootnoteDefinition(FootnoteDefinitionNode {
        label: "a".to_string(),
        text: "footnote".to_string(),
    })
}

fn text_kind() -> KmmNodeKind {
    KmmNodeKind::Text(TextSpan {
        text: "inline".to_string(),
    })
}

fn diagram_code_kind(kind: DiagramKind) -> KmmNodeKind {
    KmmNodeKind::CodeBlock(CodeBlockRole::Diagram { kind })
}

fn html_kind(role: HtmlBlockRole) -> KmmNodeKind {
    KmmNodeKind::HtmlBlock(role)
}

fn diagram_node(kind: ViewerDiagramKind) -> Option<ViewerNodeKind> {
    Some(ViewerNodeKind::Diagram { kind })
}

fn html_node(role: ViewerHtmlRole) -> Option<ViewerNodeKind> {
    Some(ViewerNodeKind::Html { role })
}
