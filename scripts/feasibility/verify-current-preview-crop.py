#!/usr/bin/env python3
"""Fail-closed provenance and geometry checks for an external preview crop."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import re
import struct
import sys
import zlib
from pathlib import Path
from typing import Any


PNG_SIGNATURE = b"\x89PNG\r\n\x1a\n"
CLEAN_ZERO_FIELDS = (
    "active_markdown_ranges",
    "code_copy_control_renders",
    "code_selection_renders",
    "diagram_control_renders",
    "hovered_markdown_spans",
    "hovered_preview_line_count",
    "image_control_renders",
    "image_hover_background_renders",
    "local_image_hover_background_renders",
)
INTERACTION_FIELDS = (
    "pointer_inside_content",
    "active_editor_line",
    *CLEAN_ZERO_FIELDS,
)
SEMVER = re.compile(
    r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)"
    r"(?:-(?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*)"
    r"(?:\.(?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*))*)?"
    r"(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?"
)


class VerificationError(ValueError):
    """An input does not satisfy its immutable declaration."""


def require_object(value: Any, name: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise VerificationError(f"{name} must be an object")
    return value


def require_int(value: Any, name: str, *, minimum: int | None = None) -> int:
    if isinstance(value, bool) or not isinstance(value, int):
        raise VerificationError(f"{name} must be an integer")
    if minimum is not None and value < minimum:
        raise VerificationError(f"{name} must be >= {minimum}")
    return value


def require_string(value: Any, name: str) -> str:
    if not isinstance(value, str):
        raise VerificationError(f"{name} must be a string")
    return value


def require_number(value: Any, name: str) -> int | float:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise VerificationError(f"{name} must be a finite number")
    if isinstance(value, float) and not math.isfinite(value):
        raise VerificationError(f"{name} must be a finite number")
    return value


def unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise VerificationError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def reject_constant(value: str) -> Any:
    raise VerificationError(f"invalid JSON numeric constant: {value}")


def finite_float(value: str) -> float:
    number = float(value)
    if not math.isfinite(number):
        raise VerificationError("non-finite JSON number")
    return number


def read_json(path: Path, name: str) -> dict[str, Any]:
    try:
        # 重複キーの上書きや非有限値を許すと、外部宣言の意味が変わってしまう。
        value = json.loads(
            path.read_text(encoding="utf-8"),
            object_pairs_hook=unique_object,
            parse_constant=reject_constant,
            parse_float=finite_float,
        )
    except (OSError, UnicodeError, ValueError) as error:
        raise VerificationError(f"cannot read {name}: {error}") from error
    return require_object(value, name)


def sha256_file(path: Path, name: str) -> str:
    digest = hashlib.sha256()
    try:
        with path.open("rb") as stream:
            for chunk in iter(lambda: stream.read(1024 * 1024), b""):
                digest.update(chunk)
    except OSError as error:
        raise VerificationError(f"cannot read {name}: {error}") from error
    return digest.hexdigest()


def png_dimensions(path: Path, name: str) -> tuple[int, int]:
    # ここでは先頭チャンクだけを検査し、画像全体のデコードはRust scorerに委ねる。
    try:
        with path.open("rb") as stream:
            header = stream.read(33)
    except OSError as error:
        raise VerificationError(f"cannot read {name}: {error}") from error
    if len(header) != 33 or header[:8] != PNG_SIGNATURE:
        raise VerificationError(f"{name} is not a PNG with an IHDR chunk")
    length, chunk_type = struct.unpack(">I4s", header[8:16])
    if length != 13 or chunk_type != b"IHDR":
        raise VerificationError(f"{name} has an invalid IHDR chunk")
    expected_crc = struct.unpack(">I", header[29:33])[0]
    actual_crc = zlib.crc32(header[12:29]) & 0xFFFFFFFF
    if actual_crc != expected_crc:
        raise VerificationError(f"{name} has an invalid IHDR CRC")
    width, height = struct.unpack(">II", header[16:24])
    if width == 0 or height == 0:
        raise VerificationError(f"{name} has invalid dimensions")
    return width, height


def validate_manifest(manifest: dict[str, Any]) -> tuple[dict[str, str], dict[str, tuple[int, int]], int, dict[str, Any]]:
    if require_int(manifest.get("schema_version"), "manifest.schema_version") != 1:
        raise VerificationError("manifest.schema_version must be 1")
    if manifest.get("producer_repo") != "https://github.com/HiroyukiFuruno/KatanA":
        raise VerificationError("manifest.producer_repo is not the declared KatanA repository")
    commit = require_string(manifest.get("producer_commit"), "manifest.producer_commit")
    if re.fullmatch(r"[0-9a-fA-F]{40}", commit) is None:
        raise VerificationError("manifest.producer_commit must be a full 40-character hash")
    if manifest.get("fixture") not in ("katana/sample_diagrams.md", "katana/sample.md"):
        raise VerificationError("manifest.fixture is not an allowed fixture")

    versions = require_object(manifest.get("dependency_versions"), "manifest.dependency_versions")
    for key in ("KDV", "KUC", "KRR"):
        version = require_string(versions.get(key), f"manifest.dependency_versions.{key}")
        if SEMVER.fullmatch(version) is None:
            raise VerificationError(f"manifest.dependency_versions.{key} must be a valid SemVer string")

    hashes_obj = require_object(manifest.get("sha256"), "manifest.sha256")
    hashes: dict[str, str] = {}
    for key in ("crop", "full", "geometry"):
        digest = require_string(hashes_obj.get(key), f"manifest.sha256.{key}")
        if re.fullmatch(r"[0-9a-fA-F]{64}", digest) is None:
            raise VerificationError(f"manifest.sha256.{key} must be a SHA-256 hex digest")
        hashes[key] = digest.lower()

    sizes_obj = require_object(manifest.get("sizes"), "manifest.sizes")
    sizes: dict[str, tuple[int, int]] = {}
    for key, expected in (("crop", (1280, 2400)), ("full", (2565, 4774))):
        size = require_object(sizes_obj.get(key), f"manifest.sizes.{key}")
        actual = (
            require_int(size.get("width"), f"manifest.sizes.{key}.width", minimum=1),
            require_int(size.get("height"), f"manifest.sizes.{key}.height", minimum=1),
        )
        if actual != expected:
            raise VerificationError(f"manifest.sizes.{key} must be {expected[0]}x{expected[1]}")
        sizes[key] = actual

    frame = require_int(manifest.get("expected_ui_frame_nr"), "manifest.expected_ui_frame_nr", minimum=1)
    contract = require_object(manifest.get("geometry_contract"), "manifest.geometry_contract")
    expected_geometry: dict[str, Any] = {
        "config": 14,
        "font_scale": 2,
        "scroll_y": 0,
        "viewport": {"x": 44, "y": 134, "width": 1187, "height": 2225},
        "physical_crop": {"x": 88, "y": 268, "width": 2374, "height": 4450},
        "content_top_y": 134,
        "full_width": 2565,
        "full_height": 4774,
    }
    for key, expected in expected_geometry.items():
        if contract.get(key) != expected:
            raise VerificationError(f"manifest.geometry_contract.{key} does not match the required geometry")
    for key in ("config", "font_scale", "scroll_y", "content_top_y"):
        require_number(contract.get(key), f"manifest.geometry_contract.{key}")
    for group in ("viewport", "physical_crop"):
        values = require_object(contract[group], f"manifest.geometry_contract.{group}")
        for key in ("x", "y", "width", "height"):
            validator = require_number if group == "viewport" else require_int
            validator(values.get(key), f"manifest.geometry_contract.{group}.{key}")
    for key in ("full_width", "full_height"):
        require_int(contract.get(key), f"manifest.geometry_contract.{key}")
    return hashes, sizes, frame, contract


def validate_interactions(value: Any, name: str) -> None:
    state = require_object(value, name)
    if state.get("pointer_inside_content") is not False:
        raise VerificationError(f"{name}.pointer_inside_content must be false")
    if "active_editor_line" not in state or state["active_editor_line"] is not None:
        raise VerificationError(f"{name}.active_editor_line must exist and be null")
    for field in CLEAN_ZERO_FIELDS:
        require_int(state.get(field), f"{name}.{field}")
        if state[field] != 0:
            raise VerificationError(f"{name}.{field} must be 0")


def verify(manifest_path: Path, crop_path: Path, full_path: Path, geometry_path: Path) -> dict[str, Any]:
    manifest = read_json(manifest_path, "manifest")
    hashes, sizes, expected_frame, contract = validate_manifest(manifest)
    candidates = {"crop": crop_path, "full": full_path, "geometry": geometry_path}
    for name, path in candidates.items():
        if sha256_file(path, name) != hashes[name]:
            raise VerificationError(f"{name} SHA-256 does not match manifest")
    for name in ("crop", "full"):
        if png_dimensions(candidates[name], name) != sizes[name]:
            raise VerificationError(f"{name} dimensions do not match manifest")

    geometry = read_json(geometry_path, "geometry")
    if require_int(geometry.get("schema_version"), "geometry.schema_version") != 1:
        raise VerificationError("geometry.schema_version must be 1")
    if geometry.get("require_clean_interaction_state") is not True:
        raise VerificationError("geometry.require_clean_interaction_state must be true")
    if require_int(geometry.get("ui_pass_frame_nr"), "geometry.ui_pass_frame_nr", minimum=1) != expected_frame:
        raise VerificationError("geometry.ui_pass_frame_nr does not match expected frame")
    state = require_object(geometry.get("interaction_state"), "geometry.interaction_state")
    if require_int(state.get("completed_ui_frame_nr"), "geometry.interaction_state.completed_ui_frame_nr", minimum=1) != expected_frame:
        raise VerificationError("interaction_state.completed_ui_frame_nr does not match expected frame")
    validate_interactions(state, "geometry.interaction_state")

    preview = require_object(geometry.get("preview_geometry"), "geometry.preview_geometry")
    viewport = require_object(preview.get("viewport"), "geometry.preview_geometry.viewport")
    crop = require_object(preview.get("physical_crop"), "geometry.preview_geometry.physical_crop")
    full = require_object(preview.get("full_screenshot"), "geometry.preview_geometry.full_screenshot")
    if not require_string(full.get("output_name"), "geometry.preview_geometry.full_screenshot.output_name").strip():
        raise VerificationError("geometry.preview_geometry.full_screenshot.output_name must not be empty")
    actual = {
        "config": require_number(geometry.get("configured_font_size"), "geometry.configured_font_size"),
        "font_scale": require_number(geometry.get("pixels_per_point"), "geometry.pixels_per_point"),
        "scroll_y": require_number(preview.get("scroll_y"), "geometry.preview_geometry.scroll_y"),
        "content_top_y": require_number(preview.get("content_top_y"), "geometry.preview_geometry.content_top_y"),
        "full_width": require_int(full.get("width"), "geometry.preview_geometry.full_screenshot.width", minimum=1),
        "full_height": require_int(full.get("height"), "geometry.preview_geometry.full_screenshot.height", minimum=1),
        "viewport": viewport,
        "physical_crop": crop,
    }
    for key in ("x", "y", "width", "height"):
        require_number(viewport.get(key), f"geometry.preview_geometry.viewport.{key}")
        require_int(crop.get(key), f"geometry.preview_geometry.physical_crop.{key}")
    for key, value in actual.items():
        if value != contract[key]:
            raise VerificationError(f"geometry.{key} does not match manifest contract")
    if crop["x"] < 0 or crop["y"] < 0 or crop["x"] + crop["width"] > sizes["full"][0] or crop["y"] + crop["height"] > sizes["full"][1]:
        raise VerificationError("physical crop is outside full image bounds")
    if (crop["x"], crop["y"], crop["width"], crop["height"]) != (
        viewport["x"] * actual["font_scale"], viewport["y"] * actual["font_scale"],
        viewport["width"] * actual["font_scale"], viewport["height"] * actual["font_scale"]
    ):
        raise VerificationError("physical crop does not match 2x viewport geometry")

    return {
        "provenance_verified": True,
        "producer_repo": manifest["producer_repo"],
        "producer_commit": manifest["producer_commit"],
        "fixture": manifest["fixture"],
        "dependency_versions": manifest["dependency_versions"],
        "sha256": hashes,
        "sizes": {key: {"width": size[0], "height": size[1]} for key, size in sizes.items()},
        "ui_frame_nr": expected_frame,
        "visual_status": "not_evaluated",
        "semantic_status": "not_evaluated",
        "interaction_status": "not_evaluated",
        "performance_status": "not_evaluated",
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--crop", type=Path)
    parser.add_argument("--full", type=Path)
    parser.add_argument("--geometry", type=Path)
    parser.add_argument("--expected-fixture", choices=("katana/sample.md", "katana/sample_diagrams.md"))
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        try:
            run_self_tests()
        except (AssertionError, VerificationError, OSError) as error:
            print(f"self-test failed: {error}", file=sys.stderr)
            return 1
        print(json.dumps({"self_test": "passed"}, sort_keys=True))
        return 0
    if any(value is None for value in (args.manifest, args.crop, args.full, args.geometry)):
        parser.error("--manifest, --crop, --full, and --geometry are required unless --self-test is used")
    try:
        result = verify(args.manifest, args.crop, args.full, args.geometry)
        if args.expected_fixture is not None and result["fixture"] != args.expected_fixture:
            raise VerificationError("manifest fixture does not match the scorer fixture")
    except VerificationError as error:
        print(f"verification failed: {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, sort_keys=True))
    return 0


def run_self_tests() -> None:
    """Kept dependency-free so the verifier can smoke-test its own boundaries."""
    from test_current_preview_crop import run_self_tests as test_suite
    test_suite()


if __name__ == "__main__":
    raise SystemExit(main())
