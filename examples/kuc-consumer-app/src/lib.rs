use katana_ui_core::component::ComponentTree;
use katana_ui_core::layout::{Column, ScrollArea, SplitPane, SplitPaneAxis, SplitPaneResizeMode};
use katana_ui_core::panel::{Panel, PanelRegion};
use katana_ui_core::render_model::{UiNodeKind, UiTree};
use katana_ui_core::theme::ThemeSnapshot;
use katana_ui_core::widget::atoms::{Input, TextArea};
use katana_ui_core::widget::molecules::{
    CloseableTabStrip, ComboBox, SearchBox, SelectBox, SelectionList,
};

mod actions;
mod fixtures;
mod queries;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct ConsumerApp {
    search: Input,
    quick_search: SearchBox,
    workspace_select: SelectBox,
    symbol_combo: ComboBox,
    notes: TextArea,
    tabs: CloseableTabStrip,
    navigation: SelectionList,
    navigation_scroll: ScrollArea,
    split: SplitPane,
}

impl ConsumerApp {
    #[must_use]
    pub fn new() -> Self {
        Self {
            search: fixtures::search_input(),
            quick_search: fixtures::quick_search_box(),
            workspace_select: fixtures::workspace_select(),
            symbol_combo: fixtures::symbol_combo(),
            notes: fixtures::notes_area(),
            tabs: fixtures::workspace_tabs(),
            navigation: fixtures::navigation_list(),
            navigation_scroll: fixtures::navigation_scroll(),
            split: SplitPane::new()
                .axis(SplitPaneAxis::Horizontal)
                .ratio_percent(26)
                .min_percent(18)
                .max_percent(50)
                .resize_mode(SplitPaneResizeMode::PointerAndKeyboard),
        }
    }

    #[must_use]
    pub fn render(&self) -> UiTree {
        ComponentTree::new(
            Panel::new("KUC consumer app", PanelRegion::Root, ThemeSnapshot::dark()).child(
                self.split
                    .clone()
                    .child(self.navigation_panel())
                    .child(self.main_panel()),
            ),
        )
        .into_tree()
    }

    fn navigation_panel(&self) -> ScrollArea {
        self.navigation_scroll
            .clone()
            .child(self.navigation.clone())
    }

    fn main_panel(&self) -> Column {
        fixtures::main_panel(
            self.tabs.clone(),
            self.search.clone(),
            self.quick_search.clone(),
            self.workspace_select.clone(),
            self.symbol_combo.clone(),
            self.notes.clone(),
        )
    }
}

impl Default for ConsumerApp {
    fn default() -> Self {
        Self::new()
    }
}

#[must_use]
pub fn root_kind(tree: &UiTree) -> UiNodeKind {
    tree.root().kind()
}
