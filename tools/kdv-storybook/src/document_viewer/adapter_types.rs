use katana_document_viewer::ViewerNodePlan;
use katana_ui_core::surface::PaintRequest;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct KucViewerAdapter;

#[derive(Debug, Clone, PartialEq)]
pub struct KucViewerPlan {
    pub paint_request: PaintRequest,
    pub node_plan: ViewerNodePlan,
    pub content_height: f32,
}
