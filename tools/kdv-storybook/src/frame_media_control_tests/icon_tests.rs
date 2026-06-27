use super::support::{MediaControlFrameSupport, frame_rect_for_hit};
use crate::KucDiagramControlResolver;
use crate::canvas::Canvas;
use crate::document_viewer::media_control_icons::KucMediaControlIconSet;
use crate::layout::preview_content_width;
use katana_ui_core_storybook::{UiTreeRenderArea, UiTreeSurfaceHost};
use std::path::Path;

const ICON_SCAN_SIZE: usize = 20;
const MIN_ICON_PIXELS: usize = 14;
const MAX_FILLED_RATIO_PERCENT: usize = 58;
const MIN_EDGE_PIXELS: usize = 8;

#[test]
fn diagram_control_icons_render_as_katana_glyphs_not_blocky_squares()
-> Result<(), Box<dyn std::error::Error>> {
    let scene = MediaControlFrameSupport::build_scene(true)?;
    let canvas = MediaControlFrameSupport::render_scene(&scene, true);

    let fullscreen = MediaControlFrameSupport::diagram_action_hits(&scene)?
        .into_iter()
        .find(|hit| hit.command == "fullscreen")
        .ok_or_else(|| std::io::Error::other("fullscreen control missing"))?;
    assert_icon_glyph_shape(&canvas, "fullscreen", frame_rect_for_hit(&fullscreen.hit));

    let internal_hits = internal_diagram_control_hits(&scene);
    assert_eq!(
        [
            "pan-up",
            "zoom-in",
            "pan-left",
            "reset-view",
            "pan-right",
            "trackpad-help",
            "pan-down",
            "zoom-out"
        ],
        internal_hits
            .iter()
            .map(|hit| hit.command.as_str())
            .collect::<Vec<_>>()
            .as_slice(),
        "diagram internal controls must keep the full KatanA control set"
    );
    for hit in internal_hits {
        assert_icon_glyph_shape(&canvas, hit.command.as_str(), hit.rect);
    }

    Ok(())
}

#[test]
fn kuc_default_diagram_control_icons_match_katana_asset_files()
-> Result<(), Box<dyn std::error::Error>> {
    let icons = KucMediaControlIconSet::katana_default();
    let katana_icon_root =
        Path::new("/Users/hiroyuki_furuno/works/private/katana/assets/icons/katana");

    for (command, relative_path, path_summary) in [
        ("copy", "ui/copy.svg", "katana.ui.copy"),
        ("copy-code", "ui/copy.svg", "katana.ui.copy"),
        ("copy-source", "ui/copy.svg", "katana.ui.copy"),
        ("close-modal", "ui/close_modal.svg", "katana.ui.close_modal"),
        ("fit", "view/fullscreen.svg", "katana.view.fullscreen"),
        (
            "fullscreen",
            "view/fullscreen.svg",
            "katana.view.fullscreen",
        ),
        (
            "open",
            "system/external_link.svg",
            "katana.system.external_link",
        ),
        ("pan-down", "view/pan_down.svg", "katana.view.pan_down"),
        ("pan-left", "view/pan_left.svg", "katana.view.pan_left"),
        ("pan-right", "view/pan_right.svg", "katana.view.pan_right"),
        ("pan-up", "view/pan_up.svg", "katana.view.pan_up"),
        (
            "reset-view",
            "view/reset_view.svg",
            "katana.view.reset_view",
        ),
        (
            "reveal-in-os",
            "system/external_link.svg",
            "katana.system.external_link",
        ),
        ("trackpad-help", "status/info.svg", "katana.status.info"),
        ("zoom-in", "view/zoom_in.svg", "katana.view.zoom_in"),
        ("zoom-out", "view/zoom_out.svg", "katana.view.zoom_out"),
    ] {
        let asset = std::fs::read_to_string(katana_icon_root.join(relative_path))?;
        let icon = icons.icon_for(command, "");
        assert_eq!(
            normalize_svg(&asset),
            normalize_svg(&icon.svg_source),
            "{command} must default to KatanA asset {relative_path}"
        );
        assert_eq!(path_summary, icon.path_summary, "{command} summary");
    }

    Ok(())
}

fn assert_icon_glyph_shape(
    canvas: &Canvas,
    command: &str,
    (left, top, right, bottom): (usize, usize, usize, usize),
) {
    let scan = centered_scan_rect(left, top, right, bottom);
    let metrics = IconGlyphMetrics::from_canvas(canvas, scan);
    let scan_area = scan.width() * scan.height();
    let max_filled = scan_area * MAX_FILLED_RATIO_PERCENT / 100;

    assert!(
        metrics.icon_pixels >= MIN_ICON_PIXELS,
        "diagram control icon must be visible: command={command} metrics={metrics:?} scan={scan:?}"
    );
    assert!(
        metrics.icon_pixels <= max_filled,
        "diagram control icon must stay a glyph, not a filled square block: command={command} metrics={metrics:?} scan={scan:?} max_filled={max_filled}"
    );
    assert!(
        metrics.edge_pixels >= MIN_EDGE_PIXELS,
        "diagram control icon must keep SVG edge detail: command={command} metrics={metrics:?} scan={scan:?}"
    );
}

fn centered_scan_rect(left: usize, top: usize, right: usize, bottom: usize) -> ScanRect {
    let width = right.saturating_sub(left);
    let height = bottom.saturating_sub(top);
    let scan_width = width.min(ICON_SCAN_SIZE);
    let scan_height = height.min(ICON_SCAN_SIZE);
    ScanRect {
        left: left + width.saturating_sub(scan_width) / 2,
        top: top + height.saturating_sub(scan_height) / 2,
        right: left + width.saturating_sub(scan_width) / 2 + scan_width,
        bottom: top + height.saturating_sub(scan_height) / 2 + scan_height,
    }
}

#[derive(Debug, Clone, Copy)]
struct ScanRect {
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

impl ScanRect {
    fn width(self) -> usize {
        self.right.saturating_sub(self.left)
    }

    fn height(self) -> usize {
        self.bottom.saturating_sub(self.top)
    }
}

#[derive(Debug)]
struct IconGlyphMetrics {
    icon_pixels: usize,
    edge_pixels: usize,
}

impl IconGlyphMetrics {
    fn from_canvas(canvas: &Canvas, rect: ScanRect) -> Self {
        let mut icon_pixels = 0usize;
        let mut edge_pixels = 0usize;
        for y in rect.top..rect.bottom.min(canvas.height()) {
            for x in rect.left..rect.right.min(canvas.width()) {
                let brightness = brightness(canvas.pixels()[y * canvas.width() + x]);
                if !is_icon_pixel(brightness) {
                    continue;
                }
                icon_pixels += 1;
                let horizontal_edge = x + 1 < rect.right.min(canvas.width())
                    && brightness_delta(brightness, canvas.pixels()[y * canvas.width() + x + 1])
                        >= 18;
                let vertical_edge = y + 1 < rect.bottom.min(canvas.height())
                    && brightness_delta(brightness, canvas.pixels()[(y + 1) * canvas.width() + x])
                        >= 18;
                edge_pixels += usize::from(horizontal_edge || vertical_edge);
            }
        }
        Self {
            icon_pixels,
            edge_pixels,
        }
    }
}

fn is_icon_pixel(brightness: u8) -> bool {
    brightness >= 150
}

fn brightness_delta(left: u8, right: u32) -> u8 {
    left.abs_diff(brightness(right))
}

fn brightness(pixel: u32) -> u8 {
    let red = ((pixel >> 16) & 0xff) as u16;
    let green = ((pixel >> 8) & 0xff) as u16;
    let blue = (pixel & 0xff) as u16;
    ((red + green + blue) / 3) as u8
}

fn normalize_svg(value: &str) -> String {
    value
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace("> <", "><")
}

fn internal_diagram_control_hits(scene: &crate::preview::PreviewScene) -> Vec<InternalControlHit> {
    UiTreeSurfaceHost::new(scene.theme.clone())
        .document_node_hits(
            scene.tree.root(),
            UiTreeRenderArea {
                x: 0,
                y: 0,
                width: preview_content_width(super::support::FRAME_WIDTH),
                height: scene.content_height.ceil().max(1.0) as usize,
                scroll_y: 0.0,
            },
        )
        .into_iter()
        .filter_map(|hit| {
            let action = KucDiagramControlResolver::internal_action_for_node(
                scene.tree.root(),
                &hit.node_id,
            )?;
            Some(InternalControlHit {
                command: action.command,
                rect: (
                    super::support::FRAME_PREVIEW_LEFT + hit.rect.x,
                    super::support::FRAME_PREVIEW_TOP + hit.rect.y,
                    super::support::FRAME_PREVIEW_LEFT + hit.rect.x + hit.rect.width,
                    super::support::FRAME_PREVIEW_TOP + hit.rect.y + hit.rect.height,
                ),
            })
        })
        .collect()
}

struct InternalControlHit {
    command: String,
    rect: (usize, usize, usize, usize),
}
