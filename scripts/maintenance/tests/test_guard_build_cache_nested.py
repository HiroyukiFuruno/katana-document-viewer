import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parents[1]))
import importlib.util

SPEC = importlib.util.spec_from_file_location("guard_build_cache_nested", Path(__file__).parents[1] / "guard-build-cache.py")
guard = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(guard)


class NestedTargetSelectionTests(unittest.TestCase):
    def test_implicit_untagged_parent_is_omitted_but_explicit_parent_is_kept(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            parent = root / "target"
            child = parent / "custom"
            child.mkdir(parents=True)
            (child / "CACHEDIR.TAG").write_text(guard.CACHEDIR_TAG + "\n")
            self.assertEqual([child.resolve()], guard.repo_targets(root, [child]))
            self.assertEqual([parent.resolve(), child.resolve()], guard.repo_targets(root, [parent, child]))


if __name__ == "__main__":
    unittest.main()
