import importlib.util
import os
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
SCRIPT = ROOT / "scripts" / "maintenance" / "guard-build-cache.py"
sys.path.insert(0, str(SCRIPT.parent))
SPEC = importlib.util.spec_from_file_location("guard_build_cache_semver", SCRIPT)
guard = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(guard)


def fixture(root: Path) -> Path:
    (root / "Cargo.toml").write_text(
        '[package]\nname="semver-cache-fixture"\nversion="0.0.0"\nedition="2021"\n',
        encoding="utf-8",
    )
    (root / "src").mkdir()
    (root / "src/main.rs").write_text("fn main() {}\n", encoding="utf-8")
    target = root / "target"
    target.mkdir()
    (target / "CACHEDIR.TAG").write_text(guard.CACHEDIR_TAG + "\n", encoding="utf-8")
    (target / "old-artifact").write_bytes(b"x" * 8)
    return target


def nested_output(root: Path, target: Path) -> list[str]:
    return [
        "cargo",
        "build",
        "--manifest-path",
        str(root / "Cargo.toml"),
        "--target-dir",
        str(target / "semver-checks" / "target"),
        "--offline",
        "-j1",
    ]


class SemverTargetOwnershipTests(unittest.TestCase):
    def test_cleanup_reclaims_root_tag_before_nested_semver_output(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = fixture(root)
            command = nested_output(root, target)
            first = guard.guard(root, [target], command, 1)
            self.assertEqual(0, first)
            self.assertTrue((target / "CACHEDIR.TAG").is_file())
            binary_name = "semver-cache-fixture.exe" if os.name == "nt" else "semver-cache-fixture"
            binary = target / "semver-checks" / "target" / "debug" / binary_name
            self.assertTrue(binary.is_file())
            second = guard.guard(root, [target], command, 1)
            self.assertEqual(0, second)
            self.assertTrue((target / "CACHEDIR.TAG").is_file())
            self.assertTrue(binary.is_file())


if __name__ == "__main__":
    unittest.main()
