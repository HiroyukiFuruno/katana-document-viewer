import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
GUARD = ROOT / "scripts" / "maintenance" / "guard-build-cache.py"
CACHEDIR_TAG = "Signature: 8a477f597d28d172789f06886806bc55\n"
GIB = 1024**3


def allocated_file_bytes(directory):
    seen = set()
    total = 0
    for path in directory.rglob("*"):
        if path.is_file() and not path.is_symlink():
            stat = path.stat()
            identity = (stat.st_dev, stat.st_ino or str(path.resolve()))
            if identity in seen:
                continue
            seen.add(identity)
            total += stat.st_blocks * 512 if hasattr(stat, "st_blocks") else stat.st_size
    return total


class CargoBuildDirectoryGuardTests(unittest.TestCase):
    def test_capacity_guard_tracks_build_directory_with_and_without_target_override(self):
        for explicit_target in (False, True):
            with self.subTest(explicit_target=explicit_target), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                environment, manifest, configured_target, build_directory, selected_target = (
                    self._prepare_fixture(root, explicit_target)
                )
                target_sentinel = self._own_cache(selected_target, "target-sentinel")
                build_sentinel = self._own_cache(build_directory, "build-sentinel")
                (build_directory / "capacity-padding").write_bytes(b"x" * (2 * 1024 * 1024))

                target_bytes = allocated_file_bytes(selected_target)
                build_bytes = allocated_file_bytes(build_directory)
                max_bytes = target_bytes + 1024 * 1024
                self.assertLessEqual(target_bytes, max_bytes)
                self.assertGreater(target_bytes + build_bytes, max_bytes)
                guard_command = [
                    sys.executable, str(GUARD), "--repo-root", str(root), "--max-gib",
                    str(max_bytes / GIB), "--", "cargo", "--offline", "build",
                    "--manifest-path", str(manifest),
                ]
                if selected_target != configured_target:
                    guard_command.extend(["--target-dir", str(selected_target)])
                guarded = subprocess.run(
                    guard_command, cwd=root, env=environment, capture_output=True, text=True,
                )
                self.assertEqual(0, guarded.returncode, guarded.stderr)
                self.assertFalse(target_sentinel.exists(), "capacity guard left target sentinel")
                self.assertFalse(build_sentinel.exists(), "capacity guard left build-directory sentinel")
                self.assertTrue((selected_target / "CACHEDIR.TAG").is_file())
                self.assertTrue((build_directory / "CACHEDIR.TAG").is_file())

    def test_manual_clean_tracks_build_directory_and_respects_target_override(self):
        for explicit_target in (False, True):
            with self.subTest(explicit_target=explicit_target), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                environment, _, configured_target, build_directory, selected_target = (
                    self._prepare_fixture(root, explicit_target)
                )
                target_sentinel = self._own_cache(selected_target, "target-sentinel")
                build_sentinel = self._own_cache(build_directory, "build-sentinel")
                unrelated_sentinel = None
                if selected_target != configured_target:
                    unrelated_sentinel = self._own_cache(configured_target, "unrelated-sentinel")

                clean_command = [
                    sys.executable, str(GUARD), "--repo-root", str(root), "--", "cargo", "clean",
                ]
                if selected_target != configured_target:
                    clean_command.extend(["--target-dir", str(selected_target)])
                cleaned = subprocess.run(
                    clean_command, cwd=root, env=environment, capture_output=True, text=True,
                )
                self.assertEqual(0, cleaned.returncode, cleaned.stderr)
                self.assertFalse(target_sentinel.exists(), "manual clean left target sentinel")
                self.assertFalse(build_sentinel.exists(), "manual clean left build-directory sentinel")
                self.assertTrue((selected_target / "CACHEDIR.TAG").is_file())
                self.assertTrue((build_directory / "CACHEDIR.TAG").is_file())
                if unrelated_sentinel is not None:
                    self.assertTrue(unrelated_sentinel.exists(), "manual clean removed nonselected target")

    def test_nested_build_directory_tags_survive_repeated_capacity_cleanup(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            environment = self._environment(root)
            manifest = self._write_crate(root)
            target = root / "artifact-cache"
            build_directory = target / "intermediate-cache"
            self._write_config(root, target, build_directory)
            metadata, build = self._cargo_build_and_metadata(root, environment, manifest)
            self.assertEqual(build_directory.resolve(), Path(metadata["build_directory"]).resolve())
            self.assertTrue(target.is_dir())
            self.assertTrue(build_directory.is_dir())
            (target / "CACHEDIR.TAG").write_text(CACHEDIR_TAG)
            (build_directory / "CACHEDIR.TAG").write_text(CACHEDIR_TAG)

            for attempt in range(2):
                (target / f"capacity-trigger-{attempt}").write_bytes(b"x" * (2 * 1024 * 1024))
                guarded = subprocess.run(
                    [
                        sys.executable, str(GUARD), "--repo-root", str(root), "--max-gib", "0.000000001",
                        "--", "cargo", "--offline", "build", "--manifest-path", str(manifest),
                    ],
                    cwd=root, env=environment, capture_output=True, text=True,
                )
                self.assertEqual(0, guarded.returncode, f"attempt {attempt + 1}: {guarded.stderr}")
                self.assertTrue((target / "CACHEDIR.TAG").is_file())
                self.assertTrue((build_directory / "CACHEDIR.TAG").is_file())

    def _prepare_fixture(self, root, explicit_target):
        environment = self._environment(root)
        manifest = self._write_crate(root)
        configured_target = root / "artifact-cache"
        build_directory = root / "intermediate-cache"
        self._write_config(root, configured_target, build_directory)
        selected_target = root / "selected-artifacts" if explicit_target else configured_target
        metadata, build = self._cargo_build_and_metadata(
            root, environment, manifest, selected_target if explicit_target else None,
        )
        self.assertIn("build_directory", metadata, "Cargo metadata omitted build_directory")
        self.assertEqual(build_directory.resolve(), Path(metadata["build_directory"]).resolve())
        self.assertEqual(0, build.returncode, build.stderr)
        self.assertTrue(selected_target.is_dir(), "Cargo did not create selected target directory")
        self.assertTrue(build_directory.is_dir(), "Cargo did not create metadata build_directory")
        return environment, manifest, configured_target, build_directory, selected_target

    def _cargo_build_and_metadata(self, root, environment, manifest, target_override=None):
        build_command = ["cargo", "--offline", "build"]
        if target_override is not None:
            build_command.extend(["--target-dir", str(target_override)])
        build = subprocess.run(
            [*build_command, "--manifest-path", str(manifest)], cwd=root, env=environment,
            capture_output=True, text=True,
        )
        self.assertEqual(0, build.returncode, build.stderr)
        metadata = subprocess.run(
            ["cargo", "--offline", "metadata", "--no-deps", "--format-version", "1"],
            cwd=root, env=environment, capture_output=True, text=True,
        )
        self.assertEqual(0, metadata.returncode, metadata.stderr)
        return json.loads(metadata.stdout), build

    def _own_cache(self, directory, sentinel_name):
        directory.mkdir(parents=True, exist_ok=True)
        (directory / "CACHEDIR.TAG").write_text(CACHEDIR_TAG)
        sentinel = directory / sentinel_name
        sentinel.write_text("owned cache\n")
        return sentinel

    def _environment(self, root):
        environment = os.environ.copy()
        for name in ("CARGO_TARGET_DIR", "CARGO_BUILD_TARGET_DIR", "CARGO_BUILD_BUILD_DIR"):
            environment.pop(name, None)
        environment["CARGO_HOME"] = str(root / "cargo-home")
        return environment

    def _write_crate(self, root):
        manifest = root / "Cargo.toml"
        manifest.write_text(
            '[package]\nname="build-dir-cache-smoke"\nversion="0.0.0"\nedition="2021"\n'
        )
        (root / "src").mkdir()
        (root / "src/main.rs").write_text("fn main() {}\n")
        return manifest

    def _write_config(self, root, target, build_directory):
        cargo_config = root / ".cargo"
        cargo_config.mkdir()
        (cargo_config / "config.toml").write_text(
            f'[build]\ntarget-dir = "{target.name}"\nbuild-dir = "{build_directory.relative_to(root)}"\n'
        )


if __name__ == "__main__":
    unittest.main()
