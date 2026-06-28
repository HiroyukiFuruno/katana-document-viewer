use crate::preview_interaction_command_support::build_scene;
use katana_document_viewer::{ViewerCommand, ViewerCommandFactory, ViewerInteractionConfig};
use katana_ui_core::render_model::{UiHostActionKind, UiHostActionPlan, UiTextSpanAction};
use std::collections::BTreeSet;

#[test]
fn preview_interaction_command_metadata_links_resolve_real_viewer_targets()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = build_scene("katana/sample_basic.md", ViewerInteractionConfig::default())?;
    let actions = UiHostActionPlan::collect_from_tree(&scene.tree)
        .into_iter()
        .filter(|action| {
            matches!(
                action.text_span_action(),
                Some(UiTextSpanAction::OpenLink { .. })
            )
        })
        .collect::<Vec<_>>();

    assert!(!actions.is_empty());
    let mut payloads = BTreeSet::new();
    for action in actions {
        payloads.insert(assert_link_action(&scene.targets, action)?);
    }
    assert!(
        payloads.contains("https://github.com"),
        "payloads: {payloads:#?}"
    );
    assert!(
        payloads.contains("mailto:test@example.com"),
        "payloads: {payloads:#?}"
    );
    Ok(())
}

fn assert_link_action(
    targets: &[katana_document_viewer::ViewerTarget],
    action: UiHostActionPlan,
) -> Result<String, Box<dyn std::error::Error>> {
    assert_eq!(UiHostActionKind::Navigation, action.kind);
    assert!(action.enabled);
    assert!(!action.label.trim().is_empty());
    let UiTextSpanAction::OpenLink { target: uri } = action
        .text_span_action()
        .ok_or_else(|| std::io::Error::other("missing typed link action"))?
    else {
        return Err(std::io::Error::other("expected link action").into());
    };
    assert!(!uri.trim().is_empty());
    assert_eq!(action.payload, uri);
    let target = targets
        .iter()
        .find(|target| target.node_id.0 == action.target.as_str())
        .ok_or_else(|| {
            std::io::Error::other(format!("missing target: {}", action.target.as_str()))
        })?;
    assert_ne!("storybook-interaction-node", target.node_id.0);
    assert!(!target.source.raw.text.trim().is_empty());
    assert!(target.rect.width > 0.0);
    assert!(target.rect.height > 0.0);
    let command = ViewerCommandFactory::open_link(target.clone(), uri.clone());
    let ViewerCommand::Link(link) = command else {
        return Err(std::io::Error::other("expected link command").into());
    };
    assert_eq!(*target, link.target);
    assert_eq!(uri, link.uri);
    Ok(link.uri)
}
