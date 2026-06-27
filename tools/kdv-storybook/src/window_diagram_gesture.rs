use super::StorybookWindow;
use crate::mouse::{DocumentPoint, StorybookHostActionRouter, StorybookMouseButton};
use katana_document_viewer::{
    DiagramControlCommand, DiagramPanCommand, DiagramPanSource, DiagramZoomCommand,
    DiagramZoomSource, ViewerCommand, ViewerTarget, ViewerVector,
};

impl StorybookWindow {
    pub(super) fn fullscreen_diagram_active(&self) -> bool {
        self.diagram_viewports
            .values()
            .any(|state| state.fullscreen_open)
    }

    pub(super) fn update_fullscreen_diagram_drag(
        &mut self,
        left_down: bool,
        pointer: Option<(f32, f32)>,
        pressed: Option<StorybookMouseButton>,
    ) -> bool {
        if !left_down || !self.fullscreen_diagram_active() {
            self.fullscreen_diagram_drag_previous = None;
            return false;
        }
        let Some(current) = pointer else {
            self.fullscreen_diagram_drag_previous = None;
            return false;
        };
        let previous = self.fullscreen_diagram_drag_previous.replace(current);
        if pressed == Some(StorybookMouseButton::Left) {
            return false;
        }
        let Some(previous) = previous else {
            return false;
        };
        self.apply_fullscreen_diagram_drag(ViewerVector {
            x: current.0 - previous.0,
            y: current.1 - previous.1,
        })
    }

    pub(super) fn apply_fullscreen_diagram_drag(&mut self, delta: ViewerVector) -> bool {
        self.apply_fullscreen_diagram_pan(delta, DiagramPanSource::Drag)
    }

    pub(super) fn apply_fullscreen_diagram_smooth_scroll(&mut self, delta: ViewerVector) -> bool {
        self.apply_fullscreen_diagram_pan(delta, DiagramPanSource::SmoothScroll)
    }

    pub(super) fn apply_fullscreen_diagram_trackpad_zoom(&mut self, multiplier: f32) -> bool {
        if multiplier <= 0.0 {
            return false;
        }
        let Some(target) = self.fullscreen_diagram_target() else {
            return false;
        };
        self.apply_viewer_command(&ViewerCommand::Diagram(DiagramControlCommand::Zoom(
            DiagramZoomCommand {
                target,
                multiplier,
                source: DiagramZoomSource::Trackpad,
            },
        )))
    }

    pub(super) fn update_document_diagram_drag(
        &mut self,
        left_down: bool,
        pointer: Option<(f32, f32)>,
        pressed: Option<StorybookMouseButton>,
        width: usize,
        height: usize,
    ) -> bool {
        if !left_down
            || self.fullscreen_diagram_active()
            || !self.interaction.diagram_controls_enabled
        {
            self.document_diagram_drag_previous = None;
            return false;
        }
        let Some(current) = pointer else {
            self.document_diagram_drag_previous = None;
            return false;
        };
        let Some(target) = self.document_diagram_target_at(current, width, height) else {
            self.document_diagram_drag_previous = None;
            return false;
        };
        let target_id = target.node_id.0.clone();
        let previous = self
            .document_diagram_drag_previous
            .replace((target_id.clone(), current));
        if pressed == Some(StorybookMouseButton::Left) {
            return true;
        }
        let Some((previous_target_id, previous)) = previous else {
            return true;
        };
        if previous_target_id != target_id {
            return true;
        }
        self.apply_document_diagram_pan(
            target,
            ViewerVector {
                x: current.0 - previous.0,
                y: current.1 - previous.1,
            },
            DiagramPanSource::Drag,
        )
    }

    pub(super) fn apply_document_diagram_trackpad_zoom_at(
        &mut self,
        pointer: Option<(f32, f32)>,
        width: usize,
        height: usize,
        multiplier: f32,
    ) -> bool {
        if multiplier <= 0.0
            || self.fullscreen_diagram_active()
            || !self.interaction.diagram_controls_enabled
        {
            return false;
        }
        let Some(pointer) = pointer else {
            return false;
        };
        let Some(target) = self.document_diagram_target_at(pointer, width, height) else {
            return false;
        };
        self.apply_viewer_command(&ViewerCommand::Diagram(DiagramControlCommand::Zoom(
            DiagramZoomCommand {
                target,
                multiplier,
                source: DiagramZoomSource::Trackpad,
            },
        )))
    }

    fn apply_fullscreen_diagram_pan(
        &mut self,
        delta: ViewerVector,
        source: DiagramPanSource,
    ) -> bool {
        if delta.x == 0.0 && delta.y == 0.0 {
            return false;
        }
        let Some(target) = self.fullscreen_diagram_target() else {
            return false;
        };
        self.apply_viewer_command(&ViewerCommand::Diagram(DiagramControlCommand::Pan(
            DiagramPanCommand {
                target,
                delta,
                source,
            },
        )))
    }

    fn fullscreen_diagram_target(&self) -> Option<ViewerTarget> {
        let scene = self.scene.as_ref()?;
        self.diagram_viewports
            .iter()
            .find(|(_, state)| state.fullscreen_open)
            .and_then(|(node_id, _)| scene.target_for_node_id(node_id))
            .cloned()
    }

    fn document_diagram_target_at(
        &self,
        pointer: (f32, f32),
        width: usize,
        height: usize,
    ) -> Option<ViewerTarget> {
        let scene = self.scene.as_ref()?;
        let point = DocumentPoint::from_scene_position(
            scene,
            pointer.0,
            pointer.1,
            self.scroll_y,
            width,
            height,
        )?;
        let router =
            StorybookHostActionRouter::for_window_with_scroll(scene, width, height, self.scroll_y);
        router.diagram_target_at(point).cloned()
    }

    fn apply_document_diagram_pan(
        &mut self,
        target: ViewerTarget,
        delta: ViewerVector,
        source: DiagramPanSource,
    ) -> bool {
        if delta.x == 0.0 && delta.y == 0.0 {
            return true;
        }
        self.apply_viewer_command(&ViewerCommand::Diagram(DiagramControlCommand::Pan(
            DiagramPanCommand {
                target,
                delta,
                source,
            },
        )))
    }
}
