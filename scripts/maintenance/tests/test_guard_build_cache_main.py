import importlib.util
import os
import sys
import tempfile
import unittest
from contextlib import redirect_stderr
from io import StringIO
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[3]
GUARD = ROOT / "scripts" / "maintenance" / "guard-build-cache.py"
sys.path.insert(0, str(GUARD.parent))
SPEC = importlib.util.spec_from_file_location("guard_build_cache_main_unit", GUARD)
guard = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(guard)


class BuildCacheMainUnitTests(unittest.TestCase):
    def test_main_resolves_relative_environment_target_from_cargo_directory(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            expected = root / "nested" / "env-cache"
            observed = {}

            def capture_targets(repo_root, explicit):
                observed["repo_root"] = repo_root
                observed["explicit"] = list(explicit)
                return []

            with patch.dict(os.environ, {"CARGO_TARGET_DIR": "env-cache"}):
                with patch.object(guard, "repo_targets", side_effect=capture_targets):
                    with patch.object(guard, "should_guard", return_value=False):
                        with patch.object(
                            guard.subprocess,
                            "run",
                            return_value=SimpleNamespace(returncode=0),
                        ):
                            result = guard.main([
                                "--repo-root", str(root), "--", "cargo", "+nightly",
                                "-Z", "unstable-options", "-C", str(root / "nested"), "build",
                            ])

            self.assertEqual(0, result)
            self.assertEqual(root, observed["repo_root"])
            self.assertEqual([expected], observed["explicit"])

    def test_main_rejects_target_outside_repository_after_cargo_directory_change(self):
        with tempfile.TemporaryDirectory() as root_directory:
            with tempfile.TemporaryDirectory() as outside_directory:
                root = Path(root_directory).resolve()
                outside = Path(outside_directory).resolve()
                stderr = StringIO()
                with patch.dict(os.environ, {"CARGO_TARGET_DIR": ""}):
                    with redirect_stderr(stderr):
                        result = guard.main([
                            "--repo-root", str(root), "--", "cargo", "+nightly",
                            "-Z", "unstable-options", "-C", str(outside), "build",
                            "--target-dir", "target",
                        ])

                self.assertEqual(2, result)
                self.assertIn("target is outside the repository scope", stderr.getvalue())


if __name__ == "__main__":
    unittest.main()
