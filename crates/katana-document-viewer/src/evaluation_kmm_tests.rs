use katana_markdown_model::{
    CodeBlockRole, DiagramKind, HtmlBlockRole, KatanaMarkdownModel, KmmDocument, KmmError,
    KmmNodeKind, MarkdownInput,
};

#[test]
fn commonmark_fixture_maps_supported_blocks_to_kmm_dto() -> Result<(), KmmError> {
    let document = parse_fixture(
        "sample_basic.md",
        include_str!("../../../assets/fixtures/katana/sample_basic.md"),
    )?;

    assert!(has_kind(&document, |kind| matches!(
        kind,
        KmmNodeKind::Heading(_)
    )));
    assert!(has_kind(&document, |kind| matches!(
        kind,
        KmmNodeKind::Paragraph
    )));
    assert!(has_kind(&document, |kind| matches!(
        kind,
        KmmNodeKind::BlockQuote
    )));
    assert!(has_kind(&document, |kind| matches!(
        kind,
        KmmNodeKind::List(_)
    )));
    assert!(has_plain_code_block(&document));
    assert!(has_kind(&document, |kind| {
        matches!(kind, KmmNodeKind::ThematicBreak)
    }));
    Ok(())
}

#[test]
fn gfm_fixture_maps_alert_table_and_task_markers_to_kmm_dto() -> Result<(), KmmError> {
    let document = parse_fixture(
        "sample_basic.md",
        include_str!("../../../assets/fixtures/katana/sample_basic.md"),
    )?;

    assert!(has_alert(&document, "NOTE"));
    assert!(has_alert(&document, "WARNING"));
    assert!(has_kind(&document, |kind| matches!(
        kind,
        KmmNodeKind::Table(_)
    )));
    assert!(has_task_marker(&document, "[x]"));
    assert!(has_task_marker(&document, "[-]"));
    Ok(())
}

#[test]
fn katana_fixture_maps_original_compatibility_to_kmm_dto() -> Result<(), KmmError> {
    let document = parse_fixture(
        "sample.ja.md",
        include_str!("../../../assets/fixtures/katana/sample.ja.md"),
    )?;

    assert!(has_html_role(&document, HtmlBlockRole::Centered));
    assert!(has_html_role(&document, HtmlBlockRole::BadgeRow));
    assert!(has_alert(&document, "NOTE"));
    assert!(has_task_marker(&document, "[-]"));
    assert!(has_diagram(&document, DiagramKind::DrawIo));
    assert!(has_raw_containing(&document, "日本語"));
    Ok(())
}

#[test]
fn math_and_external_fixtures_expose_supported_kmm_boundaries() -> Result<(), KmmError> {
    let math = parse_fixture(
        "sample_basic.md",
        include_str!("../../../assets/fixtures/katana/sample_basic.md"),
    )?;
    let success = parse_fixture(
        "sample.ja.md",
        include_str!("../../../assets/fixtures/katana/sample.ja.md"),
    )?;
    let failure = parse_fixture(
        "sample.ja.md",
        include_str!("../../../assets/fixtures/katana/sample.ja.md"),
    )?;

    assert!(has_math_block(&math));
    assert!(has_diagram(&success, DiagramKind::Mermaid));
    assert!(has_diagram(&success, DiagramKind::DrawIo));
    assert!(has_diagram(&failure, DiagramKind::PlantUml));
    Ok(())
}

fn parse_fixture(path: &str, content: &str) -> Result<KmmDocument, KmmError> {
    KatanaMarkdownModel::parse(MarkdownInput::from_content(path, content))
}

fn has_kind(document: &KmmDocument, predicate: impl Fn(&KmmNodeKind) -> bool) -> bool {
    document
        .nodes
        .iter()
        .any(|node| node_has_kind(node, &predicate))
}

fn node_has_kind(
    node: &katana_markdown_model::KmmNode,
    predicate: &impl Fn(&KmmNodeKind) -> bool,
) -> bool {
    predicate(&node.kind)
        || node
            .children
            .iter()
            .any(|child| node_has_kind(child, predicate))
}

fn has_alert(document: &KmmDocument, expected: &str) -> bool {
    has_kind(
        document,
        |kind| matches!(kind, KmmNodeKind::Alert { label } if label == expected),
    )
}

fn has_html_role(document: &KmmDocument, expected: HtmlBlockRole) -> bool {
    has_kind(
        document,
        |kind| matches!(kind, KmmNodeKind::HtmlBlock(role) if *role == expected),
    )
}

fn has_task_marker(document: &KmmDocument, expected: &str) -> bool {
    has_kind(document, |kind| {
        matches!(kind, KmmNodeKind::List(list) if list.task_markers.iter().any(|it| it == expected)
                || list.items.iter().any(|item| item.task_marker.as_deref() == Some(expected)))
    })
}

fn has_diagram(document: &KmmDocument, expected: DiagramKind) -> bool {
    has_kind(
        document,
        |kind| matches!(kind, KmmNodeKind::CodeBlock(CodeBlockRole::Diagram { kind }) if *kind == expected),
    )
}

fn has_math_block(document: &KmmDocument) -> bool {
    has_kind(document, |kind| {
        matches!(kind, KmmNodeKind::CodeBlock(CodeBlockRole::Math))
    })
}

fn has_plain_code_block(document: &KmmDocument) -> bool {
    has_kind(document, |kind| {
        matches!(kind, KmmNodeKind::CodeBlock(CodeBlockRole::Plain { .. }))
    })
}

fn has_raw_containing(document: &KmmDocument, expected: &str) -> bool {
    document
        .nodes
        .iter()
        .any(|node| node_has_raw_containing(node, expected))
}

fn node_has_raw_containing(node: &katana_markdown_model::KmmNode, expected: &str) -> bool {
    node.source.raw.text.contains(expected)
        || node
            .children
            .iter()
            .any(|child| node_has_raw_containing(child, expected))
}
