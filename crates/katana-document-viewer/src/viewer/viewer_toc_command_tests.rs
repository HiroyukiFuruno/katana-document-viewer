use super::viewer_test_support::sample_target;
use super::*;

const TOC_LEVEL: u8 = 2;
const TOC_ANCHOR_INDEX: usize = 4;

#[test]
fn toc_click_and_image_actions_return_host_commands() {
    let target = sample_target();
    let toc_command = ViewerTocCommandFactory::scroll_to(toc_item_for_target(&target, 3));
    let image_command = ViewerCommand::Image(ImageControlCommand {
        target,
        action: ImageControlAction::RevealInOs,
    });
    let host_command = ViewerCommand::Host(HostCommand::RevealPath("/tmp/diagram.png".to_string()));

    assert!(matches!(toc_command, ViewerCommand::ScrollToHeading(_)));
    assert!(matches!(
        image_command,
        ViewerCommand::Image(command) if command.action == ImageControlAction::RevealInOs
    ));
    assert!(matches!(
        host_command,
        ViewerCommand::Host(HostCommand::RevealPath(_))
    ));
}

#[test]
fn toc_item_from_outline_preserves_node_source_and_anchor() {
    let target = sample_target();
    let outline = crate::DocumentOutlineItem {
        node_id: target.node_id.clone(),
        level: TOC_LEVEL,
        text: "Diagram".to_string(),
        source: target.source.clone(),
    };

    let item = ViewerTocItem::from_outline_item(outline, target.rect, TOC_ANCHOR_INDEX);

    assert_eq!(item.node_id, target.node_id);
    assert_eq!(item.level, TOC_LEVEL);
    assert_eq!(item.text, "Diagram");
    assert_eq!(item.anchor_rect, target.rect);
    assert_eq!(item.anchor_index, TOC_ANCHOR_INDEX);
}

fn toc_item_for_target(target: &ViewerTarget, anchor_index: usize) -> ViewerTocItem {
    ViewerTocItem {
        node_id: target.node_id.clone(),
        level: 2,
        text: "Diagram".to_string(),
        source: target.source.clone(),
        anchor_rect: target.rect,
        anchor_index,
    }
}
