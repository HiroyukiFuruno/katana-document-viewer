use super::super::{ViewerNodeClassifier, ViewerNodeKind};
use super::test_support::{list_item, node, text_node};
use katana_markdown_model::{KmmNodeKind, ListNode};

#[test]
fn list_text_preserves_nested_list_children_with_depth() {
    let parent = list_with_nested_task();
    let node = node(
        KmmNodeKind::List(parent),
        "- parent\n  - [/] child",
        Vec::new(),
    );

    assert_eq!(
        "- parent\n  [/] child",
        ViewerNodeClassifier::node_text(&node, &ViewerNodeKind::List)
    );
}

fn list_with_nested_task() -> ListNode {
    let child = node(
        KmmNodeKind::List(ListNode {
            ordered: false,
            task_markers: vec!["[/]".to_string()],
            items: vec![list_item(None, Some("[/]"), vec![text_node("child")])],
        }),
        "  - [/] child",
        Vec::new(),
    );
    let mut parent_item = list_item(None, None, vec![text_node("parent")]);
    parent_item.children.push(child);
    ListNode {
        ordered: false,
        task_markers: Vec::new(),
        items: vec![parent_item],
    }
}
