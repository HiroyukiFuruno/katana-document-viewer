use super::KucNodeFactory;
use super::node_factory_tests_support::{has_style_class, viewer_node};
use katana_document_viewer::{ViewerDiagramKind, ViewerHtmlRole, ViewerNodeKind};
use katana_ui_core::render_model::{UiDimension, UiNodeKind, UiTone};

#[test]
fn text_roles_cover_html_nodes() {
    let factory = KucNodeFactory::new(&[], 120);
    assert_html_text_role(
        &factory,
        ViewerHtmlRole::Centered,
        "html-centered-preview",
        "center",
    );
    assert_html_text_role(
        &factory,
        ViewerHtmlRole::Generic,
        "html-block-preview",
        "html",
    );
    assert_html_text_role(
        &factory,
        ViewerHtmlRole::Right,
        "html-right-preview",
        "right",
    );
    assert_html_text_role(&factory, ViewerHtmlRole::Left, "html-left-preview", "left");
}

#[test]
fn text_roles_cover_table_list_and_rule_nodes() {
    let factory = KucNodeFactory::new(&[], 120);

    assert_text_role(&factory, ViewerNodeKind::Table, "table", "A | B");
    assert_eq!(
        UiNodeKind::Column,
        factory
            .viewer_node(&viewer_node(ViewerNodeKind::List, "- item"))
            .kind()
    );
    let mut rule_node = viewer_node(ViewerNodeKind::Rule, "");
    rule_node.rect.height = 34.0;
    let rule = factory.viewer_node(&rule_node);
    assert_eq!(UiNodeKind::Divider, rule.kind());
    assert!(!has_style_class(&rule, "kdv-document-rule"));
    assert_eq!(UiDimension::Px(120), rule.props().common.width);
    assert_eq!(UiDimension::Px(34), rule.props().common.height);
    assert_eq!(UiDimension::Px(0), rule.props().common.padding.top);
    assert!(rule.props().common.border.visible);
    assert_eq!(2, rule.props().common.border.width_px);
}

#[test]
fn rule_node_uses_typed_line_offset_hint() {
    let factory = KucNodeFactory::new(&[], 120);
    let mut rule_node = viewer_node(ViewerNodeKind::Rule, "");
    rule_node.rect.height = 34.0;
    rule_node.rule_line_offset_px = 9;

    let rule = factory.viewer_node(&rule_node);

    assert_eq!(UiNodeKind::Divider, rule.kind());
    assert_eq!(UiDimension::Px(9), rule.props().common.padding.top);
}

#[test]
fn text_roles_cover_alert_and_footnote_nodes() {
    let factory = KucNodeFactory::new(&[], 120);

    let footnote = factory.viewer_node(&viewer_node(
        ViewerNodeKind::FootnoteDefinition {
            label: "note".to_string(),
        },
        "note. body ↩",
    ));
    assert_eq!(UiNodeKind::Row, footnote.kind());
    assert_eq!("list-marker", footnote.children()[0].props().text.role);
    assert_eq!("footnote", footnote.children()[1].props().text.role);
    assert_eq!("body ↩", footnote.children()[1].props().label);
    let alert = factory.viewer_node(&viewer_node(
        ViewerNodeKind::Alert {
            label: "TIP".to_string(),
        },
        "TIP: body",
    ));
    assert_eq!("alert", alert.props().text.role);
    assert_eq!("Tip\nbody", alert.props().label);
    assert!(!has_style_class(&alert, "kdv-alert-tip"));
    assert_eq!(UiTone::Success, alert.props().severity);
    assert_eq!("alert-tip", alert.props().common.border.color_token);
}

#[test]
fn alert_nodes_normalize_gfm_kind_labels() {
    let factory = KucNodeFactory::new(&[], 120);

    assert_alert_contract(
        &factory,
        "note",
        "note: body",
        "Note\nbody",
        "alert-note",
        UiTone::Accent,
    );
    assert_alert_contract(
        &factory,
        "tip",
        "TIP: body",
        "Tip\nbody",
        "alert-tip",
        UiTone::Success,
    );
    assert_alert_contract(
        &factory,
        "important",
        "important: body",
        "Important\nbody",
        "alert-important",
        UiTone::Accent,
    );
    assert_alert_contract(
        &factory,
        "warning",
        "Warning: body",
        "Warning\nbody",
        "alert-warning",
        UiTone::Warning,
    );
    assert_alert_contract(
        &factory,
        "caution",
        "CAUTION: body",
        "Caution\nbody",
        "alert-caution",
        UiTone::Danger,
    );
}

#[test]
fn markdown_blocks_use_viewer_row_width_for_resize_and_hit_alignment() {
    let factory = KucNodeFactory::new(&[], 320);
    let body = factory.viewer_node(&viewer_node(ViewerNodeKind::Paragraph, "body"));
    let table = factory.viewer_node(&viewer_node(ViewerNodeKind::Table, "A | B\n1 | 2"));
    let mut rule_node = viewer_node(ViewerNodeKind::Rule, "");
    rule_node.rect.height = 34.0;
    let rule = factory.viewer_node(&rule_node);
    let diagram = factory.viewer_node(&viewer_node(
        ViewerNodeKind::Diagram {
            kind: ViewerDiagramKind::Mermaid,
        },
        "graph TD; A-->B",
    ));
    let centered = factory.viewer_node(&viewer_node(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Centered,
        },
        "center",
    ));
    let right = factory.viewer_node(&viewer_node(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Right,
        },
        "right",
    ));
    let left = factory.viewer_node(&viewer_node(
        ViewerNodeKind::Html {
            role: ViewerHtmlRole::Left,
        },
        "left",
    ));

    assert_eq!(UiDimension::Px(320), body.props().common.width);
    assert_eq!(UiDimension::Px(320), table.props().common.width);
    assert_eq!(UiDimension::Px(320), rule.props().common.width);
    assert_eq!(UiDimension::Px(320), diagram.props().common.width);
    assert_eq!(UiDimension::Px(320), centered.props().common.width);
    assert_eq!(UiDimension::Px(320), right.props().common.width);
    assert_eq!(UiDimension::Px(320), left.props().common.width);
}

fn assert_text_role(
    factory: &KucNodeFactory<'_>,
    kind: ViewerNodeKind,
    role: &str,
    expected_label: &str,
) {
    let node = viewer_node(kind, expected_label);
    let ui_node = factory.viewer_node(&node);

    assert_eq!(role, ui_node.props().text.role);
    assert_eq!(expected_label, ui_node.props().label);
}

fn assert_html_text_role(
    factory: &KucNodeFactory<'_>,
    html_role: ViewerHtmlRole,
    text_role: &str,
    expected_label: &str,
) {
    assert_text_role(
        factory,
        ViewerNodeKind::Html { role: html_role },
        text_role,
        expected_label,
    );
}

fn assert_alert_contract(
    factory: &KucNodeFactory<'_>,
    kind: &str,
    text: &str,
    expected_label: &str,
    expected_color: &str,
    expected_tone: UiTone,
) {
    let alert = factory.viewer_node(&viewer_node(
        ViewerNodeKind::Alert {
            label: kind.to_string(),
        },
        text,
    ));

    assert_eq!("alert", alert.props().text.role);
    assert_eq!(expected_label, alert.props().label);
    assert_eq!(expected_color, alert.props().common.border.color_token);
    assert_eq!(expected_tone, alert.props().severity);
}
