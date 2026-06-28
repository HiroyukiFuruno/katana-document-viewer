use super::frame_surface_dump::{SurfaceDump, SurfaceDumpImage};
use super::frame_surface_similarity::SurfaceParityScorer;
use super::{FrameRenderRequest, StorybookFrameRenderer};
use crate::canvas::Canvas;
use crate::catalog::StorybookFixture;
use crate::layout::{HEADER_HEIGHT, SIDEBAR_WIDTH, preview_content_width};
use crate::preview::PreviewBuilder;
use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use katana_document_viewer::{
    ViewerInteractionConfig, ViewerMode, ViewerSearchState, ViewerTarget, ViewerViewport,
};
use katana_ui_core::render_model::{UiDimension, UiNode, UiNodeKind};
use std::collections::BTreeMap;
use std::path::PathBuf;

const SURFACE_WIDTH: usize = 1280;
const FRAME_WIDTH: usize = SIDEBAR_WIDTH + SURFACE_WIDTH * 2 + 32;
const FRAME_MIN_HEIGHT: usize = 12000;
const PREVIEW_HEIGHT: f32 = 11800.0;
const MINIMUM_SURFACE_SCORE: u8 = 95;
const RGBA_CHANNELS: usize = 4;
const VIEWER_SURFACE_PADDING_TOP: usize = 16;
const EXPORT_MEDIA_VERTICAL_MARGIN_PX: i32 = 18;
const KATANA_VIEWER_SURFACE_PARITY_FAST_FIXTURES: &[&str] = &[
    "direct/html-alignment.htm",
    "direct/html-alignment.html",
    "direct/kdv-icon.png",
    "direct/kdv-icon.svg",
    "katana/sample_html.md",
];

const KATANA_VIEWER_SURFACE_PARITY_DIAGRAM_FIXTURES: &[&str] = &[
    "direct/sample.md",
    "direct/sample.drawio",
    "direct/sample.drowio",
    "direct/sample.mermaid",
    "direct/sample.mmd",
    "direct/sample.plantuml",
    "direct/sample.puml",
    "katana/sample.md",
    "katana/sample_basic.md",
    "katana/sample_diagrams.md",
];

const DIAGRAM_FIXTURE_EXTENSIONS: &[&str] = &[
    ".drawio",
    ".drowio",
    ".mermaid",
    ".mmd",
    ".plantuml",
    ".puml",
];

#[test]
#[ignore = "score gate runs exact surface parity fixtures explicitly"]
fn storybook_frame_matches_export_surface_for_katana_viewer()
-> Result<(), Box<dyn std::error::Error>> {
    SurfaceParitySupport::assert_fixtures(KATANA_VIEWER_SURFACE_PARITY_FAST_FIXTURES)
}

#[test]
fn fast_surface_parity_fixtures_do_not_include_diagram_sources() {
    for fixture in KATANA_VIEWER_SURFACE_PARITY_FAST_FIXTURES {
        assert!(
            !is_diagram_heavy_fixture(fixture),
            "fast surface parity fixture must not include diagram-heavy source: {fixture}"
        );
    }
}

#[test]
fn surface_parity_frame_hosts_full_export_surface_width() {
    assert!(
        preview_content_width(FRAME_WIDTH) >= SURFACE_WIDTH,
        "surface parity frame must host the full export surface width: expected at least {SURFACE_WIDTH}, got {}",
        preview_content_width(FRAME_WIDTH)
    );
}

#[test]
#[ignore = "score gate runs exact diagram-heavy surface parity fixtures explicitly"]
fn storybook_frame_matches_export_surface_for_katana_viewer_diagrams()
-> Result<(), Box<dyn std::error::Error>> {
    SurfaceParitySupport::assert_fixtures(KATANA_VIEWER_SURFACE_PARITY_DIAGRAM_FIXTURES)
}

#[test]
fn katana_sample_export_surface_tree_places_table_at_target_plus_padding()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = SurfaceParitySupport::build_export_tree_scene("katana/sample.md", false)?;
    let badge_note_y = SurfaceParitySupport::direct_child_y_for_label(
        scene.tree.root(),
        "Three badges should appear",
    )
    .ok_or("badge note child must be present in export surface tree")?;
    let badge_note_target_y =
        SurfaceParitySupport::target_y_for_raw(&scene, "Three badges should appear")
            .ok_or("badge note target must be present")?;
    let long_inline_code_y = SurfaceParitySupport::direct_child_y_for_label(
        scene.tree.root(),
        "This is a very long line to verify horizontal scrolling",
    )
    .ok_or("long inline code child must be present in export surface tree")?;
    let long_inline_code_target_y = SurfaceParitySupport::target_y_for_raw(
        &scene,
        "This is a very long line to verify horizontal scrolling",
    )
    .ok_or("long inline code target must be present")?;
    let table_y = SurfaceParitySupport::direct_child_y_for_role_and_label(
        scene.tree.root(),
        "table",
        "Table after list",
    )
    .ok_or("table child must be present in export surface tree")?;
    let target_y = SurfaceParitySupport::target_y_for_raw(&scene, "Table after list")
        .ok_or("table target must be present")?;

    assert_eq!(badge_note_target_y, badge_note_y);
    assert_eq!(long_inline_code_target_y, long_inline_code_y);
    assert_eq!(target_y, table_y);
    Ok(())
}

#[test]
fn katana_sample_export_surface_tree_places_blockquote_at_target_plus_padding()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = SurfaceParitySupport::build_export_tree_scene("katana/sample.md", false)?;
    let blockquote_y = SurfaceParitySupport::tree_y_for_label(scene.tree.root(), "Bold quote")
        .ok_or_else(|| {
            format!(
                "decorated blockquote child must be present in export surface tree: {:?}",
                SurfaceParitySupport::ui_nodes_by_role(&scene, "blockquote")
            )
        })?;
    let target_y = SurfaceParitySupport::target_y_for_raw(&scene, "> **Bold quote**")
        .ok_or("decorated blockquote target must be present")?;

    assert_eq!(target_y, blockquote_y);
    Ok(())
}

#[test]
fn katana_sample_export_surface_tree_places_note_block_at_target_plus_padding()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = SurfaceParitySupport::build_export_tree_scene("katana/sample.md", false)?;
    let note_y = SurfaceParitySupport::tree_y_for_label(
        scene.tree.root(),
        "Note GitHub",
    )
    .ok_or_else(|| {
        format!(
            "legacy note block child must be present in export surface tree: blockquotes={:?} alerts={:?}",
            SurfaceParitySupport::ui_nodes_by_role(&scene, "blockquote"),
            SurfaceParitySupport::ui_nodes_by_role(&scene, "alert")
        )
    })?;
    let target_y = SurfaceParitySupport::target_y_for_raw(&scene, "> **Note**")
        .ok_or("legacy note target must be present")?;

    assert_eq!(target_y, note_y);
    Ok(())
}

#[test]
fn katana_sample_export_surface_tree_places_graph_diagram_at_target_plus_padding()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = SurfaceParitySupport::build_export_tree_scene("katana/sample.md", false)?;
    let target = SurfaceParitySupport::target_for_raw(&scene, "```mermaid\ngraph LR")
        .ok_or("graph diagram target must be present")?;
    let diagram_y = SurfaceParitySupport::direct_child_y_for_semantic_id(
        scene.tree.root(),
        target.node_id.0.as_str(),
    )
    .ok_or("graph diagram child must be present in export surface tree")?;

    assert_eq!(
        target.rect.y.round() as i32 - EXPORT_MEDIA_VERTICAL_MARGIN_PX,
        diagram_y,
        "export surface tree must keep KDV plan gap before graph diagrams and reserve export media top margin: {}",
        SurfaceParitySupport::direct_children_summary_around(
            scene.tree.root(),
            target.rect.y.round() as i32,
            80,
        )
    );
    Ok(())
}

#[test]
fn katana_sample_long_inline_code_uses_export_surface_text_width()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = SurfaceParitySupport::build_export_tree_scene("katana/sample.md", false)?;
    let node = SurfaceParitySupport::direct_child_for_label(
        scene.tree.root(),
        "This is a very long line to verify horizontal scrolling",
    )
    .ok_or("long inline code child must be present in export surface tree")?;

    assert_eq!(
        UiDimension::Px(1168),
        node.props().common.width,
        "long inline code must use KatanA/export surface text width, not full surface width"
    );
    Ok(())
}

#[test]
fn direct_html_export_surface_tree_preserves_right_link_span()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = SurfaceParitySupport::build_export_tree_scene("direct/html-alignment.htm", false)?;
    let node =
        SurfaceParitySupport::direct_child_for_label(scene.tree.root(), "Right aligned link")
            .ok_or("right aligned link child must be present in export surface tree")?;

    assert_eq!("html-right", node.props().text.role);
    assert_eq!(UiDimension::Px(1168), node.props().common.width);
    assert_eq!(
        "https://example.com/kdv",
        node.props().text.spans[0].link_target
    );
    Ok(())
}

#[test]
fn direct_html_surface_parity_scene_preserves_right_link_span()
-> Result<(), Box<dyn std::error::Error>> {
    let rendered = SurfaceParitySupport::render_fixture("direct/html-alignment.htm", false)?;
    let node = SurfaceParitySupport::direct_child_for_label(
        rendered.scene.tree.root(),
        "Right aligned link",
    )
    .ok_or("right aligned link child must be present in surface parity scene")?;

    assert_eq!("html-right", node.props().text.role);
    assert_eq!(
        "https://example.com/kdv",
        node.props().text.spans[0].link_target
    );
    Ok(())
}

fn is_diagram_heavy_fixture(fixture: &str) -> bool {
    DIAGRAM_FIXTURE_EXTENSIONS
        .iter()
        .any(|extension| fixture.ends_with(extension))
        || matches!(
            fixture,
            "direct/sample.md"
                | "katana/sample.md"
                | "katana/sample_basic.md"
                | "katana/sample_diagrams.md"
        )
}

impl SurfaceParitySupport {
    fn assert_fixtures(fixtures: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(selected) = std::env::var("KDV_STORYBOOK_SURFACE_PARITY_FIXTURE") {
            for fixture in selected
                .split(',')
                .map(str::trim)
                .filter(|it| !it.is_empty())
            {
                Self::assert_fixture(fixture)?;
            }
            return Ok(());
        }
        for fixture in fixtures {
            Self::assert_fixture(fixture)?;
        }
        Ok(())
    }

    fn assert_fixture(fixture: &str) -> Result<(), Box<dyn std::error::Error>> {
        let rendered = SurfaceParitySupport::render_fixture(fixture, false)?;
        let surface = rendered
            .scene
            .surface
            .as_ref()
            .ok_or("loaded preview scene did not expose export surface")?;
        let preview = SurfaceParitySupport::preview_rgba(&rendered.canvas, surface.height);
        let report = SurfaceParityScorer::report(
            &surface.rgba,
            &preview,
            surface.width as usize,
            surface.height as usize,
        );
        if let Ok(directory) = std::env::var("KDV_STORYBOOK_SURFACE_DUMP_DIR") {
            SurfaceDump::write_pair(
                std::path::Path::new(&directory),
                fixture,
                SurfaceDumpImage::new(
                    &surface.rgba,
                    surface.width as usize,
                    surface.height as usize,
                ),
                SurfaceDumpImage::new(&preview, surface.width as usize, surface.height as usize),
            )?;
        }
        if report.score >= MINIMUM_SURFACE_SCORE {
            return Ok(());
        }
        let reference_bounds = SurfaceParitySupport::content_bounds(
            &surface.rgba,
            surface.width as usize,
            surface.height as usize,
        );
        let preview_bounds = SurfaceParitySupport::content_bounds(
            &preview,
            surface.width as usize,
            surface.height as usize,
        );
        let reference_first = SurfaceParitySupport::first_content_sample(
            &surface.rgba,
            surface.width as usize,
            surface.height as usize,
        );
        let preview_first = SurfaceParitySupport::first_content_sample(
            &preview,
            surface.width as usize,
            surface.height as usize,
        );
        let preview_right = SurfaceParitySupport::first_content_on_right_edge(
            &preview,
            surface.width as usize,
            surface.height as usize,
        );
        let preview_bottom = SurfaceParitySupport::first_content_on_bottom_edge(
            &preview,
            surface.width as usize,
            surface.height as usize,
        );
        let preview_after_reference_bottom = SurfaceParitySupport::first_content_after_y(
            &preview,
            surface.width as usize,
            surface.height as usize,
            reference_bounds.map_or(0, |bounds| bounds.max_y.saturating_add(1)),
        );
        let right_target = preview_right
            .and_then(|sample| SurfaceParitySupport::target_at(&rendered.scene, sample.y));
        let bottom_target = preview_bottom
            .and_then(|sample| SurfaceParitySupport::target_at(&rendered.scene, sample.y));
        let after_reference_target = preview_after_reference_bottom
            .and_then(|sample| SurfaceParitySupport::target_at(&rendered.scene, sample.y));
        let right_node = right_target.as_ref().and_then(|target| {
            SurfaceParitySupport::ui_node_sample(&rendered.scene, &target.node_id)
        });
        let last_target = SurfaceParitySupport::last_target(&rendered.scene);
        let code_nodes = SurfaceParitySupport::ui_nodes_by_role(&rendered.scene, "code");
        let table_nodes = SurfaceParitySupport::ui_nodes_by_role(&rendered.scene, "table");
        let accordion_nodes =
            SurfaceParitySupport::ui_nodes_by_role(&rendered.scene, "html-accordion");
        let accordion_body_nodes =
            SurfaceParitySupport::ui_nodes_by_role(&rendered.scene, "html-accordion-body");
        let row_loss_targets = SurfaceParitySupport::row_loss_targets(
            &rendered.scene,
            &report.reference_row_loss_bands,
        );
        let table_row_theme = rendered.scene.theme.color("table-row-background");
        let table_header_theme = rendered.scene.theme.color("table-header-background");

        assert!(
            report.score >= MINIMUM_SURFACE_SCORE,
            "{fixture} storybook surface score {}/{} average={} content={} row={} row_ref_to_preview={} row_preview_to_ref={} row_loss_targets={:?} surface_height={} content_height={} table_row_theme={:?} table_header_theme={:?} reference={:?} preview={:?} reference_first={:?} preview_first={:?} preview_right={:?} preview_bottom={:?} preview_after_reference_bottom={:?} right_target={:?} bottom_target={:?} after_reference_target={:?} last_target={:?} right_node={:?} code_nodes={:?} table_nodes={:?} accordion_nodes={:?} accordion_body_nodes={:?}",
            report.score,
            MINIMUM_SURFACE_SCORE,
            report.average_score,
            report.content_score,
            report.row_score,
            report.reference_to_candidate_row_score,
            report.candidate_to_reference_row_score,
            row_loss_targets,
            surface.height,
            rendered.scene.content_height,
            table_row_theme,
            table_header_theme,
            reference_bounds,
            preview_bounds,
            reference_first,
            preview_first,
            preview_right,
            preview_bottom,
            preview_after_reference_bottom,
            right_target,
            bottom_target,
            after_reference_target,
            last_target,
            right_node,
            code_nodes,
            table_nodes,
            accordion_nodes,
            accordion_body_nodes
        );
        Ok(())
    }
}

struct SurfaceParitySupport;

impl SurfaceParitySupport {
    fn build_export_tree_scene(
        path: &str,
        dark: bool,
    ) -> Result<crate::preview::PreviewScene, Box<dyn std::error::Error>> {
        let fixture = Self::fixture(path);
        let builder = PreviewBuilder::default();
        builder.build_scene(PreviewBuildRequest {
            fixture: &fixture,
            viewport: Self::viewport(PREVIEW_HEIGHT),
            dark,
            theme: None,
            interaction: Self::surface_parity_interaction(),
            mode: ViewerMode::Document,
            typography: Default::default(),
            search: ViewerSearchState::default(),
            diagram_viewports: BTreeMap::new(),
            image_viewports: BTreeMap::new(),
            task_state_overrides: BTreeMap::new(),
            accordion_open_overrides: BTreeMap::new(),
            copied_code_node_ids: Default::default(),
            asset_mode: PreviewBuildAssetMode::VisibleAndNearViewport,
            attach_surface: false,
            export_surface: true,
        })
    }

    fn render_fixture(
        path: &str,
        dark: bool,
    ) -> Result<RenderedSurfaceFixture, Box<dyn std::error::Error>> {
        let fixture = Self::fixture(path);
        let builder = PreviewBuilder::default();
        let initial_scene = Self::build_parity_scene_with_surface_attachment(
            &builder,
            &fixture,
            Self::viewport(PREVIEW_HEIGHT),
            dark,
            Self::surface_parity_interaction(),
            BTreeMap::new(),
            false,
        )?;
        let accordion_open_overrides = Self::accordion_open_overrides(&initial_scene);
        let scene = Self::build_parity_scene(
            &builder,
            &fixture,
            Self::viewport(initial_scene.content_height.max(PREVIEW_HEIGHT)),
            dark,
            Self::surface_parity_interaction(),
            accordion_open_overrides,
        )?;
        let frame_height = scene
            .surface
            .as_ref()
            .map(|surface| surface.height as usize + HEADER_HEIGHT + 96)
            .unwrap_or(FRAME_MIN_HEIGHT)
            .max(FRAME_MIN_HEIGHT);
        let canvas = StorybookFrameRenderer::render(FrameRenderRequest {
            width: FRAME_WIDTH,
            height: frame_height,
            fixtures: &[fixture],
            selected_index: 0,
            scene: Some(&scene),
            scroll_y: 0.0,
            sidebar_scroll: Default::default(),
            file_tree_state: Default::default(),
            settings_state: &Default::default(),
            dark,
            interaction: &Self::surface_parity_interaction(),
            typography: Default::default(),
            last_command_label: "none",
            task_context_menu: None,
            hovered_node_id: None,
            hovered_action_node_id: None,
            animation_phase: 0,
        });
        Ok(RenderedSurfaceFixture { canvas, scene })
    }

    fn fixture(path: &str) -> StorybookFixture {
        StorybookFixture {
            label: path.to_string(),
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(format!("../../assets/fixtures/{path}")),
        }
    }

    fn viewport(height: f32) -> ViewerViewport {
        ViewerViewport {
            width: SURFACE_WIDTH as f32,
            height,
        }
    }

    fn direct_child_y_for_role_and_label(
        root: &UiNode,
        role: &str,
        label_fragment: &str,
    ) -> Option<i32> {
        let column = root.children().first()?;
        let mut y = Self::padding_top_px(column);
        for child in column.children() {
            if child.props().text.role == role && child.props().label.contains(label_fragment) {
                return Some(y);
            }
            y += Self::node_height_px(child);
        }
        None
    }

    fn direct_child_y_for_label(root: &UiNode, label_fragment: &str) -> Option<i32> {
        Self::direct_child_with_y_for_label(root, label_fragment).map(|(_, y)| y)
    }

    fn direct_child_y_for_semantic_id(root: &UiNode, semantic_node_id: &str) -> Option<i32> {
        let column = root.children().first()?;
        let mut y = Self::padding_top_px(column);
        for child in column.children() {
            if child.props().common.semantic_node_id == semantic_node_id {
                return Some(y);
            }
            y += Self::node_height_px(child);
        }
        None
    }

    fn direct_child_for_label<'a>(root: &'a UiNode, label_fragment: &str) -> Option<&'a UiNode> {
        Self::direct_child_with_y_for_label(root, label_fragment).map(|(node, _)| node)
    }

    fn tree_y_for_label(root: &UiNode, label_fragment: &str) -> Option<i32> {
        Self::tree_node_with_y_for_label(root, label_fragment, 0).map(|(_, y)| y)
    }

    fn tree_node_with_y_for_label<'a>(
        node: &'a UiNode,
        label_fragment: &str,
        y: i32,
    ) -> Option<(&'a UiNode, i32)> {
        if node.props().label.contains(label_fragment) {
            return Some((node, y));
        }
        let mut child_y = y + Self::padding_top_px(node);
        for child in node.children() {
            if let Some(result) = Self::tree_node_with_y_for_label(child, label_fragment, child_y) {
                return Some(result);
            }
            child_y += Self::node_height_px(child);
        }
        None
    }

    fn direct_child_with_y_for_label<'a>(
        root: &'a UiNode,
        label_fragment: &str,
    ) -> Option<(&'a UiNode, i32)> {
        let column = root.children().first()?;
        let mut y = Self::padding_top_px(column);
        for child in column.children() {
            if child.props().label.contains(label_fragment) {
                return Some((child, y));
            }
            y += Self::node_height_px(child);
        }
        None
    }

    fn direct_children_summary_around(root: &UiNode, center_y: i32, radius: i32) -> String {
        let Some(column) = root.children().first() else {
            return "no content column".to_string();
        };
        let mut y = Self::padding_top_px(column);
        let mut samples = Vec::new();
        for child in column.children() {
            let height = Self::node_height_px(child);
            if y + height >= center_y - radius && y <= center_y + radius {
                samples.push(format!(
                    "y={y} h={height} kind={:?} role={} semantic={} label={:?}",
                    child.kind(),
                    child.props().text.role,
                    child.props().common.semantic_node_id,
                    child.props().label.chars().take(48).collect::<String>()
                ));
            }
            y += height;
        }
        samples.join(" | ")
    }

    fn target_y_for_raw(scene: &crate::preview::PreviewScene, raw_fragment: &str) -> Option<i32> {
        Self::target_for_raw(scene, raw_fragment).map(|target| target.rect.y.round() as i32)
    }

    fn target_for_raw<'a>(
        scene: &'a crate::preview::PreviewScene,
        raw_fragment: &str,
    ) -> Option<&'a ViewerTarget> {
        scene
            .targets
            .iter()
            .find(|target| target.source.raw.text.contains(raw_fragment))
    }

    fn padding_top_px(node: &UiNode) -> i32 {
        Self::dimension_px(&node.props().common.padding.top)
    }

    fn node_height_px(node: &UiNode) -> i32 {
        if node.kind() == UiNodeKind::Text {
            return Self::dimension_px(&node.props().common.height);
        }
        Self::dimension_px(&node.props().common.height)
    }

    fn dimension_px(value: &UiDimension) -> i32 {
        match value {
            UiDimension::Px(value) => i32::from(*value),
            _ => 0,
        }
    }

    fn build_parity_scene(
        builder: &PreviewBuilder,
        fixture: &StorybookFixture,
        viewport: ViewerViewport,
        dark: bool,
        interaction: ViewerInteractionConfig,
        accordion_open_overrides: BTreeMap<String, bool>,
    ) -> Result<crate::preview::PreviewScene, Box<dyn std::error::Error>> {
        Self::build_parity_scene_with_surface_attachment(
            builder,
            fixture,
            viewport,
            dark,
            interaction,
            accordion_open_overrides,
            true,
        )
    }

    fn build_parity_scene_with_surface_attachment(
        builder: &PreviewBuilder,
        fixture: &StorybookFixture,
        viewport: ViewerViewport,
        dark: bool,
        interaction: ViewerInteractionConfig,
        accordion_open_overrides: BTreeMap<String, bool>,
        attach_surface: bool,
    ) -> Result<crate::preview::PreviewScene, Box<dyn std::error::Error>> {
        builder.build_scene(PreviewBuildRequest {
            fixture,
            viewport,
            dark,
            theme: None,
            interaction,
            mode: ViewerMode::Document,
            typography: Default::default(),
            search: ViewerSearchState::default(),
            diagram_viewports: BTreeMap::new(),
            image_viewports: BTreeMap::new(),
            task_state_overrides: BTreeMap::new(),
            accordion_open_overrides,
            copied_code_node_ids: Default::default(),
            asset_mode: PreviewBuildAssetMode::VisibleAndNearViewport,
            attach_surface,
            export_surface: true,
        })
    }

    fn accordion_open_overrides(scene: &crate::preview::PreviewScene) -> BTreeMap<String, bool> {
        let mut overrides = BTreeMap::new();
        Self::collect_accordion_open_overrides(scene.tree.root(), &mut overrides);
        overrides
    }

    fn collect_accordion_open_overrides(
        node: &katana_ui_core::render_model::UiNode,
        overrides: &mut BTreeMap<String, bool>,
    ) {
        if node.props().text.role == "html-accordion" {
            overrides.insert(node.id().as_str().to_string(), true);
        }
        for child in node.children() {
            Self::collect_accordion_open_overrides(child, overrides);
        }
    }

    fn surface_parity_interaction() -> ViewerInteractionConfig {
        ViewerInteractionConfig {
            hover_highlight_enabled: false,
            selection_enabled: false,
            image_controls_enabled: false,
            diagram_controls_enabled: false,
            code_controls_enabled: false,
        }
    }

    fn preview_rgba(canvas: &Canvas, surface_height: u32) -> Vec<u8> {
        let mut rgba = Vec::with_capacity(SURFACE_WIDTH * surface_height as usize * RGBA_CHANNELS);
        let start_x = SIDEBAR_WIDTH + 16;
        let start_y = HEADER_HEIGHT + 16;
        for y in 0..surface_height as usize {
            let row = start_y + y;
            for x in start_x..start_x + SURFACE_WIDTH {
                Self::push_color(&mut rgba, canvas.pixels()[row * canvas.width() + x]);
            }
        }
        rgba
    }

    fn push_color(rgba: &mut Vec<u8>, color: u32) {
        rgba.push(((color >> 16) & 0xff) as u8);
        rgba.push(((color >> 8) & 0xff) as u8);
        rgba.push((color & 0xff) as u8);
        rgba.push(0xff);
    }

    fn content_bounds(rgba: &[u8], width: usize, height: usize) -> Option<ContentBounds> {
        let background = rgba.first_chunk::<RGBA_CHANNELS>().copied()?;
        let mut bounds = ContentBounds::empty();
        for y in 0..height {
            for x in 0..width {
                let offset = (y * width + x) * RGBA_CHANNELS;
                if Self::is_content(&rgba[offset..offset + RGBA_CHANNELS], &background) {
                    bounds.include(x, y);
                }
            }
        }
        bounds.into_option()
    }

    fn is_content(pixel: &[u8], background: &[u8; RGBA_CHANNELS]) -> bool {
        pixel
            .iter()
            .zip(background.iter())
            .take(3)
            .map(|(left, right)| left.abs_diff(*right) as u16)
            .sum::<u16>()
            >= 30
    }

    fn first_content_sample(rgba: &[u8], width: usize, height: usize) -> Option<ContentSample> {
        let background = rgba.first_chunk::<RGBA_CHANNELS>().copied()?;
        for y in 0..height {
            for x in 0..width {
                let offset = (y * width + x) * RGBA_CHANNELS;
                let pixel = &rgba[offset..offset + RGBA_CHANNELS];
                if Self::is_content(pixel, &background) {
                    return Some(ContentSample {
                        x,
                        y,
                        rgb: [pixel[0], pixel[1], pixel[2]],
                    });
                }
            }
        }
        None
    }

    fn first_content_on_right_edge(
        rgba: &[u8],
        width: usize,
        height: usize,
    ) -> Option<ContentSample> {
        let background = rgba.first_chunk::<RGBA_CHANNELS>().copied()?;
        let x = width.checked_sub(1)?;
        for y in 0..height {
            let offset = (y * width + x) * RGBA_CHANNELS;
            let pixel = &rgba[offset..offset + RGBA_CHANNELS];
            if Self::is_content(pixel, &background) {
                return Some(ContentSample {
                    x,
                    y,
                    rgb: [pixel[0], pixel[1], pixel[2]],
                });
            }
        }
        None
    }

    fn first_content_on_bottom_edge(
        rgba: &[u8],
        width: usize,
        height: usize,
    ) -> Option<ContentSample> {
        let background = rgba.first_chunk::<RGBA_CHANNELS>().copied()?;
        let y = height.checked_sub(1)?;
        for x in 0..width {
            let offset = (y * width + x) * RGBA_CHANNELS;
            let pixel = &rgba[offset..offset + RGBA_CHANNELS];
            if Self::is_content(pixel, &background) {
                return Some(ContentSample {
                    x,
                    y,
                    rgb: [pixel[0], pixel[1], pixel[2]],
                });
            }
        }
        None
    }

    fn first_content_after_y(
        rgba: &[u8],
        width: usize,
        height: usize,
        start_y: usize,
    ) -> Option<ContentSample> {
        let background = rgba.first_chunk::<RGBA_CHANNELS>().copied()?;
        for y in start_y..height {
            for x in 0..width {
                let offset = (y * width + x) * RGBA_CHANNELS;
                let pixel = &rgba[offset..offset + RGBA_CHANNELS];
                if Self::is_content(pixel, &background) {
                    return Some(ContentSample {
                        x,
                        y,
                        rgb: [pixel[0], pixel[1], pixel[2]],
                    });
                }
            }
        }
        None
    }

    fn target_at(scene: &crate::preview::PreviewScene, y: usize) -> Option<TargetSample> {
        let y = y.saturating_sub(VIEWER_SURFACE_PADDING_TOP) as f32;
        scene.targets.iter().find_map(|target| {
            if y < target.rect.y || y > target.rect.y + target.rect.height {
                return None;
            }
            Some(TargetSample {
                node_id: target.node_id.0.clone(),
                artifact_id: target.artifact_id.0.clone(),
                y: target.rect.y.round() as i32,
                height: target.rect.height.round() as i32,
                line: target.source.line_column_range.start.line,
                raw: target.source.raw.text.chars().take(80).collect(),
            })
        })
    }

    fn last_target(scene: &crate::preview::PreviewScene) -> Option<TargetSample> {
        scene
            .targets
            .iter()
            .max_by(|left, right| {
                (left.rect.y + left.rect.height).total_cmp(&(right.rect.y + right.rect.height))
            })
            .map(|target| TargetSample {
                node_id: target.node_id.0.clone(),
                artifact_id: target.artifact_id.0.clone(),
                y: target.rect.y.round() as i32,
                height: target.rect.height.round() as i32,
                line: target.source.line_column_range.start.line,
                raw: target.source.raw.text.chars().take(80).collect(),
            })
    }

    fn ui_node_sample(scene: &crate::preview::PreviewScene, node_id: &str) -> Option<UiNodeSample> {
        Self::find_ui_node(scene.tree.root(), node_id).map(|node| UiNodeSample {
            id: node.id().as_str().to_string(),
            kind: format!("{:?}", node.kind()),
            label: node.props().label.chars().take(80).collect(),
            role: node.props().text.role.clone(),
            height: format!("{:?}", node.props().common.height),
        })
    }

    fn ui_nodes_by_role(scene: &crate::preview::PreviewScene, role: &str) -> Vec<UiNodeSample> {
        let mut nodes = Vec::new();
        Self::collect_ui_nodes_by_role(scene.tree.root(), role, &mut nodes);
        nodes
    }

    fn row_loss_targets(
        scene: &crate::preview::PreviewScene,
        bands: &[super::frame_surface_similarity::RowLossBand],
    ) -> Vec<RowLossTargetSample> {
        bands
            .iter()
            .map(|band| RowLossTargetSample {
                start: band.start,
                end: band.end,
                loss: band.loss,
                target: SurfaceParitySupport::target_at(scene, band.start),
            })
            .collect()
    }

    fn collect_ui_nodes_by_role(
        node: &katana_ui_core::render_model::UiNode,
        role: &str,
        nodes: &mut Vec<UiNodeSample>,
    ) {
        if node.props().text.role == role {
            nodes.push(UiNodeSample {
                id: node.id().as_str().to_string(),
                kind: format!("{:?}", node.kind()),
                label: node.props().label.chars().take(80).collect(),
                role: node.props().text.role.clone(),
                height: format!("{:?}", node.props().common.height),
            });
        }
        for child in node.children() {
            Self::collect_ui_nodes_by_role(child, role, nodes);
        }
    }

    fn find_ui_node<'a>(
        node: &'a katana_ui_core::render_model::UiNode,
        node_id: &str,
    ) -> Option<&'a katana_ui_core::render_model::UiNode> {
        if node.id().as_str() == node_id {
            return Some(node);
        }
        node.children()
            .iter()
            .find_map(|child| Self::find_ui_node(child, node_id))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ContentBounds {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ContentSample {
    x: usize,
    y: usize,
    rgb: [u8; 3],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TargetSample {
    node_id: String,
    artifact_id: String,
    y: i32,
    height: i32,
    line: usize,
    raw: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UiNodeSample {
    id: String,
    kind: String,
    label: String,
    role: String,
    height: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RowLossTargetSample {
    start: usize,
    end: usize,
    loss: usize,
    target: Option<TargetSample>,
}

impl ContentBounds {
    fn empty() -> Self {
        Self {
            min_x: usize::MAX,
            min_y: usize::MAX,
            max_x: 0,
            max_y: 0,
        }
    }

    fn include(&mut self, x: usize, y: usize) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }

    fn into_option(self) -> Option<Self> {
        if self.min_x == usize::MAX {
            None
        } else {
            Some(self)
        }
    }
}

struct RenderedSurfaceFixture {
    canvas: Canvas,
    scene: crate::preview::PreviewScene,
}
