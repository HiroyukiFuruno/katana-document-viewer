use crate::export_assets::ExportAssetResolver;
use katana_markdown_model::{ImageNode, KmmNode, KmmNodeKind};

use super::super::{SurfaceBlock, SurfaceImageBlock};
use super::{BuildGraph, SurfaceBlockFactory};

impl SurfaceBlockFactory {
    pub(super) fn append_standalone_image(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
    ) -> bool {
        let Some(image) = Self::standalone_image_node(node) else {
            return false;
        };
        let Some(path) =
            ExportAssetResolver::resolve_file_path(&graph.snapshot.source_uri, &image.src)
        else {
            return false;
        };
        let Some(block) = SurfaceImageBlock::from_path(&path, None, image.alt.clone()) else {
            return false;
        };
        blocks.push(SurfaceBlock::Image(block));
        true
    }

    fn standalone_image_node(node: &KmmNode) -> Option<&ImageNode> {
        if node.children.len() != 1 {
            return None;
        }
        let image = node.children.first()?;
        match &image.kind {
            KmmNodeKind::Image(image) => Some(image),
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "image_tests.rs"]
mod tests;
