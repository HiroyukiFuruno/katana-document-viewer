use super::mouse_document_point::DocumentPoint;
use crate::KucDiagramControlResolver;
use crate::layout::{preview_content_width, preview_viewport_height};
use crate::preview::PreviewScene;
use katana_document_viewer::{ViewerRect, ViewerTarget};
use katana_ui_core::render_model::UiNodeId;
use katana_ui_core_storybook::{
    UiTreeHitRect, UiTreeHostActionHit, UiTreeInteractionTarget, UiTreeNodeHit, UiTreeRenderArea,
    UiTreeSurfaceHost,
};
use std::sync::Arc;

pub(crate) struct StorybookHostActionHits;

pub(crate) struct StorybookHostActionRouter<'a> {
    scene: &'a PreviewScene,
    hits: Arc<Vec<UiTreeHostActionHit>>,
    node_hits: Vec<UiTreeNodeHit>,
}

pub(super) struct StorybookResolvedHostActionHit<'a> {
    hit: UiTreeHostActionHit,
    target: &'a ViewerTarget,
}

impl StorybookHostActionHits {
    pub(crate) fn hits(scene: &PreviewScene, window_width: usize) -> Vec<UiTreeHostActionHit> {
        Self::hits_for_preview_width(scene, Self::interaction_width(scene, window_width))
    }

    pub(crate) fn hits_for_preview_width(
        scene: &PreviewScene,
        preview_width: usize,
    ) -> Vec<UiTreeHostActionHit> {
        Self::hits_arc_for_preview_width(scene, preview_width)
            .as_ref()
            .clone()
    }

    pub(crate) fn hits_arc_for_preview_width(
        scene: &PreviewScene,
        preview_width: usize,
    ) -> Arc<Vec<UiTreeHostActionHit>> {
        scene.host_action_cache.hits_or_insert(preview_width, || {
            UiTreeSurfaceHost::new(scene.theme.clone()).document_host_action_hits(
                scene.tree.root(),
                UiTreeRenderArea {
                    x: 0,
                    y: 0,
                    width: preview_width,
                    height: scene.content_height.ceil().max(1.0) as usize,
                    scroll_y: 0.0,
                },
            )
        })
    }

    pub(crate) fn node_hits_arc_for_preview_width(
        scene: &PreviewScene,
        preview_width: usize,
    ) -> Arc<Vec<UiTreeNodeHit>> {
        scene
            .host_action_cache
            .node_hits_or_insert(preview_width, || {
                Self::viewport_node_hits(scene, preview_width, scene.content_height, 0.0)
            })
    }

    fn viewport_node_hits(
        scene: &PreviewScene,
        preview_width: usize,
        preview_height: f32,
        scroll_y: f32,
    ) -> Vec<UiTreeNodeHit> {
        UiTreeSurfaceHost::new(scene.theme.clone()).viewport_node_hits(
            scene.tree.root(),
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: preview_width,
                height: preview_height.ceil().max(1.0) as usize,
                scroll_y,
            },
        )
    }

    pub(crate) fn viewport_hits_for_preview_width(
        scene: &PreviewScene,
        preview_width: usize,
        preview_height: usize,
        scroll_y: f32,
    ) -> Arc<Vec<UiTreeHostActionHit>> {
        let (hits, _) = Self::viewport_interaction_hits_for_preview_width(
            scene,
            preview_width,
            preview_height,
            scroll_y,
        );
        hits
    }

    pub(crate) fn viewport_interaction_hits_for_preview_width(
        scene: &PreviewScene,
        preview_width: usize,
        preview_height: usize,
        scroll_y: f32,
    ) -> (Arc<Vec<UiTreeHostActionHit>>, Vec<UiTreeNodeHit>) {
        let slideshow = scene.mode == katana_document_viewer::ViewerMode::Slideshow;
        let effective_scroll_y = DocumentPoint::effective_scroll_y(scene, scroll_y);
        let tree = if slideshow {
            Some(
                scene
                    .tree
                    .with_scroll_area_offset_y(effective_scroll_y.round().max(0.0) as u32),
            )
        } else {
            None
        };
        let root = tree
            .as_ref()
            .map_or_else(|| scene.tree.root(), |tree| tree.root());
        if !slideshow
            && !scene.fullscreen_diagram_active()
            && scene.tree.root().props().scroll_area.offset_y == 0
        {
            return Self::document_cached_viewport_interaction_hits_for_preview_width(
                scene,
                preview_width,
                preview_height,
                effective_scroll_y,
            );
        }
        let (hits, node_hits) = UiTreeSurfaceHost::new(scene.theme.clone())
            .viewport_interaction_hits(
                root,
                UiTreeRenderArea {
                    x: 0,
                    y: 0,
                    width: preview_width,
                    height: preview_height.max(1),
                    scroll_y: if slideshow || scene.fullscreen_diagram_active() {
                        0.0
                    } else {
                        Self::render_scroll_delta(scene, scroll_y)
                    },
                },
            );
        if slideshow {
            return (Arc::new(hits), node_hits);
        }
        let hits = Arc::new(
            hits.into_iter()
                .map(|mut hit| {
                    hit.rect = Self::viewport_rect_to_document_rect(hit.rect, effective_scroll_y);
                    hit
                })
                .collect(),
        );
        let node_hits = node_hits
            .into_iter()
            .map(|mut hit| {
                hit.rect = Self::viewport_rect_to_document_rect(hit.rect, effective_scroll_y);
                hit
            })
            .collect();
        (hits, node_hits)
    }

    fn document_cached_viewport_interaction_hits_for_preview_width(
        scene: &PreviewScene,
        preview_width: usize,
        preview_height: usize,
        effective_scroll_y: f32,
    ) -> (Arc<Vec<UiTreeHostActionHit>>, Vec<UiTreeNodeHit>) {
        let hits = Arc::new(
            Self::hits_arc_for_preview_width(scene, preview_width)
                .iter()
                .filter_map(|hit| {
                    let mut hit = hit.clone();
                    hit.rect = Self::document_rect_clipped_to_viewport(
                        hit.rect,
                        effective_scroll_y,
                        preview_height,
                    )?;
                    Some(hit)
                })
                .collect(),
        );
        let node_hits = Self::node_hits_arc_for_preview_width(scene, preview_width)
            .iter()
            .filter_map(|hit| {
                let mut hit = hit.clone();
                hit.rect = Self::document_rect_clipped_to_viewport(
                    hit.rect,
                    effective_scroll_y,
                    preview_height,
                )?;
                Some(hit)
            })
            .collect();
        (hits, node_hits)
    }

    pub(crate) fn interaction_width(scene: &PreviewScene, window_width: usize) -> usize {
        if scene.fullscreen_diagram_active() {
            return window_width.max(1);
        }
        preview_content_width(window_width)
    }

    pub(crate) fn interaction_height(scene: &PreviewScene, window_height: usize) -> usize {
        if scene.fullscreen_diagram_active() {
            return window_height.max(1);
        }
        preview_viewport_height(window_height)
    }

    fn render_scroll_delta(scene: &PreviewScene, requested_scroll_y: f32) -> f32 {
        let tree_offset = scene.tree.root().props().scroll_area.offset_y as f32;
        (requested_scroll_y - tree_offset).max(0.0)
    }

    fn viewport_rect_to_document_rect(
        mut rect: UiTreeHitRect,
        effective_scroll_y: f32,
    ) -> UiTreeHitRect {
        rect.y = rect
            .y
            .saturating_add(effective_scroll_y.round().max(0.0) as usize);
        rect
    }

    fn document_rect_clipped_to_viewport(
        rect: UiTreeHitRect,
        effective_scroll_y: f32,
        preview_height: usize,
    ) -> Option<UiTreeHitRect> {
        let viewport_top = effective_scroll_y.round().max(0.0) as usize;
        let viewport_bottom = viewport_top.saturating_add(preview_height.max(1));
        let rect_bottom = rect.y.saturating_add(rect.height);
        let visible_top = rect.y.max(viewport_top);
        let visible_bottom = rect_bottom.min(viewport_bottom);
        if visible_bottom <= visible_top {
            return None;
        }
        Some(UiTreeHitRect {
            x: rect.x,
            y: visible_top,
            width: rect.width,
            height: visible_bottom.saturating_sub(visible_top),
        })
    }
}

impl<'a> StorybookHostActionRouter<'a> {
    pub(crate) fn for_window(scene: &'a PreviewScene, window_width: usize) -> Self {
        let preview_width = StorybookHostActionHits::interaction_width(scene, window_width);
        Self {
            scene,
            hits: StorybookHostActionHits::hits_arc_for_preview_width(scene, preview_width),
            node_hits: Self::node_hits(scene, preview_width),
        }
    }

    pub(crate) fn for_window_with_scroll(
        scene: &'a PreviewScene,
        window_width: usize,
        window_height: usize,
        scroll_y: f32,
    ) -> Self {
        let preview_width = StorybookHostActionHits::interaction_width(scene, window_width);
        let preview_height = StorybookHostActionHits::interaction_height(scene, window_height);
        let (hits, node_hits) =
            StorybookHostActionHits::viewport_interaction_hits_for_preview_width(
                scene,
                preview_width,
                preview_height,
                scroll_y,
            );
        Self {
            scene,
            hits,
            node_hits,
        }
    }

    #[cfg(test)]
    pub(super) fn for_preview_width(scene: &'a PreviewScene, preview_width: usize) -> Self {
        Self {
            scene,
            hits: StorybookHostActionHits::hits_arc_for_preview_width(scene, preview_width),
            node_hits: Self::node_hits(scene, preview_width),
        }
    }

    #[cfg(test)]
    pub(super) fn hits(&self) -> &[UiTreeHostActionHit] {
        self.hits.as_slice()
    }

    pub(super) fn hits_at(
        &self,
        point: DocumentPoint,
    ) -> impl Iterator<Item = UiTreeHostActionHit> + '_ {
        UiTreeSurfaceHost::hits_at(self.hits.as_slice(), point.x, point.y).into_iter()
    }

    pub(super) fn cursor_at(&self, point: DocumentPoint) -> katana_ui_core::render_model::UiCursor {
        if self.internal_diagram_action_at(point).is_some() {
            return katana_ui_core::render_model::UiCursor::Pointer;
        }
        self.interaction_target_at(point)
            .map(|target| target.cursor)
            .unwrap_or(katana_ui_core::render_model::UiCursor::Default)
    }

    pub(super) fn hovered_action_node_id_at(
        &self,
        point: DocumentPoint,
    ) -> Option<katana_ui_core::render_model::UiNodeId> {
        self.interaction_target_at(point).and_then(|target| {
            if target.action.is_some() {
                return Some(target.hover_node_id());
            }
            None
        })
    }

    pub(super) fn hovered_node_id_at(
        &self,
        point: DocumentPoint,
    ) -> Option<katana_ui_core::render_model::UiNodeId> {
        self.interaction_target_at(point)
            .map(|target| target.hover_node_id())
    }

    pub(super) fn interaction_target_at(
        &self,
        point: DocumentPoint,
    ) -> Option<UiTreeInteractionTarget> {
        UiTreeSurfaceHost::interaction_target_for_hits_at(
            self.hits.as_slice(),
            &self.node_hits,
            point.x,
            point.y,
        )
    }

    pub(crate) fn diagram_target_at(&self, point: DocumentPoint) -> Option<&'a ViewerTarget> {
        if self.internal_diagram_action_at(point).is_some() || self.hits_at(point).next().is_some()
        {
            return None;
        }
        let target = self.interaction_target_at(point)?;
        target
            .semantic_node_id
            .as_ref()
            .and_then(|node_id| self.diagram_target_for_node_id(node_id.as_str()))
            .or_else(|| self.diagram_target_for_node_id(target.node_id.as_str()))
    }

    fn diagram_target_for_node_id(&self, node_id: &str) -> Option<&'a ViewerTarget> {
        if !self.scene.diagram_node_ids.contains(node_id) {
            return None;
        }
        self.target_for_node_id(node_id)
    }

    pub(super) fn resolved_hits_at(
        &self,
        point: DocumentPoint,
    ) -> impl Iterator<Item = StorybookResolvedHostActionHit<'a>> + '_ {
        self.hits_at(point)
            .filter_map(|hit| self.resolve_hit_target(hit))
    }

    pub(super) fn resolve_hit_target(
        &self,
        hit: UiTreeHostActionHit,
    ) -> Option<StorybookResolvedHostActionHit<'a>> {
        let node_id = hit.action.target.clone();
        self.resolve_hit_target_for_node_id(node_id.as_str(), hit)
    }

    pub(super) fn resolve_hit_target_for_node_id(
        &self,
        node_id: &str,
        hit: UiTreeHostActionHit,
    ) -> Option<StorybookResolvedHostActionHit<'a>> {
        let target = self.target_for_node_id(node_id)?;
        Some(StorybookResolvedHostActionHit { hit, target })
    }

    pub(super) fn target_for_node_id(&self, node_id: &str) -> Option<&'a ViewerTarget> {
        self.scene.target_for_node_id(node_id)
    }

    pub(super) fn scene_target_for_internal_anchor(
        &self,
        anchor: &str,
    ) -> Option<&'a ViewerTarget> {
        self.scene.target_for_internal_anchor(anchor)
    }

    pub(super) fn internal_diagram_action_at(
        &self,
        point: DocumentPoint,
    ) -> Option<katana_document_viewer::ViewerMediaControlAction> {
        KucDiagramControlResolver::internal_action_at(
            self.scene.tree.root(),
            &self.node_hits,
            point.x,
            point.y,
        )
    }

    pub(super) fn internal_diagram_node_id_at(&self, point: DocumentPoint) -> Option<UiNodeId> {
        KucDiagramControlResolver::internal_control_node_id_at(
            self.scene.tree.root(),
            &self.node_hits,
            point.x,
            point.y,
        )
    }

    pub(crate) fn internal_diagram_point_for_action(&self, action: &str) -> Option<(f32, f32)> {
        self.node_hits.iter().find_map(|hit| {
            let resolved = KucDiagramControlResolver::internal_action_for_node(
                self.scene.tree.root(),
                &hit.node_id,
            )?;
            if resolved.command != action {
                return None;
            }
            Some(hit.rect.center_point())
        })
    }

    fn node_hits(scene: &PreviewScene, preview_width: usize) -> Vec<UiTreeNodeHit> {
        StorybookHostActionHits::node_hits_arc_for_preview_width(scene, preview_width)
            .as_ref()
            .clone()
    }
}

impl StorybookResolvedHostActionHit<'_> {
    pub(super) fn hit(&self) -> &UiTreeHostActionHit {
        &self.hit
    }

    pub(super) fn target(&self) -> &ViewerTarget {
        self.target
    }

    pub(super) fn hit_rect(&self) -> ViewerRect {
        ViewerRect {
            x: self.hit.rect.x as f32,
            y: self.hit.rect.y as f32,
            width: self.hit.rect.width as f32,
            height: self.hit.rect.height as f32,
        }
    }
}
