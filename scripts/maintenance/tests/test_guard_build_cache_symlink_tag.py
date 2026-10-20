import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
SCRIPT = ROOT / "scripts" / "maintenance" / "guard-build-cache.py"
sys.path.insert(0, str(SCRIPT.parent))
SPEC = importlib.util.spec_from_file_location("guard_build_cache_symlink_tag", SCRIPT)
guard = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(guard)


def symlink_fixture(root: Path) -> tuple[Path, Path, Path]:
    manifest = root / "Cargo.toml"
    manifest.write_text('[package]\nname="symlink-cache-fixture"\nversion="0.0.0"\nedition="2021"\n')
    (root / "src").mkdir()
    (root / "src/main.rs").write_text("fn main() {}\n")
    target = root / "target"
    target.mkdir()
    external = root.parent / "external-CACHEDIR.TAG"
    external.write_text(guard.CACHEDIR_TAG + "\n")
    (target / "CACHEDIR.TAG").symlink_to(external)
    unrelated = target / "unrelated-output"
    unrelated.write_bytes(b"preserve this output")
    return manifest, target, external


class SymlinkTagOwnershipTests(unittest.TestCase):
    def test_threshold_cleanup_rejects_external_tag_and_preserves_contents(self):
        with tempfile.TemporaryDirectory() as directory:
            root = (Path(directory) / "repo").resolve()
            root.mkdir()
            manifest, target, external = symlink_fixture(root)
            result = guard.main([
                "--repo-root", str(root), "--max-gib", "0.000000001", "--",
                "cargo", "build", "--manifest-path", str(manifest), "--offline", "-j1",
            ])
            self.assertEqual(2, result)
            self.assertTrue((target / "CACHEDIR.TAG").is_symlink())
            self.assertTrue((target / "unrelated-output").is_file())
            self.assertEqual(b"preserve this output", (target / "unrelated-output").read_bytes())
            self.assertEqual(guard.CACHEDIR_TAG + "\n", external.read_text())

    def test_maintenance_rejects_external_tag_and_preserves_contents(self):
        with tempfile.TemporaryDirectory() as directory:
            root = (Path(directory) / "repo").resolve()
            root.mkdir()
            manifest, target, external = symlink_fixture(root)
            result = guard.main([
                "--repo-root", str(root), "--max-gib", "1", "--",
                "cargo", "clean", "--manifest-path", str(manifest),
            ])
            self.assertEqual(2, result)
            self.assertTrue((target / "CACHEDIR.TAG").is_symlink())
            self.assertTrue((target / "unrelated-output").is_file())
            self.assertEqual(b"preserve this output", (target / "unrelated-output").read_bytes())
            self.assertEqual(guard.CACHEDIR_TAG + "\n", external.read_text())


if __name__ == "__main__":
    unittest.main()
