use super::{CONTENT_DISTANCE_THRESHOLD, RGB_CHANNELS};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ContentBounds {
    pub(super) min_x: usize,
    pub(super) min_y: usize,
    pub(super) max_x: usize,
    pub(super) max_y: usize,
}

impl ContentBounds {
    pub(super) fn collect(
        rgb: &[u8],
        width: usize,
        height: usize,
        background: [u8; RGB_CHANNELS],
    ) -> Option<Self> {
        let mut bounds = Self::empty();
        for y in 0..height {
            for x in 0..width {
                let offset = (y * width + x) * RGB_CHANNELS;
                if Self::pixel_distance(&rgb[offset..offset + RGB_CHANNELS], &background)
                    >= CONTENT_DISTANCE_THRESHOLD
                {
                    bounds.include(x, y);
                }
            }
        }
        bounds.into_option()
    }

    pub(super) fn empty() -> Self {
        Self {
            min_x: usize::MAX,
            min_y: usize::MAX,
            max_x: 0,
            max_y: 0,
        }
    }

    pub(super) fn max_delta(self, other: Self) -> usize {
        self.min_x
            .abs_diff(other.min_x)
            .max(self.min_y.abs_diff(other.min_y))
            .max(self.max_x.abs_diff(other.max_x))
            .max(self.max_y.abs_diff(other.max_y))
    }

    pub(super) fn pixel_distance(left: &[u8], right: &[u8; RGB_CHANNELS]) -> u16 {
        left.iter()
            .zip(right.iter())
            .map(|(left, right)| left.abs_diff(*right) as u16)
            .sum()
    }

    fn include(&mut self, x: usize, y: usize) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }

    fn into_option(self) -> Option<Self> {
        if self.min_x == usize::MAX {
            None
        } else {
            Some(self)
        }
    }
}
