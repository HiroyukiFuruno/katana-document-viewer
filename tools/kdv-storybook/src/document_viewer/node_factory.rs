use crate::document_viewer::asset_index::KucArtifactIndex;
use crate::document_viewer::media_control_icons::KucMediaControlIconSet;
use crate::document_viewer::node_labels::{CODE_FONT_ROLE, KucNodeLabels};
use katana_document_viewer::{
    Artifact, DiagramViewportState, ViewerHtmlRole, ViewerInteractionConfig, ViewerNode,
    ViewerNodeKind, ViewerTableProjection, ViewerTaskState, ViewerTypographyConfig,
};
use katana_ui_core::atom::{Divider, Text};
use katana_ui_core::layout::{Alignment, Row, Stack};
use katana_ui_core::render_model::{
    UiBorder, UiDimension, UiEdgeInsets, UiNode, UiNodeKind, UiPosition, UiVisualRole,
};
use std::collections::{BTreeMap, BTreeSet};

const KATANA_MEDIA_ROW_IMAGE_TOP_INSET_PX: u16 = 0;
const KATANA_HTML_TOP_ADJUSTMENT_PX: u16 = 7;
const RGBA_CHANNEL_COUNT: usize = 4;
type RgbaChannels = [u8; RGBA_CHANNEL_COUNT];

pub(crate) struct KucNodeFactory<'a> {
    artifacts: KucArtifactIndex<'a>,
    content_width: u32,
    max_media_width: u32,
    interaction: ViewerInteractionConfig,
    diagram_viewports: Option<&'a BTreeMap<String, DiagramViewportState>>,
    image_viewports: Option<&'a BTreeMap<String, DiagramViewportState>>,
    task_state_overrides: Option<&'a BTreeMap<String, ViewerTaskState>>,
    hovered_node_id: Option<&'a str>,
    accordion_open_overrides: Option<&'a BTreeMap<String, bool>>,
    copied_code_node_ids: Option<&'a BTreeSet<String>>,
    media_control_icons: KucMediaControlIconSet,
    typography: ViewerTypographyConfig,
    export_surface: bool,
    viewer_background: Option<RgbaChannels>,
    fullscreen_viewport_width: Option<u32>,
    fullscreen_viewport_height: Option<u32>,
    table_projections: BTreeMap<String, ViewerTableProjection>,
}

impl<'a> KucNodeFactory<'a> {
    pub(crate) fn new(artifacts: &'a [Artifact], max_media_width: u32) -> Self {
        Self {
            artifacts: KucArtifactIndex::new(artifacts),
            content_width: max_media_width,
            max_media_width,
            interaction: ViewerInteractionConfig {
                hover_highlight_enabled: false,
                selection_enabled: false,
                image_controls_enabled: false,
                diagram_controls_enabled: false,
                code_controls_enabled: false,
            },
            diagram_viewports: None,
            image_viewports: None,
            task_state_overrides: None,
            hovered_node_id: None,
            accordion_open_overrides: None,
            copied_code_node_ids: None,
            media_control_icons: KucMediaControlIconSet::katana_default(),
            typography: ViewerTypographyConfig::default(),
            export_surface: false,
            viewer_background: None,
            fullscreen_viewport_width: None,
            fullscreen_viewport_height: None,
            table_projections: BTreeMap::new(),
        }
    }

    pub(crate) fn table_projections(
        mut self,
        projections: BTreeMap<String, ViewerTableProjection>,
    ) -> Self {
        self.table_projections = projections;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_table_projection(
        mut self,
        node_id: &str,
        projection: ViewerTableProjection,
    ) -> Self {
        self.table_projections
            .insert(node_id.to_owned(), projection);
        self
    }

    pub(crate) fn interaction(mut self, value: ViewerInteractionConfig) -> Self {
        self.interaction = value;
        self
    }

    pub(crate) fn with_media_max_width(mut self, value: u32) -> Self {
        self.max_media_width = value;
        self
    }

    pub(crate) fn diagram_viewports(
        mut self,
        value: &'a BTreeMap<String, DiagramViewportState>,
    ) -> Self {
        self.diagram_viewports = Some(value);
        self
    }

    pub(crate) fn image_viewports(
        mut self,
        value: &'a BTreeMap<String, DiagramViewportState>,
    ) -> Self {
        self.image_viewports = Some(value);
        self
    }

    pub(crate) fn task_state_overrides(
        mut self,
        value: &'a BTreeMap<String, ViewerTaskState>,
    ) -> Self {
        self.task_state_overrides = Some(value);
        self
    }

    pub(crate) fn copied_code_node_ids(mut self, value: &'a BTreeSet<String>) -> Self {
        self.copied_code_node_ids = Some(value);
        self
    }

    pub(crate) fn typography(mut self, value: ViewerTypographyConfig) -> Self {
        self.typography = value;
        self
    }

    pub(crate) fn viewer_background(mut self, value: Option<RgbaChannels>) -> Self {
        self.viewer_background = value;
        self
    }

    pub(crate) fn media_control_icons(mut self, value: &KucMediaControlIconSet) -> Self {
        self.media_control_icons = value.clone();
        self
    }

    pub(crate) fn export_surface(mut self, value: bool) -> Self {
        self.export_surface = value;
        self
    }

    #[cfg(test)]
    pub(crate) fn fullscreen_viewport_height(mut self, value: u32) -> Self {
        self.fullscreen_viewport_height = Some(value.max(1));
        self
    }

    pub(crate) fn fullscreen_viewport_size(mut self, width: u32, height: u32) -> Self {
        self.fullscreen_viewport_width = Some(width.max(1));
        self.fullscreen_viewport_height = Some(height.max(1));
        self
    }

    pub(crate) fn viewer_node(&self, node: &ViewerNode) -> UiNode {
        let rendered = match &node.kind {
            ViewerNodeKind::Diagram { .. } | ViewerNodeKind::Image | ViewerNodeKind::Math => {
                self.media_node(node)
            }
            ViewerNodeKind::Code { .. } => self.code_node(node),
            ViewerNodeKind::Html {
                role: ViewerHtmlRole::Accordion,
            } => self.accordion_node(node),
            ViewerNodeKind::Html { .. } => self.html_node(node),
            ViewerNodeKind::Rule => {
                let rule_line_offset_px = node.rule_line_offset_px;
                let divider: UiNode = Divider::new("horizontal rule").into();
                let common = divider.props().common.clone().padding(UiEdgeInsets {
                    top: UiDimension::Px(rule_line_offset_px),
                    ..UiEdgeInsets::default()
                });
                divider
                    .width(UiDimension::Px(Self::rule_width(self.content_width)))
                    .common(common)
                    .border(UiBorder::solid(
                        if self.export_surface { 2 } else { 1 },
                        0,
                        "document.rule.border",
                    ))
            }
            ViewerNodeKind::Alert { .. } => self.alert_node(node),
            ViewerNodeKind::Table => self.table_node(node),
            ViewerNodeKind::List => self.list_node(node),
            ViewerNodeKind::BlockQuote => self.blockquote_node(node),
            ViewerNodeKind::FootnoteDefinition { .. } => self.footnote_node(node),
            _ => self.text_node(node),
        };
        self.hover_node_if_needed(self.node_with_viewer_height(rendered, node), node)
    }

    fn html_node(&self, node: &ViewerNode) -> UiNode {
        self.html_badge_row_node(node)
            .or_else(|| self.html_image_node(node))
            .unwrap_or_else(|| self.text_node(node))
    }

    fn text_node(&self, node: &ViewerNode) -> UiNode {
        let recovery_spans = Self::normalizable_html_data_image_source_spans(node);
        let export_wrapped_spans = self.export_wrapped_paragraph_spans(node);
        let export_wrapped_label = self.export_wrapped_centered_html_label(node);
        let mut text = Text::new(
            export_wrapped_spans
                .as_ref()
                .map(|spans| {
                    spans
                        .iter()
                        .map(|span| span.text.as_str())
                        .collect::<String>()
                })
                .or_else(|| export_wrapped_label.clone())
                .unwrap_or_else(|| Self::text_label(node)),
        )
        .font_role(self.font_role_for_node(node))
        .text_role(self.text_role_for_node(node))
        .wrap(Self::text_wrap_for_node(node))
        .selectable(self.interaction.selection_enabled);
        if let Some(spans) = recovery_spans {
            text = text.text_spans(spans);
        } else if let Some(spans) = export_wrapped_spans {
            text = text.text_spans(spans);
        } else if export_wrapped_label.is_none()
            && !node.spans.is_empty()
            && !Self::is_html_image_source_recovery(node)
        {
            text = text.text_spans(Self::text_spans(&node.spans));
        }
        let rendered: UiNode = text.into();
        let rendered = self.html_margin_node(rendered, node);
        let rendered = self.centered_link_row_node(rendered, node);
        if node.spans.iter().any(|span| !span.link_target.is_empty()) {
            return rendered
                .stable_node_id(node.node_id.0.clone())
                .stable_state_id(node.node_id.0.clone());
        }
        rendered
    }

    fn centered_link_row_node(&self, ui_node: UiNode, node: &ViewerNode) -> UiNode {
        if self.export_surface
            || !matches!(
                node.kind,
                ViewerNodeKind::Html {
                    role: ViewerHtmlRole::Centered
                }
            )
            || !node.spans.iter().any(|span| !span.link_target.is_empty())
        {
            return ui_node;
        }
        UiNode::from(Row::new().align(Alignment::Center).child(ui_node))
            .height(UiDimension::Px(self.interactive_html_parent_row_height()))
    }

    fn interactive_html_parent_row_height(&self) -> u16 {
        (f32::from(self.typography.preview_font_size) * 1.5
            + f32::from(KATANA_HTML_TOP_ADJUSTMENT_PX))
        .ceil()
        .min(f32::from(u16::MAX)) as u16
    }

    fn html_margin_node(&self, ui_node: UiNode, node: &ViewerNode) -> UiNode {
        if !matches!(node.kind, ViewerNodeKind::Html { .. }) || node.html_margin_left_px == 0 {
            return ui_node;
        }
        let common = ui_node.props().common.clone().margin(UiEdgeInsets {
            left: UiDimension::Px(node.html_margin_left_px),
            ..UiEdgeInsets::default()
        });
        ui_node.common(common)
    }

    fn node_with_viewer_height(&self, ui_node: UiNode, node: &ViewerNode) -> UiNode {
        let common = ui_node
            .props()
            .common
            .clone()
            .semantic_node_id(node.node_id.0.clone());
        let ui_node = ui_node.common(common);
        if matches!(node.kind, ViewerNodeKind::Code { .. }) {
            return ui_node
                .stable_node_id(node.node_id.0.clone())
                .stable_state_id(node.node_id.0.clone());
        }
        if (ui_node.kind() == UiNodeKind::Text
            || matches!(node.kind, ViewerNodeKind::List)
            || Self::uses_native_interactive_html_height(node))
            && !self.export_surface
        {
            return ui_node
                .width(self.viewer_width_for_node(node))
                .stable_node_id(node.node_id.0.clone())
                .stable_state_id(node.node_id.0.clone());
        }
        let width = self.viewer_width_for_node(node);
        let height = self.viewer_height_for_rendered_node(&ui_node, node);
        if Self::uses_media_row_wrapper(&ui_node, node) {
            return Self::media_row_wrapper(ui_node, node, width, height);
        }
        ui_node
            .width(width)
            .height(height)
            .stable_node_id(node.node_id.0.clone())
            .stable_state_id(node.node_id.0.clone())
    }

    fn uses_native_interactive_html_height(node: &ViewerNode) -> bool {
        matches!(
            node.kind,
            ViewerNodeKind::Html {
                role: ViewerHtmlRole::BadgeRow
            }
        ) || (matches!(
            node.kind,
            ViewerNodeKind::Html {
                role: ViewerHtmlRole::Centered
            }
        ) && node.spans.iter().any(|span| !span.link_target.is_empty()))
    }

    fn uses_media_row_wrapper(ui_node: &UiNode, node: &ViewerNode) -> bool {
        matches!(node.kind, ViewerNodeKind::Diagram { .. })
            && ui_node.kind() == UiNodeKind::ImageSurface
            && matches!(
                ui_node.props().visual_role,
                UiVisualRole::MediaFrame | UiVisualRole::ExportMediaFrame
            )
    }

    fn viewer_height_for_rendered_node(&self, ui_node: &UiNode, node: &ViewerNode) -> UiDimension {
        if matches!(node.kind, ViewerNodeKind::Diagram { .. })
            && self.diagram_fullscreen_open(node)
            && matches!(ui_node.props().visual_role, UiVisualRole::MediaFrame)
            && !matches!(ui_node.props().common.height, UiDimension::Auto)
        {
            return ui_node.props().common.height.clone();
        }
        if matches!(node.kind, ViewerNodeKind::Diagram { .. })
            && matches!(ui_node.props().visual_role, UiVisualRole::ExportMediaFrame)
            && !matches!(ui_node.props().common.height, UiDimension::Auto)
        {
            return ui_node.props().common.height.clone();
        }
        let viewer_height = Self::viewer_height(node);
        if self.export_surface
            && ui_node.kind() == UiNodeKind::Text
            && matches!(node.kind, ViewerNodeKind::Paragraph)
            && let UiDimension::Px(viewer_height_px) = viewer_height
        {
            let wrapped_height = self.body_line_height_px().saturating_mul(
                u16::try_from(ui_node.props().label.lines().count().max(1)).unwrap_or(u16::MAX),
            );
            return UiDimension::Px(viewer_height_px.max(wrapped_height));
        }
        if matches!(node.kind, ViewerNodeKind::Diagram { .. })
            && matches!(ui_node.props().visual_role, UiVisualRole::MediaFrame)
            && let (UiDimension::Px(viewer), UiDimension::Px(media)) =
                (&viewer_height, &ui_node.props().common.height)
        {
            return UiDimension::Px((*viewer).max(*media));
        }
        viewer_height
    }

    fn media_row_wrapper(
        ui_node: UiNode,
        node: &ViewerNode,
        width: UiDimension,
        height: UiDimension,
    ) -> UiNode {
        let node_id = node.node_id.0.clone();
        let visual_role = ui_node.props().visual_role;
        let media = ui_node.position(UiPosition::Absolute).margin(UiEdgeInsets {
            top: UiDimension::Px(KATANA_MEDIA_ROW_IMAGE_TOP_INSET_PX),
            ..UiEdgeInsets::default()
        });
        let wrapper: UiNode = Stack::new().child(media).into();
        let common = wrapper
            .props()
            .common
            .clone()
            .semantic_node_id(node_id.clone());
        wrapper
            .common(common)
            .visual_role(visual_role)
            .width(width)
            .height(height)
            .stable_node_id(node_id.clone())
            .stable_state_id(node_id)
    }

    fn viewer_width_for_node(&self, node: &ViewerNode) -> UiDimension {
        if matches!(node.kind, ViewerNodeKind::Diagram { .. }) && self.diagram_fullscreen_open(node)
        {
            return UiDimension::Px(Self::clamped_viewport_extent(
                self.fullscreen_viewport_width
                    .unwrap_or(self.max_media_width),
            ));
        }
        let width = match node.kind {
            ViewerNodeKind::Diagram { .. } | ViewerNodeKind::Image | ViewerNodeKind::Math => {
                self.max_media_width
            }
            _ => self.content_width,
        };
        UiDimension::Px(Self::clamped_width(width))
    }

    fn clamped_width(value: u32) -> u16 {
        value.min(u32::from(u16::MAX)) as u16
    }

    fn rule_width(max_media_width: u32) -> u16 {
        u16::try_from(max_media_width.min(u32::from(u16::MAX))).unwrap_or(u16::MAX)
    }
}

#[path = "node_factory_accordion.rs"]
mod accordion;
#[path = "node_factory_alert.rs"]
mod alert;
#[path = "node_factory_blockquote.rs"]
mod blockquote;
#[path = "node_factory_code.rs"]
mod code;
#[path = "node_factory_footnote.rs"]
mod footnote;
#[path = "node_factory_hover.rs"]
mod hover;
#[path = "node_factory_html_badge.rs"]
mod html_badge;
#[path = "node_factory_html_badge_parser.rs"]
mod html_badge_parser;
#[path = "node_factory_html_image.rs"]
mod html_image;
#[path = "node_factory_list_impl.rs"]
mod list_impl;
#[path = "node_factory_media_controls.rs"]
mod media_controls;
#[path = "node_factory_media_diagram_controls.rs"]
mod media_diagram_controls;
#[path = "node_factory_media_geometry.rs"]
mod media_geometry;
#[path = "node_factory_media_image_controls.rs"]
mod media_image_controls;
#[path = "node_factory_media_impl.rs"]
mod media_impl;
#[path = "node_factory_metrics.rs"]
mod metrics;
#[path = "node_factory_table.rs"]
mod table;
#[path = "node_factory_task_state.rs"]
mod task_state;
#[path = "node_factory_text.rs"]
mod text;

#[cfg(test)]
#[path = "node_factory_test_modules.rs"]
mod test_modules;
