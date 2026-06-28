use std::collections::BTreeMap;

const RGB_CHANNELS: usize = 3;
const VISUAL_CHANNEL_DELTA_MIN: u8 = 8;
const NON_BACKGROUND_PIXEL_MIN: usize = 128;
const NON_BACKGROUND_RATIO_DENOMINATOR: usize = 250;

pub(crate) struct RgbVisualContentStats {
    minimum: [u8; RGB_CHANNELS],
    maximum: [u8; RGB_CHANNELS],
    color_bucket_counts: BTreeMap<u16, usize>,
    visible_pixel_count: usize,
}

impl RgbVisualContentStats {
    pub(crate) fn empty() -> Self {
        Self {
            minimum: [u8::MAX; RGB_CHANNELS],
            maximum: [u8::MIN; RGB_CHANNELS],
            color_bucket_counts: BTreeMap::new(),
            visible_pixel_count: 0,
        }
    }

    pub(crate) fn observe(&mut self, pixel: [u8; RGB_CHANNELS]) {
        self.visible_pixel_count += 1;
        for (index, channel) in pixel.into_iter().enumerate() {
            self.minimum[index] = self.minimum[index].min(channel);
            self.maximum[index] = self.maximum[index].max(channel);
        }
        *self
            .color_bucket_counts
            .entry(Self::color_bucket(pixel))
            .or_insert(0) += 1;
    }

    pub(crate) fn has_visible_content(&self) -> bool {
        self.has_visible_contrast() && self.has_enough_non_background_pixels()
    }

    fn has_visible_contrast(&self) -> bool {
        self.visible_pixel_count > 0
            && self
                .maximum
                .iter()
                .zip(self.minimum.iter())
                .any(|(maximum, minimum)| {
                    maximum.saturating_sub(*minimum) > VISUAL_CHANNEL_DELTA_MIN
                })
    }

    fn has_enough_non_background_pixels(&self) -> bool {
        self.non_background_pixel_count() >= self.required_non_background_pixels()
    }

    fn non_background_pixel_count(&self) -> usize {
        let dominant_count = self
            .color_bucket_counts
            .values()
            .copied()
            .max()
            .unwrap_or(0);
        self.visible_pixel_count.saturating_sub(dominant_count)
    }

    fn required_non_background_pixels(&self) -> usize {
        NON_BACKGROUND_PIXEL_MIN.max(
            self.visible_pixel_count
                .saturating_div(NON_BACKGROUND_RATIO_DENOMINATOR),
        )
    }

    fn color_bucket(pixel: [u8; RGB_CHANNELS]) -> u16 {
        let red = (pixel[0] >> 4) as u16;
        let green = (pixel[1] >> 4) as u16;
        let blue = (pixel[2] >> 4) as u16;
        (red << 8) | (green << 4) | blue
    }
}
