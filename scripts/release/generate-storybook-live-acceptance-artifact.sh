#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

# Default artifacts:
# - target/acceptance/kdv-storybook-live-interactive.png
# - target/acceptance/kdv-storybook-live-light-toggle.png
# - target/acceptance/kdv-storybook-live-acceptance-artifacts.sha256
OUT_DIR="${STORYBOOK_LIVE_ACCEPTANCE_DIR:-target/acceptance}"
LIVE_OUT="${STORYBOOK_LIVE_ACCEPTANCE_OUT:-$OUT_DIR/kdv-storybook-live-interactive.png}"
LIGHT_OUT="${STORYBOOK_LIVE_ACCEPTANCE_LIGHT_OUT:-$OUT_DIR/kdv-storybook-live-light-toggle.png}"
MANIFEST_OUT="${STORYBOOK_LIVE_ACCEPTANCE_MANIFEST_OUT:-$OUT_DIR/kdv-storybook-live-acceptance-artifacts.sha256}"
LOG_OUT="${STORYBOOK_LIVE_ACCEPTANCE_LOG:-$OUT_DIR/kdv-storybook-live-acceptance.log}"
CANVAS_WIDTH="${STORYBOOK_LIVE_ACCEPTANCE_WIDTH:-1280}"
CANVAS_HEIGHT="${STORYBOOK_LIVE_ACCEPTANCE_HEIGHT:-932}"
EXPECTED_WIDTH="${STORYBOOK_LIVE_ACCEPTANCE_EXPECTED_WIDTH:-2560}"
EXPECTED_HEIGHT="${STORYBOOK_LIVE_ACCEPTANCE_EXPECTED_HEIGHT:-1864}"
MIN_CHANGED_PIXELS="${STORYBOOK_LIVE_ACCEPTANCE_MIN_CHANGED_PIXELS:-100000}"
MIN_BRIGHT_DELTA="${STORYBOOK_LIVE_ACCEPTANCE_MIN_BRIGHT_DELTA:-100000}"
MIN_DARK_DELTA="${STORYBOOK_LIVE_ACCEPTANCE_MIN_DARK_DELTA:-50000}"
MIN_BRIGHT_CONTENT_PIXELS="${STORYBOOK_LIVE_INTERACTIVE_MIN_BRIGHT_PIXELS:-20000}"
MIN_UNIQUE_COLORS="${STORYBOOK_LIVE_INTERACTIVE_MIN_UNIQUE_COLORS:-128}"

mkdir -p "$OUT_DIR"
rm -f "$LIVE_OUT" "$LIGHT_OUT" "$MANIFEST_OUT" "$LOG_OUT"

target/release/kdv-storybook \
  --live-acceptance-artifact \
  --width "$CANVAS_WIDTH" \
  --height "$CANVAS_HEIGHT" \
  --screenshot-output "$LIVE_OUT" \
  --light-screenshot-output "$LIGHT_OUT" \
  >> "$LOG_OUT" 2>&1

test -s "$LIVE_OUT"
test -s "$LIGHT_OUT"

if ! /usr/bin/grep -q "storybook live acceptance headless artifact ready" "$LOG_OUT"; then
  echo "live acceptance headless artifact marker missing from log" >&2
  cat "$LOG_OUT" >&2 || true
  exit 1
fi

if ! /usr/bin/grep -q "storybook live acceptance clicked dark toggle" "$LOG_OUT"; then
  echo "live acceptance dark toggle action marker missing from log" >&2
  cat "$LOG_OUT" >&2 || true
  exit 1
fi

python3 - \
  "$LIVE_OUT" \
  "$LIGHT_OUT" \
  "$EXPECTED_WIDTH" \
  "$EXPECTED_HEIGHT" \
  "$MIN_CHANGED_PIXELS" \
  "$MIN_BRIGHT_DELTA" \
  "$MIN_DARK_DELTA" \
  "$MIN_BRIGHT_CONTENT_PIXELS" \
  "$MIN_UNIQUE_COLORS" <<'PY' | tee -a "$LOG_OUT"
import pathlib
import struct
import sys
import zlib


def load_png(path: pathlib.Path) -> tuple[int, int, int, list[bytes]]:
    data = path.read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise ValueError(f"{path} is not a PNG")

    pos = 8
    width = height = color_type = None
    idat: list[bytes] = []
    while pos < len(data):
        size = struct.unpack(">I", data[pos : pos + 4])[0]
        kind = data[pos + 4 : pos + 8]
        payload = data[pos + 8 : pos + 8 + size]
        pos += 12 + size
        if kind == b"IHDR":
            width, height, bit_depth, color_type, _, _, interlace = struct.unpack(
                ">IIBBBBB", payload
            )
            if bit_depth != 8 or color_type not in (2, 6) or interlace != 0:
                raise ValueError(f"{path} uses unsupported PNG format")
        elif kind == b"IDAT":
            idat.append(payload)
        elif kind == b"IEND":
            break

    if width is None or height is None or color_type is None:
        raise ValueError(f"{path} is missing IHDR")

    bytes_per_pixel = 4 if color_type == 6 else 3
    stride = width * bytes_per_pixel
    raw = zlib.decompress(b"".join(idat))
    rows: list[bytes] = []
    previous = bytearray(stride)
    index = 0
    for _ in range(height):
        filter_type = raw[index]
        index += 1
        current = bytearray(raw[index : index + stride])
        index += stride
        for i in range(stride):
            left = current[i - bytes_per_pixel] if i >= bytes_per_pixel else 0
            up = previous[i]
            upper_left = previous[i - bytes_per_pixel] if i >= bytes_per_pixel else 0
            if filter_type == 1:
                current[i] = (current[i] + left) & 0xFF
            elif filter_type == 2:
                current[i] = (current[i] + up) & 0xFF
            elif filter_type == 3:
                current[i] = (current[i] + ((left + up) // 2)) & 0xFF
            elif filter_type == 4:
                estimate = left + up - upper_left
                left_distance = abs(estimate - left)
                up_distance = abs(estimate - up)
                upper_left_distance = abs(estimate - upper_left)
                predictor = (
                    left
                    if left_distance <= up_distance and left_distance <= upper_left_distance
                    else up
                    if up_distance <= upper_left_distance
                    else upper_left
                )
                current[i] = (current[i] + predictor) & 0xFF
            elif filter_type != 0:
                raise ValueError(f"{path} uses unsupported PNG filter {filter_type}")
        rows.append(bytes(current))
        previous = current
    return width, height, bytes_per_pixel, rows


def luminance(red: int, green: int, blue: int) -> int:
    return (299 * red + 587 * green + 114 * blue) // 1000


def brightness_counts(rows: list[bytes], bytes_per_pixel: int) -> tuple[int, int]:
    bright = 0
    dark = 0
    for row in rows:
        for index in range(0, len(row), bytes_per_pixel):
            value = luminance(row[index], row[index + 1], row[index + 2])
            bright += value > 230
            dark += value < 40
    return bright, dark


def content_metrics(
    rows: list[bytes], bytes_per_pixel: int, unique_threshold: int
) -> tuple[int, int]:
    bright = 0
    unique: set[tuple[int, int, int]] = set()
    for row in rows:
        for index in range(0, len(row), bytes_per_pixel):
            red = row[index]
            green = row[index + 1]
            blue = row[index + 2]
            bright += luminance(red, green, blue) > 140
            if len(unique) <= unique_threshold:
                unique.add((red, green, blue))
    return bright, len(unique)


def compare(before: pathlib.Path, after: pathlib.Path) -> tuple[int, int, int, int, int, int, int]:
    width, height, bytes_per_pixel, before_rows = load_png(before)
    after_width, after_height, after_bpp, after_rows = load_png(after)
    expected_width = int(sys.argv[3])
    expected_height = int(sys.argv[4])
    if (width, height) != (expected_width, expected_height):
        raise ValueError(
            f"{before} has unexpected size {width}x{height}, "
            f"expected {expected_width}x{expected_height}"
        )
    if (after_width, after_height) != (expected_width, expected_height):
        raise ValueError(
            f"{after} has unexpected size {after_width}x{after_height}, "
            f"expected {expected_width}x{expected_height}"
        )
    if bytes_per_pixel != after_bpp:
        raise ValueError("live acceptance screenshots have different PNG formats")

    changed = 0
    for before_row, after_row in zip(before_rows, after_rows):
        for index in range(0, len(before_row), bytes_per_pixel):
            delta = (
                abs(before_row[index] - after_row[index])
                + abs(before_row[index + 1] - after_row[index + 1])
                + abs(before_row[index + 2] - after_row[index + 2])
            )
            changed += delta > 30

    before_bright, before_dark = brightness_counts(before_rows, bytes_per_pixel)
    after_bright, after_dark = brightness_counts(after_rows, bytes_per_pixel)
    content_bright, unique_colors = content_metrics(
        before_rows, bytes_per_pixel, int(sys.argv[9])
    )
    return (
        changed,
        before_bright,
        after_bright,
        before_dark,
        after_dark,
        content_bright,
        unique_colors,
    )


live_path = pathlib.Path(sys.argv[1])
light_path = pathlib.Path(sys.argv[2])
(
    changed_pixels,
    before_bright,
    after_bright,
    before_dark,
    after_dark,
    content_bright,
    unique_colors,
) = compare(live_path, light_path)
bright_delta = after_bright - before_bright
dark_delta = before_dark - after_dark
min_changed = int(sys.argv[5])
min_bright_delta = int(sys.argv[6])
min_dark_delta = int(sys.argv[7])
min_bright_content = int(sys.argv[8])
min_unique_colors = int(sys.argv[9])

if content_bright < min_bright_content or unique_colors < min_unique_colors:
    raise SystemExit(
        "live acceptance headless interactive screenshot did not render content: "
        f"bright_content_pixels={content_bright} unique_colors={unique_colors}"
    )

print(
    "storybook live acceptance interactive content ready source=headless "
    f"bright_content_pixels={content_bright} unique_colors={unique_colors}"
)

if (
    changed_pixels < min_changed
    or bright_delta < min_bright_delta
    or dark_delta < min_dark_delta
):
    raise SystemExit(
        "live acceptance light screenshot did not switch to light theme: "
        f"changed_pixels={changed_pixels} bright_delta={bright_delta} "
        f"dark_delta={dark_delta}"
    )

print(
    "storybook live acceptance theme switch verified: "
    f"changed_pixels={changed_pixels} bright_delta={bright_delta} dark_delta={dark_delta}"
)
PY

/usr/bin/shasum -a 256 "$LIVE_OUT" "$LIGHT_OUT" "$LOG_OUT" > "$MANIFEST_OUT"
test -s "$MANIFEST_OUT"

printf 'storybook live acceptance screenshot: %s\n' "$LIVE_OUT"
printf 'storybook live light-toggle screenshot: %s\n' "$LIGHT_OUT"
printf 'storybook live acceptance log: %s\n' "$LOG_OUT"
printf 'storybook live acceptance manifest: %s\n' "$MANIFEST_OUT"
