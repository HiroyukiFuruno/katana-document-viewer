import shutil
import os
import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]


class BuildCacheIntegrationTests(unittest.TestCase):
    def test_storybook_gate_accepts_guard_and_original_cargo(self):
        for cargo in (None, "cargo"):
            environment = os.environ.copy()
            environment.pop("JUST", None)
            environment.pop("CARGO", None)
            if cargo is not None:
                environment["CARGO"] = cargo
            result = subprocess.run(["bash", "scripts/check-storybook-entrypoint.sh"],
                                    cwd=ROOT, env=environment, capture_output=True, text=True)
            self.assertEqual(0, result.returncode, result.stderr)
            self.assertIn("storybook-entrypoint-check: ok", result.stdout)
        environment["CARGO"] = "unrecognized-wrapper"
        result = subprocess.run(["bash", "scripts/check-storybook-entrypoint.sh"],
                                cwd=ROOT, env=environment, capture_output=True, text=True)
        self.assertNotEqual(0, result.returncode)
        self.assertIn("boundary evidence gate", result.stderr)

    def test_default_cargo_wrapper_runs_in_checkout_with_spaces(self):
        cargo_line = next(line for line in (ROOT / "Justfile").read_text().splitlines()
                          if line.startswith("CARGO :="))
        with tempfile.TemporaryDirectory(prefix="cache guard ") as directory:
            root = Path(directory).resolve()
            scripts = root / "scripts" / "maintenance"
            scripts.mkdir(parents=True)
            for name in ("cargo-guard", "guard-build-cache.py"):
                shutil.copy2(ROOT / "scripts" / "maintenance" / name, scripts / name)
            recipe = 'set shell := ["bash", "-uc"]\n' + cargo_line + '\nprobe:\n    {{CARGO}} --version\n'
            recipe += "    python3 -c 'from pathlib import Path; print(Path.cwd())'\n"
            (root / "Justfile").write_text(recipe)
            environment = os.environ.copy()
            for name in ("CARGO", "CARGO_TARGET_DIR", "KDV_BUILD_CACHE_MAX_GIB"):
                environment.pop(name, None)
            nested = root / "nested"
            nested.mkdir()
            for cwd in (root, nested):
                result = subprocess.run(["just", "--justfile", str(root / "Justfile"), "probe"],
                                        check=False, capture_output=True, text=True, env=environment, cwd=cwd)
                self.assertEqual(0, result.returncode, result.stderr)
                self.assertIn("cargo ", result.stdout)
                self.assertIn(str(root), result.stdout)


if __name__ == "__main__":
    unittest.main()
