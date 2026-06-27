use super::runtime_test_support::{CONTENT_HEIGHT, LAST_ANCHOR_Y, RuntimeTestData as Data};
use super::*;
use crate::{DocumentOutline, DocumentOutlineItem};

#[test]
fn toc_items_are_built_from_snapshot_outline_without_markdown_reparse() {
    let target = Data::viewer_target("heading", 120.0);
    let outline = DocumentOutline {
        items: vec![DocumentOutlineItem {
            node_id: target.node_id.clone(),
            level: 2,
            text: "Heading".to_string(),
            source: target.source.clone(),
        }],
    };
    let layout = ViewerLayoutEngine::from_anchors(
        Data::viewport(),
        vec![ViewerRenderedAnchor {
            target,
            anchor_index: 3,
        }],
        CONTENT_HEIGHT,
        0.0,
    );

    let items = ViewerTocModel::from_outline(&outline, &layout);

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].text, "Heading");
    assert_eq!(items[0].anchor_index, 3);
}

#[test]
fn toc_click_scrolls_to_rendered_heading_anchor() {
    let target = Data::viewer_target("last-heading", LAST_ANCHOR_Y);
    let outline = DocumentOutline {
        items: vec![DocumentOutlineItem {
            node_id: target.node_id.clone(),
            level: 1,
            text: "Last".to_string(),
            source: target.source.clone(),
        }],
    };
    let layout = ViewerLayoutEngine::from_anchors(
        Data::viewport(),
        vec![ViewerRenderedAnchor {
            target,
            anchor_index: 0,
        }],
        CONTENT_HEIGHT,
        0.0,
    );
    let items = ViewerTocModel::from_outline(&outline, &layout);

    let scroll_y = ViewerTocModel::scroll_y_for_item(&layout, Data::viewport(), &items[0]);

    assert_eq!(scroll_y, LAST_ANCHOR_Y);
}

#[test]
fn active_heading_comes_from_rendered_anchor_map() {
    let first = ViewerRenderedAnchor {
        target: Data::viewer_target("first", 50.0),
        anchor_index: 0,
    };
    let second = ViewerRenderedAnchor {
        target: Data::viewer_target("second", 150.0),
        anchor_index: 1,
    };
    let layout = ViewerLayoutEngine::from_anchors(
        Data::viewport(),
        vec![first, second.clone()],
        CONTENT_HEIGHT,
        0.0,
    );

    let active = ViewerTocModel::active_anchor(&layout, 180.0);

    assert_eq!(active, Some(second));
}
