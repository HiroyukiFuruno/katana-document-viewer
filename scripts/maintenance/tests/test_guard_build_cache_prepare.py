import tempfile
import unittest
from pathlib import Path

from test_guard_build_cache import guard
from target_ownership import prepare_empty_target


class EmptyTargetOwnershipTests(unittest.TestCase):
    def test_new_and_empty_targets_get_ownership_tags(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name in ("new", "empty"):
                target = root / name
                if name == "empty":
                    target.mkdir()
                prepare_empty_target(root, target)
                self.assertTrue(guard.owned_target(target))

    def test_nonempty_unowned_directory_is_never_claimed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = root / "target"
            target.mkdir()
            (target / "unrelated").write_text("keep", encoding="utf-8")
            prepare_empty_target(root, target)
            self.assertFalse(guard.owned_target(target))
            self.assertEqual("keep", (target / "unrelated").read_text(encoding="utf-8"))
