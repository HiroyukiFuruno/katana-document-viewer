use super::{FrameRenderRequest, StorybookFrameRenderer};
use crate::args::StorybookArgs;
use crate::catalog::{FixtureCatalog, StorybookFixture};
use crate::preview::PreviewBuilder;
use crate::preview_build_request::{PreviewBuildAssetMode, PreviewBuildRequest};
use crate::preview_build_support::PreviewBuildSupport;
use crate::window::StorybookWindow;
use katana_document_viewer::{
    ViewerInteractionConfig, ViewerMode, ViewerSearchState, ViewerTarget, ViewerViewport,
};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const FRAME_WIDTH: usize = 1280;
const FRAME_HEIGHT: usize = 720;
const LARGE_FRAME_WIDTH: usize = 2048;
const LARGE_FRAME_HEIGHT: usize = 1496;
const PREVIEW_WIDTH: f32 = 900.0;
const PREVIEW_HEIGHT: f32 = 600.0;
const STORYBOOK_SWITCH_BUDGET: Duration = Duration::from_millis(600);
const SCROLL_FRAME_COUNT: usize = 12;
const STORYBOOK_SCROLL_BUDGET: Duration = Duration::from_millis(600);
const DEEP_SCROLL_FRAME_COUNT: usize = 6;
const DEEP_SCROLL_BUDGET: Duration = Duration::from_millis(600);
const WINDOW_SCROLL_FRAME_COUNT: usize = 8;
const WINDOW_SCROLL_BUDGET: Duration = Duration::from_millis(130);
const WINDOW_SCROLL_HOVER_BUDGET: Duration = Duration::from_millis(180);
const WINDOW_HOVER_RESOLUTION_BUDGET: Duration = Duration::from_millis(180);
const SCROLL_PERFORMANCE_ARTIFACT_ENV: &str = "KDV_STORYBOOK_SCROLL_PERFORMANCE_ARTIFACT";
const DEFAULT_SCROLL_PERFORMANCE_ARTIFACT: &str =
    "target/acceptance/kdv-storybook-scroll-performance.txt";
const SCROLL_PERFORMANCE_MAX_FULL_REDRAW_FALLBACKS: usize = 0;

#[test]
#[ignore = "release-only performance gate"]
fn lazy_storybook_switch_renders_frame_inside_interactive_budget()
-> Result<(), Box<dyn std::error::Error>> {
    warm_storybook_renderer()?;
    let fixture = StorybookFixture {
        label: "katana/sample_diagrams.md".to_string(),
        path: fixture_path("katana/sample_diagrams.md"),
    };
    let builder = PreviewBuilder::default();
    let viewport = ViewerViewport {
        width: PREVIEW_WIDTH,
        height: PREVIEW_HEIGHT,
    };
    let started = Instant::now();
    let source_started = Instant::now();
    let source = builder.source_for_fixture(&fixture)?;
    let source_elapsed = source_started.elapsed();
    let config_started = Instant::now();
    let config = PreviewBuildSupport::preview_config(
        viewport,
        0.0,
        true,
        ViewerInteractionConfig::default(),
        ViewerMode::Document,
        Default::default(),
        ViewerSearchState::default(),
    );
    let config_elapsed = config_started.elapsed();
    let output_started = Instant::now();
    let output = builder.render_output(&source, &config)?;
    let output_elapsed = output_started.elapsed();
    let request = PreviewBuildRequest {
        fixture: &fixture,
        viewport,
        dark: true,
        theme: None,
        interaction: ViewerInteractionConfig::default(),
        mode: ViewerMode::Document,
        typography: Default::default(),
        search: ViewerSearchState::default(),
        diagram_viewports: BTreeMap::new(),
        image_viewports: BTreeMap::new(),
        task_state_overrides: BTreeMap::new(),
        accordion_open_overrides: BTreeMap::new(),
        copied_code_node_ids: Default::default(),
        asset_mode: PreviewBuildAssetMode::Lazy,
        attach_surface: false,
        export_surface: false,
    };
    let scene_started = Instant::now();
    let scene = builder.scene_from_output(&source, &request, output, Default::default())?;
    let scene_elapsed = scene_started.elapsed();
    let build_elapsed = started.elapsed();
    let interaction = ViewerInteractionConfig::default();
    let render_started = Instant::now();
    let request = FrameRenderRequest {
        width: FRAME_WIDTH,
        height: FRAME_HEIGHT,
        fixtures: &[fixture],
        selected_index: 0,
        scene: Some(&scene),
        scroll_y: 0.0,
        sidebar_scroll: Default::default(),
        file_tree_state: Default::default(),
        settings_state: &Default::default(),
        dark: true,
        interaction: &interaction,
        typography: Default::default(),
        last_command_label: "none",
        task_context_menu: None,
        hovered_node_id: None,
        hovered_action_node_id: None,
        animation_phase: 0,
    };
    let sidebar_started = Instant::now();
    let sidebar = StorybookFrameRenderer::render_sidebar(&request);
    let sidebar_elapsed = sidebar_started.elapsed();
    let content_started = Instant::now();
    let _canvas = StorybookFrameRenderer::render_with_sidebar(request, &sidebar);
    let content_elapsed = content_started.elapsed();
    let render_elapsed = render_started.elapsed();
    let elapsed = started.elapsed();

    assert!(
        elapsed <= STORYBOOK_SWITCH_BUDGET,
        "lazy Storybook switch frame took {elapsed:?}, source {source_elapsed:?}, config {config_elapsed:?}, output {output_elapsed:?}, scene {scene_elapsed:?}, build {build_elapsed:?}, render {render_elapsed:?}, sidebar {sidebar_elapsed:?}, content {content_elapsed:?}, budget {STORYBOOK_SWITCH_BUDGET:?}"
    );
    Ok(())
}

fn warm_storybook_renderer() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample_basic.md".to_string(),
        path: fixture_path("katana/sample_basic.md"),
    };
    let scene = PreviewBuilder::default().build_lazy_with_mode_and_search(
        &fixture,
        ViewerViewport {
            width: PREVIEW_WIDTH,
            height: PREVIEW_HEIGHT,
        },
        true,
        ViewerInteractionConfig::default(),
        ViewerMode::Document,
        ViewerSearchState::default(),
    )?;
    let sidebar = render_sidebar_for_scroll(&fixture, &scene);
    render_scroll_frame_with_sidebar(&fixture, &scene, 0.0, &sidebar);
    Ok(())
}

#[test]
#[ignore = "release-only performance gate"]
fn cached_round_trip_storybook_switch_stays_inside_interactive_budget()
-> Result<(), Box<dyn std::error::Error>> {
    let first = StorybookFixture {
        label: "katana/sample.md".to_string(),
        path: fixture_path("katana/sample.md"),
    };
    let second = StorybookFixture {
        label: "katana/sample_basic.md".to_string(),
        path: fixture_path("katana/sample_basic.md"),
    };
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![first, second],
        },
        PreviewBuilder::default(),
    );

    storybook.update_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    let _ = storybook.render_canvas_for_tests(FRAME_WIDTH, FRAME_HEIGHT);
    storybook.select_fixture_index_for_tests(1);
    storybook.update_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    let _ = storybook.render_canvas_for_tests(FRAME_WIDTH, FRAME_HEIGHT);

    let started = Instant::now();
    storybook.select_fixture_index_for_tests(0);
    let scene_started = Instant::now();
    storybook.update_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    let scene_elapsed = scene_started.elapsed();
    let render_started = Instant::now();
    let _ = storybook.render_canvas_for_tests(FRAME_WIDTH, FRAME_HEIGHT);
    let render_elapsed = render_started.elapsed();
    let elapsed = started.elapsed();

    assert!(
        elapsed <= STORYBOOK_SWITCH_BUDGET,
        "cached round-trip switch took {elapsed:?}, scene {scene_elapsed:?}, render {render_elapsed:?}, sidebar_misses {}, budget {STORYBOOK_SWITCH_BUDGET:?}",
        storybook.sidebar_frame_cache_misses_for_tests()
    );
    Ok(())
}

#[test]
#[ignore = "release-only performance gate"]
fn repeated_storybook_scroll_frames_stay_inside_interactive_budget()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample_diagrams.md".to_string(),
        path: fixture_path("katana/sample_diagrams.md"),
    };
    let scene = PreviewBuilder::default().build_lazy_with_mode_and_search(
        &fixture,
        ViewerViewport {
            width: PREVIEW_WIDTH,
            height: PREVIEW_HEIGHT,
        },
        true,
        ViewerInteractionConfig::default(),
        ViewerMode::Document,
        ViewerSearchState::default(),
    )?;
    let sidebar = render_sidebar_for_scroll(&fixture, &scene);
    render_scroll_frame_with_sidebar(&fixture, &scene, 0.0, &sidebar);

    let started = Instant::now();
    for index in 0..SCROLL_FRAME_COUNT {
        render_scroll_frame_with_sidebar(&fixture, &scene, (index as f32 + 1.0) * 96.0, &sidebar);
    }
    let elapsed = started.elapsed();

    assert!(
        elapsed <= STORYBOOK_SCROLL_BUDGET,
        "{SCROLL_FRAME_COUNT} Storybook scroll frames took {elapsed:?}, budget {STORYBOOK_SCROLL_BUDGET:?}"
    );
    Ok(())
}

#[test]
#[ignore = "release-only performance gate"]
fn deep_storybook_scroll_frames_stay_inside_interactive_budget()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample.md".to_string(),
        path: fixture_path("katana/sample.md"),
    };
    let scene = PreviewBuilder::default().build_lazy_with_mode_and_search(
        &fixture,
        ViewerViewport {
            width: PREVIEW_WIDTH,
            height: PREVIEW_HEIGHT,
        },
        true,
        ViewerInteractionConfig::default(),
        ViewerMode::Document,
        ViewerSearchState::default(),
    )?;
    let max_scroll = (scene.content_height - PREVIEW_HEIGHT).max(0.0);
    let base_scroll = max_scroll.min(10_000.0);

    assert!(
        base_scroll > PREVIEW_HEIGHT,
        "deep scroll fixture must be taller than the viewport: content_height={}",
        scene.content_height
    );

    let sidebar = render_sidebar_for_scroll(&fixture, &scene);
    render_scroll_frame_with_sidebar(&fixture, &scene, base_scroll, &sidebar);
    let mut frame_times = Vec::new();
    let started = Instant::now();
    for index in 0..DEEP_SCROLL_FRAME_COUNT {
        let frame_started = Instant::now();
        render_scroll_frame_with_sidebar(
            &fixture,
            &scene,
            base_scroll + (index as f32 * 48.0),
            &sidebar,
        );
        frame_times.push(frame_started.elapsed());
    }
    let elapsed = started.elapsed();

    assert!(
        elapsed <= DEEP_SCROLL_BUDGET,
        "{DEEP_SCROLL_FRAME_COUNT} deep Storybook scroll frames took {elapsed:?}, frames {frame_times:?}, visible {}, budget {DEEP_SCROLL_BUDGET:?}",
        visible_target_summary(&scene.targets, base_scroll, PREVIEW_HEIGHT)
    );
    Ok(())
}

#[test]
#[ignore = "release-only performance gate"]
fn interactive_window_deep_scroll_frames_stay_inside_budget()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample.md".to_string(),
        path: fixture_path("katana/sample.md"),
    };
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![fixture],
        },
        PreviewBuilder::default(),
    );
    storybook.update_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    let max_scroll = storybook
        .scene_for_tests()
        .map(|scene| (scene.content_height - PREVIEW_HEIGHT).max(0.0))
        .unwrap_or(0.0);
    let base_scroll = max_scroll.min(10_000.0);
    storybook.scroll_y_for_tests(base_scroll);
    storybook.render_cached_scroll_canvas_scaled_for_tests(FRAME_WIDTH, FRAME_HEIGHT, 2.0)?;

    let mut frame_times = Vec::new();
    let started = Instant::now();
    for index in 0..WINDOW_SCROLL_FRAME_COUNT {
        storybook.scroll_y_for_tests(base_scroll + (index as f32 * 48.0));
        let frame_started = Instant::now();
        storybook.render_cached_scroll_canvas_scaled_for_tests(FRAME_WIDTH, FRAME_HEIGHT, 2.0)?;
        frame_times.push(frame_started.elapsed());
    }
    let elapsed = started.elapsed();

    assert!(
        elapsed <= WINDOW_SCROLL_BUDGET,
        "{WINDOW_SCROLL_FRAME_COUNT} interactive window scroll frames took {elapsed:?}, frames {frame_times:?}, budget {WINDOW_SCROLL_BUDGET:?}"
    );
    Ok(())
}

#[test]
#[ignore = "release-only performance gate"]
fn interactive_window_diagram_scroll_frames_stay_inside_budget()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample_diagrams.md".to_string(),
        path: fixture_path("katana/sample_diagrams.md"),
    };
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![fixture],
        },
        PreviewBuilder::default(),
    );
    storybook.update_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    let max_scroll = storybook
        .scene_for_tests()
        .map(|scene| (scene.content_height - PREVIEW_HEIGHT).max(0.0))
        .unwrap_or(0.0);
    let base_scroll = max_scroll.min(1_800.0);
    storybook.scroll_y_for_tests(base_scroll);
    let _canvas = storybook.render_canvas_for_tests(FRAME_WIDTH, FRAME_HEIGHT);

    let mut frame_times = Vec::new();
    let started = Instant::now();
    for index in 0..WINDOW_SCROLL_FRAME_COUNT {
        storybook.scroll_y_for_tests(base_scroll + (index as f32 * 48.0));
        let frame_started = Instant::now();
        let _canvas = storybook.render_canvas_for_tests(FRAME_WIDTH, FRAME_HEIGHT);
        frame_times.push(frame_started.elapsed());
    }
    let elapsed = started.elapsed();

    assert!(
        elapsed <= WINDOW_SCROLL_BUDGET,
        "{WINDOW_SCROLL_FRAME_COUNT} interactive diagram scroll frames took {elapsed:?}, frames {frame_times:?}, budget {WINDOW_SCROLL_BUDGET:?}"
    );
    Ok(())
}

#[test]
#[ignore = "release-only performance gate"]
fn interactive_window_loaded_diagram_scroll_frames_stay_inside_budget()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample_diagrams.md".to_string(),
        path: fixture_path("katana/sample_diagrams.md"),
    };
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![fixture],
        },
        PreviewBuilder::default(),
    );
    storybook.update_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    storybook.wait_loaded_asset_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    let max_scroll = storybook
        .scene_for_tests()
        .map(|scene| (scene.content_height - PREVIEW_HEIGHT).max(0.0))
        .unwrap_or(0.0);
    let base_scroll = max_scroll.min(1_800.0);
    storybook.scroll_y_for_tests(base_scroll);
    let _canvas = storybook.render_canvas_for_tests(FRAME_WIDTH, FRAME_HEIGHT);

    let mut frame_times = Vec::new();
    let started = Instant::now();
    for index in 0..WINDOW_SCROLL_FRAME_COUNT {
        storybook.scroll_y_for_tests(base_scroll + (index as f32 * 48.0));
        let frame_started = Instant::now();
        let _canvas = storybook.render_canvas_for_tests(FRAME_WIDTH, FRAME_HEIGHT);
        frame_times.push(frame_started.elapsed());
    }
    let elapsed = started.elapsed();

    assert!(
        elapsed <= WINDOW_SCROLL_BUDGET,
        "{WINDOW_SCROLL_FRAME_COUNT} interactive loaded diagram scroll frames took {elapsed:?}, frames {frame_times:?}, budget {WINDOW_SCROLL_BUDGET:?}"
    );
    Ok(())
}

#[test]
#[ignore = "release-only performance gate"]
fn interactive_window_loaded_diagram_scaled_scroll_frames_stay_inside_budget()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample_diagrams.md".to_string(),
        path: fixture_path("katana/sample_diagrams.md"),
    };
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![fixture],
        },
        PreviewBuilder::default(),
    );
    storybook.update_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    storybook.wait_loaded_asset_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    let max_scroll = storybook
        .scene_for_tests()
        .map(|scene| (scene.content_height - PREVIEW_HEIGHT).max(0.0))
        .unwrap_or(0.0);
    let base_scroll = max_scroll.min(1_800.0);
    storybook.scroll_y_for_tests(base_scroll);
    storybook.render_cached_scroll_canvas_scaled_for_tests(FRAME_WIDTH, FRAME_HEIGHT, 2.0)?;

    let mut frame_times = Vec::new();
    let started = Instant::now();
    for index in 0..WINDOW_SCROLL_FRAME_COUNT {
        storybook.scroll_y_for_tests(base_scroll + (index as f32 * 48.0));
        let frame_started = Instant::now();
        storybook.render_cached_scroll_canvas_scaled_for_tests(FRAME_WIDTH, FRAME_HEIGHT, 2.0)?;
        frame_times.push(frame_started.elapsed());
    }
    let elapsed = started.elapsed();

    assert!(
        elapsed <= WINDOW_SCROLL_BUDGET,
        "{WINDOW_SCROLL_FRAME_COUNT} interactive loaded diagram 2x scroll frames took {elapsed:?}, frames {frame_times:?}, budget {WINDOW_SCROLL_BUDGET:?}"
    );
    Ok(())
}

#[test]
#[ignore = "release-only performance gate"]
fn interactive_window_loaded_diagram_wheel_present_frames_stay_inside_budget()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample_diagrams.md".to_string(),
        path: fixture_path("katana/sample_diagrams.md"),
    };
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![fixture],
        },
        PreviewBuilder::default(),
    );
    storybook.update_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    storybook.wait_loaded_asset_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    let max_scroll = storybook
        .scene_for_tests()
        .map(|scene| (scene.content_height - PREVIEW_HEIGHT).max(0.0))
        .unwrap_or(0.0);
    let base_scroll = max_scroll.min(1_800.0);
    storybook.scroll_y_for_tests(base_scroll);
    storybook.render_cached_scroll_canvas_scaled_for_tests(FRAME_WIDTH, FRAME_HEIGHT, 2.0)?;

    let mut frame_times = Vec::new();
    let started = Instant::now();
    for _ in 0..WINDOW_SCROLL_FRAME_COUNT {
        let frame_started = Instant::now();
        storybook.render_wheel_scroll_presented_frame_for_tests(
            -1.0,
            FRAME_WIDTH,
            FRAME_HEIGHT,
            2.0,
        )?;
        frame_times.push(frame_started.elapsed());
    }
    let elapsed = started.elapsed();

    assert!(
        elapsed <= WINDOW_SCROLL_BUDGET,
        "{WINDOW_SCROLL_FRAME_COUNT} interactive loaded diagram wheel+present frames took {elapsed:?}, frames {frame_times:?}, budget {WINDOW_SCROLL_BUDGET:?}"
    );
    Ok(())
}

#[test]
#[ignore = "release-only performance gate"]
fn interactive_large_window_loaded_diagram_wheel_present_frames_stay_inside_budget()
-> Result<(), Box<dyn std::error::Error>> {
    let sample = large_window_loaded_diagram_wheel_performance_sample()?;

    assert_large_window_loaded_diagram_scroll_performance(&sample);
    Ok(())
}

#[test]
#[ignore = "release-only performance artifact"]
fn interactive_large_window_loaded_diagram_scroll_performance_artifact_is_actionable()
-> Result<(), Box<dyn std::error::Error>> {
    let sample = large_window_loaded_diagram_wheel_performance_sample()?;

    assert_large_window_loaded_diagram_scroll_performance(&sample);
    write_scroll_performance_artifact(&sample)?;
    Ok(())
}

struct ScrollPerformanceSample {
    elapsed: Duration,
    frame_times: Vec<Duration>,
    render_times: Vec<Duration>,
    present_times: Vec<Duration>,
    apply_times: Vec<Duration>,
    ensure_times: Vec<Duration>,
    redraw_times: Vec<Duration>,
    update_times: Vec<Duration>,
    asset_times: Vec<Duration>,
    full_preview_redraw_fallback_count: usize,
}

fn large_window_loaded_diagram_wheel_performance_sample()
-> Result<ScrollPerformanceSample, Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample_diagrams.md".to_string(),
        path: fixture_path("katana/sample_diagrams.md"),
    };
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![fixture],
        },
        PreviewBuilder::default(),
    );
    storybook.update_scene_for_tests(LARGE_FRAME_WIDTH, LARGE_FRAME_HEIGHT)?;
    storybook
        .wait_scroll_performance_asset_scene_for_tests(LARGE_FRAME_WIDTH, LARGE_FRAME_HEIGHT)?;
    let max_scroll = storybook
        .scene_for_tests()
        .map(|scene| (scene.content_height - PREVIEW_HEIGHT).max(0.0))
        .unwrap_or(0.0);
    let base_scroll = max_scroll.min(1_800.0);
    storybook.scroll_y_for_tests(base_scroll);
    storybook.render_cached_scroll_canvas_scaled_for_tests(
        LARGE_FRAME_WIDTH,
        LARGE_FRAME_HEIGHT,
        2.0,
    )?;

    let mut frame_times = Vec::new();
    let mut render_times = Vec::new();
    let mut present_times = Vec::new();
    let mut apply_times = Vec::new();
    let mut ensure_times = Vec::new();
    let mut redraw_times = Vec::new();
    let mut update_times = Vec::new();
    let mut asset_times = Vec::new();
    let mut full_preview_redraw_fallback_count = 0;
    let started = Instant::now();
    for _ in 0..WINDOW_SCROLL_FRAME_COUNT {
        let frame_started = Instant::now();
        let render_started = Instant::now();
        let phases = storybook.render_wheel_scroll_cached_frame_phase_times_for_tests(
            -1.0,
            LARGE_FRAME_WIDTH,
            LARGE_FRAME_HEIGHT,
            2.0,
        )?;
        if phases.full_preview_redraw_fallback {
            full_preview_redraw_fallback_count += 1;
        }
        apply_times.push(phases.apply);
        ensure_times.push(phases.ensure_presented);
        redraw_times.push(phases.redraw_band);
        update_times.push(phases.update_presented);
        asset_times.push(phases.asset_defer);
        render_times.push(render_started.elapsed());
        let present_started = Instant::now();
        let presented =
            storybook.present_cached_frame_for_tests(LARGE_FRAME_WIDTH, LARGE_FRAME_HEIGHT)?;
        if presented.pixels().is_empty() {
            return Err("presented large wheel scroll frame is empty".into());
        }
        present_times.push(present_started.elapsed());
        frame_times.push(frame_started.elapsed());
    }
    let elapsed = started.elapsed();

    Ok(ScrollPerformanceSample {
        elapsed,
        frame_times,
        render_times,
        present_times,
        apply_times,
        ensure_times,
        redraw_times,
        update_times,
        asset_times,
        full_preview_redraw_fallback_count,
    })
}

fn assert_large_window_loaded_diagram_scroll_performance(sample: &ScrollPerformanceSample) {
    assert!(
        sample.elapsed <= WINDOW_SCROLL_BUDGET,
        "{WINDOW_SCROLL_FRAME_COUNT} interactive large window loaded diagram wheel+present frames took {:?}, frames {:?}, render {:?}, apply {:?}, ensure {:?}, redraw {:?}, update {:?}, asset {:?}, present {:?}, full redraw fallbacks {}, budget {WINDOW_SCROLL_BUDGET:?}",
        sample.elapsed,
        sample.frame_times,
        sample.render_times,
        sample.apply_times,
        sample.ensure_times,
        sample.redraw_times,
        sample.update_times,
        sample.asset_times,
        sample.present_times,
        sample.full_preview_redraw_fallback_count,
    );
    assert_eq!(
        SCROLL_PERFORMANCE_MAX_FULL_REDRAW_FALLBACKS, sample.full_preview_redraw_fallback_count,
        "large loaded diagram wheel scroll must keep the presented-band path and avoid full preview redraw fallback"
    );
}

fn write_scroll_performance_artifact(
    sample: &ScrollPerformanceSample,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::var_os(SCROLL_PERFORMANCE_ARTIFACT_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_SCROLL_PERFORMANCE_ARTIFACT));
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut text = String::new();
    writeln!(text, "scenario=large_loaded_diagram_wheel_present")?;
    writeln!(text, "fixture=katana/sample_diagrams.md")?;
    writeln!(text, "window_width={LARGE_FRAME_WIDTH}")?;
    writeln!(text, "window_height={LARGE_FRAME_HEIGHT}")?;
    writeln!(text, "scale=2.0")?;
    writeln!(text, "frame_count={WINDOW_SCROLL_FRAME_COUNT}")?;
    writeln!(text, "budget_ms={}", duration_ms(WINDOW_SCROLL_BUDGET))?;
    writeln!(text, "elapsed_ms={}", duration_ms(sample.elapsed))?;
    writeln!(
        text,
        "max_frame_ms={}",
        duration_ms(max_duration(&sample.frame_times))
    )?;
    writeln!(
        text,
        "max_render_ms={}",
        duration_ms(max_duration(&sample.render_times))
    )?;
    writeln!(
        text,
        "max_present_ms={}",
        duration_ms(max_duration(&sample.present_times))
    )?;
    writeln!(
        text,
        "max_apply_ms={}",
        duration_ms(max_duration(&sample.apply_times))
    )?;
    writeln!(
        text,
        "max_ensure_presented_ms={}",
        duration_ms(max_duration(&sample.ensure_times))
    )?;
    writeln!(
        text,
        "max_redraw_band_ms={}",
        duration_ms(max_duration(&sample.redraw_times))
    )?;
    writeln!(
        text,
        "max_update_presented_ms={}",
        duration_ms(max_duration(&sample.update_times))
    )?;
    writeln!(
        text,
        "max_asset_defer_ms={}",
        duration_ms(max_duration(&sample.asset_times))
    )?;
    writeln!(
        text,
        "full_preview_redraw_fallback_count={}",
        sample.full_preview_redraw_fallback_count
    )?;
    std::fs::write(path, text)?;
    Ok(())
}

fn max_duration(values: &[Duration]) -> Duration {
    values.iter().copied().max().unwrap_or(Duration::ZERO)
}

fn duration_ms(duration: Duration) -> String {
    format!("{:.3}", duration.as_secs_f64() * 1000.0)
}

#[test]
#[ignore = "release-only performance gate"]
fn interactive_window_hovered_diagram_wheel_present_uses_kuc_presentation()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample_diagrams.md".to_string(),
        path: fixture_path("katana/sample_diagrams.md"),
    };
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![fixture],
        },
        PreviewBuilder::default(),
    );
    storybook.update_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    storybook.wait_loaded_asset_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    let max_scroll = storybook
        .scene_for_tests()
        .map(|scene| (scene.content_height - PREVIEW_HEIGHT).max(0.0))
        .unwrap_or(0.0);
    let base_scroll = max_scroll.min(1_800.0);
    let preview_area =
        crate::layout::StorybookPreviewArea::for_window(FRAME_WIDTH, FRAME_HEIGHT, 0.0);
    storybook.scroll_y_for_tests(base_scroll);
    storybook.render_cached_scroll_canvas_scaled_for_tests(FRAME_WIDTH, FRAME_HEIGHT, 2.0)?;
    storybook.update_document_hover_for_canvas_point(
        preview_area.x.saturating_add(320) as f32,
        preview_area.y.saturating_add(260) as f32,
        FRAME_WIDTH,
        FRAME_HEIGHT,
    );

    storybook.render_wheel_scroll_presented_frame_for_tests(
        -1.0,
        FRAME_WIDTH,
        FRAME_HEIGHT,
        2.0,
    )?;
    Ok(())
}

#[test]
#[ignore = "release-only performance gate"]
fn interactive_window_pending_diagram_wheel_present_frames_stay_inside_budget()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample_diagrams.md".to_string(),
        path: fixture_path("katana/sample_diagrams.md"),
    };
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![fixture],
        },
        PreviewBuilder::default(),
    );
    storybook.update_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    let max_scroll = storybook
        .scene_for_tests()
        .map(|scene| (scene.content_height - PREVIEW_HEIGHT).max(0.0))
        .unwrap_or(0.0);
    let base_scroll = max_scroll.min(1_800.0);
    storybook.scroll_y_for_tests(base_scroll);
    storybook.render_cached_scroll_canvas_scaled_for_tests(FRAME_WIDTH, FRAME_HEIGHT, 2.0)?;

    let mut frame_times = Vec::new();
    let started = Instant::now();
    for _ in 0..WINDOW_SCROLL_FRAME_COUNT {
        let frame_started = Instant::now();
        storybook.render_pending_wheel_scroll_presented_frame_for_tests(
            -1.0,
            FRAME_WIDTH,
            FRAME_HEIGHT,
            2.0,
        )?;
        frame_times.push(frame_started.elapsed());
    }
    let elapsed = started.elapsed();

    assert!(
        elapsed <= WINDOW_SCROLL_BUDGET,
        "{WINDOW_SCROLL_FRAME_COUNT} interactive pending diagram wheel+present frames took {elapsed:?}, frames {frame_times:?}, budget {WINDOW_SCROLL_BUDGET:?}"
    );
    Ok(())
}

#[test]
#[ignore = "release-only performance gate"]
fn interactive_window_loaded_diagram_scroll_hover_frames_stay_inside_budget()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample_diagrams.md".to_string(),
        path: fixture_path("katana/sample_diagrams.md"),
    };
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![fixture],
        },
        PreviewBuilder::default(),
    );
    storybook.update_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    storybook.wait_loaded_asset_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    let max_scroll = storybook
        .scene_for_tests()
        .map(|scene| (scene.content_height - PREVIEW_HEIGHT).max(0.0))
        .unwrap_or(0.0);
    let base_scroll = max_scroll.min(1_800.0);
    let hover_x = crate::layout::StorybookPreviewArea::for_window(FRAME_WIDTH, FRAME_HEIGHT, 0.0)
        .x
        .saturating_add(320) as f32;
    let hover_y = crate::layout::StorybookPreviewArea::for_window(FRAME_WIDTH, FRAME_HEIGHT, 0.0)
        .y
        .saturating_add(260) as f32;
    storybook.scroll_y_for_tests(base_scroll);
    let _canvas = storybook.render_canvas_for_tests(FRAME_WIDTH, FRAME_HEIGHT);

    let mut frame_times = Vec::new();
    let mut hover_clear_times = Vec::new();
    let mut render_times = Vec::new();
    storybook.update_document_hover_for_canvas_point(hover_x, hover_y, FRAME_WIDTH, FRAME_HEIGHT);
    let started = Instant::now();
    for index in 0..WINDOW_SCROLL_FRAME_COUNT {
        storybook.scroll_y_for_tests(base_scroll + (index as f32 * 48.0));
        let frame_started = Instant::now();
        let hover_clear_started = Instant::now();
        storybook.clear_document_hover_state();
        hover_clear_times.push(hover_clear_started.elapsed());
        let render_started = Instant::now();
        let _canvas = storybook.render_canvas_for_tests(FRAME_WIDTH, FRAME_HEIGHT);
        render_times.push(render_started.elapsed());
        frame_times.push(frame_started.elapsed());
    }
    let elapsed = started.elapsed();

    assert!(
        elapsed <= WINDOW_SCROLL_HOVER_BUDGET,
        "{WINDOW_SCROLL_FRAME_COUNT} interactive loaded diagram scroll+hover frames took {elapsed:?}, frames {frame_times:?}, hover_clear {hover_clear_times:?}, render {render_times:?}, budget {WINDOW_SCROLL_HOVER_BUDGET:?}"
    );
    Ok(())
}

#[test]
#[ignore = "release-only performance gate"]
fn interactive_window_loaded_diagram_hover_resolution_stays_inside_budget()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture = StorybookFixture {
        label: "katana/sample_diagrams.md".to_string(),
        path: fixture_path("katana/sample_diagrams.md"),
    };
    let mut storybook = StorybookWindow::new(
        StorybookArgs::default(),
        FixtureCatalog {
            fixtures: vec![fixture],
        },
        PreviewBuilder::default(),
    );
    storybook.update_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    storybook.wait_loaded_asset_scene_for_tests(FRAME_WIDTH, FRAME_HEIGHT)?;
    let max_scroll = storybook
        .scene_for_tests()
        .map(|scene| (scene.content_height - PREVIEW_HEIGHT).max(0.0))
        .unwrap_or(0.0);
    let base_scroll = max_scroll.min(1_800.0);
    let preview_area =
        crate::layout::StorybookPreviewArea::for_window(FRAME_WIDTH, FRAME_HEIGHT, 0.0);
    storybook.scroll_y_for_tests(base_scroll);
    let _canvas = storybook.render_canvas_for_tests(FRAME_WIDTH, FRAME_HEIGHT);

    let mut hover_times = Vec::new();
    let started = Instant::now();
    for index in 0..WINDOW_SCROLL_FRAME_COUNT {
        let hover_started = Instant::now();
        storybook.update_document_hover_for_canvas_point(
            preview_area.x.saturating_add(260 + index * 8) as f32,
            preview_area.y.saturating_add(220 + index * 4) as f32,
            FRAME_WIDTH,
            FRAME_HEIGHT,
        );
        hover_times.push(hover_started.elapsed());
    }
    let elapsed = started.elapsed();

    assert!(
        elapsed <= WINDOW_HOVER_RESOLUTION_BUDGET,
        "{WINDOW_SCROLL_FRAME_COUNT} interactive loaded diagram hover resolutions took {elapsed:?}, hover {hover_times:?}, budget {WINDOW_HOVER_RESOLUTION_BUDGET:?}"
    );
    Ok(())
}

fn render_sidebar_for_scroll(
    fixture: &StorybookFixture,
    scene: &crate::preview::PreviewScene,
) -> crate::canvas::Canvas {
    let interaction = ViewerInteractionConfig::default();
    let request = FrameRenderRequest {
        width: FRAME_WIDTH,
        height: FRAME_HEIGHT,
        fixtures: std::slice::from_ref(fixture),
        selected_index: 0,
        scene: Some(scene),
        scroll_y: 0.0,
        sidebar_scroll: Default::default(),
        file_tree_state: Default::default(),
        settings_state: &Default::default(),
        dark: true,
        interaction: &interaction,
        typography: Default::default(),
        last_command_label: "none",
        task_context_menu: None,
        hovered_node_id: None,
        hovered_action_node_id: None,
        animation_phase: 0,
    };
    StorybookFrameRenderer::render_sidebar(&request)
}

fn render_scroll_frame_with_sidebar(
    fixture: &StorybookFixture,
    scene: &crate::preview::PreviewScene,
    scroll_y: f32,
    sidebar: &crate::canvas::Canvas,
) {
    let interaction = ViewerInteractionConfig::default();
    let request = FrameRenderRequest {
        width: FRAME_WIDTH,
        height: FRAME_HEIGHT,
        fixtures: std::slice::from_ref(fixture),
        selected_index: 0,
        scene: Some(scene),
        scroll_y,
        sidebar_scroll: Default::default(),
        file_tree_state: Default::default(),
        settings_state: &Default::default(),
        dark: true,
        interaction: &interaction,
        typography: Default::default(),
        last_command_label: "none",
        task_context_menu: None,
        hovered_node_id: None,
        hovered_action_node_id: None,
        animation_phase: 0,
    };
    let _canvas = StorybookFrameRenderer::render_with_sidebar(request, sidebar);
}

fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../assets/fixtures/{path}"))
}

fn visible_target_summary(targets: &[ViewerTarget], scroll_y: f32, height: f32) -> String {
    let bottom = scroll_y + height;
    targets
        .iter()
        .filter(|target| target.rect.y < bottom && target.rect.y + target.rect.height > scroll_y)
        .take(6)
        .map(|target| {
            let label = target
                .source
                .raw
                .text
                .lines()
                .next()
                .unwrap_or("")
                .chars()
                .take(32)
                .collect::<String>();
            format!("{}@{:.0}+{:.0}", label, target.rect.y, target.rect.height)
        })
        .collect::<Vec<_>>()
        .join(" | ")
}
