use super::{BuildGraph, SurfaceBlockFactory, SurfaceHtmlMarkup};
use crate::export_assets::ExportAssetResolver;
use crate::export_surface::SurfaceImageBlock;

impl SurfaceBlockFactory {
    pub(super) fn local_html_image_block(
        graph: &BuildGraph,
        fragment: &str,
    ) -> Option<SurfaceImageBlock> {
        let image = SurfaceHtmlMarkup::extract_img_refs(fragment)
            .into_iter()
            .find(|image| {
                ExportAssetResolver::resolve_file_path(&graph.snapshot.source_uri, &image.src)
                    .is_some_and(|path| path.exists())
            })?;
        let path = ExportAssetResolver::resolve_file_path(&graph.snapshot.source_uri, &image.src)?;
        SurfaceImageBlock::from_path(&path, image.width, image.alt)
    }

    pub(super) fn data_html_image_block(fragment: &str) -> Option<SurfaceImageBlock> {
        let image = SurfaceHtmlMarkup::extract_img_refs(fragment)
            .into_iter()
            .find(|image| image.src.starts_with("data:image/"))?;
        SurfaceImageBlock::from_data_uri(&image.src, image.width, image.alt)
    }
}
