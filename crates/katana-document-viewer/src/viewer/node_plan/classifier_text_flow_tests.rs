use super::super::ViewerNodeClassifier;
use super::test_support::{node, text_node};
use katana_markdown_model::{DescriptionItem, KmmNode, KmmNodeKind};

#[test]
fn helper_text_methods_cover_direct_flow_paths() {
    let block_quote_text: fn(&KmmNode) -> String = ViewerNodeClassifier::block_quote_text;
    let description_text: fn(&[DescriptionItem]) -> String =
        ViewerNodeClassifier::description_list_text;
    let inline_nodes_text: fn(&[KmmNode]) -> String = ViewerNodeClassifier::inline_nodes_text;
    let quote = node(KmmNodeKind::BlockQuote, "> quoted\n> body", Vec::new());
    let descriptions = vec![DescriptionItem {
        term: "Term".to_string(),
        description: "Definition".to_string(),
    }];

    assert_eq!("quoted\nbody", block_quote_text(&quote));
    assert_eq!("Term: Definition", description_text(&descriptions));
    assert_eq!("child", inline_nodes_text(&[text_node("child")]));
}
