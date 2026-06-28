use super::{FrameRenderRequest, StorybookFrameRenderer};
use crate::canvas::Canvas;
use crate::catalog::StorybookFixture;
use crate::frame_preview_pixels::FramePreviewPixels;
use crate::frame_role_pixel_case::{FixtureRolePixelCase, RolePixelExpectation};
use crate::layout::StorybookPreviewArea;
use crate::mouse::StorybookHostActionHits;
use crate::preview::{PreviewBuilder, PreviewScene};
use crate::preview_theme_bridge::KucThemeBridge;
use katana_document_viewer::KdvThemeSnapshot;
use katana_document_viewer::{
    ViewerInteractionConfig, ViewerMediaControlAction, ViewerMediaControlKind, ViewerViewport,
};
use katana_ui_core::render_model::UiNode;
use katana_ui_core_storybook::{UiTreeRenderArea, UiTreeStorybookHost};
use std::path::PathBuf;

const FRAME_WIDTH: usize = 1280;
const FRAME_HEIGHT: usize = 12000;
const PREVIEW_WIDTH: f32 = 900.0;
const PREVIEW_HEIGHT: f32 = 11800.0;
const KUC_DARK_SELECTION: u32 = 0x264f78;
const ANY_PREVIEW_CONTENT: u32 = u32::MAX;

#[test]
fn required_kuc_roles_reach_fixture_frame_pixels() -> Result<(), Box<dyn std::error::Error>> {
    let code_background_color = dark_theme_rgb("code-background")?;
    let table_row_color = dark_theme_rgb("table-row-background")?;
    let alert_warning_color = dark_theme_rgb("alert-warning")?;
    let quote_background_color = dark_theme_rgb("quote-background")?;
    let footnote_background_color = dark_theme_rgb("footnote-background")?;
    for case in fixture_cases(
        code_background_color,
        table_row_color,
        alert_warning_color,
        quote_background_color,
        footnote_background_color,
    ) {
        let rendered = render_fixture(case.fixture)?;
        for role in case.roles {
            assert!(
                count_role(rendered.scene.tree.root(), role.name) >= role.minimum_nodes,
                "{} missing role {}",
                case.fixture,
                role.name
            );
            assert!(
                count_pixels(&rendered.canvas, role.pixel_color) >= role.minimum_pixels,
                "{} role {} missing color #{:06x}",
                case.fixture,
                role.name,
                role.pixel_color
            );
        }
        if case.fixture == "katana/sample_basic.md" {
            assert!(
                has_syntax_pixel(rendered.scene.tree.root(), &rendered.canvas),
                "{} missing syntax-colored frame pixel",
                case.fixture
            );
        }
    }
    Ok(())
}

#[test]
fn katana_alert_nodes_keep_left_rule_style_in_storybook_path()
-> Result<(), Box<dyn std::error::Error>> {
    let rendered = render_fixture("katana/sample_basic.md")?;
    let alerts = collect_alert_nodes(rendered.scene.tree.root());
    assert!(alerts.len() >= 5, "sample_basic alert nodes missing");

    let alert_background = dark_theme_rgb("alert-background")?;
    assert_alert_token_reaches_scene(&alerts, "alert-note");
    assert_alert_token_reaches_scene(&alerts, "alert-tip");
    assert_alert_token_reaches_scene(&alerts, "alert-important");
    assert_alert_token_reaches_scene(&alerts, "alert-warning");
    assert_alert_token_reaches_scene(&alerts, "alert-caution");
    assert_alert_color_reaches_frame(&rendered.canvas, "alert-note")?;
    assert_alert_color_reaches_frame(&rendered.canvas, "alert-tip")?;
    assert_alert_color_reaches_frame(&rendered.canvas, "alert-important")?;
    assert_alert_color_reaches_frame(&rendered.canvas, "alert-warning")?;
    assert_alert_color_reaches_frame(&rendered.canvas, "alert-caution")?;

    for alert in alerts {
        let canvas = render_single_alert(alert)?;
        assert!(
            count_color(&canvas, alert_background) <= 4,
            "alert must not be rendered as a filled card: {}",
            alert.props().label
        );
    }
    Ok(())
}

#[test]
fn code_copy_control_does_not_render_blue_filled_button_in_storybook_frame()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = fixture("katana/sample_basic.md");
    let scene = PreviewBuilder::default().build(
        &fixture,
        ViewerViewport {
            width: PREVIEW_WIDTH,
            height: PREVIEW_HEIGHT,
        },
        true,
        ViewerInteractionConfig::default(),
    )?;
    let hit = StorybookHostActionHits::hits(&scene, FRAME_WIDTH)
        .into_iter()
        .filter(|hit| hit.action.action_id == code_copy_action_id())
        .min_by_key(|hit| hit.rect.area())
        .ok_or_else(|| std::io::Error::other("missing code copy host action hit"))?;
    let frame_height = 900;
    let scroll_y = (hit.rect.y as f32 - 120.0).max(0.0);
    let canvas = render_scene_with_scroll(&fixture, &scene, frame_height, scroll_y);
    let area = StorybookPreviewArea::for_window(FRAME_WIDTH, frame_height, scroll_y);
    let x = area.x.saturating_add(hit.rect.x);
    let y = area
        .y
        .saturating_add((hit.rect.y as f32 - scroll_y).round().max(0.0) as usize);

    assert_eq!(
        0,
        count_color_in_rect(
            &canvas,
            KUC_DARK_SELECTION,
            x,
            y,
            hit.rect.width,
            hit.rect.height,
        ),
        "code copy control must not render the old filled blue button chrome"
    );
    Ok(())
}

fn fixture_cases(
    code_background_color: u32,
    table_row_color: u32,
    alert_warning_color: u32,
    quote_background_color: u32,
    footnote_background_color: u32,
) -> Vec<FixtureRolePixelCase> {
    vec![
        FixtureRolePixelCase {
            fixture: "katana/sample_basic.md",
            roles: vec![
                RolePixelExpectation::new("heading", KUC_DARK_SELECTION, 1, 512),
                RolePixelExpectation::new("code", code_background_color, 1, 512),
                RolePixelExpectation::new("table", table_row_color, 1, 512),
                RolePixelExpectation::new("alert", alert_warning_color, 1, 16),
                RolePixelExpectation::new("blockquote", quote_background_color, 1, 128),
                RolePixelExpectation::new("footnote", footnote_background_color, 1, 128),
                RolePixelExpectation::new("list-marker", KUC_DARK_SELECTION, 1, 512),
            ],
        },
        FixtureRolePixelCase {
            fixture: "direct/html-alignment.html",
            roles: vec![
                RolePixelExpectation::new("html-centered-preview", ANY_PREVIEW_CONTENT, 1, 128),
                RolePixelExpectation::new("html-right-preview", ANY_PREVIEW_CONTENT, 1, 128),
                RolePixelExpectation::new("html-left-preview", ANY_PREVIEW_CONTENT, 1, 128),
                RolePixelExpectation::new("table", table_row_color, 1, 128),
            ],
        },
    ]
}

fn dark_theme_rgb(name: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let theme = KucThemeBridge::from_kdv(&KdvThemeSnapshot::katana_dark())?;
    let rgba = theme
        .color(name)
        .ok_or_else(|| format!("missing KUC theme color token: {name}"))?;
    Ok(((rgba[0] as u32) << 16) | ((rgba[1] as u32) << 8) | rgba[2] as u32)
}

fn assert_alert_token_reaches_scene(alerts: &[&UiNode], token: &str) {
    assert!(
        alerts
            .iter()
            .any(|node| node.props().common.border.color_token == token),
        "alert token missing from KDV->KUC scene: {token}"
    );
}

fn assert_alert_color_reaches_frame(
    canvas: &Canvas,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let color = alert_token_color(token)?;
    assert!(
        count_color(canvas, color) >= 16,
        "alert color #{color:06x} missing from Storybook frame for {token}"
    );
    Ok(())
}

fn alert_token_color(token: &str) -> Result<u32, Box<dyn std::error::Error>> {
    match token {
        "alert-tip" => dark_theme_rgb("alert-tip"),
        "alert-important" => dark_theme_rgb("alert-important"),
        "alert-warning" => dark_theme_rgb("alert-warning"),
        "alert-caution" => dark_theme_rgb("alert-caution"),
        "alert-note" => dark_theme_rgb("alert-note"),
        token => Err(format!("unexpected alert color token: {token}").into()),
    }
}

fn render_single_alert(node: &UiNode) -> Result<Canvas, Box<dyn std::error::Error>> {
    let theme = KucThemeBridge::from_kdv(&KdvThemeSnapshot::katana_dark())?;
    let background = dark_theme_rgb("background")?;
    let mut canvas = Canvas::new(420, 160, background);
    UiTreeStorybookHost::new(theme).render(
        &mut canvas,
        node,
        UiTreeRenderArea {
            x: 16,
            y: 16,
            width: 380,
            height: 128,
            scroll_y: 0.0,
        },
    );
    Ok(canvas)
}

fn collect_alert_nodes(node: &UiNode) -> Vec<&UiNode> {
    let mut alerts = Vec::new();
    collect_alert_nodes_into(node, &mut alerts);
    alerts
}

fn collect_alert_nodes_into<'a>(node: &'a UiNode, alerts: &mut Vec<&'a UiNode>) {
    if node.props().text.role == "alert" {
        alerts.push(node);
    }
    for child in node.children() {
        collect_alert_nodes_into(child, alerts);
    }
}

fn render_fixture(path: &str) -> Result<RenderedFixture, Box<dyn std::error::Error>> {
    let fixture = fixture(path);
    let scene = PreviewBuilder::default().build(
        &fixture,
        ViewerViewport {
            width: PREVIEW_WIDTH,
            height: PREVIEW_HEIGHT,
        },
        true,
        ViewerInteractionConfig::default(),
    )?;
    let height = frame_height(&scene);
    let canvas = StorybookFrameRenderer::render(FrameRenderRequest {
        width: FRAME_WIDTH,
        height,
        fixtures: &[fixture],
        selected_index: 0,
        scene: Some(&scene),
        scroll_y: 0.0,
        sidebar_scroll: Default::default(),
        file_tree_state: Default::default(),
        settings_state: &Default::default(),
        dark: true,
        interaction: &ViewerInteractionConfig::default(),
        typography: Default::default(),
        last_command_label: "none",
        task_context_menu: None,
        hovered_node_id: None,
        hovered_action_node_id: None,
        animation_phase: 0,
    });
    Ok(RenderedFixture { canvas, scene })
}

fn render_scene_with_scroll(
    fixture: &StorybookFixture,
    scene: &PreviewScene,
    height: usize,
    scroll_y: f32,
) -> Canvas {
    StorybookFrameRenderer::render(FrameRenderRequest {
        width: FRAME_WIDTH,
        height,
        fixtures: std::slice::from_ref(fixture),
        selected_index: 0,
        scene: Some(scene),
        scroll_y,
        sidebar_scroll: Default::default(),
        file_tree_state: Default::default(),
        settings_state: &Default::default(),
        dark: true,
        interaction: &ViewerInteractionConfig::default(),
        typography: Default::default(),
        last_command_label: "none",
        task_context_menu: None,
        hovered_node_id: None,
        hovered_action_node_id: None,
        animation_phase: 0,
    })
}

fn frame_height(scene: &PreviewScene) -> usize {
    scene
        .surface
        .as_ref()
        .map(|surface| surface.height as usize + crate::layout::HEADER_HEIGHT + 96)
        .unwrap_or(FRAME_HEIGHT)
        .max(FRAME_HEIGHT)
}

fn count_role(node: &UiNode, role: &str) -> usize {
    usize::from(node.props().text.role == role)
        + node
            .children()
            .iter()
            .map(|child| count_role(child, role))
            .sum::<usize>()
}

fn count_color(canvas: &Canvas, color: u32) -> usize {
    FramePreviewPixels::count_color(canvas, color)
}

fn count_color_in_rect(
    canvas: &Canvas,
    color: u32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> usize {
    let x_end = x.saturating_add(width).min(canvas.width());
    let y_end = y.saturating_add(height).min(canvas.height());
    let mut count = 0;
    for row in y..y_end {
        count += (x..x_end)
            .filter(|column| canvas.pixels()[row * canvas.width() + *column] == color)
            .count();
    }
    count
}

fn count_pixels(canvas: &Canvas, color: u32) -> usize {
    if color == ANY_PREVIEW_CONTENT {
        return FramePreviewPixels::count_non_background(canvas, true);
    }
    count_color(canvas, color)
}

fn has_syntax_pixel(node: &UiNode, canvas: &Canvas) -> bool {
    node.props().text.spans.iter().any(|span| {
        let color = span.style.color_rgba;
        color[3] > 0 && span.style.monospace && count_color(canvas, rgba_to_rgb(color)) > 0
    }) || node
        .children()
        .iter()
        .any(|child| has_syntax_pixel(child, canvas))
}

fn rgba_to_rgb(value: [u8; 4]) -> u32 {
    (u32::from(value[0]) << 16) | (u32::from(value[1]) << 8) | u32::from(value[2])
}

fn fixture(path: &str) -> StorybookFixture {
    StorybookFixture {
        label: path.to_string(),
        path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../assets/fixtures/{path}")),
    }
}

fn code_copy_action_id() -> String {
    ViewerMediaControlAction::host_action_id_for(ViewerMediaControlKind::Code, "copy-code")
}

struct RenderedFixture {
    canvas: Canvas,
    scene: PreviewScene,
}
