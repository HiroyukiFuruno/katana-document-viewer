"""Pure boundary tests for verify-current-preview-crop.py."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import struct
import subprocess
import sys
import tempfile
import unittest
import zlib
from pathlib import Path
from typing import Any


VERIFIER_PATH = Path(__file__).with_name("verify-current-preview-crop.py")
SPEC = importlib.util.spec_from_file_location("current_preview_crop_verifier", VERIFIER_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("cannot load verifier module")
verifier = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(verifier)


def png_chunk(kind: bytes, data: bytes) -> bytes:
    return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)


def png_image(width: int, height: int) -> bytes:
    data = struct.pack(">IIBBBBB", width, height, 1, 0, 0, 0, 0)
    rows = (b"\x00" + bytes((width + 7) // 8)) * height
    return b"\x89PNG\r\n\x1a\n" + png_chunk(b"IHDR", data) + png_chunk(b"IDAT", zlib.compress(rows)) + png_chunk(b"IEND", b"")


def clean_state() -> dict[str, Any]:
    result: dict[str, Any] = {
        "pointer_inside_content": False,
        "active_editor_line": None,
        "active_markdown_ranges": 0,
        "code_copy_control_renders": 0,
        "code_selection_renders": 0,
        "diagram_control_renders": 0,
        "hovered_markdown_spans": 0,
        "hovered_preview_line_count": 0,
        "image_control_renders": 0,
        "image_hover_background_renders": 0,
        "local_image_hover_background_renders": 0,
        "completed_ui_frame_nr": 1223,
        "markdown_section_renders": 17,
        "pointer_position": {"x": 1.0, "y": 1.0},
    }
    return result


def geometry_document() -> dict[str, Any]:
    # 実producerのJSON構造を固定し、検証器の都合でfixtureを作り替えない。
    return {
        "schema_version": 1,
        "ui_pass_frame_nr": 1223,
        "configured_font_size": 14.0,
        "pixels_per_point": 2.0,
        "require_clean_interaction_state": True,
        "preview_geometry": {
            "scroll_y": 0.0,
            "viewport": {"x": 44.0, "y": 134.0, "width": 1187.0, "height": 2225.0},
            "physical_crop": {"x": 88, "y": 268, "width": 2374, "height": 4450},
            "content_top_y": 134.0,
            "full_screenshot": {"width": 2565, "height": 4774, "output_name": "sample-diagrams-full"},
        },
        "interaction_state": clean_state(),
    }


def manifest_document() -> dict[str, Any]:
    return {
        "schema_version": 1,
        "producer_repo": "https://github.com/HiroyukiFuruno/KatanA",
        "producer_commit": "a" * 40,
        "fixture": "katana/sample_diagrams.md",
        "dependency_versions": {"KDV": "0.5.8", "KUC": "0.4.1", "KRR": "0.7.2"},
        "sha256": {"crop": "0" * 64, "full": "0" * 64, "geometry": "0" * 64},
        "sizes": {"crop": {"width": 1280, "height": 2400}, "full": {"width": 2565, "height": 4774}},
        "expected_ui_frame_nr": 1223,
        "geometry_contract": {
            "config": 14,
            "font_scale": 2,
            "scroll_y": 0,
            "viewport": {"x": 44, "y": 134, "width": 1187, "height": 2225},
            "physical_crop": {"x": 88, "y": 268, "width": 2374, "height": 4450},
            "content_top_y": 134,
            "full_width": 2565,
            "full_height": 4774,
        },
    }


class CurrentPreviewCropTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.crop = self.root / "crop.png"
        self.full = self.root / "full.png"
        self.geometry = self.root / "geometry.json"
        self.manifest = self.root / "manifest.json"
        self.crop.write_bytes(png_image(1280, 2400))
        self.full.write_bytes(png_image(2565, 4774))
        self.write_geometry(geometry_document())
        self.declaration = manifest_document()
        self.refresh_hashes()
        self.write_manifest()

    def tearDown(self) -> None:
        self.temp.cleanup()

    def write_geometry(self, value: dict[str, Any]) -> None:
        self.geometry.write_text(json.dumps(value), encoding="utf-8")

    def write_manifest(self) -> None:
        self.manifest.write_text(json.dumps(self.declaration), encoding="utf-8")

    def refresh_hashes(self) -> None:
        self.declaration["sha256"] = {
            "crop": hashlib.sha256(self.crop.read_bytes()).hexdigest(),
            "full": hashlib.sha256(self.full.read_bytes()).hexdigest(),
            "geometry": hashlib.sha256(self.geometry.read_bytes()).hexdigest(),
        }

    def reject(self, message: str) -> None:
        with self.assertRaisesRegex(verifier.VerificationError, message):
            self.verify_preserving_inputs()

    def verify_preserving_inputs(self) -> dict[str, Any]:
        before = {path: path.read_bytes() for path in (self.manifest, self.crop, self.full, self.geometry)}
        try:
            return verifier.verify(self.manifest, self.crop, self.full, self.geometry)
        finally:
            self.assertEqual({path: path.read_bytes() for path in before}, before)

    def set_geometry(self, doc: dict[str, Any]) -> None:
        self.write_geometry(doc)
        self.refresh_hashes()
        self.write_manifest()

    def test_real_producer_shape_is_verified_without_scores(self) -> None:
        result = self.verify_preserving_inputs()
        self.assertIs(result["provenance_verified"], True)
        self.assertEqual(result["ui_frame_nr"], 1223)
        for category in ("visual", "semantic", "interaction", "performance"):
            self.assertEqual(result[f"{category}_status"], "not_evaluated")

    def test_sha_mismatch_fails(self) -> None:
        self.declaration["sha256"]["crop"] = "f" * 64
        self.write_manifest()
        self.reject("crop SHA-256")

    def test_dimension_mismatch_fails(self) -> None:
        self.crop.write_bytes(png_image(1279, 2400))
        self.refresh_hashes()
        self.write_manifest()
        self.reject("crop dimensions")

    def test_frame_mismatch_fails(self) -> None:
        for key in ("ui_pass_frame_nr", "completed_ui_frame_nr"):
            for value in (1224, 0, -1, True, 1223.0):
                with self.subTest(key=key, value=value):
                    doc = geometry_document()
                    target = doc if key == "ui_pass_frame_nr" else doc["interaction_state"]
                    target[key] = value
                    self.set_geometry(doc)
                    self.reject(key)

    def test_each_interaction_violation_fails(self) -> None:
        for field in verifier.INTERACTION_FIELDS:
            with self.subTest(field=field):
                doc = geometry_document()
                if field == "pointer_inside_content":
                    doc["interaction_state"][field] = True
                elif field == "active_editor_line":
                    doc["interaction_state"][field] = 1
                else:
                    doc["interaction_state"][field] = 1
                self.set_geometry(doc)
                self.reject(field)

    def test_malformed_png_fails(self) -> None:
        self.crop.write_bytes(b"not a png")
        self.refresh_hashes()
        self.write_manifest()
        self.reject("not a PNG")

    def test_missing_manifest_field_fails(self) -> None:
        del self.declaration["dependency_versions"]["KRR"]
        self.write_manifest()
        self.reject("dependency_versions.KRR")

    def test_boolean_is_not_an_integer(self) -> None:
        self.declaration["expected_ui_frame_nr"] = True
        self.write_manifest()
        self.reject("expected_ui_frame_nr must be an integer")

    def test_geometry_contract_mismatch_fails(self) -> None:
        self.declaration["geometry_contract"]["font_scale"] = 3
        self.write_manifest()
        self.reject("geometry_contract.font_scale")

    def test_producer_schema_version_is_required(self) -> None:
        for value in (2, True, 1.0, None):
            with self.subTest(value=value):
                doc = geometry_document()
                doc["schema_version"] = value
                self.set_geometry(doc)
                self.reject("schema_version")
        doc = geometry_document()
        del doc["schema_version"]
        self.set_geometry(doc)
        self.reject("schema_version")

    def test_invented_geometry_wrapper_is_rejected(self) -> None:
        doc = geometry_document()
        wrapped = {"geometry": doc, "interaction_state": doc["interaction_state"]}
        self.set_geometry(wrapped)
        self.reject("schema_version")

    def test_clean_flag_must_be_true_at_top_level(self) -> None:
        for value in (False, 1, None):
            with self.subTest(value=value):
                doc = geometry_document()
                doc["require_clean_interaction_state"] = value
                self.set_geometry(doc)
                self.reject("require_clean_interaction_state")
        doc = geometry_document()
        del doc["require_clean_interaction_state"]
        doc["interaction_state"]["require_clean_interaction_state"] = True
        self.set_geometry(doc)
        self.reject("require_clean_interaction_state")

    def test_each_interaction_field_must_exist(self) -> None:
        for field in verifier.INTERACTION_FIELDS:
            with self.subTest(field=field):
                doc = geometry_document()
                del doc["interaction_state"][field]
                self.set_geometry(doc)
                self.reject(field)

    def test_each_zero_field_rejects_boolean_and_float(self) -> None:
        for field in verifier.CLEAN_ZERO_FIELDS:
            for value in (False, 0.0):
                with self.subTest(field=field, value=value):
                    doc = geometry_document()
                    doc["interaction_state"][field] = value
                    self.set_geometry(doc)
                    self.reject(field)

    def test_logical_geometry_accepts_int_or_float(self) -> None:
        doc = geometry_document()
        for field in ("configured_font_size", "pixels_per_point"):
            doc[field] = int(doc[field])
        preview = doc["preview_geometry"]
        for field in ("scroll_y", "content_top_y"):
            preview[field] = int(preview[field])
        preview["viewport"] = {key: int(value) for key, value in preview["viewport"].items()}
        self.set_geometry(doc)
        self.assertTrue(self.verify_preserving_inputs()["provenance_verified"])

    def test_logical_geometry_rejects_boolean_nan_and_mismatch(self) -> None:
        paths = [
            ("configured_font_size",), ("pixels_per_point",),
            ("preview_geometry", "scroll_y"), ("preview_geometry", "content_top_y"),
            *(("preview_geometry", "viewport", key) for key in ("x", "y", "width", "height")),
        ]
        for path in paths:
            for value in (False, float("nan"), 999.0):
                with self.subTest(path=path, value=value):
                    doc = geometry_document()
                    target = doc
                    for key in path[:-1]:
                        target = target[key]
                    target[path[-1]] = value
                    self.set_geometry(doc)
                    self.reject("finite|numeric constant|manifest contract")

    def test_physical_geometry_requires_integers(self) -> None:
        for group, keys in (("physical_crop", ("x", "y", "width", "height")), ("full_screenshot", ("width", "height"))):
            for key in keys:
                for value in (False, 0.0):
                    with self.subTest(group=group, key=key, value=value):
                        doc = geometry_document()
                        doc["preview_geometry"][group][key] = value
                        self.set_geometry(doc)
                        self.reject("must be an integer")

    def test_output_name_is_required(self) -> None:
        doc = geometry_document()
        del doc["preview_geometry"]["full_screenshot"]["output_name"]
        self.set_geometry(doc)
        self.reject("output_name")

    def test_manifest_logical_geometry_rejects_boolean(self) -> None:
        self.declaration["geometry_contract"]["scroll_y"] = False
        self.write_manifest()
        self.reject("finite number")

    def test_manifest_accepts_finite_float_geometry(self) -> None:
        contract = self.declaration["geometry_contract"]
        for key in ("config", "font_scale", "scroll_y", "content_top_y"):
            contract[key] = float(contract[key])
        contract["viewport"] = {key: float(value) for key, value in contract["viewport"].items()}
        self.write_manifest()
        self.assertTrue(self.verify_preserving_inputs()["provenance_verified"])

    def test_json_duplicate_keys_are_rejected(self) -> None:
        for target, field in ((self.manifest, "expected_ui_frame_nr"), (self.geometry, "ui_pass_frame_nr")):
            with self.subTest(target=target.name):
                self.write_manifest()
                self.write_geometry(geometry_document())
                raw = target.read_text(encoding="utf-8")
                target.write_text(raw.replace(f'"{field}": 1223', f'"{field}": 0, "{field}": 1223'), encoding="utf-8")
                if target == self.geometry:
                    self.refresh_hashes()
                    self.write_manifest()
                self.reject("duplicate JSON key")

    def test_json_nonfinite_numbers_are_rejected(self) -> None:
        for target in (self.manifest, self.geometry):
            for token in ("NaN", "Infinity", "-Infinity", "1e999"):
                with self.subTest(target=target.name, token=token):
                    self.write_geometry(geometry_document())
                    self.refresh_hashes()
                    self.write_manifest()
                    raw = target.read_text(encoding="utf-8")
                    target.write_text(raw[:-1] + f', "hostile_numeric_declaration": {token}' + "}", encoding="utf-8")
                    if target == self.geometry:
                        self.refresh_hashes()
                        self.write_manifest()
                    self.reject("numeric constant|non-finite")

    def test_dependency_versions_require_valid_semver(self) -> None:
        for key in ("KDV", "KUC", "KRR"):
            for value in ("", " ", "v1.2.3", "1.2", "01.2.3", "1.2.3-01", "1.2.3+", 123):
                with self.subTest(key=key, value=value):
                    self.declaration = manifest_document()
                    self.declaration["dependency_versions"][key] = value
                    self.refresh_hashes()
                    self.write_manifest()
                    self.reject(f"dependency_versions.{key}")

    def test_dependency_versions_accept_prerelease_and_build(self) -> None:
        self.declaration["dependency_versions"] = {"KDV": "1.2.3-rc.1+build.001", "KUC": "0.0.0", "KRR": "1.2.3-alpha-beta"}
        self.write_manifest()
        self.assertTrue(self.verify_preserving_inputs()["provenance_verified"])

    def test_bad_ihdr_crc_is_rejected(self) -> None:
        png = bytearray(self.crop.read_bytes())
        png[29] ^= 1
        self.crop.write_bytes(png)
        self.refresh_hashes()
        self.write_manifest()
        self.reject("IHDR CRC")

    def test_cli_success_and_failure_preserve_inputs(self) -> None:
        args = [sys.executable, "-B", str(VERIFIER_PATH), "--manifest", str(self.manifest), "--crop", str(self.crop), "--full", str(self.full), "--geometry", str(self.geometry)]
        before = {path: path.read_bytes() for path in (self.manifest, self.crop, self.full, self.geometry)}
        result = subprocess.run(args, capture_output=True, text=True, check=False)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(json.loads(result.stdout)["provenance_verified"])
        self.assertEqual({path: path.read_bytes() for path in before}, before)
        doc = geometry_document()
        del doc["interaction_state"]["active_editor_line"]
        self.set_geometry(doc)
        before = {path: path.read_bytes() for path in before}
        result = subprocess.run(args, capture_output=True, text=True, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(result.stdout, "")
        self.assertIn("active_editor_line must exist", result.stderr)
        self.assertNotIn("Traceback", result.stderr)
        self.assertEqual({path: path.read_bytes() for path in before}, before)

    def test_cli_pins_the_scorer_fixture(self) -> None:
        args = [sys.executable, "-B", str(VERIFIER_PATH), "--manifest", str(self.manifest), "--crop", str(self.crop), "--full", str(self.full), "--geometry", str(self.geometry), "--expected-fixture"]
        before = {path: path.read_bytes() for path in (self.manifest, self.crop, self.full, self.geometry)}
        matching = subprocess.run([*args, "katana/sample_diagrams.md"], capture_output=True, text=True, check=False)
        self.assertEqual(matching.returncode, 0, matching.stderr)
        mismatching = subprocess.run([*args, "katana/sample.md"], capture_output=True, text=True, check=False)
        self.assertNotEqual(mismatching.returncode, 0)
        self.assertIn("does not match the scorer fixture", mismatching.stderr)
        self.assertEqual(mismatching.stdout, "")
        self.assertEqual({path: path.read_bytes() for path in before}, before)

    def test_cli_pins_the_typography_scorer_fixture(self) -> None:
        self.declaration["fixture"] = "katana/sample.md"
        self.write_manifest()
        args = [sys.executable, "-B", str(VERIFIER_PATH), "--manifest", str(self.manifest), "--crop", str(self.crop), "--full", str(self.full), "--geometry", str(self.geometry), "--expected-fixture"]
        before = {path: path.read_bytes() for path in (self.manifest, self.crop, self.full, self.geometry)}
        matching = subprocess.run([*args, "katana/sample.md"], capture_output=True, text=True, check=False)
        self.assertEqual(matching.returncode, 0, matching.stderr)
        mismatching = subprocess.run([*args, "katana/sample_diagrams.md"], capture_output=True, text=True, check=False)
        self.assertNotEqual(mismatching.returncode, 0)
        self.assertIn("does not match the scorer fixture", mismatching.stderr)
        self.assertEqual(mismatching.stdout, "")
        self.assertEqual({path: path.read_bytes() for path in before}, before)


def write_native_fixture(directory: Path, lane: str, corruption: str, fixture: str = "katana/sample_diagrams.md") -> None:
    # native回帰も同じ公開factoryを使い、破損時は宣言hashを更新して実デコードを検証する。
    if list(directory.iterdir()) or lane not in ("crop", "full") or fixture not in ("katana/sample.md", "katana/sample_diagrams.md"):
        raise ValueError("native fixture requires an empty directory and a PNG lane")
    case = CurrentPreviewCropTests()
    case.setUp()
    try:
        case.declaration["fixture"] = fixture
        case.write_manifest()
        case.verify_preserving_inputs()
        path = case.crop if lane == "crop" else case.full
        png = path.read_bytes()
        if corruption == "header-only":
            path.write_bytes(png[:33])
        elif corruption == "crc":
            length = struct.unpack(">I", png[33:37])[0]
            damaged = bytearray(png)
            damaged[41 + length] ^= 1
            path.write_bytes(damaged)
        elif corruption == "zlib":
            length = struct.unpack(">I", png[33:37])[0]
            payload = bytearray(png[41:41 + length])
            payload[0] = 0
            path.write_bytes(png[:33] + png_chunk(b"IDAT", payload) + png[45 + length:])
        elif corruption != "none":
            raise ValueError("unknown native PNG corruption")
        case.refresh_hashes()
        case.write_manifest()
        for source in (case.manifest, case.crop, case.full, case.geometry):
            with (directory / source.name).open("xb") as output:
                output.write(source.read_bytes())
    finally:
        case.tearDown()


def run_self_tests() -> None:
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(CurrentPreviewCropTests)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    if not result.wasSuccessful():
        raise AssertionError("current preview crop self-tests failed")


if __name__ == "__main__":
    run_self_tests()
