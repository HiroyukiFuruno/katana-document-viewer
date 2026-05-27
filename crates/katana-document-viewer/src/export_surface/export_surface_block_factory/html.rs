use crate::export_assets::ExportAssetResolver;
use crate::export_semantics::EvaluatedMarkdownFragment;
use crate::export_surface_text::SurfaceTextParser;
use crate::forge::BuildGraph;
use crate::html_sanitizer::HtmlFragmentNormalizer;
use crate::theme::KdvThemeSnapshot;
use katana_markdown_model::{HtmlBlockRole, KmmNode};

use super::super::markup::{SurfaceDetailsParts, SurfaceHtmlMarkup};
use super::super::{SurfaceBadgeRowBlock, SurfaceBlock, SurfaceImageBlock};
use super::SurfaceBlockFactory;
use crate::export_surface_helpers::{BODY_MAX_CHARS, WrappedText};
use crate::export_surface_line::SurfaceLine;

impl SurfaceBlockFactory {
    pub(super) fn append_html(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        node: &KmmNode,
        role: &HtmlBlockRole,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        let fragment = HtmlFragmentNormalizer::normalize(&node.source.raw.text);
        if Self::append_special_html(blocks, graph, &fragment, quote_depth, list_depth, theme) {
            return;
        }
        if matches!(role, HtmlBlockRole::BadgeRow) {
            Self::append_badge_row(blocks, &fragment);
            return;
        }
        let text = Self::normalized_html_text(&fragment);
        if Self::is_centered_html(role, &fragment) {
            Self::append_centered_html(blocks, &fragment, text);
            return;
        }
        Self::append_wrapped(blocks, text, quote_depth, list_depth);
    }

    fn append_special_html(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        fragment: &str,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) -> bool {
        if Self::append_details(blocks, graph, fragment, quote_depth, list_depth, theme) {
            return true;
        }
        if let Some(local_image) = Self::local_html_image_block(graph, fragment) {
            blocks.push(SurfaceBlock::Image(local_image));
            return true;
        }
        if let Some(data_image) = Self::data_html_image_block(fragment) {
            blocks.push(SurfaceBlock::Image(data_image));
            return true;
        }
        false
    }

    fn append_badge_row(blocks: &mut Vec<SurfaceBlock>, fragment: &str) {
        blocks.push(SurfaceBlock::BadgeRow(SurfaceBadgeRowBlock::new(
            SurfaceHtmlMarkup::badge_row_badges(fragment),
        )));
    }

    fn normalized_html_text(fragment: &str) -> String {
        SurfaceHtmlMarkup::normalize_text(&SurfaceTextParser::html_fragment_text(fragment))
    }

    fn is_centered_html(role: &HtmlBlockRole, fragment: &str) -> bool {
        matches!(role, HtmlBlockRole::Centered) || SurfaceHtmlMarkup::has_center_alignment(fragment)
    }

    fn append_centered_html(blocks: &mut Vec<SurfaceBlock>, fragment: &str, text: String) {
        let spans = SurfaceHtmlMarkup::centered_html_spans(fragment);
        if !spans.is_empty() && text.chars().count() <= BODY_MAX_CHARS {
            blocks.push(SurfaceBlock::Line(SurfaceLine::centered_spans(spans)));
            return;
        }
        for chunk in WrappedText::new(&text, BODY_MAX_CHARS) {
            blocks.push(SurfaceBlock::Line(SurfaceLine::body_centered(chunk)));
        }
    }

    fn local_html_image_block(graph: &BuildGraph, fragment: &str) -> Option<SurfaceImageBlock> {
        let image = SurfaceHtmlMarkup::extract_img_refs(fragment)
            .into_iter()
            .find(|image| {
                ExportAssetResolver::resolve_file_path(&graph.snapshot.source_uri, &image.src)
                    .is_some_and(|path| path.exists())
            })?;
        let path = ExportAssetResolver::resolve_file_path(&graph.snapshot.source_uri, &image.src)?;
        SurfaceImageBlock::from_path(&path, image.width, image.alt)
    }

    fn data_html_image_block(fragment: &str) -> Option<SurfaceImageBlock> {
        let image = SurfaceHtmlMarkup::extract_img_refs(fragment)
            .into_iter()
            .find(|image| image.src.starts_with("data:image/"))?;
        SurfaceImageBlock::from_data_uri(&image.src, image.width, image.alt)
    }

    fn append_details(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        fragment: &str,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) -> bool {
        let Some(parts) = SurfaceDetailsParts::parse(fragment) else {
            return false;
        };
        Self::append_details_summary(blocks, &parts, quote_depth, list_depth);
        Self::append_details_body(blocks, graph, &parts, quote_depth, list_depth, theme);
        true
    }

    fn append_details_summary(
        blocks: &mut Vec<SurfaceBlock>,
        parts: &SurfaceDetailsParts,
        quote_depth: u32,
        list_depth: u32,
    ) {
        Self::append_wrapped(
            blocks,
            SurfaceHtmlMarkup::normalize_text(parts.summary),
            quote_depth,
            list_depth,
        );
    }

    fn append_details_body(
        blocks: &mut Vec<SurfaceBlock>,
        graph: &BuildGraph,
        parts: &SurfaceDetailsParts,
        quote_depth: u32,
        list_depth: u32,
        theme: &KdvThemeSnapshot,
    ) {
        let fragment = EvaluatedMarkdownFragment::evaluate("surface-details.md", parts.body.trim());
        if !fragment.has_nodes() {
            Self::append_wrapped(
                blocks,
                SurfaceHtmlMarkup::normalize_text(parts.body),
                quote_depth,
                list_depth,
            );
            return;
        }
        for node in fragment.nodes() {
            Self::append_node(blocks, graph, node, quote_depth, list_depth, theme);
        }
    }
}

#[cfg(test)]
#[path = "html_image_tests.rs"]
mod image_tests;

#[cfg(test)]
#[path = "html_tests.rs"]
mod tests;
