use super::{DocumentViewerStorybookHost, KucViewerConfig, KucViewerError, KucViewerPlan};
use katana_document_viewer::{
    KDV_INTERACTIVE_PREVIEW_SURFACE_PADDING_PX, MarkdownSource, PreviewConfig, PreviewOutput,
    PreviewOutputFactory, PreviewSurfaceImage, ViewerInteractionConfig, ViewerMode,
    ViewerSlideshowControlAction, ViewerViewport,
};
use katana_ui_core::render_model::{UI_TASK_TOGGLE_ACTION_ID, UiCursor, UiNode, UiNodeId, UiTone};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core_storybook::{
    Canvas, UiTreeHostActionHit, UiTreeRenderArea, UiTreeStorybookHost,
};

const CONTENT_HEIGHT: f32 = 480.0;
const PREVIEW_SURFACE_PADDING_PX: f32 = KDV_INTERACTIVE_PREVIEW_SURFACE_PADDING_PX as f32;
const KUC_PREVIEW_TOP_PADDING_PX: f32 = PREVIEW_SURFACE_PADDING_PX + 2.0;
const KUC_PREVIEW_VERTICAL_PADDING_PX: f32 =
    KUC_PREVIEW_TOP_PADDING_PX + PREVIEW_SURFACE_PADDING_PX;
const LONG_DOCUMENT_PARAGRAPH_COUNT: usize = 30;
const VIEWPORT_WIDTH: f32 = 320.0;
const VIEWPORT_HEIGHT: f32 = 240.0;
const RENDER_AREA_WIDTH: usize = 320;
const RENDER_AREA_HEIGHT: usize = 240;
const RUNTIME_SURFACE_WIDTH: u32 = 320;
const RUNTIME_SURFACE_HEIGHT: u32 = 320;
const RUNTIME_SURFACE_CONTENT_HEIGHT: u32 = 320;

trait DocumentViewerStorybookHostTestExt {
    fn render(
        &self,
        canvas: &mut Canvas,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
    ) -> Result<KucViewerPlan, KucViewerError>;

    fn host_action_hits(
        &self,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
    ) -> Result<Vec<UiTreeHostActionHit>, KucViewerError>;

    fn cursor_at(
        &self,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Result<UiCursor, KucViewerError>;

    fn hovered_action_node_id_at(
        &self,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Result<Option<UiNodeId>, KucViewerError>;
}

impl DocumentViewerStorybookHostTestExt for DocumentViewerStorybookHost {
    fn render(
        &self,
        canvas: &mut Canvas,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
    ) -> Result<KucViewerPlan, KucViewerError> {
        let plan = self.project(output, config)?;
        ui_tree_host(config).render(canvas, plan.paint_request.tree().root(), area);
        Ok(plan)
    }

    fn host_action_hits(
        &self,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
    ) -> Result<Vec<UiTreeHostActionHit>, KucViewerError> {
        let plan = self.project(output, config)?;
        Ok(ui_tree_host(config).host_action_hits(plan.paint_request.tree().root(), area))
    }

    fn cursor_at(
        &self,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Result<UiCursor, KucViewerError> {
        let hits = self.host_action_hits(output, config, area)?;
        Ok(UiTreeStorybookHost::cursor_for_host_action_hits_at(
            &hits, x, y,
        ))
    }

    fn hovered_action_node_id_at(
        &self,
        output: &PreviewOutput,
        config: &KucViewerConfig,
        area: UiTreeRenderArea,
        x: f32,
        y: f32,
    ) -> Result<Option<UiNodeId>, KucViewerError> {
        let hits = self.host_action_hits(output, config, area)?;
        Ok(UiTreeStorybookHost::hovered_action_node_id_for_host_action_hits_at(&hits, x, y))
    }
}

fn ui_tree_host(config: &KucViewerConfig) -> UiTreeStorybookHost {
    UiTreeStorybookHost::new(config.theme.clone())
}

#[test]
fn document_viewer_host_projects_preview_output_into_kuc_tree()
-> Result<(), Box<dyn std::error::Error>> {
    let output = preview_output("Body")?;
    let plan = DocumentViewerStorybookHost::default().project(&output, &config())?;

    assert_eq!("light", plan.paint_request.tree().root().props().theme_id);
    Ok(())
}

#[test]
fn document_viewer_scroll_extent_uses_node_plan_not_source_line_estimate()
-> Result<(), Box<dyn std::error::Error>> {
    let output = preview_output_with_content_height("Body", 10_000.0)?;
    let plan = DocumentViewerStorybookHost::default().project(&output, &config())?;
    let expected = plan.node_plan.content_height + KUC_PREVIEW_VERTICAL_PADDING_PX;

    assert_eq!(expected, plan.content_height);
    assert!(plan.content_height < output.content_height);
    Ok(())
}

#[test]
fn document_viewer_scroll_extent_keeps_bottom_tail_space_for_long_document()
-> Result<(), Box<dyn std::error::Error>> {
    let output = preview_output_with_content_height(&long_document(), 1.0)?;
    let viewer_config = config();
    let plan = DocumentViewerStorybookHost::default().project(&output, &viewer_config)?;
    let document_height = plan.node_plan.content_height + KUC_PREVIEW_VERTICAL_PADDING_PX;
    let rendered_top_padding = KUC_PREVIEW_TOP_PADDING_PX;
    let last_anchor_y = plan
        .node_plan
        .nodes
        .iter()
        .map(|node| node.rect.y + rendered_top_padding)
        .max_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal))
        .ok_or("missing last anchor")?;

    assert!(document_height > viewer_config.viewport.height);
    assert_eq!(
        last_anchor_y,
        plan.content_height - viewer_config.viewport.height
    );
    Ok(())
}

#[test]
fn document_viewer_scroll_extent_uses_runtime_surface_extent_when_attached()
-> Result<(), Box<dyn std::error::Error>> {
    let mut output = preview_output_with_content_height(&long_document(), 12_000.0)?;
    output.content_height = 480.0;
    output.surface = Some(PreviewSurfaceImage {
        fingerprint: "surface".to_string(),
        width: RUNTIME_SURFACE_WIDTH,
        height: RUNTIME_SURFACE_HEIGHT,
        origin_y: 0,
        content_height: RUNTIME_SURFACE_CONTENT_HEIGHT,
        rgba: Vec::new(),
    });

    let plan =
        DocumentViewerStorybookHost::default().project(&output, &config().export_surface(true))?;

    assert_eq!(480.0, plan.content_height);
    Ok(())
}

#[test]
fn document_viewer_host_returns_task_toggle_hit_and_pointer_cursor()
-> Result<(), Box<dyn std::error::Error>> {
    let output = preview_output("- [ ] todo")?;
    let area = render_area();
    let config = config().interaction(ViewerInteractionConfig {
        hover_highlight_enabled: false,
        selection_enabled: false,
        image_controls_enabled: false,
        diagram_controls_enabled: false,
        code_controls_enabled: false,
    });
    let host = DocumentViewerStorybookHost::default();
    let hit = host
        .host_action_hits(&output, &config, area)?
        .into_iter()
        .find(|hit| hit.action.action_id == UI_TASK_TOGGLE_ACTION_ID)
        .ok_or_else(|| std::io::Error::other("missing host action hit"))?;
    let (x, y) = hit.center_point();

    assert_eq!(UI_TASK_TOGGLE_ACTION_ID, hit.action.action_id);
    assert_eq!(
        UiCursor::Pointer,
        host.cursor_at(&output, &config, area, x, y)?
    );
    assert_eq!(
        Some(hit.action.target.clone()),
        host.hovered_action_node_id_at(&output, &config, area, x, y)?
    );
    Ok(())
}

#[test]
fn document_viewer_host_renders_projected_tree_with_config_theme()
-> Result<(), Box<dyn std::error::Error>> {
    let output = preview_output("Body")?;
    let mut canvas = Canvas::new(render_area().width, render_area().height, 0x00000000);
    let plan = DocumentViewerStorybookHost::default().render(
        &mut canvas,
        &output,
        &config().theme(ThemeSnapshot::dark()),
        render_area(),
    )?;

    assert_eq!("dark", plan.paint_request.tree().root().props().theme_id);
    Ok(())
}

#[test]
fn document_viewer_slideshow_exposes_real_control_host_actions()
-> Result<(), Box<dyn std::error::Error>> {
    let output = preview_output_with_mode("Slide\n\nNext", ViewerMode::Slideshow)?;
    let actions = DocumentViewerStorybookHost::default()
        .host_action_hits(&output, &config(), render_area())?
        .into_iter()
        .map(|hit| hit.action.action_id)
        .collect::<Vec<_>>();

    assert!(actions.contains(&ViewerSlideshowControlAction::PreviousPage.host_action_id()));
    assert!(actions.contains(&ViewerSlideshowControlAction::NextPage.host_action_id()));
    assert!(actions.contains(&ViewerSlideshowControlAction::Close.host_action_id()));
    Ok(())
}

#[test]
fn document_viewer_alerts_normalize_gfm_kind_contracts() -> Result<(), Box<dyn std::error::Error>> {
    assert_alert_contract("NOTE", "Note\nbody", "alert-note", UiTone::Accent)?;
    assert_alert_contract("TIP", "Tip\nbody", "alert-tip", UiTone::Success)?;
    assert_alert_contract(
        "IMPORTANT",
        "Important\nbody",
        "alert-important",
        UiTone::Accent,
    )?;
    assert_alert_contract("WARNING", "Warning\nbody", "alert-warning", UiTone::Warning)?;
    assert_alert_contract("CAUTION", "Caution\nbody", "alert-caution", UiTone::Danger)?;
    Ok(())
}

fn preview_output(
    content: &str,
) -> Result<katana_document_viewer::PreviewOutput, katana_document_viewer::PreviewError> {
    preview_output_with_content_height(content, CONTENT_HEIGHT)
}

fn preview_output_with_content_height(
    content: &str,
    content_height: f32,
) -> Result<katana_document_viewer::PreviewOutput, katana_document_viewer::PreviewError> {
    preview_output_with_config(content, content_height, PreviewConfig::default())
}

fn preview_output_with_mode(
    content: &str,
    mode: ViewerMode,
) -> Result<katana_document_viewer::PreviewOutput, katana_document_viewer::PreviewError> {
    preview_output_with_config(
        content,
        CONTENT_HEIGHT,
        PreviewConfig {
            mode,
            ..PreviewConfig::default()
        },
    )
}

fn preview_output_with_config(
    content: &str,
    content_height: f32,
    preview_config: PreviewConfig,
) -> Result<katana_document_viewer::PreviewOutput, katana_document_viewer::PreviewError> {
    PreviewOutputFactory::from_source(
        &MarkdownSource {
            content: content.to_string(),
            document_id: Some("storybook.md".to_string()),
        },
        &preview_config,
        content_height,
    )
}

fn long_document() -> String {
    (0..LONG_DOCUMENT_PARAGRAPH_COUNT)
        .map(|index| format!("paragraph {index}"))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn config() -> KucViewerConfig {
    KucViewerConfig::new(
        "storybook-host",
        ViewerViewport {
            width: VIEWPORT_WIDTH,
            height: VIEWPORT_HEIGHT,
        },
    )
}

fn render_area() -> UiTreeRenderArea {
    UiTreeRenderArea {
        x: 0,
        y: 0,
        width: RENDER_AREA_WIDTH,
        height: RENDER_AREA_HEIGHT,
        scroll_y: 0.0,
    }
}

fn assert_alert_contract(
    kind: &str,
    expected_label: &str,
    expected_color: &str,
    expected_tone: UiTone,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = preview_output(&format!("> [!{kind}]\n> body"))?;
    let plan = DocumentViewerStorybookHost::default().project(&output, &config())?;
    let alert = find_alert_node(plan.paint_request.tree().root())
        .ok_or_else(|| std::io::Error::other("missing alert node"))?;

    assert_eq!("alert", alert.props().text.role);
    assert_eq!(expected_label, alert.props().label);
    assert_eq!(expected_color, alert.props().common.border.color_token);
    assert_eq!(expected_tone, alert.props().severity);
    Ok(())
}

fn find_alert_node(node: &UiNode) -> Option<&UiNode> {
    if node.props().text.role == "alert" {
        return Some(node);
    }
    node.children().iter().find_map(find_alert_node)
}
