#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorybookPalette {
    dark: bool,
}

impl StorybookPalette {
    pub fn new(dark: bool) -> Self {
        Self { dark }
    }

    pub fn background(self) -> u32 {
        self.pick(0x1e1e1e, 0xf7f7f7)
    }

    pub fn sidebar_background(self) -> u32 {
        self.pick(0x191919, 0xffffff)
    }

    pub fn header(self) -> u32 {
        self.pick(0x202020, 0xececec)
    }

    pub fn preview_background(self) -> u32 {
        self.pick(0x1e1e1e, 0xffffff)
    }

    pub fn text(self) -> u32 {
        self.pick(0xe7e7e7, 0x202020)
    }

    fn pick(self, dark: u32, light: u32) -> u32 {
        if self.dark { dark } else { light }
    }
}
