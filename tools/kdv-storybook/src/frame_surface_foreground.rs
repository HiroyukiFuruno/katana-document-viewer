use super::{
    CONTENT_DISTANCE_THRESHOLD, PERCENT_SCALE, RGB_CHANNELS, content_bounds::ContentBounds,
};
use std::collections::BTreeSet;

const CONTENT_TILE_SIZE: usize = 4;
const CONTENT_TILE_RADIUS: usize = 2;
const LARGE_SURFACE_TILE_RADIUS: usize = 6;
const LARGE_SURFACE_MIN_EDGE: usize = 512;

pub(super) struct ForegroundPreservation;

impl ForegroundPreservation {
    pub(super) fn score(
        reference: &[u8],
        candidate: &[u8],
        width: usize,
        height: usize,
        reference_background: [u8; RGB_CHANNELS],
        candidate_background: [u8; RGB_CHANNELS],
    ) -> u8 {
        let reference_cells =
            ForegroundCells::collect(reference, width, height, reference_background);
        let candidate_cells =
            ForegroundCells::collect(candidate, width, height, candidate_background);
        let tile_radius = Self::tile_radius(width, height);
        let preserved = reference_cells
            .cells
            .iter()
            .filter(|cell| candidate_cells.contains_near(**cell, tile_radius))
            .count();
        Self::ratio_score(preserved, reference_cells.cells.len())
    }

    fn is_background(
        rgb: &[u8],
        width: usize,
        x: usize,
        y: usize,
        background: [u8; RGB_CHANNELS],
    ) -> bool {
        let offset = (y * width + x) * RGB_CHANNELS;
        ContentBounds::pixel_distance(&rgb[offset..offset + RGB_CHANNELS], &background)
            < CONTENT_DISTANCE_THRESHOLD
    }

    fn ratio_score(preserved: usize, total: usize) -> u8 {
        if total == 0 {
            return 100;
        }
        (preserved as f64 / total as f64 * PERCENT_SCALE)
            .ceil()
            .clamp(0.0, PERCENT_SCALE) as u8
    }

    fn tile_radius(width: usize, height: usize) -> usize {
        if width.min(height) >= LARGE_SURFACE_MIN_EDGE {
            return LARGE_SURFACE_TILE_RADIUS;
        }
        CONTENT_TILE_RADIUS
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ForegroundCells {
    cells: BTreeSet<ForegroundCell>,
}

impl ForegroundCells {
    fn collect(rgb: &[u8], width: usize, height: usize, background: [u8; RGB_CHANNELS]) -> Self {
        let mut cells = BTreeSet::new();
        for y in 0..height {
            for x in 0..width {
                if ForegroundPreservation::is_background(rgb, width, x, y, background) {
                    continue;
                }
                cells.insert(ForegroundCell::from_pixel(x, y));
            }
        }
        Self { cells }
    }

    fn contains_near(&self, cell: ForegroundCell, radius: usize) -> bool {
        for y in cell.y.saturating_sub(radius)..=cell.y + radius {
            for x in cell.x.saturating_sub(radius)..=cell.x + radius {
                if self.cells.contains(&ForegroundCell { x, y }) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ForegroundCell {
    x: usize,
    y: usize,
}

impl ForegroundCell {
    fn from_pixel(x: usize, y: usize) -> Self {
        Self {
            x: x / CONTENT_TILE_SIZE,
            y: y / CONTENT_TILE_SIZE,
        }
    }
}
