use super::support::AssetLoaderSupportTestFixtures;
use crate::preview_runtime::asset_loader_support::PreviewAssetLoaderSupport;
use katana_markdown_model::KmmNodeId;

#[test]
fn find_node_reaches_nested_list_node() {
    let nodes = vec![AssetLoaderSupportTestFixtures::list_parent_node()];
    let target = KmmNodeId("list-child".to_string());

    let found = PreviewAssetLoaderSupport::find_node(&nodes, &target);

    assert_eq!(Some(target), found.map(|node| node.id.clone()));
}

#[test]
fn find_node_reaches_root_node() {
    let nodes = vec![AssetLoaderSupportTestFixtures::build_root_node()];

    let found = PreviewAssetLoaderSupport::find_node(&nodes, &KmmNodeId("root-node".to_string()));

    assert_eq!(
        Some(KmmNodeId("root-node".to_string())),
        found.map(|node| node.id.clone())
    );
}

#[test]
fn find_node_reaches_regular_nested_child() {
    let child = AssetLoaderSupportTestFixtures::build_regular_child_node();
    let parent = AssetLoaderSupportTestFixtures::build_regular_parent_node(child);

    assert_eq!(
        Some(KmmNodeId("regular-child".to_string())),
        PreviewAssetLoaderSupport::find_node(&[parent], &KmmNodeId("regular-child".to_string()))
            .map(|node| node.id.clone())
    );
}

#[test]
fn find_node_ignores_different_node_types_without_match() {
    let nodes = vec![AssetLoaderSupportTestFixtures::list_parent_node()];

    assert!(
        PreviewAssetLoaderSupport::find_node(&nodes, &KmmNodeId("missing".to_string())).is_none()
    );
}
