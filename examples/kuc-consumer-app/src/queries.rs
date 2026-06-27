use super::ConsumerApp;
use katana_ui_core::render_model::UiNode;
use katana_ui_core::widget::molecules::{CloseableTabGroupId, CloseableTabId};

impl ConsumerApp {
    #[must_use]
    pub fn query(&self) -> String {
        self.search.state_snapshot().interaction.value
    }

    #[must_use]
    pub fn quick_search_query(&self) -> String {
        UiNode::from(self.quick_search.clone())
            .props()
            .interaction
            .value
            .clone()
    }

    #[must_use]
    pub fn workspace_value(&self) -> String {
        UiNode::from(self.workspace_select.clone())
            .props()
            .interaction
            .value
            .clone()
    }

    #[must_use]
    pub fn symbol_value(&self) -> String {
        UiNode::from(self.symbol_combo.clone())
            .props()
            .interaction
            .value
            .clone()
    }

    #[must_use]
    pub fn notes_resize_delta(&self) -> (u16, u16) {
        (
            self.notes.state().resize_width_delta,
            self.notes.state().resize_height_delta,
        )
    }

    #[must_use]
    pub fn split_ratio(&self) -> u8 {
        self.split.ratio_percent_value()
    }

    #[must_use]
    pub fn navigation_scroll_offset(&self) -> (u32, u32) {
        (
            self.navigation_scroll.offset_x(),
            self.navigation_scroll.offset_y(),
        )
    }

    #[must_use]
    pub fn active_tab_id(&self) -> Option<&CloseableTabId> {
        self.tabs.state().active_tab_id.as_ref()
    }

    #[must_use]
    pub fn tab_count(&self) -> usize {
        self.tabs.options().tabs.len()
    }

    #[must_use]
    pub fn tab_pinned(&self, id: &str) -> bool {
        self.tabs
            .options()
            .tabs
            .iter()
            .find(|tab| tab.id.as_str() == id)
            .is_some_and(|tab| tab.pinned)
    }

    #[must_use]
    pub fn tab_group_id(&self, id: &str) -> Option<&CloseableTabGroupId> {
        self.tabs
            .options()
            .tabs
            .iter()
            .find(|tab| tab.id.as_str() == id)
            .and_then(|tab| tab.group_id.as_ref())
    }
}
