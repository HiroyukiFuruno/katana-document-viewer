use super::color::color_distance;
use super::{PreviewBounds, TextBand};
use crate::canvas::Canvas;
use std::collections::BTreeMap;

const DARK_BACKGROUND_COLOR: u32 = 0x151515;
const DARK_HEADING_RULE_COLOR: u32 = 0x264f78;
const TEXT_BAND_GAP: usize = 2;

pub(crate) struct TextBandCollector<'a> {
    canvas: &'a Canvas,
    preview: PreviewBounds,
}

impl<'a> TextBandCollector<'a> {
    pub(crate) fn new(canvas: &'a Canvas, preview: PreviewBounds) -> Self {
        Self { canvas, preview }
    }

    pub(crate) fn collect(&self) -> Vec<TextBand> {
        let mut bands = Vec::new();
        let mut current = None;
        for y in self.preview.y..self.preview.bottom().min(self.canvas.height()) {
            current = self.collect_row(y, current, &mut bands);
        }
        if let Some(band) = current {
            bands.push(band);
        }
        bands
    }

    fn collect_row(
        &self,
        y: usize,
        current: Option<TextBand>,
        bands: &mut Vec<TextBand>,
    ) -> Option<TextBand> {
        let Some(row) = self.row_bounds(y) else {
            return Self::finish_stale_band(y, current, bands);
        };
        Some(match current {
            Some(mut band) if y <= band.max_y + TEXT_BAND_GAP => {
                band.merge(row);
                band
            }
            Some(band) => {
                bands.push(band);
                row
            }
            None => row,
        })
    }

    fn finish_stale_band(
        y: usize,
        current: Option<TextBand>,
        bands: &mut Vec<TextBand>,
    ) -> Option<TextBand> {
        match current {
            Some(band) if y > band.max_y + TEXT_BAND_GAP => {
                bands.push(band);
                None
            }
            band => band,
        }
    }

    fn row_bounds(&self, y: usize) -> Option<TextBand> {
        let background = self.row_background_color(y);
        let mut row = TextBand::empty(y);
        for x in self.preview.x..self.preview.right().min(self.canvas.width()) {
            let index = y * self.canvas.width() + x;
            if Self::is_text_pixel(self.canvas.pixels()[index], background) {
                row.observe(x);
            }
        }
        row.valid()
    }

    fn row_background_color(&self, y: usize) -> u32 {
        let mut counts = BTreeMap::new();
        for x in self.preview.x..self.preview.right().min(self.canvas.width()) {
            let index = y * self.canvas.width() + x;
            let count = counts.entry(self.canvas.pixels()[index]).or_insert(0usize);
            *count += 1;
        }
        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(value, _)| value)
            .unwrap_or(DARK_BACKGROUND_COLOR)
    }

    fn is_text_pixel(pixel: u32, background: u32) -> bool {
        !matches!(pixel, DARK_BACKGROUND_COLOR | DARK_HEADING_RULE_COLOR)
            && color_distance(pixel, background) > 8
    }
}
