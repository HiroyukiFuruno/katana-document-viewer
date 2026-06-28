use super::StorybookWindow;
use crate::mouse::{StorybookMouseAccordion, StorybookPointer};

impl StorybookWindow {
    pub(super) fn apply_accordion(
        &mut self,
        pointer: StorybookPointer,
        width: usize,
        height: usize,
    ) -> bool {
        let Some(scene) = self.scene.as_ref() else {
            return false;
        };
        let Some(hit) =
            StorybookMouseAccordion::toggle_for_click(scene, self.scroll_y, pointer, width, height)
        else {
            return false;
        };
        hit.apply_to_open_overrides(&mut self.accordion_open_overrides);
        self.invalidate_loaded_scene();
        self.last_command_label = "accordion".to_string();
        true
    }
}

#[cfg(test)]
use katana_ui_core::render_model::{UiNode, UiNodeKind};

#[cfg(test)]
fn accordion_open(node: &UiNode, node_id: &str) -> Option<bool> {
    if node.id().as_str() == node_id && node.kind() == UiNodeKind::Accordion {
        return Some(node.props().interaction.open);
    }
    node.children()
        .iter()
        .find_map(|child| accordion_open(child, node_id))
}

#[cfg(test)]
mod tests {
    use super::{StorybookWindow, accordion_open};
    use crate::args::StorybookArgs;
    use crate::catalog::{FixtureCatalog, StorybookFixture};
    use crate::mouse::mouse_test_support::pointer_for_accordion;
    use crate::preview::PreviewBuilder;
    use std::path::PathBuf;

    #[test]
    fn accordion_click_updates_kuc_state_override() -> Result<(), Box<dyn std::error::Error>> {
        let mut storybook = StorybookWindow::new(
            StorybookArgs::default(),
            catalog_with("direct/html-alignment.html"),
            PreviewBuilder::default(),
        );
        storybook.update_scene(1000, 700)?;
        let node_id = details_node_id(&storybook)?;
        let initial = rendered_open(&storybook, node_id.as_str())?;

        let scene = storybook.scene.as_ref().ok_or("scene missing")?;
        let hit = pointer_for_accordion(scene)?;
        storybook.scroll_y = hit.scroll_y;
        let pointer = hit.pointer;
        assert!(storybook.apply_accordion(pointer, 1000, 700));
        assert_eq!(
            Some(!initial),
            storybook.accordion_open_overrides.get(&node_id).copied()
        );

        storybook.update_scene(1000, 700)?;
        assert_eq!(!initial, rendered_open(&storybook, node_id.as_str())?);
        Ok(())
    }

    fn rendered_open(
        storybook: &StorybookWindow,
        node_id: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let scene = storybook.scene.as_ref().ok_or("scene missing")?;
        accordion_open(scene.tree.root(), node_id).ok_or_else(|| "accordion missing".into())
    }

    fn details_node_id(storybook: &StorybookWindow) -> Result<String, Box<dyn std::error::Error>> {
        let scene = storybook.scene.as_ref().ok_or("scene missing")?;
        scene
            .targets
            .iter()
            .find(|target| target.source.raw.text.contains("<details"))
            .map(|target| target.node_id.0.clone())
            .ok_or_else(|| "details target missing".into())
    }

    fn catalog_with(label: &str) -> FixtureCatalog {
        FixtureCatalog {
            fixtures: vec![StorybookFixture {
                label: label.to_string(),
                path: fixture_path(&format!("assets/fixtures/{label}")),
            }],
        }
    }

    fn fixture_path(path: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../{path}"))
    }
}
