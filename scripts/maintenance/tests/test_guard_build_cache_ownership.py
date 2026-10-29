import tempfile
import unittest
from pathlib import Path
import sys


SCRIPT_DIR = Path(__file__).parents[1]
sys.path.insert(0, str(SCRIPT_DIR))
from target_ownership import reject_source_target


ROOT = Path(__file__).resolve().parents[3]


class TargetOwnershipTests(unittest.TestCase):
    def test_git_metadata_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / ".git").mkdir()
            target = root / ".git" / "target"
            target.mkdir()
            with self.assertRaises(ValueError):
                reject_source_target(root, target)

    def test_real_git_root_rejects_tracked_source(self):
        with self.assertRaises(ValueError):
            reject_source_target(ROOT, ROOT / "scripts" / "maintenance")

    def test_nongit_source_and_manifest_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "src"
            source.mkdir()
            with self.assertRaises(ValueError):
                reject_source_target(root, source)
            target = root / "build-cache"
            target.mkdir()
            (target / "Cargo.toml").write_text("[package]\nname='fixture'\n")
            with self.assertRaises(ValueError):
                reject_source_target(root, target)

    def test_arbitrary_nongit_target_is_allowed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = root / "build-cache"
            target.mkdir()
            reject_source_target(root, target)


if __name__ == "__main__":
    unittest.main()
