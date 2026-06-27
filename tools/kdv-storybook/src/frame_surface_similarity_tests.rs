use super::{ContentBounds, SurfaceParityScorer};

#[test]
fn bounds_score_uses_surface_relative_error_not_raw_pixels() {
    let reference = ContentBounds {
        min_x: 52,
        min_y: 77,
        max_x: 1223,
        max_y: 16973,
    };
    let candidate = ContentBounds {
        min_x: 56,
        min_y: 79,
        max_x: 1223,
        max_y: 16747,
    };

    assert_eq!(
        99,
        SurfaceParityScorer::bounds_score(Some(reference), Some(candidate), 1280, 17040)
    );
}

#[test]
fn bounds_score_still_fails_missing_content() {
    assert_eq!(
        0,
        SurfaceParityScorer::bounds_score(Some(ContentBounds::empty()), None, 1280, 720)
    );
}

#[test]
fn surface_parity_rejects_both_blank_surfaces() {
    let reference = white_surface(32, 32);
    let candidate = white_surface(32, 32);

    let report = SurfaceParityScorer::report(&reference, &candidate, 32, 32);

    assert_eq!(0, report.score, "{report:#?}");
}

#[test]
fn content_score_rejects_same_bounds_missing_foreground_pixels() {
    let reference = surface_with_points(
        &[
            (1, 1),
            (30, 1),
            (1, 30),
            (30, 30),
            (15, 15),
            (16, 15),
            (15, 16),
            (16, 16),
        ],
        32,
        32,
    );
    let candidate = surface_with_points(&[(1, 1), (30, 1), (1, 30), (30, 30)], 32, 32);

    let report = SurfaceParityScorer::report(&reference, &candidate, 32, 32);

    assert!(report.average_score >= 95, "{report:#?}");
    assert!(
        report.score < 95,
        "same bounds must not pass when foreground content is missing: {report:#?}"
    );
}

#[test]
fn content_score_rejects_large_surface_missing_center_content() {
    let mut reference_points = edge_points(640, 640);
    reference_points.extend(block_points(300, 300, 40, 40));
    let candidate = surface_with_points(&edge_points(640, 640), 640, 640);
    let reference = surface_with_points(&reference_points, 640, 640);

    let report = SurfaceParityScorer::report(&reference, &candidate, 640, 640);

    assert!(report.average_score >= 95, "{report:#?}");
    assert!(
        report.score < 95,
        "large-surface tolerance must not hide missing center content: {report:#?}"
    );
}

#[test]
fn content_score_rejects_large_surface_row_shift_inside_same_bounds() {
    let mut reference_points = edge_points(640, 640);
    reference_points.extend(block_points(160, 220, 320, 3));
    let mut candidate_points = edge_points(640, 640);
    candidate_points.extend(block_points(160, 240, 320, 3));
    let reference = surface_with_points(&reference_points, 640, 640);
    let candidate = surface_with_points(&candidate_points, 640, 640);

    let report = SurfaceParityScorer::report(&reference, &candidate, 640, 640);

    assert!(report.average_score >= 95, "{report:#?}");
    assert!(
        report.score < 95,
        "same bounds and nearby foreground tiles must not hide shifted rows: {report:#?}"
    );
}

#[test]
fn dominant_background_uses_most_common_surface_color_not_first_pixel() {
    let mut surface = [30, 30, 30, 255].repeat(16);
    surface[0] = 37;
    surface[1] = 37;
    surface[2] = 38;

    let rgb_pair = super::SurfacePixels::rgba_pair_to_rgb(&surface, &surface);
    assert!(rgb_pair.is_ok(), "same-sized rgba surface must convert");
    let Ok((rgb, _)) = rgb_pair else {
        return;
    };

    assert_eq!([30, 30, 30], SurfaceParityScorer::dominant_background(&rgb));
}

#[test]
fn surface_parity_scores_dimension_mismatch_instead_of_ignoring_it() {
    let reference = surface_with_points(&[(1, 1), (30, 14)], 32, 32);
    let candidate = surface_with_points(&[(1, 1), (30, 14)], 32, 16);

    let report =
        SurfaceParityScorer::report_with_dimensions(&reference, &candidate, 32, 32, 32, 16);

    assert!(report.dimension_score < 100, "{report:#?}");
    assert!(
        report.score < 95,
        "dimension mismatch must not pass as visual parity: {report:#?}"
    );
}

#[test]
fn surface_parity_composites_transparent_reference_background_before_scoring() {
    let reference = transparent_surface_with_points(&edge_points(32, 32), 32, 32);
    let candidate = surface_with_points(&edge_points(32, 32), 32, 32);

    let report = SurfaceParityScorer::report(&reference, &candidate, 32, 32);

    assert!(report.average_score >= 95, "{report:#?}");
    assert!(
        report.score >= 95,
        "transparent export background must not make an identical preview fail: {report:#?}"
    );
}

#[test]
fn surface_parity_keeps_candidate_extra_content_visible_on_transparent_reference() {
    let reference = transparent_surface_with_points(&edge_points(64, 64), 64, 64);
    let mut candidate_points = edge_points(64, 64);
    candidate_points.extend(block_points(24, 24, 16, 16));
    let candidate = surface_with_points(&candidate_points, 64, 64);

    let report = SurfaceParityScorer::report(&reference, &candidate, 64, 64);

    assert!(
        report.score < 95,
        "candidate-only foreground must still fail after transparent background normalization: {report:#?}"
    );
}

fn edge_points(width: usize, height: usize) -> Vec<(usize, usize)> {
    vec![
        (1, 1),
        (width - 2, 1),
        (1, height - 2),
        (width - 2, height - 2),
    ]
}

fn block_points(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    for yy in y..y + height {
        for xx in x..x + width {
            points.push((xx, yy));
        }
    }
    points
}

fn surface_with_points(points: &[(usize, usize)], width: usize, height: usize) -> Vec<u8> {
    let mut rgba = white_surface(width, height);
    for (x, y) in points {
        let offset = (y * width + x) * 4;
        rgba[offset] = 0;
        rgba[offset + 1] = 0;
        rgba[offset + 2] = 0;
        rgba[offset + 3] = 255;
    }
    rgba
}

fn transparent_surface_with_points(
    points: &[(usize, usize)],
    width: usize,
    height: usize,
) -> Vec<u8> {
    let mut rgba = vec![0; width * height * 4];
    for (x, y) in points {
        let offset = (y * width + x) * 4;
        rgba[offset] = 0;
        rgba[offset + 1] = 0;
        rgba[offset + 2] = 0;
        rgba[offset + 3] = 255;
    }
    rgba
}

fn white_surface(width: usize, height: usize) -> Vec<u8> {
    vec![255; width * height * 4]
}
