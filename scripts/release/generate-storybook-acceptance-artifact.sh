#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

if [[ -n "${MAGICK:-}" ]]; then
  MAGICK_BIN="$MAGICK"
elif command -v magick >/dev/null 2>&1; then
  MAGICK_BIN="$(command -v magick)"
elif [[ -x /opt/homebrew/bin/magick ]]; then
  MAGICK_BIN="/opt/homebrew/bin/magick"
else
  echo "magick is required to generate the Storybook acceptance artifact" >&2
  exit 1
fi

FONT="${STORYBOOK_ACCEPTANCE_FONT:-/System/Library/Fonts/Helvetica.ttc}"
OUT="${STORYBOOK_ACCEPTANCE_OUT:-target/acceptance/kdv-storybook-acceptance-contact-sheet.png}"
TEXT_OUT="${STORYBOOK_TEXT_REGRESSION_OUT:-target/acceptance/kdv-storybook-text-regression-crops.png}"
REFERENCE_COMPARISON_OUT="${STORYBOOK_REFERENCE_COMPARISON_OUT:-target/acceptance/kdv-storybook-katana-reference-comparison.png}"
MANIFEST_OUT="${STORYBOOK_ACCEPTANCE_MANIFEST_OUT:-target/acceptance/kdv-storybook-acceptance-artifacts.sha256}"

require_png_size() {
  local path="$1"
  local expected_width="$2"
  local expected_height="$3"
  local actual

  actual="$("$MAGICK_BIN" identify -format '%w %h' "$path")"
  if [[ "$actual" != "$expected_width $expected_height" ]]; then
    echo "unexpected crop size for $path: got $actual, expected $expected_width $expected_height" >&2
    exit 1
  fi
}

require_min_unique_colors() {
  local path="$1"
  local minimum="$2"
  local actual

  actual="$("$MAGICK_BIN" identify -format '%k' "$path")"
  if (( actual < minimum )); then
    echo "crop has too few unique colors for $path: got $actual, expected at least $minimum" >&2
    exit 1
  fi
}

require_min_bright_pixels() {
  local path="$1"
  local minimum="$2"
  local actual

  actual="$("$MAGICK_BIN" "$path" -alpha off -colorspace Gray -threshold 35% -format '%[fx:mean*w*h]' info:)"
  if ! awk -v actual="$actual" -v minimum="$minimum" 'BEGIN { exit !(actual >= minimum) }'; then
    echo "crop has too few bright text pixels for $path: got $actual, expected at least $minimum" >&2
    exit 1
  fi
}

require_min_blue_pixels() {
  local path="$1"
  local minimum="$2"
  local actual

  actual="$("$MAGICK_BIN" "$path" -alpha off \
    -fx '(b > r + 0.08 && b > g + 0.02 && b > 0.35) ? 1 : 0' \
    -format '%[fx:mean*w*h]' info:)"
  if ! awk -v actual="$actual" -v minimum="$minimum" 'BEGIN { exit !(actual >= minimum) }'; then
    echo "crop has too few link-blue pixels for $path: got $actual, expected at least $minimum" >&2
    exit 1
  fi
}

require_min_changed_pixels() {
  local before="$1"
  local after="$2"
  local minimum="$3"
  local actual

  actual="$("$MAGICK_BIN" compare -metric AE "$before" "$after" null: 2>&1 || true)"
  actual="${actual%% *}"
  if ! awk -v actual="$actual" -v minimum="$minimum" 'BEGIN { exit !(actual >= minimum) }'; then
    echo "crop pair did not change enough: before=$before after=$after got $actual, expected at least $minimum" >&2
    exit 1
  fi
}

require_min_edge_pixels() {
  local path="$1"
  local minimum="$2"
  local actual

  actual="$("$MAGICK_BIN" "$path" -alpha off -edge 1 -colorspace Gray -threshold 10% -format '%[fx:mean*w*h]' info:)"
  if ! awk -v actual="$actual" -v minimum="$minimum" 'BEGIN { exit !(actual >= minimum) }'; then
    echo "crop has too few edge pixels for $path: got $actual, expected at least $minimum" >&2
    exit 1
  fi
}

required=(
  target/kdv-storybook-window-hover-smoke.png
  target/kdv-storybook-window-sidebar-smoke.png
  target/kdv-storybook-window-sidebar-smoke-file-hover.png
  target/kdv-storybook-window-sidebar-smoke-file-click.png
  target/kdv-storybook-window-sidebar-smoke-settings-hover.png
  target/kdv-storybook-window-sidebar-smoke-settings-click.png
  target/kdv-storybook-window-sidebar-narrow-smoke.png
  target/kdv-storybook-window-sidebar-narrow-smoke-file-hover.png
  target/kdv-storybook-window-sidebar-narrow-smoke-file-click.png
  target/kdv-storybook-window-sidebar-narrow-smoke-settings-hover.png
  target/kdv-storybook-window-sidebar-narrow-smoke-settings-click.png
  target/kdv-storybook-window-sidebar-large-smoke.png
  target/kdv-storybook-window-sidebar-large-smoke-file-hover.png
  target/kdv-storybook-window-sidebar-large-smoke-file-click.png
  target/kdv-storybook-window-sidebar-large-smoke-settings-hover.png
  target/kdv-storybook-window-sidebar-large-smoke-settings-click.png
  target/kdv-storybook-window-diagram-smoke.png
  target/kdv-storybook-window-html-margin-smoke.png
  target/kdv-storybook-window-hover-wide-smoke.png
  target/kdv-storybook-window-drawio-diagram-smoke.png
  target/kdv-storybook-window-drawio-diagram-smoke-hover-zoom-in.png
  target/kdv-storybook-window-drawio-diagram-smoke-zoom-in.png
  target/kdv-storybook-window-drawio-diagram-smoke-hover-pan-right.png
  target/kdv-storybook-window-drawio-diagram-smoke-pan-right.png
  target/kdv-storybook-window-drawio-diagram-smoke-hover-pan-down.png
  target/kdv-storybook-window-drawio-diagram-smoke-pan-down.png
  target/kdv-storybook-window-drawio-diagram-smoke-hover-reset-view.png
  target/kdv-storybook-window-drawio-diagram-smoke-reset-view.png
  target/kdv-storybook-window-drawio-diagram-smoke-hover-pan-left.png
  target/kdv-storybook-window-drawio-diagram-smoke-pan-left.png
  target/kdv-storybook-window-drawio-diagram-smoke-hover-pan-up.png
  target/kdv-storybook-window-drawio-diagram-smoke-pan-up.png
  target/kdv-storybook-window-drawio-diagram-smoke-hover-zoom-out.png
  target/kdv-storybook-window-drawio-diagram-smoke-zoom-out.png
  target/kdv-storybook-window-drawio-diagram-smoke-hover-trackpad-help.png
  target/kdv-storybook-window-drawio-diagram-smoke-trackpad-help.png
  target/kdv-storybook-window-drawio-diagram-smoke-hover-fullscreen.png
  target/kdv-storybook-window-drawio-diagram-smoke-fullscreen.png
  target/kdv-storybook-window-footnote-smoke.png
  target/kdv-storybook-window-footnote-smoke-reference.png
  target/kdv-storybook-window-footnote-smoke-definition.png
  target/kdv-storybook-window-table-smoke.png
  target/kdv-storybook-window-code-copy-smoke.png
  target/kdv-storybook-window-code-copy-smoke-hover.png
  target/kdv-storybook-window-code-copy-smoke-copied.png
  target/kdv-storybook-window-selection-smoke.png
  target/kdv-storybook-window-slideshow-smoke.png
  target/kdv-storybook-window-slideshow-smoke-mode.png
  target/kdv-storybook-window-slideshow-smoke-next.png
  target/kdv-storybook-window-slideshow-smoke-previous.png
  target/kdv-storybook-window-slideshow-smoke-close.png
  target/kdv-storybook-window-diagram-smoke-hover-zoom-in.png
  target/kdv-storybook-window-diagram-smoke-zoom-in.png
  target/kdv-storybook-window-diagram-smoke-hover-pan-right.png
  target/kdv-storybook-window-diagram-smoke-pan-right.png
  target/kdv-storybook-window-diagram-smoke-hover-pan-down.png
  target/kdv-storybook-window-diagram-smoke-pan-down.png
  target/kdv-storybook-window-diagram-smoke-hover-reset-view.png
  target/kdv-storybook-window-diagram-smoke-reset-view.png
  target/kdv-storybook-window-diagram-smoke-hover-pan-left.png
  target/kdv-storybook-window-diagram-smoke-pan-left.png
  target/kdv-storybook-window-diagram-smoke-hover-pan-up.png
  target/kdv-storybook-window-diagram-smoke-pan-up.png
  target/kdv-storybook-window-diagram-smoke-hover-zoom-out.png
  target/kdv-storybook-window-diagram-smoke-zoom-out.png
  target/kdv-storybook-window-diagram-smoke-hover-trackpad-help.png
  target/kdv-storybook-window-diagram-smoke-trackpad-help.png
  target/kdv-storybook-window-diagram-smoke-hover-fullscreen.png
  target/kdv-storybook-window-diagram-smoke-fullscreen.png
  target/acceptance/kdv-storybook-scroll-performance.txt
)

score_required=(
  target/acceptance/preview-crop-reference/katana_sample_md-preview-crop_reference.ppm
  target/acceptance/preview-crop-reference/katana_sample_md-preview-crop_preview.ppm
  target/acceptance/preview-crop-reference/katana_sample_diagrams_md-preview-crop_reference.ppm
  target/acceptance/preview-crop-reference/katana_sample_diagrams_md-preview-crop_preview.ppm
)

missing=()
for path in "${required[@]}"; do
  if [[ ! -s "$path" ]]; then
    missing+=("$path")
  fi
done
for path in "${score_required[@]}"; do
  if [[ ! -s "$path" ]]; then
    missing+=("$path")
  fi
done

if (( ${#missing[@]} > 0 )); then
  echo "missing Storybook screenshot artifact(s):" >&2
  printf '  %s\n' "${missing[@]}" >&2
  echo "run: /opt/homebrew/bin/rtk just storybook-acceptance-artifact" >&2
  exit 1
fi
grep -q '^scenario=large_loaded_diagram_wheel_present$' target/acceptance/kdv-storybook-scroll-performance.txt
grep -q '^full_preview_redraw_fallback_count=0$' target/acceptance/kdv-storybook-scroll-performance.txt

mkdir -p "$(dirname "$OUT")"
mkdir -p "$(dirname "$TEXT_OUT")"
mkdir -p "$(dirname "$REFERENCE_COMPARISON_OUT")"
mkdir -p "$(dirname "$MANIFEST_OUT")"

for stale_sheet in "$(dirname "$OUT")"/kdv-storybook-acceptance-contact-sheet-*.png; do
  if [[ -e "$stale_sheet" ]]; then
    /bin/rm -f -- "$stale_sheet"
  fi
done

"$MAGICK_BIN" montage -font "$FONT" \
  -label 'viewer text / hover / status' target/kdv-storybook-window-hover-smoke.png \
  -label 'sidebar hit targets' target/kdv-storybook-window-sidebar-smoke.png \
  -label 'sidebar narrow layout' target/kdv-storybook-window-sidebar-narrow-smoke.png \
  -label 'sidebar large layout' target/kdv-storybook-window-sidebar-large-smoke.png \
  -label 'mermaid controls' target/kdv-storybook-window-diagram-smoke.png \
  -label 'drawio controls' target/kdv-storybook-window-drawio-diagram-smoke.png \
  -label 'wide text / link / padding' target/kdv-storybook-window-hover-wide-smoke.png \
  -label 'footnote jump' target/kdv-storybook-window-footnote-smoke.png \
  -label 'tables / cell layout' target/kdv-storybook-window-table-smoke.png \
  -label 'code copy copied state' target/kdv-storybook-window-code-copy-smoke-copied.png \
  -label 'selection / copy' target/kdv-storybook-window-selection-smoke.png \
  -label 'slideshow base' target/kdv-storybook-window-slideshow-smoke.png \
  -label 'slideshow mode' target/kdv-storybook-window-slideshow-smoke-mode.png \
  -label 'slideshow next' target/kdv-storybook-window-slideshow-smoke-next.png \
  -label 'slideshow previous' target/kdv-storybook-window-slideshow-smoke-previous.png \
  -label 'slideshow close' target/kdv-storybook-window-slideshow-smoke-close.png \
  -geometry 480x320+16+34 \
  -tile 4x4 \
  -background '#202020' \
  -fill '#f2f2f2' \
  -pointsize 18 \
  "$OUT"

test -s "$OUT"
crop_dir="target/acceptance/text-regression-crops"
review_dir="$crop_dir/review"
comparison_dir="$crop_dir/reference-comparison"
mkdir -p "$crop_dir"
mkdir -p "$review_dir"
mkdir -p "$comparison_dir"

"$MAGICK_BIN" target/kdv-storybook-window-hover-smoke.png -crop 740x180+500+45 +repage "$crop_dir/title-body.png"
"$MAGICK_BIN" target/kdv-storybook-window-hover-smoke.png -crop 640x100+590+235 +repage "$crop_dir/language-link.png"
"$MAGICK_BIN" target/kdv-storybook-window-hover-smoke.png -crop 720x390+500+275 +repage "$crop_dir/html-margin-center.png"
"$MAGICK_BIN" target/kdv-storybook-window-html-margin-smoke.png -crop 740x260+500+75 +repage "$crop_dir/direct-html-margin-left.png"
"$MAGICK_BIN" target/kdv-storybook-window-hover-smoke-hover.png -crop 740x180+500+45 +repage "$crop_dir/hover-highlight.png"
"$MAGICK_BIN" target/kdv-storybook-window-hover-wide-smoke.png -crop 1480x460+500+80 +repage "$crop_dir/wide-title-link-html.png"
"$MAGICK_BIN" target/kdv-storybook-window-diagram-smoke-hover-reset-view.png -crop 110x150+1160+145 +repage "$crop_dir/diagram-control-icons.png"
"$MAGICK_BIN" target/kdv-storybook-window-table-smoke.png -crop 740x780+526+80 +repage "$crop_dir/table-section.png"

require_png_size "$crop_dir/title-body.png" 740 180
require_png_size "$crop_dir/language-link.png" 640 100
require_png_size "$crop_dir/html-margin-center.png" 720 390
require_png_size "$crop_dir/direct-html-margin-left.png" 740 260
require_png_size "$crop_dir/hover-highlight.png" 740 180
require_png_size "$crop_dir/wide-title-link-html.png" 1480 460
require_png_size "$crop_dir/diagram-control-icons.png" 110 150
require_png_size "$crop_dir/table-section.png" 740 780

require_min_unique_colors "$crop_dir/title-body.png" 300
require_min_unique_colors "$crop_dir/language-link.png" 80
require_min_unique_colors "$crop_dir/html-margin-center.png" 80
require_min_unique_colors "$crop_dir/direct-html-margin-left.png" 150
require_min_unique_colors "$crop_dir/hover-highlight.png" 300
require_min_unique_colors "$crop_dir/wide-title-link-html.png" 300
require_min_unique_colors "$crop_dir/diagram-control-icons.png" 30
require_min_unique_colors "$crop_dir/table-section.png" 300

require_min_bright_pixels "$crop_dir/title-body.png" 2500
require_min_bright_pixels "$crop_dir/language-link.png" 300
require_min_bright_pixels "$crop_dir/html-margin-center.png" 5000
require_min_bright_pixels "$crop_dir/direct-html-margin-left.png" 1500
require_min_bright_pixels "$crop_dir/hover-highlight.png" 2500
require_min_bright_pixels "$crop_dir/wide-title-link-html.png" 7500
require_min_bright_pixels "$crop_dir/diagram-control-icons.png" 300
require_min_bright_pixels "$crop_dir/table-section.png" 5000

require_min_blue_pixels "$crop_dir/language-link.png" 100
require_min_blue_pixels "$crop_dir/wide-title-link-html.png" 100
require_min_changed_pixels "$crop_dir/title-body.png" "$crop_dir/hover-highlight.png" 250
require_min_edge_pixels "$crop_dir/diagram-control-icons.png" 300
require_min_edge_pixels "$crop_dir/table-section.png" 10000

"$MAGICK_BIN" "$crop_dir/title-body.png" -background '#202020' -gravity center -extent 740x180 "$review_dir/title-body-review.png"
"$MAGICK_BIN" "$crop_dir/language-link.png" -background '#202020' -gravity center -extent 740x100 "$review_dir/language-link-review.png"
"$MAGICK_BIN" "$crop_dir/html-margin-center.png" -background '#202020' -gravity center -extent 740x390 "$review_dir/html-margin-center-review.png"
"$MAGICK_BIN" "$crop_dir/direct-html-margin-left.png" -background '#202020' -gravity center -extent 740x260 "$review_dir/direct-html-margin-left-review.png"
"$MAGICK_BIN" "$crop_dir/hover-highlight.png" -background '#202020' -gravity center -extent 740x180 "$review_dir/hover-highlight-review.png"
"$MAGICK_BIN" "$crop_dir/wide-title-link-html.png" -resize 740x230 -background '#202020' -gravity center -extent 740x230 "$review_dir/wide-title-link-html-review.png"
"$MAGICK_BIN" "$crop_dir/diagram-control-icons.png" -background '#202020' -gravity center -extent 740x180 "$review_dir/diagram-control-icons-review.png"
"$MAGICK_BIN" "$crop_dir/table-section.png" -background '#202020' -gravity center -extent 740x780 "$review_dir/table-section-review.png"

"$MAGICK_BIN" target/acceptance/preview-crop-reference/katana_sample_md-preview-crop_reference.ppm -crop 1280x520+0+0 +repage -resize 740x300 -background '#202020' -gravity center -extent 740x320 "$comparison_dir/sample-top-reference.png"
"$MAGICK_BIN" target/acceptance/preview-crop-reference/katana_sample_md-preview-crop_preview.ppm -crop 1280x520+0+0 +repage -resize 740x300 -background '#202020' -gravity center -extent 740x320 "$comparison_dir/sample-top-candidate.png"
"$MAGICK_BIN" target/acceptance/preview-crop-reference/katana_sample_diagrams_md-preview-crop_reference.ppm -crop 1280x920+0+250 +repage -resize 740x532 -background '#202020' -gravity center -extent 740x560 "$comparison_dir/sample-diagrams-reference.png"
"$MAGICK_BIN" target/acceptance/preview-crop-reference/katana_sample_diagrams_md-preview-crop_preview.ppm -crop 1280x920+0+250 +repage -resize 740x532 -background '#202020' -gravity center -extent 740x560 "$comparison_dir/sample-diagrams-candidate.png"
"$MAGICK_BIN" "$comparison_dir/sample-top-reference.png" "$comparison_dir/sample-top-candidate.png" -compose difference -composite -auto-level -background '#202020' -gravity center -extent 740x320 "$comparison_dir/sample-top-diff.png"
"$MAGICK_BIN" "$comparison_dir/sample-diagrams-reference.png" "$comparison_dir/sample-diagrams-candidate.png" -compose difference -composite -auto-level -background '#202020' -gravity center -extent 740x560 "$comparison_dir/sample-diagrams-diff.png"

require_png_size "$review_dir/title-body-review.png" 740 180
require_png_size "$review_dir/language-link-review.png" 740 100
require_png_size "$review_dir/html-margin-center-review.png" 740 390
require_png_size "$review_dir/direct-html-margin-left-review.png" 740 260
require_png_size "$review_dir/hover-highlight-review.png" 740 180
require_png_size "$review_dir/wide-title-link-html-review.png" 740 230
require_png_size "$review_dir/diagram-control-icons-review.png" 740 180
require_png_size "$review_dir/table-section-review.png" 740 780
require_png_size "$comparison_dir/sample-top-reference.png" 740 320
require_png_size "$comparison_dir/sample-top-candidate.png" 740 320
require_png_size "$comparison_dir/sample-top-diff.png" 740 320
require_png_size "$comparison_dir/sample-diagrams-reference.png" 740 560
require_png_size "$comparison_dir/sample-diagrams-candidate.png" 740 560
require_png_size "$comparison_dir/sample-diagrams-diff.png" 740 560

require_min_unique_colors "$comparison_dir/sample-top-reference.png" 300
require_min_unique_colors "$comparison_dir/sample-top-candidate.png" 300
require_min_unique_colors "$comparison_dir/sample-top-diff.png" 60
require_min_unique_colors "$comparison_dir/sample-diagrams-reference.png" 150
require_min_unique_colors "$comparison_dir/sample-diagrams-candidate.png" 150
require_min_unique_colors "$comparison_dir/sample-diagrams-diff.png" 60
require_min_blue_pixels "$comparison_dir/sample-top-reference.png" 100
require_min_blue_pixels "$comparison_dir/sample-top-candidate.png" 100
require_min_edge_pixels "$comparison_dir/sample-diagrams-reference.png" 1000
require_min_edge_pixels "$comparison_dir/sample-diagrams-candidate.png" 1000

"$MAGICK_BIN" montage -font "$FONT" \
  -label 'title and body text crop' "$review_dir/title-body-review.png" \
  -label 'language link underline crop' "$review_dir/language-link-review.png" \
  -label 'html centering and margin-left crop' "$review_dir/html-margin-center-review.png" \
  -label 'direct html margin-left fixture crop' "$review_dir/direct-html-margin-left-review.png" \
  -label 'hover highlight crop' "$review_dir/hover-highlight-review.png" \
  -label 'wide title / link / html crop' "$review_dir/wide-title-link-html-review.png" \
  -label 'diagram control icon crop' "$review_dir/diagram-control-icons-review.png" \
  -label 'table section crop' "$review_dir/table-section-review.png" \
  -geometry +16+42 \
  -tile 1x8 \
  -background '#202020' \
  -fill '#f2f2f2' \
  -pointsize 18 \
  "$TEXT_OUT"

test -s "$TEXT_OUT"

"$MAGICK_BIN" montage -font "$FONT" \
  -label 'KatanA text / link reference' "$comparison_dir/sample-top-reference.png" \
  -label 'KDV text / link candidate' "$comparison_dir/sample-top-candidate.png" \
  -label 'KatanA/KDV text diff heatmap' "$comparison_dir/sample-top-diff.png" \
  -label 'KatanA SVG / diagram reference' "$comparison_dir/sample-diagrams-reference.png" \
  -label 'KDV SVG / diagram candidate' "$comparison_dir/sample-diagrams-candidate.png" \
  -label 'KatanA/KDV SVG diff heatmap' "$comparison_dir/sample-diagrams-diff.png" \
  -geometry +16+42 \
  -tile 3x2 \
  -background '#202020' \
  -fill '#f2f2f2' \
  -pointsize 18 \
  "$REFERENCE_COMPARISON_OUT"
test -s "$REFERENCE_COMPARISON_OUT"

/usr/bin/shasum -a 256 \
  "$OUT" \
  "$TEXT_OUT" \
  "$REFERENCE_COMPARISON_OUT" \
  "$crop_dir/title-body.png" \
  "$crop_dir/language-link.png" \
  "$crop_dir/html-margin-center.png" \
  "$crop_dir/direct-html-margin-left.png" \
  "$crop_dir/hover-highlight.png" \
  "$crop_dir/wide-title-link-html.png" \
  "$crop_dir/diagram-control-icons.png" \
  "$crop_dir/table-section.png" \
  "$comparison_dir/sample-top-reference.png" \
  "$comparison_dir/sample-top-candidate.png" \
  "$comparison_dir/sample-top-diff.png" \
  "$comparison_dir/sample-diagrams-reference.png" \
  "$comparison_dir/sample-diagrams-candidate.png" \
  "$comparison_dir/sample-diagrams-diff.png" \
  "${score_required[@]}" \
  "${required[@]}" \
  > "$MANIFEST_OUT"
test -s "$MANIFEST_OUT"
printf 'storybook acceptance contact sheet: %s\n' "$OUT"
printf 'storybook text regression crops: %s\n' "$TEXT_OUT"
printf 'storybook KatanA reference comparison: %s\n' "$REFERENCE_COMPARISON_OUT"
printf 'storybook acceptance artifact manifest: %s\n' "$MANIFEST_OUT"
