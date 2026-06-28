use super::super::ViewerNodeClassifier;
use super::test_support::{cell, list_item, text_node};
use katana_markdown_model::{ListItemNode, ListNode, TableNode, TableRow};

#[test]
fn helper_text_methods_cover_direct_structured_paths() {
    let table_text: fn(&TableNode) -> String = ViewerNodeClassifier::table_text;
    let has_table_separator_row: fn(&TableNode) -> bool =
        ViewerNodeClassifier::has_table_separator_row;
    let list_text: fn(&ListNode) -> String = ViewerNodeClassifier::list_text;
    let list_marker: fn(&ListItemNode, bool) -> String = ViewerNodeClassifier::list_marker;
    let alert_text: fn(&str, &str) -> String = ViewerNodeClassifier::alert_text;
    let footnote_text: fn(&str) -> String = ViewerNodeClassifier::footnote_text;

    assert_eq!("A | B\n1 | 2", table_text(&table_node()));
    assert!(!has_table_separator_row(&table_node()));
    assert!(has_table_separator_row(&gfm_table_node()));
    assert_eq!("3. child\n[/]\n1. default", list_text(&ordered_list_node()));
    let ordered_item = list_item(Some(9), None, Vec::new());
    let unordered_item = list_item(None, None, Vec::new());
    assert_eq!("9.", list_marker(&ordered_item, true));
    assert_eq!("-", list_marker(&unordered_item, false));
    assert_eq!("NOTE: body", alert_text("NOTE", "> [!NOTE]\n> body"));
    assert_eq!("[ref]", footnote_text("ref"));
}

fn gfm_table_node() -> TableNode {
    TableNode {
        alignments: Vec::new(),
        rows: vec![
            TableRow {
                cells: vec![cell("A"), cell("B")],
            },
            TableRow {
                cells: vec![cell("---"), cell("---")],
            },
        ],
    }
}

fn table_node() -> TableNode {
    TableNode {
        alignments: Vec::new(),
        rows: vec![
            TableRow {
                cells: vec![cell("A"), cell("B")],
            },
            TableRow {
                cells: vec![cell("1"), cell("2")],
            },
        ],
    }
}

fn ordered_list_node() -> ListNode {
    ListNode {
        ordered: true,
        task_markers: vec!["[/]".to_string()],
        items: vec![
            list_item(Some(3), None, vec![text_node("child")]),
            list_item(None, Some("[/]"), Vec::new()),
            list_item(None, None, vec![text_node("default")]),
        ],
    }
}
