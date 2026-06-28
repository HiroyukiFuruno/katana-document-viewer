use super::super::StorybookWindow;
use crate::args::StorybookArgs;
use crate::catalog::{FixtureCatalog, StorybookFixture};
use crate::media_host_action::StorybookMediaHostAction;
use crate::preview::PreviewBuilder;
use katana_document_viewer::{ViewerMediaControlKind, ViewerTarget};
use katana_ui_core::render_model::{UiHostActionPlan, UiNode};
use std::path::PathBuf;

pub(super) fn storybook(label: &str) -> StorybookWindow {
    StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![StorybookFixture {
                label: label.to_string(),
                path: fixture_path(&format!("assets/fixtures/{label}")),
            }],
        },
        PreviewBuilder::default(),
    )
}

pub(super) fn find_node_state_id(
    storybook: &StorybookWindow,
    style: &str,
    value: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let root = storybook.scene.as_ref().ok_or("scene missing")?.tree.root();
    find_node(root, style, value)
        .map(|node| node.props().state_id.as_str().to_string())
        .ok_or_else(|| std::io::Error::other("node missing").into())
}

pub(super) fn find_node_value(
    storybook: &StorybookWindow,
    state_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let root = storybook.scene.as_ref().ok_or("scene missing")?.tree.root();
    find_state(root, state_id)
        .map(|node| node.props().interaction.value.clone())
        .ok_or_else(|| std::io::Error::other("state node missing").into())
}

pub(super) fn first_target_containing(
    storybook: &StorybookWindow,
    text: &str,
) -> Result<ViewerTarget, Box<dyn std::error::Error>> {
    storybook
        .scene
        .as_ref()
        .ok_or("scene missing")?
        .targets
        .iter()
        .find(|target| target.source.raw.text.contains(text))
        .cloned()
        .ok_or_else(|| std::io::Error::other("target missing").into())
}

pub(super) fn media_control_target(
    storybook: &StorybookWindow,
    kind: ViewerMediaControlKind,
    action: &str,
) -> Result<ViewerTarget, Box<dyn std::error::Error>> {
    let scene = storybook.scene.as_ref().ok_or("scene missing")?;
    let node_id = UiHostActionPlan::collect_from_tree(&scene.tree)
        .into_iter()
        .filter_map(|plan| {
            StorybookMediaHostAction::from_host_action_plan(&plan).and_then(|media_action| {
                let viewer_action = media_action.into_viewer_action();
                if viewer_action.kind == kind && viewer_action.command == action {
                    Some(viewer_action.node_id)
                } else {
                    None
                }
            })
        })
        .find(|candidate_node_id| {
            scene
                .targets
                .iter()
                .any(|target| target.node_id.0 == *candidate_node_id)
        })
        .ok_or_else(|| std::io::Error::other("media control target missing"))?;

    scene
        .targets
        .iter()
        .find(|target| target.node_id.0 == node_id)
        .cloned()
        .ok_or_else(|| std::io::Error::other("media control target missing").into())
}

pub(super) fn media_control_button_label(
    storybook: &StorybookWindow,
    kind: ViewerMediaControlKind,
    action: &str,
    viewer_node_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let scene = storybook.scene.as_ref().ok_or("scene missing")?;
    let button_node_id = UiHostActionPlan::collect_from_tree(&scene.tree)
        .into_iter()
        .find_map(|plan| {
            StorybookMediaHostAction::from_host_action_plan(&plan).and_then(|media_action| {
                let viewer_action = media_action.into_viewer_action();
                if viewer_action.kind == kind
                    && viewer_action.command == action
                    && viewer_action.node_id == viewer_node_id
                {
                    Some(plan.target)
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| std::io::Error::other("media control button missing"))?;
    find_node_id(scene.tree.root(), button_node_id.as_str())
        .map(|node| node.props().label.clone())
        .ok_or_else(|| std::io::Error::other("media control button node missing").into())
}

fn find_node<'a>(node: &'a UiNode, style: &str, value: &str) -> Option<&'a UiNode> {
    if has_style(node, style) && node.props().interaction.value == value {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| find_node(child, style, value))
}

fn find_state<'a>(node: &'a UiNode, state_id: &str) -> Option<&'a UiNode> {
    if node.props().state_id.as_str() == state_id {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| find_state(child, state_id))
}

fn find_node_id<'a>(node: &'a UiNode, node_id: &str) -> Option<&'a UiNode> {
    if node.id().as_str() == node_id {
        return Some(node);
    }
    node.children()
        .iter()
        .find_map(|child| find_node_id(child, node_id))
}

fn has_style(node: &UiNode, style: &str) -> bool {
    node.props()
        .style_classes
        .iter()
        .any(|value| value == style)
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../{path}"))
}
