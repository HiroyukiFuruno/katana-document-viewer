import importlib.util
import shutil
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
GUARD = ROOT / "scripts" / "maintenance" / "guard-build-cache.py"
sys.path.insert(0, str(GUARD.parent))
SPEC = importlib.util.spec_from_file_location("guard_build_cache_integration", GUARD)
guard = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(guard)


def bash_executable():
    # WindowsのWSL起動用bashではなく、既存CIと同じGit Bashを使う。
    if os.name == "nt":
        for variable in ("ProgramFiles", "ProgramFiles(x86)"):
            directory = os.environ.get(variable)
            if directory:
                candidate = Path(directory) / "Git" / "bin" / "bash.exe"
                if candidate.is_file():
                    return str(candidate)
    executable = shutil.which("bash")
    if executable is None:
        raise FileNotFoundError("bash executable is required")
    return executable


class BuildCacheIntegrationTests(unittest.TestCase):
    def test_nongit_source_target_is_rejected_before_cleanup(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "Cargo.toml").write_text(
                '[package]\nname="source-guard-smoke"\nversion="0.0.0"\nedition="2021"\n'
            )
            source = root / "src"
            source.mkdir()
            sentinel = source / "source.rs"
            sentinel.write_text("fn main() {}\n")
            (source / "CACHEDIR.TAG").write_text(
                "Signature: 8a477f597d28d172789f06886806bc55\n"
            )
            result = subprocess.run(
                [sys.executable, str(GUARD), "--repo-root", str(root), "--target-dir", str(source), "--", "cargo", "build"],
                cwd=root, capture_output=True, text=True,
            )
            self.assertEqual(2, result.returncode)
            self.assertTrue(sentinel.exists())

    def test_wrapped_cargo_measures_separated_and_equals_target_dirs(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            manifest = root / "Cargo.toml"
            manifest.write_text(
                '[package]\nname="cache-guard-smoke"\nversion="0.0.0"\nedition="2021"\n'
            )
            (root / "src").mkdir()
            (root / "src/main.rs").write_text("fn main() {}\n")
            for name, option in (("separated", "--target-dir"), ("equals", "--target-dir=")):
                target = root / f"build-cache-{name}"
                built = subprocess.run(
                    ["cargo", "build", "--manifest-path", str(manifest), "--target-dir", str(target)],
                    cwd=root, capture_output=True, text=True,
                )
                self.assertEqual(0, built.returncode, built.stderr)
                sentinel = target / "sentinel"
                sentinel.write_text("owned cache\n")
                (target / "CACHEDIR.TAG").write_text(
                    "Signature: 8a477f597d28d172789f06886806bc55\n"
                )
                target_arg = [option, str(target)] if option == "--target-dir" else [f"{option}{target}"]
                result = subprocess.run(
                    [sys.executable, str(GUARD), "--repo-root", str(root), "--max-gib", "0.000000001", "--", "cargo", "build", "--manifest-path", str(manifest), *target_arg],
                    cwd=root, capture_output=True, text=True,
                )
                self.assertEqual(0, result.returncode, result.stderr)
                self.assertFalse(sentinel.exists())

    def test_nested_explicit_target_is_cleaned_without_parent(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            manifest = root / "Cargo.toml"
            manifest.write_text('[package]\nname="nested-cache"\nversion="0.0.0"\nedition="2021"\n')
            (root / "src").mkdir()
            (root / "src/main.rs").write_text("fn main() {}\n")
            target = root / "target" / "custom"
            built = subprocess.run(["cargo", "build", "--manifest-path", str(manifest), "--target-dir", str(target)],
                                   cwd=root, capture_output=True, text=True)
            self.assertEqual(0, built.returncode, built.stderr)
            sentinel = root / "target" / "parent-sentinel"
            sentinel.write_text("keep\n")
            self.assertTrue((target / "CACHEDIR.TAG").is_file())
            self.assertFalse((target.parent / "CACHEDIR.TAG").exists())
            obsolete = target / "obsolete-artifact"
            obsolete.write_bytes(b"old")
            result = subprocess.run([sys.executable, str(GUARD), "--repo-root", str(root), "--max-gib", "0.000000001",
                                     "--", "cargo", "build", "--manifest-path", str(manifest), "--target-dir", str(target)],
                                    cwd=root, capture_output=True, text=True)
            self.assertEqual(0, result.returncode, result.stderr)
            self.assertTrue(sentinel.exists())
            self.assertFalse(obsolete.exists())

    def test_nested_targets_do_not_double_count_shared_filesystem(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "Cargo.toml").write_text('[package]\nname="dedup-cache"\nversion="0.0.0"\nedition="2021"\n')
            (root / "src").mkdir()
            (root / "src/main.rs").write_text("fn main() {}\n")
            parent = root / "target" / "parent"
            child = parent / "nested"
            child.mkdir(parents=True)
            for target in (parent, child):
                (target / "CACHEDIR.TAG").write_text(guard.CACHEDIR_TAG + "\n")
            sentinel = parent / "parent-sentinel"
            sentinel.write_bytes(b"sentinel")
            (child / "child-artifact").write_bytes(b"artifact")
            limit = guard.target_bytes(parent)
            result = guard.guard(root, [parent, child], [sys.executable, "-c", ""], limit)
            self.assertEqual(0, result)
            self.assertTrue(sentinel.exists())

    def test_storybook_gate_accepts_guard_and_original_cargo(self):
        for cargo in (None, "cargo"):
            environment = os.environ.copy()
            environment.pop("JUST", None)
            environment.pop("CARGO", None)
            if cargo is not None:
                environment["CARGO"] = cargo
            result = subprocess.run([bash_executable(), "scripts/check-storybook-entrypoint.sh"],
                                    cwd=ROOT, env=environment, capture_output=True, text=True, encoding="utf-8")
            self.assertEqual(0, result.returncode, result.stderr + result.stdout)
            self.assertIn("storybook-entrypoint-check: ok", result.stdout)
        environment["CARGO"] = "unrecognized-wrapper"
        result = subprocess.run([bash_executable(), "scripts/check-storybook-entrypoint.sh"],
                                cwd=ROOT, env=environment, capture_output=True, text=True, encoding="utf-8")
        self.assertNotEqual(0, result.returncode)
        self.assertIn("boundary evidence gate", result.stderr)

    def test_default_cargo_wrapper_runs_in_checkout_with_spaces(self):
        cargo_line = next(line for line in (ROOT / "Justfile").read_text(encoding="utf-8").splitlines()
                          if line.startswith("CARGO :="))
        with tempfile.TemporaryDirectory(prefix="cache guard ") as directory:
            root = Path(directory).resolve()
            scripts = root / "scripts" / "maintenance"
            scripts.mkdir(parents=True)
            for name in ("cargo-guard", "guard-build-cache.py", "cargo_target_args.py", "target_ownership.py"):
                shutil.copy2(ROOT / "scripts" / "maintenance" / name, scripts / name)
            recipe = 'set shell := ["bash", "-uc"]\n' + cargo_line + '\nprobe:\n    {{CARGO}} --version\n'
            recipe += "    python3 -c 'from pathlib import Path; print(Path.cwd())'\n"
            (root / "Justfile").write_text(recipe, encoding="utf-8")
            environment = os.environ.copy()
            for name in ("CARGO", "CARGO_TARGET_DIR", "KDV_BUILD_CACHE_MAX_GIB"):
                environment.pop(name, None)
            nested = root / "nested"
            nested.mkdir()
            for cwd in (root, nested):
                result = subprocess.run(["just", "--justfile", str(root / "Justfile"), "probe"],
                                        check=False, capture_output=True, text=True, encoding="utf-8", env=environment, cwd=cwd)
                self.assertEqual(0, result.returncode, result.stderr + result.stdout)
                self.assertIn("cargo ", result.stdout)
                self.assertIn(str(root), result.stdout)


if __name__ == "__main__":
    unittest.main()
