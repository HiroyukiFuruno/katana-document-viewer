use super::{ACTIVE_FRAME_DELAY, IDLE_FRAME_DELAY, INTERACTION_FRAME_DELAY};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FrameLoopChanges {
    pub(super) scene_changed: bool,
    pub(super) input_changed: bool,
    pub(super) scroll_changed: bool,
    pub(super) hover_changed: bool,
    pub(super) asset_changed: bool,
}

impl FrameLoopChanges {
    pub(crate) fn needs_redraw(self) -> bool {
        self.scene_changed
            || self.input_changed
            || self.scroll_changed
            || self.hover_changed
            || self.asset_changed
    }

    pub(crate) fn delay(self, asset_pending: bool) -> Duration {
        if self.scroll_changed || self.input_changed || self.hover_changed {
            return INTERACTION_FRAME_DELAY;
        }
        if self.needs_redraw() || asset_pending {
            return ACTIVE_FRAME_DELAY;
        }
        IDLE_FRAME_DELAY
    }

    pub(crate) fn delay_after_frame(self, asset_pending: bool, elapsed: Duration) -> Duration {
        self.delay(asset_pending).saturating_sub(elapsed)
    }

    pub(crate) fn should_defer_asset_update(self) -> bool {
        self.scroll_changed || self.input_changed
    }

    pub(crate) fn should_pause_loading_animation(self) -> bool {
        self.scroll_changed || self.input_changed
    }

    pub(crate) fn can_redraw_preview_only(self, animation_changed: bool) -> bool {
        !self.scene_changed
            && !self.asset_changed
            && (self.scroll_changed || self.hover_changed || animation_changed)
            && (!self.input_changed || self.scroll_changed)
    }

    #[cfg(test)]
    pub(crate) const fn idle() -> Self {
        Self {
            scene_changed: false,
            input_changed: false,
            scroll_changed: false,
            hover_changed: false,
            asset_changed: false,
        }
    }

    #[cfg(test)]
    pub(crate) const fn scene_changed() -> Self {
        Self {
            scene_changed: true,
            input_changed: false,
            scroll_changed: false,
            hover_changed: false,
            asset_changed: false,
        }
    }

    #[cfg(test)]
    pub(crate) const fn scroll_changed() -> Self {
        Self {
            scene_changed: false,
            input_changed: false,
            scroll_changed: true,
            hover_changed: false,
            asset_changed: false,
        }
    }

    #[cfg(test)]
    pub(crate) const fn hover_changed() -> Self {
        Self {
            scene_changed: false,
            input_changed: false,
            scroll_changed: false,
            hover_changed: true,
            asset_changed: false,
        }
    }

    #[cfg(test)]
    pub(crate) const fn asset_changed() -> Self {
        Self {
            scene_changed: false,
            input_changed: false,
            scroll_changed: false,
            hover_changed: false,
            asset_changed: true,
        }
    }

    #[cfg(test)]
    pub(crate) const fn input_changed() -> Self {
        Self {
            scene_changed: false,
            input_changed: true,
            scroll_changed: false,
            hover_changed: false,
            asset_changed: false,
        }
    }

    #[cfg(test)]
    pub(crate) const fn scroll_and_input_changed() -> Self {
        Self {
            scene_changed: false,
            input_changed: true,
            scroll_changed: true,
            hover_changed: false,
            asset_changed: false,
        }
    }
}
