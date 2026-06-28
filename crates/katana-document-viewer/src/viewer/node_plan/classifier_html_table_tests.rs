use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::node;
use katana_markdown_model::{HtmlBlockRole, KmmNodeKind};

#[test]
fn node_kind_for_node_promotes_html_table_to_table() {
    let table = node(
        html_kind(),
        "<table><tr><td>A</td></tr></table>",
        Vec::new(),
    );

    assert_eq!(
        Some(ViewerNodeKind::Table),
        ViewerNodeClassifier::node_kind_for_node(&table)
    );
}

fn html_kind() -> KmmNodeKind {
    KmmNodeKind::HtmlBlock(HtmlBlockRole::Generic)
}
