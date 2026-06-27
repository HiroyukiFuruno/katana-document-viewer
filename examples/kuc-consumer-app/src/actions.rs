use super::{ConsumerApp, fixtures};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::layout::SplitPaneResizeSource;
use katana_ui_core::widget::atoms::TextAreaAction;
use katana_ui_core::widget::molecules::{
    CloseableTab, CloseableTabContextCommand, CloseableTabGroup, CloseableTabGroupId,
    CloseableTabGroupTarget, CloseableTabId, CloseableTabStripAction, CloseableTabStripEvent,
};

impl ConsumerApp {
    pub fn set_query(&mut self, value: impl Into<String>) -> bool {
        let action = UiAction::input_value(self.search.state_id().clone(), value);
        self.search.apply_action(&action).handled
    }

    pub fn invoke_search_callback(&mut self, callback: &str) -> Vec<UiCallbackLog> {
        let action = UiAction::invoke_callback(self.search.state_id().clone(), callback);
        self.search.apply_action(&action).callback_log
    }

    pub fn set_quick_search(&mut self, value: impl Into<String>) -> bool {
        let action = UiAction::input_value(self.quick_search.state_id().clone(), value);
        self.quick_search.apply_action(&action).handled
    }

    pub fn submit_quick_search(&mut self) -> Vec<UiCallbackLog> {
        let action = UiAction::search_submitted(self.quick_search.state_id().clone());
        self.quick_search.apply_action(&action).callback_log
    }

    pub fn select_workspace(&mut self, selected_index: usize) -> bool {
        let action =
            UiAction::select_box_selected(self.workspace_select.state_id().clone(), selected_index);
        self.workspace_select.apply_action(&action).handled
    }

    pub fn filter_symbol(&mut self, value: impl Into<String>) -> bool {
        let action = UiAction::input_value(self.symbol_combo.state_id().clone(), value);
        self.symbol_combo.apply_action(&action).handled
    }

    pub fn select_symbol(&mut self, selected_index: usize) -> bool {
        let action =
            UiAction::select_box_selected(self.symbol_combo.state_id().clone(), selected_index);
        self.symbol_combo.apply_action(&action).handled
    }

    pub fn resize_notes(&mut self, width_delta: u16, height_delta: u16) -> bool {
        self.notes
            .apply_text_area_action(TextAreaAction::resize(width_delta, height_delta))
            .handled
    }

    pub fn select_navigation(&mut self, selected_index: usize) -> bool {
        let action =
            UiAction::set_selected_index(self.navigation.state_id().clone(), selected_index);
        self.navigation.apply_action(&action).handled
    }

    pub fn scroll_navigation(&mut self, dx: i32, dy: i32) -> bool {
        let action = UiAction::scroll_by(self.navigation_scroll.state_id().clone(), dx, dy);
        self.navigation_scroll.apply_action(&action).handled
    }

    pub fn resize_split(&mut self, delta_percent: i8) -> bool {
        let action = UiAction::split_pane_resize_by(
            self.split.state_id().clone(),
            delta_percent,
            SplitPaneResizeSource::Keyboard,
        );
        self.split.apply_action(&action).handled
    }

    pub fn add_tab(&mut self, id: &str, title: &str) -> Vec<CloseableTabStripEvent> {
        self.tabs.apply_action(CloseableTabStripAction::AddTab {
            tab: CloseableTab::new(id, title).svg_icon(fixtures::doc_icon()),
            activate: true,
        })
    }

    pub fn close_tab(&mut self, id: &str) -> Vec<CloseableTabStripEvent> {
        self.tabs.apply_action(CloseableTabStripAction::CloseTab {
            tab_id: CloseableTabId::new(id),
        })
    }

    pub fn close_other_tabs(&mut self, id: &str) -> Vec<CloseableTabStripEvent> {
        self.tabs
            .apply_action(CloseableTabStripAction::CloseOthers {
                tab_id: CloseableTabId::new(id),
            })
    }

    pub fn close_tabs_to_right(&mut self, id: &str) -> Vec<CloseableTabStripEvent> {
        self.tabs
            .apply_action(CloseableTabStripAction::CloseToRight {
                tab_id: CloseableTabId::new(id),
            })
    }

    pub fn close_tabs_to_left(&mut self, id: &str) -> Vec<CloseableTabStripEvent> {
        self.tabs
            .apply_action(CloseableTabStripAction::CloseToLeft {
                tab_id: CloseableTabId::new(id),
            })
    }

    pub fn close_all_tabs(&mut self) -> Vec<CloseableTabStripEvent> {
        self.tabs.apply_action(CloseableTabStripAction::CloseAll)
    }

    pub fn pin_tab(&mut self, id: &str) -> Vec<CloseableTabStripEvent> {
        self.tabs.apply_action(CloseableTabStripAction::PinTab {
            tab_id: CloseableTabId::new(id),
        })
    }

    pub fn unpin_tab(&mut self, id: &str) -> Vec<CloseableTabStripEvent> {
        self.tabs.apply_action(CloseableTabStripAction::UnpinTab {
            tab_id: CloseableTabId::new(id),
        })
    }

    pub fn move_tab_to_existing_group(
        &mut self,
        id: &str,
        group_id: &str,
    ) -> Vec<CloseableTabStripEvent> {
        self.tabs
            .apply_action(CloseableTabStripAction::MoveToGroup {
                tab_id: CloseableTabId::new(id),
                target: CloseableTabGroupTarget::Existing(CloseableTabGroupId::new(group_id)),
            })
    }

    pub fn move_tab_to_new_group(
        &mut self,
        id: &str,
        group_id: &str,
        label: &str,
    ) -> Vec<CloseableTabStripEvent> {
        self.tabs
            .apply_action(CloseableTabStripAction::MoveToGroup {
                tab_id: CloseableTabId::new(id),
                target: CloseableTabGroupTarget::NewGroup(CloseableTabGroup::new(group_id, label)),
            })
    }

    pub fn apply_tab_context_command(
        &mut self,
        id: &str,
        command: CloseableTabContextCommand,
    ) -> Option<Vec<CloseableTabStripEvent>> {
        command
            .to_tab_action(CloseableTabId::new(id))
            .map(|action| self.tabs.apply_action(action))
    }
}
