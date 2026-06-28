use serde::{Deserialize, Serialize};

pub const VIEWER_TEXT_COLOR_CHANNELS: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerTextSpan {
    pub text: String,
    pub style: ViewerTextStyle,
    pub link_target: String,
}

impl ViewerTextSpan {
    #[must_use]
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: ViewerTextStyle::default(),
            link_target: String::new(),
        }
    }

    #[must_use]
    pub fn styled(text: impl Into<String>, style: ViewerTextStyle) -> Self {
        Self {
            text: text.into(),
            style,
            link_target: String::new(),
        }
    }

    #[must_use]
    pub fn linked(
        text: impl Into<String>,
        link_target: impl Into<String>,
        style: ViewerTextStyle,
    ) -> Self {
        Self {
            text: text.into(),
            style,
            link_target: link_target.into(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ViewerTextStyle {
    pub bold: bool,
    pub italic: bool,
    pub monospace: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub highlight: bool,
    pub current_highlight: bool,
    pub inline_code: bool,
    pub inline_math: bool,
    pub emoji: bool,
    pub color_rgba: [u8; VIEWER_TEXT_COLOR_CHANNELS],
}

impl ViewerTextStyle {
    #[must_use]
    pub fn monospace(mut self) -> Self {
        self.monospace = true;
        self
    }

    #[must_use]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    #[must_use]
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    #[must_use]
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    #[must_use]
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    #[must_use]
    pub fn highlight(mut self) -> Self {
        self.highlight = true;
        self
    }

    #[must_use]
    pub fn current_highlight(mut self) -> Self {
        self.highlight = true;
        self.current_highlight = true;
        self
    }

    #[must_use]
    pub fn inline_code(mut self) -> Self {
        self.monospace = true;
        self.inline_code = true;
        self
    }

    #[must_use]
    pub fn inline_math(mut self) -> Self {
        self.inline_math = true;
        self
    }

    #[must_use]
    pub fn emoji(mut self) -> Self {
        self.emoji = true;
        self
    }

    #[must_use]
    pub fn color_rgba(mut self, value: [u8; VIEWER_TEXT_COLOR_CHANNELS]) -> Self {
        self.color_rgba = value;
        self
    }

    #[must_use]
    pub fn link(self) -> Self {
        self.underline()
    }
}
