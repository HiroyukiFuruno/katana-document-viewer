import importlib.util
import os
import sys
import tempfile
import unittest
from unittest.mock import patch
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
    external.write_bytes((guard.CACHEDIR_TAG + "\n").encode())
    (target / "CACHEDIR.TAG").symlink_to(external)
    unrelated = target / "unrelated-output"
    unrelated.write_bytes(b"preserve this output")
    return manifest, target, external


class SymlinkTagOwnershipTests(unittest.TestCase):
    def test_replacement_before_open_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = (Path(directory) / "repo").resolve()
            root.mkdir()
            manifest, target, external = symlink_fixture(root)
            tag = target / "CACHEDIR.TAG"
            tag.unlink()
            tag.write_text(guard.CACHEDIR_TAG + "\n")
            real_lstat = Path.lstat
            replaced = False

            def replace_after_lstat(path):
                nonlocal replaced
                result = real_lstat(path)
                if path == tag and not replaced:
                    replaced = True
                    tag.unlink()
                    tag.symlink_to(external)
                return result

            with patch.object(Path, "lstat", replace_after_lstat):
                result = guard.main([
                    "--repo-root", str(root), "--max-gib", "1", "--",
                    "cargo", "clean",
                ])
            self.assertEqual(2, result)
            self.assertTrue(tag.is_symlink())
            self.assertEqual(b"preserve this output", (target / "unrelated-output").read_bytes())
            self.assertEqual((guard.CACHEDIR_TAG + "\n").encode(), external.read_bytes())

    @unittest.skipUnless(os.name != "nt", "POSIX FIFO required")
    def test_fifo_replaced_after_lstat_is_nonblocking(self):
        with tempfile.TemporaryDirectory() as directory:
            root = (Path(directory) / "repo").resolve()
            root.mkdir()
            _, target, _ = symlink_fixture(root)
            tag = target / "CACHEDIR.TAG"
            tag.unlink()
            tag.write_text(guard.CACHEDIR_TAG + "\n")
            real_lstat = Path.lstat
            real_open = guard.os.open
            flags_seen = []
            def replace_and_check(path, flags, mode=0o777):
                self.assertTrue(flags & os.O_NONBLOCK)
                flags_seen.append(flags)
                return real_open(path, flags, mode)
            def replace_after_lstat(path):
                result = real_lstat(path)
                if path == tag:
                    tag.unlink()
                    os.mkfifo(tag)
                return result
            with patch.object(Path, "lstat", replace_after_lstat), patch.object(guard.os, "open", replace_and_check):
                self.assertFalse(guard.owned_target(target))
            self.assertTrue(flags_seen[0] & os.O_NONBLOCK)
            self.assertTrue(tag.is_fifo())

    @unittest.skipUnless(os.name != "nt", "POSIX O_NOFOLLOW required")
    def test_without_nofollow_fails_closed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = (Path(directory) / "repo").resolve()
            root.mkdir()
            _, target, external = symlink_fixture(root)
            tag = target / "CACHEDIR.TAG"
            tag.unlink()
            tag.write_text(guard.CACHEDIR_TAG + "\n")
            with patch.object(guard.os, "O_NOFOLLOW", 0, create=True):
                self.assertFalse(guard.owned_target(target))
                real_lstat = Path.lstat

                def replace_after_lstat(path):
                    result = real_lstat(path)
                    if path == tag:
                        tag.unlink()
                        tag.symlink_to(external)
                    return result

                with patch.object(Path, "lstat", replace_after_lstat):
                    self.assertFalse(guard.owned_target(target))
            self.assertTrue(tag.is_symlink())
            self.assertEqual(b"preserve this output", (target / "unrelated-output").read_bytes())

    @unittest.skipUnless(os.name == "nt", "Windows native handle required")
    def test_windows_regular_tag_is_accepted(self):
        with tempfile.TemporaryDirectory() as directory:
            target = Path(directory) / "target"
            target.mkdir()
            (target / "CACHEDIR.TAG").write_bytes((guard.CACHEDIR_TAG + "\n").encode())
            self.assertTrue(guard.owned_target(target))

    def test_rename_then_same_inode_symlink_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = (Path(directory) / "repo").resolve()
            root.mkdir()
            _, target, external = symlink_fixture(root)
            tag = target / "CACHEDIR.TAG"
            tag.unlink()
            tag.write_text(guard.CACHEDIR_TAG + "\n")
            renamed = target / "renamed-tag"
            real_lstat = Path.lstat
            replaced = False
            def replace_with_same_inode(path):
                nonlocal replaced
                result = real_lstat(path)
                if path == tag and not replaced:
                    replaced = True
                    tag.rename(renamed)
                    tag.symlink_to(renamed)
                return result
            with patch.object(Path, "lstat", replace_with_same_inode):
                self.assertFalse(guard.owned_target(target))
            self.assertEqual(guard.CACHEDIR_TAG + "\n", renamed.read_text())
            self.assertEqual((guard.CACHEDIR_TAG + "\n").encode(), external.read_bytes())

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
            self.assertEqual((guard.CACHEDIR_TAG + "\n").encode(), external.read_bytes())

    def test_maintenance_rejects_external_tag_and_preserves_contents(self):
        with tempfile.TemporaryDirectory() as directory:
            root = (Path(directory) / "repo").resolve()
            root.mkdir()
            manifest, target, external = symlink_fixture(root)
            result = guard.main([
                "--repo-root", str(root), "--max-gib", "1", "--",
                "cargo", "clean",
            ])
            self.assertEqual(2, result)
            self.assertTrue((target / "CACHEDIR.TAG").is_symlink())
            self.assertTrue((target / "unrelated-output").is_file())
            self.assertEqual(b"preserve this output", (target / "unrelated-output").read_bytes())
            self.assertEqual((guard.CACHEDIR_TAG + "\n").encode(), external.read_bytes())


if __name__ == "__main__":
    unittest.main()
