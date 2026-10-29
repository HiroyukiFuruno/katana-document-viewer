import os
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
GUARD = ROOT / "scripts" / "maintenance" / "guard-build-cache.py"
CACHEDIR_TAG = "Signature: 8a477f597d28d172789f06886806bc55\n"


class CargoConfigBuildCacheTests(unittest.TestCase):
    @unittest.skipIf(os.name == "nt" or (hasattr(os, "geteuid") and os.geteuid() == 0), "requires POSIX user permission enforcement")
    def test_unreadable_config_refuses_cleanup_and_preserves_both_targets(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "Cargo.toml").write_text('[package]\nname="unreadable-config"\nversion="0.0.0"\n')
            (root / "src").mkdir()
            (root / "src/main.rs").write_text("fn main() {}\n")
            config = root / ".cargo"
            config.mkdir()
            (config / "config.toml").write_text('[build]\ntarget-dir="configured-cache"\n')
            sentinels = []
            for name in ("target", "configured-cache"):
                target = root / name
                target.mkdir()
                (target / "CACHEDIR.TAG").write_text(CACHEDIR_TAG)
                sentinel = target / "sentinel"
                sentinel.write_text("preserve\n")
                sentinels.append(sentinel)
            environment = os.environ.copy()
            for name in ("CARGO_TARGET_DIR", "CARGO_BUILD_TARGET_DIR", "CARGO_BUILD_BUILD_DIR"):
                environment.pop(name, None)
            environment["CARGO_HOME"] = str(root / "cargo-home")
            config.chmod(0)
            try:
                result = subprocess.run(
                    [sys.executable, str(GUARD), "--repo-root", str(root), "--", "cargo", "clean"],
                    cwd=root, env=environment, capture_output=True, text=True,
                )
            finally:
                config.chmod(0o700)
            self.assertEqual(2, result.returncode, result.stderr)
            self.assertTrue(all(sentinel.exists() for sentinel in sentinels))

    def run_configured_build(self, cargo_arguments, config_setup):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = root / "owned-cache"
            config_setup(root, target)
            self._run_fixture(root, target, cargo_arguments)

    def test_workspace_cargo_config_target_dir_is_guarded(self):
        def configure(root, target):
            config = root / ".cargo"
            config.mkdir()
            (config / "config.toml").write_text(
                '[build]\ntarget-dir = "owned-cache"\n'
            )

        self.run_configured_build([], configure)

    def test_config_file_passed_to_cargo_is_guarded(self):
        def configure(root, target):
            config = root / "alternate-cargo-config.toml"
            config.write_text(f'[build]\ntarget-dir = {json.dumps(str(target))}\n')

        self.run_configured_build_with_config_file(configure)

    def run_configured_build_with_config_file(self, configure):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = root / "owned-cache"
            configure(root, target)
            config = root / "alternate-cargo-config.toml"
            self._run_fixture(root, target, ["--config", str(config)])

    def test_inline_config_target_dir_is_guarded(self):
        # Cargo自身に引用符付きの相対TOML値を解決させる。
        self.run_configured_build(
            ["--config", 'build.target-dir="owned-cache"'],
            lambda root, target: None,
        )

    def test_cargo_clean_uses_workspace_config_target_dir_only(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = root / "owned-cache"
            config = root / ".cargo"
            config.mkdir()
            (config / "config.toml").write_text(
                '[build]\ntarget-dir = "owned-cache"\n'
            )
            manifest = root / "Cargo.toml"
            manifest.write_text(
                '[package]\nname="config-clean-smoke"\nversion="0.0.0"\nedition="2021"\n'
            )
            (root / "src").mkdir()
            (root / "src/main.rs").write_text("fn main() {}\n")
            environment = os.environ.copy()
            for name in ("CARGO_TARGET_DIR", "CARGO_BUILD_TARGET_DIR", "CARGO_BUILD_BUILD_DIR"):
                environment.pop(name, None)
            environment["CARGO_HOME"] = str(root / "cargo-home")

            build = subprocess.run(
                ["cargo", "--offline", "build"],
                cwd=root, env=environment, capture_output=True, text=True,
            )
            self.assertEqual(0, build.returncode, build.stderr)
            self.assertTrue(target.is_dir(), "Cargo did not use the configured target directory")

            default_target = root / "target"
            default_target.mkdir()
            configured_sentinel = target / "configured-sentinel"
            default_sentinel = default_target / "default-sentinel"
            configured_sentinel.write_text("remove configured cache\n")
            default_sentinel.write_text("preserve unrelated target\n")
            for owned_target in (target, default_target):
                (owned_target / "CACHEDIR.TAG").write_text(CACHEDIR_TAG)

            cleaned = subprocess.run(
                [sys.executable, str(GUARD), "--repo-root", str(root), "--", "cargo", "clean"],
                cwd=root, env=environment, capture_output=True, text=True,
            )
            self.assertEqual(0, cleaned.returncode, cleaned.stderr)
            self.assertFalse(configured_sentinel.exists(), "guard did not clean configured target")
            self.assertTrue(default_sentinel.exists(), "guard cleaned the unrelated default target")

    def test_nested_cargo_clean_uses_nested_target_directory(self):
        for configured in (True, False):
            with self.subTest(configured_target=configured), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                (root / "Cargo.toml").write_text(
                    '[package]\nname="parent-clean-smoke"\nversion="0.0.0"\nedition="2021"\n'
                )
                parent_target = root / "target"
                parent_target.mkdir()
                (parent_target / "CACHEDIR.TAG").write_text(CACHEDIR_TAG)
                parent_sentinel = parent_target / "parent-sentinel"
                parent_sentinel.write_text("preserve parent target\n")

                nested = root / "nested"
                nested.mkdir()
                (nested / "Cargo.toml").write_text(
                    '[package]\nname="nested-clean-smoke"\nversion="0.0.0"\nedition="2021"\n\n'
                    '[workspace]\n'
                )
                (nested / "src").mkdir()
                (nested / "src/main.rs").write_text("fn main() {}\n")
                if configured:
                    cargo_config = nested / ".cargo"
                    cargo_config.mkdir()
                    (cargo_config / "config.toml").write_text(
                        '[build]\ntarget-dir = "build-cache"\n'
                    )
                    nested_target = nested / "build-cache"
                else:
                    nested_target = nested / "target"

                environment = os.environ.copy()
                for name in ("CARGO_TARGET_DIR", "CARGO_BUILD_TARGET_DIR", "CARGO_BUILD_BUILD_DIR"):
                    environment.pop(name, None)
                environment["CARGO_HOME"] = str(root / "cargo-home")
                build = subprocess.run(
                    ["cargo", "--offline", "build"],
                    cwd=nested, env=environment, capture_output=True, text=True,
                )
                self.assertEqual(0, build.returncode, build.stderr)
                self.assertTrue(nested_target.is_dir(), "Cargo did not use the nested target directory")
                (nested_target / "CACHEDIR.TAG").write_text(CACHEDIR_TAG)
                nested_sentinel = nested_target / "nested-sentinel"
                nested_sentinel.write_text("remove nested target\n")

                cleaned = subprocess.run(
                    [sys.executable, str(GUARD), "--repo-root", str(root), "--", "cargo", "clean"],
                    cwd=nested, env=environment, capture_output=True, text=True,
                )
                self.assertEqual(0, cleaned.returncode, cleaned.stderr)
                self.assertFalse(nested_sentinel.exists(), "guard did not clean nested Cargo target")
                self.assertTrue(parent_sentinel.exists(), "guard cleaned the parent repository target")

    def _run_fixture(self, root, target, cargo_arguments):
        manifest = root / "Cargo.toml"
        manifest.write_text(
            '[package]\nname="config-cache-smoke"\nversion="0.0.0"\nedition="2021"\n'
        )
        (root / "src").mkdir()
        (root / "src/main.rs").write_text("fn main() {}\n")
        environment = os.environ.copy()
        for name in ("CARGO_TARGET_DIR", "CARGO_HOME", "CARGO_BUILD_TARGET_DIR", "CARGO_BUILD_BUILD_DIR"):
            environment.pop(name, None)
        environment["CARGO_HOME"] = str(root / "cargo-home")
        build = subprocess.run(
            ["cargo", "--offline", *cargo_arguments, "build", "--manifest-path", str(manifest)],
            cwd=root, env=environment, capture_output=True, text=True,
        )
        self.assertEqual(0, build.returncode, build.stderr)
        self.assertTrue(target.is_dir(), "Cargo did not use the configured target directory")
        sentinel = target / "sentinel"
        sentinel.write_text("owned cache\n")
        (target / "CACHEDIR.TAG").write_text(CACHEDIR_TAG)
        guarded = subprocess.run(
            [sys.executable, str(GUARD), "--repo-root", str(root), "--max-gib", "0.000000001",
             "--", "cargo", "--offline", *cargo_arguments, "build", "--manifest-path", str(manifest)],
            cwd=root, env=environment, capture_output=True, text=True,
        )
        self.assertEqual(0, guarded.returncode, guarded.stderr)
        self.assertFalse(sentinel.exists(), "guard left the configured target sentinel")


if __name__ == "__main__":
    unittest.main()
