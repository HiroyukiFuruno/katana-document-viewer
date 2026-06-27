use super::ConsumerApp;
use katana_ui_core::render_model::{UiNodeKind, UiScrollAreaAxis, UiSplitPaneAxis};
use katana_ui_core::widget::molecules::{
    CloseableTabContextCommand, CloseableTabId, CloseableTabStripEvent,
};

#[test]
fn consumer_app_builds_tree_from_kuc_public_api() {
    let app = ConsumerApp::new();
    let tree = app.render();
    let split = &tree.root().children()[0];
    let navigation = &split.children()[0];
    let content = &split.children()[1];

    assert_eq!(UiNodeKind::Panel, tree.root().kind());
    assert_eq!(UiNodeKind::SplitPane, split.kind());
    assert_eq!(UiSplitPaneAxis::Horizontal, split.props().split_pane.axis);
    assert_eq!(UiNodeKind::ScrollArea, navigation.kind());
    assert_eq!(
        UiScrollAreaAxis::Vertical,
        navigation.props().scroll_area.axis
    );
    assert_eq!(UiNodeKind::Column, content.kind());
    assert_eq!(4, content.children().len());
    assert_eq!(UiNodeKind::CloseableTabStrip, content.children()[0].kind());
    assert_eq!(UiNodeKind::Toolbar, content.children()[1].kind());
    assert_eq!(
        UiNodeKind::Input,
        content.children()[1].children()[0].kind()
    );
    assert_eq!(
        UiNodeKind::SearchBox,
        content.children()[1].children()[1].kind()
    );
    assert_eq!(
        UiNodeKind::SelectBox,
        content.children()[1].children()[2].kind()
    );
    assert_eq!(
        UiNodeKind::ComboBox,
        content.children()[1].children()[3].kind()
    );
    assert_eq!(UiNodeKind::TextArea, content.children()[3].kind());
}

#[test]
fn consumer_app_handles_input_textarea_scroll_split_and_tabs() {
    let mut app = ConsumerApp::new();

    assert!(app.set_query("src/main.rs"));
    let callback_log = app.invoke_search_callback("consumer.search.clear");
    assert!(app.set_quick_search("render"));
    let quick_search_log = app.submit_quick_search();
    assert!(app.select_workspace(2));
    assert!(app.filter_symbol("ren"));
    assert!(app.select_symbol(1));
    assert!(app.resize_notes(42, 12));
    assert!(app.select_navigation(2));
    assert!(app.scroll_navigation(0, 96));
    assert!(app.resize_split(10));
    let added = app.add_tab("preview", "Preview");
    let closed = app.close_tab("editor");

    assert_eq!("src/main.rs", app.query());
    assert_eq!("consumer.search.clear", callback_log[0].action);
    assert_eq!("render", app.quick_search_query());
    assert_eq!("search_submitted", quick_search_log[0].action);
    assert_eq!("docs", app.workspace_value());
    assert_eq!("render", app.symbol_value());
    assert_eq!((42, 12), app.notes_resize_delta());
    assert_eq!((0, 96), app.navigation_scroll_offset());
    assert_eq!(36, app.split_ratio());
    assert_eq!(Some(&CloseableTabId::new("preview")), app.active_tab_id());
    assert_eq!(
        vec![CloseableTabStripEvent::TabAdded {
            tab_id: CloseableTabId::new("preview")
        }],
        added
    );
    assert_eq!(
        vec![CloseableTabStripEvent::TabClosed {
            tab_id: CloseableTabId::new("editor")
        }],
        closed
    );
}

#[test]
fn consumer_app_handles_workspace_tab_bulk_actions() {
    let mut right_app = ConsumerApp::new();
    right_app.add_tab("preview", "Preview");
    right_app.add_tab("logs", "Logs");
    let closed_right = right_app.close_tabs_to_right("preview");

    let mut left_app = ConsumerApp::new();
    left_app.add_tab("preview", "Preview");
    left_app.add_tab("logs", "Logs");
    let closed_left = left_app.close_tabs_to_left("logs");

    let mut others_app = ConsumerApp::new();
    others_app.add_tab("preview", "Preview");
    others_app.add_tab("logs", "Logs");
    let closed_others = others_app.close_other_tabs("preview");

    let mut all_app = ConsumerApp::new();
    all_app.add_tab("preview", "Preview");
    let closed_all = all_app.close_all_tabs();

    assert_eq!(
        vec![CloseableTabStripEvent::TabClosed {
            tab_id: CloseableTabId::new("logs")
        }],
        closed_right
    );
    assert_eq!(2, closed_left.len());
    assert_eq!(2, closed_others.len());
    assert_eq!(2, closed_all.len());
    assert_eq!(3, right_app.tab_count());
    assert_eq!(2, left_app.tab_count());
    assert_eq!(2, others_app.tab_count());
    assert_eq!(1, all_app.tab_count());
    assert_eq!(
        Some(&CloseableTabId::new("preview")),
        right_app.active_tab_id()
    );
    assert_eq!(Some(&CloseableTabId::new("logs")), left_app.active_tab_id());
    assert_eq!(
        Some(&CloseableTabId::new("preview")),
        others_app.active_tab_id()
    );
    assert_eq!(Some(&CloseableTabId::new("home")), all_app.active_tab_id());
}

#[test]
fn consumer_app_handles_workspace_tab_context_commands() {
    let mut app = ConsumerApp::new();
    app.add_tab("preview", "Preview");
    app.add_tab("logs", "Logs");

    let closed_others =
        app.apply_tab_context_command("preview", CloseableTabContextCommand::CloseOthers);
    let move_without_target =
        app.apply_tab_context_command("preview", CloseableTabContextCommand::MoveToNewGroup);

    assert_eq!(Some(2), closed_others.as_ref().map(Vec::len));
    assert_eq!(None, move_without_target);
    assert_eq!(2, app.tab_count());
    assert_eq!(Some(&CloseableTabId::new("preview")), app.active_tab_id());
}

#[test]
fn consumer_app_observes_tab_pin_and_group_events() {
    let mut app = ConsumerApp::new();
    app.add_tab("preview", "Preview");

    let pinned = app.pin_tab("preview");
    let unpinned = app.unpin_tab("preview");
    let existing_group = app.move_tab_to_existing_group("preview", "source");
    let new_group = app.move_tab_to_new_group("preview", "review", "Review");

    assert_eq!(
        vec![CloseableTabStripEvent::TabPinChanged {
            tab_id: CloseableTabId::new("preview"),
            pinned: true
        }],
        pinned
    );
    assert_eq!(
        vec![CloseableTabStripEvent::TabPinChanged {
            tab_id: CloseableTabId::new("preview"),
            pinned: false
        }],
        unpinned
    );
    assert_eq!(
        vec![CloseableTabStripEvent::TabGroupChanged {
            tab_id: CloseableTabId::new("preview"),
            group_id: Some("source".into())
        }],
        existing_group
    );
    assert_eq!(
        vec![
            CloseableTabStripEvent::GroupCreated {
                group_id: "review".into()
            },
            CloseableTabStripEvent::TabGroupChanged {
                tab_id: CloseableTabId::new("preview"),
                group_id: Some("review".into())
            }
        ],
        new_group
    );
    assert!(!app.tab_pinned("preview"));
    assert_eq!(Some(&"review".into()), app.tab_group_id("preview"));
}
