use super::mouse_host_action::StorybookHostActionRouter;
use super::{DocumentPoint, StorybookMouseButton, StorybookPointer};
use crate::preview::PreviewScene;
use std::collections::BTreeMap;

pub(crate) struct StorybookMouseAccordion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StorybookAccordionToggleHit {
    node_id: String,
    requested_open: bool,
}

impl StorybookAccordionToggleHit {
    #[cfg(test)]
    pub(crate) fn node_id(&self) -> &str {
        self.node_id.as_str()
    }

    pub(crate) fn apply_to_open_overrides(&self, overrides: &mut BTreeMap<String, bool>) {
        overrides.insert(self.node_id.clone(), self.requested_open);
    }
}

impl StorybookMouseAccordion {
    pub(crate) fn toggle_for_click(
        scene: &PreviewScene,
        scroll_y: f32,
        pointer: StorybookPointer,
        window_width: usize,
        window_height: usize,
    ) -> Option<StorybookAccordionToggleHit> {
        if pointer.button != StorybookMouseButton::Left {
            return None;
        }
        let point = DocumentPoint::from_scene_pointer(
            scene,
            pointer,
            scroll_y,
            window_width,
            window_height,
        )?;
        let router = StorybookHostActionRouter::for_window_with_scroll(
            scene,
            window_width,
            window_height,
            scroll_y,
        );
        Self::toggle_at_point(point, &router)
    }

    fn toggle_at_point(
        point: DocumentPoint,
        router: &StorybookHostActionRouter,
    ) -> Option<StorybookAccordionToggleHit> {
        router.hits_at(point).find_map(|hit| {
            let action = hit.action.text_span_action()?.accordion_toggle_action()?;
            Some(StorybookAccordionToggleHit {
                node_id: action.node_id,
                requested_open: action.requested_open,
            })
        })
    }
}
