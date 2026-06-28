use crate::sidebar_settings_task_change::{
    StorybookTaskStateChange, StorybookTaskStateChangeInput,
};
use katana_ui_core::molecule::{SettingsList, SettingsListAction, SettingsListEvent};
use katana_ui_core::render_model::UiNodeId;
use std::collections::BTreeSet;

const TASK_LOCATION_HISTORY_LIMIT: usize = 3;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct StorybookSettingsState {
    collapsed_section_ids: BTreeSet<String>,
    hovered_node_id: Option<UiNodeId>,
    task_changes: Vec<StorybookTaskStateChange>,
}

impl StorybookSettingsState {
    pub(crate) fn apply_to_list(&self, list: &mut SettingsList) {
        for section_id in &self.collapsed_section_ids {
            let _ = list.apply_settings_action(SettingsListAction::ToggleSection {
                section_id: section_id.clone(),
            });
        }
    }

    pub(crate) fn apply_events(&mut self, events: &[SettingsListEvent]) {
        for event in events {
            if let SettingsListEvent::SectionCollapsed {
                section_id,
                collapsed,
            } = event
            {
                self.update_section(section_id, *collapsed);
            }
        }
    }

    pub(crate) fn hovered_node_id(&self) -> Option<UiNodeId> {
        self.hovered_node_id.clone()
    }

    pub(crate) fn set_hovered_node_id(&mut self, node_id: Option<UiNodeId>) -> bool {
        if self.hovered_node_id == node_id {
            return false;
        }
        self.hovered_node_id = node_id;
        true
    }

    pub(crate) fn record_task_change(&mut self, change: StorybookTaskStateChangeInput<'_>) {
        self.task_changes
            .push(StorybookTaskStateChange::new(change));
    }

    pub(crate) fn clear_task_changes(&mut self) {
        self.task_changes.clear();
    }

    pub(crate) fn task_change_count(&self) -> usize {
        self.task_changes.len()
    }

    pub(crate) fn last_task_change_label(&self) -> String {
        self.task_changes
            .last()
            .map_or_else(|| "none".to_string(), StorybookTaskStateChange::label)
    }

    pub(crate) fn last_task_location_label(&self) -> String {
        self.task_changes.last().map_or_else(
            || "none".to_string(),
            StorybookTaskStateChange::location_label,
        )
    }

    pub(crate) fn last_task_target_label(&self) -> String {
        self.task_changes.last().map_or_else(
            || "none".to_string(),
            StorybookTaskStateChange::target_label,
        )
    }

    pub(crate) fn last_task_span_label(&self) -> String {
        self.task_changes
            .last()
            .map_or_else(|| "none".to_string(), StorybookTaskStateChange::span_label)
    }

    pub(crate) fn recent_task_location_history_label(&self) -> String {
        if self.task_changes.is_empty() {
            return "none".to_string();
        }
        let first_index = self
            .task_changes
            .len()
            .saturating_sub(TASK_LOCATION_HISTORY_LIMIT);
        self.task_changes
            .iter()
            .enumerate()
            .skip(first_index)
            .map(|(index, change)| format!("#{} {}", index + 1, change.location_label()))
            .collect::<Vec<String>>()
            .join(" | ")
    }

    pub(crate) fn last_task_source_label(&self) -> String {
        self.task_changes.last().map_or_else(
            || "none".to_string(),
            StorybookTaskStateChange::source_label,
        )
    }

    #[cfg(test)]
    pub(crate) fn task_change_location_labels(&self) -> Vec<String> {
        self.task_changes
            .iter()
            .map(StorybookTaskStateChange::location_label)
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn is_collapsed(&self, section_id: &str) -> bool {
        self.collapsed_section_ids.contains(section_id)
    }

    fn update_section(&mut self, section_id: &str, collapsed: bool) {
        if collapsed {
            self.collapsed_section_ids.insert(section_id.to_string());
            return;
        }
        self.collapsed_section_ids.remove(section_id);
    }
}
