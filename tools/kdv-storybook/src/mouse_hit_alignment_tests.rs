use super::mouse_host_action::{StorybookHostActionHits, StorybookHostActionRouter};
use super::mouse_test_support::{
    WINDOW_HEIGHT, WINDOW_WIDTH, pointer_for_task, sample_basic_scene,
    sample_diagram_controls_scene,
};
use super::{
    DocumentPoint, StorybookMouse, StorybookMouseAccordion, StorybookMouseButton, StorybookPointer,
};
use crate::layout::StorybookPreviewArea;
use crate::media_host_action::StorybookMediaHostAction;
use katana_document_viewer::ViewerCommand;
use katana_ui_core::render_model::{UiTaskMarker, UiTextSpanAction};
use katana_ui_core_storybook::{UiTreeHostActionHit, UiTreeRenderArea, UiTreeSurfaceHost};

#[test]
fn link_hit_rect_center_click_matches_kuc_drawn_action() -> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_basic_scene()?;
    let hit = hit_for_link_label(&scene, "Normal link")?;
    let pointer = CanvasPointer::from_hit_rect_center(&hit);

    let command = StorybookMouse::command_for_click(
        &scene,
        pointer.scroll_y,
        pointer.pointer,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .ok_or_else(|| std::io::Error::other("missing link command at KUC hit rect center"))?;

    let ViewerCommand::Link(link) = command else {
        return Err(std::io::Error::other("expected link command at hit rect center").into());
    };
    assert_eq!("https://github.com", link.uri);
    Ok(())
}

#[test]
fn task_hit_rect_center_click_toggles_task() -> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_basic_scene()?;
    let hit = hit_for_task_marker(&scene, "[ ]")?;
    let pointer = CanvasPointer::from_hit_rect_center(&hit);

    let command = StorybookMouse::command_for_click(
        &scene,
        pointer.scroll_y,
        pointer.pointer,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .ok_or_else(|| std::io::Error::other("missing task command at KUC hit rect center"))?;

    assert!(matches!(command, ViewerCommand::Task(_)));
    Ok(())
}

#[test]
fn task_test_pointer_uses_kuc_hit_rect_center() -> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_basic_scene()?;
    let hit = hit_for_task_marker(&scene, "[ ]")?;
    let pointer = pointer_for_task(&scene, "[ ]")?;
    let (center_x, center_y) = hit.center_point();
    let area = StorybookPreviewArea::for_window(WINDOW_WIDTH, WINDOW_HEIGHT, pointer.scroll_y);
    let (document_x, document_y) = area
        .document_point(pointer.pointer.x, pointer.pointer.y)
        .ok_or_else(|| std::io::Error::other("task pointer outside preview area"))?;

    assert_eq!(center_x.round(), document_x.round());
    assert_eq!(center_y.round(), document_y.round());
    Ok(())
}

#[test]
fn diagram_control_hit_rect_center_click_matches_drawn_button()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_diagram_controls_scene()?;
    let hit = hit_for_media_action(&scene, "fullscreen")?;
    let pointer = CanvasPointer::from_hit_rect_center(&hit);

    let command = StorybookMouse::command_for_click(
        &scene,
        pointer.scroll_y,
        pointer.pointer,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .ok_or_else(|| std::io::Error::other("missing diagram command at KUC hit rect center"))?;

    assert!(matches!(command, ViewerCommand::Diagram(_)));
    Ok(())
}

#[test]
fn viewport_router_uses_render_scroll_delta_when_tree_has_root_scroll_offset()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_basic_scene()?;
    let hit = hit_for_link_label(&scene, "Normal link")?;
    let (_, document_y) = hit.center_point();
    let root_offset = (document_y - 120.0).round().max(1.0) as u32;
    let mut scrolled_scene = scene.clone();
    scrolled_scene.tree = scrolled_scene.tree.with_scroll_area_offset_y(root_offset);
    let scroll_y = root_offset as f32;
    let (raw_hits, _) = UiTreeSurfaceHost::new(scrolled_scene.theme.clone())
        .viewport_interaction_hits(
            scrolled_scene.tree.root(),
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: crate::layout::preview_content_width(WINDOW_WIDTH),
                height: crate::layout::preview_viewport_height(WINDOW_HEIGHT),
                scroll_y: 0.0,
            },
        );
    let expected = raw_hits
        .into_iter()
        .filter(|hit| hit.action.text_span_action().is_some())
        .find_map(|mut hit| {
            hit.rect.y = hit.rect.y.saturating_add(root_offset as usize);
            (hit.rect.width > 0 && hit.rect.height > 0).then_some(hit)
        })
        .ok_or_else(|| std::io::Error::other("missing visible KUC text action hit"))?;
    let expected_label = expected.action.label.clone();
    let (document_x, document_y) = expected.center_point();
    let area = StorybookPreviewArea::for_window(WINDOW_WIDTH, WINDOW_HEIGHT, scroll_y);
    let (canvas_x, canvas_y) = area.canvas_point_for_document_point(document_x, document_y);
    let (point_x, point_y) = area
        .document_point(canvas_x, canvas_y)
        .ok_or_else(|| std::io::Error::other("KUC hit center outside preview area"))?;
    let point = DocumentPoint {
        x: point_x,
        y: point_y,
    };
    let router = StorybookHostActionRouter::for_window_with_scroll(
        &scrolled_scene,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        scroll_y,
    );

    let resolved = router
        .resolved_hits_at(point)
        .find(|resolved| resolved.hit().action.label == expected_label)
        .ok_or_else(|| {
            std::io::Error::other(
                "visible KUC text action hit must survive root scroll offset without double scroll",
            )
        })?;
    assert_eq!(expected_label, resolved.hit().action.label);

    let command = StorybookMouse::command_for_click(
        &scrolled_scene,
        scroll_y,
        StorybookPointer::new(canvas_x, canvas_y, StorybookMouseButton::Left),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .ok_or_else(|| std::io::Error::other("missing command for root-offset text action hit"))?;
    assert!(
        matches!(
            command,
            ViewerCommand::Link(_) | ViewerCommand::ScrollToHeading(_)
        ),
        "expected a text action command for visible KUC hit"
    );
    Ok(())
}

#[test]
fn accordion_hit_rect_center_click_resolves_toggle() -> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_basic_scene()?;
    let hit = hit_for_accordion(&scene)?;
    let pointer = CanvasPointer::from_hit_rect_center(&hit);

    let toggle = StorybookMouseAccordion::toggle_for_click(
        &scene,
        pointer.scroll_y,
        pointer.pointer,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .ok_or_else(|| std::io::Error::other("missing accordion toggle at KUC hit rect center"))?;

    assert!(!toggle.node_id().trim().is_empty());
    Ok(())
}

#[test]
fn accordion_click_accepts_kuc_viewport_surface_hit() -> Result<(), Box<dyn std::error::Error>> {
    let scene = sample_basic_scene()?;
    let hit = hit_for_accordion(&scene)?;
    let (_, document_y) = hit.center_point();
    let scroll_y = (document_y - 120.0).max(0.0);
    let hit = visible_viewport_action_hit(&scene, scroll_y, |hit| {
        matches!(
            hit.action.text_span_action(),
            Some(UiTextSpanAction::ToggleAccordion { .. })
        )
    })?;
    let pointer = CanvasPointer::from_hit_rect_center_with_scroll(&hit, scroll_y);

    let toggle = StorybookMouseAccordion::toggle_for_click(
        &scene,
        pointer.scroll_y,
        pointer.pointer,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .ok_or_else(|| {
        std::io::Error::other("missing accordion toggle for root-offset KUC viewport hit")
    })?;

    assert!(!toggle.node_id().trim().is_empty());
    Ok(())
}

struct CanvasPointer {
    pointer: StorybookPointer,
    scroll_y: f32,
}

impl CanvasPointer {
    fn from_hit_rect_center(hit: &UiTreeHostActionHit) -> Self {
        let (document_x, document_y) = hit.center_point();
        let scroll_y = (document_y - 120.0).max(0.0);
        let area = StorybookPreviewArea::for_window(WINDOW_WIDTH, WINDOW_HEIGHT, scroll_y);
        let (x, y) = area.canvas_point_for_document_point(document_x, document_y);
        Self {
            pointer: StorybookPointer::new(x, y, StorybookMouseButton::Left),
            scroll_y,
        }
    }

    fn from_hit_rect_center_with_scroll(hit: &UiTreeHostActionHit, scroll_y: f32) -> Self {
        let (document_x, document_y) = hit.center_point();
        let area = StorybookPreviewArea::for_window(WINDOW_WIDTH, WINDOW_HEIGHT, scroll_y);
        let (x, y) = area.canvas_point_for_document_point(document_x, document_y);
        Self {
            pointer: StorybookPointer::new(x, y, StorybookMouseButton::Left),
            scroll_y,
        }
    }
}

fn hit_for_link_label(
    scene: &crate::preview::PreviewScene,
    label: &str,
) -> Result<UiTreeHostActionHit, Box<dyn std::error::Error>> {
    hit_for(scene, |hit| {
        matches!(
            hit.action.text_span_action(),
            Some(UiTextSpanAction::OpenLink { .. })
        ) && hit.action.label == label
    })
}

fn hit_for_task_marker(
    scene: &crate::preview::PreviewScene,
    marker: &str,
) -> Result<UiTreeHostActionHit, Box<dyn std::error::Error>> {
    let expected_marker = UiTaskMarker::from_marker(marker)
        .ok_or_else(|| std::io::Error::other("unsupported task marker"))?;
    hit_for(scene, |hit| {
        hit.action
            .task_control_action_from_root(scene.tree.root())
            .is_some_and(|action| action.current_marker == expected_marker)
    })
}

fn hit_for_media_action(
    scene: &crate::preview::PreviewScene,
    action_id: &str,
) -> Result<UiTreeHostActionHit, Box<dyn std::error::Error>> {
    hit_for(scene, |hit| {
        StorybookMediaHostAction::from_host_action_plan(&hit.action)
            .map(|action| action.into_viewer_action())
            .is_some_and(|action| action.command == action_id)
    })
}

fn hit_for_accordion(
    scene: &crate::preview::PreviewScene,
) -> Result<UiTreeHostActionHit, Box<dyn std::error::Error>> {
    hit_for(scene, |hit| {
        matches!(
            hit.action.text_span_action(),
            Some(UiTextSpanAction::ToggleAccordion { .. })
        )
    })
}

fn hit_for(
    scene: &crate::preview::PreviewScene,
    predicate: impl Fn(&UiTreeHostActionHit) -> bool,
) -> Result<UiTreeHostActionHit, Box<dyn std::error::Error>> {
    StorybookHostActionHits::hits(scene, WINDOW_WIDTH)
        .into_iter()
        .find(predicate)
        .ok_or_else(|| std::io::Error::other("missing KUC host action hit").into())
}

fn visible_viewport_action_hit(
    scene: &crate::preview::PreviewScene,
    scroll_y: f32,
    predicate: impl Fn(&UiTreeHostActionHit) -> bool,
) -> Result<UiTreeHostActionHit, Box<dyn std::error::Error>> {
    let tree_offset = scene.tree.root().props().scroll_area.offset_y as f32;
    let (hits, _) = UiTreeSurfaceHost::new(scene.theme.clone()).viewport_interaction_hits(
        scene.tree.root(),
        UiTreeRenderArea {
            x: 0,
            y: 0,
            width: crate::layout::preview_content_width(WINDOW_WIDTH),
            height: crate::layout::preview_viewport_height(WINDOW_HEIGHT),
            scroll_y: (scroll_y - tree_offset).max(0.0),
        },
    );
    hits.into_iter()
        .filter(predicate)
        .find_map(|mut hit| {
            hit.rect.y = hit.rect.y.saturating_add(
                DocumentPoint::effective_scroll_y(scene, scroll_y)
                    .round()
                    .max(0.0) as usize,
            );
            (hit.rect.width > 0 && hit.rect.height > 0).then_some(hit)
        })
        .ok_or_else(|| std::io::Error::other("missing visible KUC viewport action hit").into())
}
