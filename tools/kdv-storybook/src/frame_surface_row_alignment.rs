use super::content_bounds::ContentBounds;
use super::{CONTENT_DISTANCE_THRESHOLD, PERCENT_SCALE, RGB_CHANNELS};

const ROW_TOLERANCE: usize = 2;
const ROW_PROFILE_TILE_WIDTH: usize = 4;

pub(super) struct RowAlignment;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct RowAlignmentReport {
    pub(super) score: u8,
    pub(super) reference_to_candidate_score: u8,
    pub(super) candidate_to_reference_score: u8,
    pub(super) reference_loss_bands: Vec<RowLossBand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RowLossBand {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) loss: usize,
}

impl RowAlignment {
    pub(super) fn report(
        reference: &[u8],
        candidate: &[u8],
        width: usize,
        height: usize,
        reference_background: [u8; RGB_CHANNELS],
        candidate_background: [u8; RGB_CHANNELS],
    ) -> RowAlignmentReport {
        let reference_profile = RowProfile::collect(reference, width, height, reference_background);
        let candidate_profile = RowProfile::collect(candidate, width, height, candidate_background);
        let reference_to_candidate_score =
            Self::directional_score(&reference_profile, &candidate_profile);
        let candidate_to_reference_score =
            Self::directional_score(&candidate_profile, &reference_profile);
        RowAlignmentReport {
            score: reference_to_candidate_score.min(candidate_to_reference_score),
            reference_to_candidate_score,
            candidate_to_reference_score,
            reference_loss_bands: Self::top_loss_bands(&reference_profile, &candidate_profile),
        }
    }

    fn directional_score(reference: &RowProfile, candidate: &RowProfile) -> u8 {
        let total = reference.total();
        if total == 0 {
            return 100;
        }
        let preserved = reference
            .counts
            .iter()
            .enumerate()
            .map(|(row, count)| (*count).min(candidate.max_near(row)))
            .sum::<usize>();
        (preserved as f64 / total as f64 * PERCENT_SCALE)
            .round()
            .clamp(0.0, PERCENT_SCALE) as u8
    }

    fn top_loss_bands(reference: &RowProfile, candidate: &RowProfile) -> Vec<RowLossBand> {
        let mut bands = Vec::new();
        let mut current: Option<RowLossBand> = None;
        for (row, count) in reference.counts.iter().enumerate() {
            let loss = count.saturating_sub(candidate.max_near(row));
            if loss == 0 {
                if let Some(band) = current.take() {
                    bands.push(band);
                }
                continue;
            }
            match &mut current {
                Some(band) => {
                    band.end = row;
                    band.loss += loss;
                }
                None => {
                    current = Some(RowLossBand {
                        start: row,
                        end: row,
                        loss,
                    });
                }
            }
        }
        if let Some(band) = current {
            bands.push(band);
        }
        bands.sort_by_key(|band| std::cmp::Reverse(band.loss));
        bands.truncate(12);
        bands
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RowProfile {
    counts: Vec<usize>,
}

impl RowProfile {
    fn collect(rgb: &[u8], width: usize, height: usize, background: [u8; RGB_CHANNELS]) -> Self {
        let mut counts = vec![0; height];
        let tile_count = width.div_ceil(ROW_PROFILE_TILE_WIDTH);
        for (row, count) in counts.iter_mut().enumerate() {
            let mut occupied_tiles = vec![false; tile_count];
            for column in 0..width {
                let offset = (row * width + column) * RGB_CHANNELS;
                if ContentBounds::pixel_distance(&rgb[offset..offset + RGB_CHANNELS], &background)
                    >= CONTENT_DISTANCE_THRESHOLD
                {
                    occupied_tiles[column / ROW_PROFILE_TILE_WIDTH] = true;
                }
            }
            *count = usize::from(occupied_tiles.into_iter().any(|occupied| occupied));
        }
        Self { counts }
    }

    fn total(&self) -> usize {
        self.counts.iter().sum()
    }

    fn max_near(&self, row: usize) -> usize {
        let start = row.saturating_sub(ROW_TOLERANCE);
        let end = (row + ROW_TOLERANCE + 1).min(self.counts.len());
        self.counts[start..end].iter().copied().max().unwrap_or(0)
    }
}
