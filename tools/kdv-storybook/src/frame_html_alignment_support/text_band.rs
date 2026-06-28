use super::PreviewBounds;

const MINIMUM_TEXT_PIXELS_PER_ROW: usize = 4;
const MAXIMUM_TEXT_PIXELS_PER_ROW: usize = 850;

#[derive(Clone, Copy, Debug)]
pub(crate) struct TextBand {
    pub(crate) max_x: usize,
    min_x: usize,
    min_y: usize,
    pub(crate) max_y: usize,
    pixels: usize,
}

impl TextBand {
    pub(crate) fn empty(y: usize) -> Self {
        Self {
            min_x: usize::MAX,
            max_x: 0,
            min_y: y,
            max_y: y,
            pixels: 0,
        }
    }

    pub(crate) fn observe(&mut self, x: usize) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.pixels += 1;
    }

    pub(crate) fn valid(self) -> Option<Self> {
        (self.pixels >= MINIMUM_TEXT_PIXELS_PER_ROW && self.pixels <= MAXIMUM_TEXT_PIXELS_PER_ROW)
            .then_some(self)
    }

    pub(crate) fn merge(&mut self, other: Self) {
        self.min_x = self.min_x.min(other.min_x);
        self.max_x = self.max_x.max(other.max_x);
        self.min_y = self.min_y.min(other.min_y);
        self.max_y = self.max_y.max(other.max_y);
        self.pixels += other.pixels;
    }

    pub(crate) fn center_x(&self) -> f32 {
        (self.min_x + self.max_x) as f32 / 2.0
    }

    pub(crate) fn min_x(&self) -> usize {
        self.min_x
    }

    pub(crate) fn min_y(&self) -> usize {
        self.min_y
    }

    pub(crate) fn height(&self) -> usize {
        self.max_y - self.min_y + 1
    }

    pub(crate) fn is_centered(&self, preview: PreviewBounds) -> bool {
        (self.center_x() - preview.center_x()).abs() <= 120.0
    }

    pub(crate) fn assert_centered(&self, preview: PreviewBounds) {
        assert!(
            self.is_centered(preview),
            "expected centered band: {self:?} preview={preview:?}"
        );
    }

    pub(crate) fn assert_right_aligned(&self, preview: PreviewBounds) {
        assert!(
            self.max_x + 80 >= preview.right(),
            "expected right-aligned band: {self:?} preview={preview:?}"
        );
    }

    pub(crate) fn assert_left_aligned(&self, preview: PreviewBounds) {
        assert!(
            self.min_x <= preview.x + 80,
            "expected left-aligned band: {self:?} preview={preview:?}"
        );
    }
}
