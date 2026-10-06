import importlib.util
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


SCRIPT = Path(__file__).parents[1] / "guard-build-cache.py"
sys.path.insert(0, str(SCRIPT.parent))
SPEC = importlib.util.spec_from_file_location("guard_build_cache", SCRIPT)
guard = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(guard)


def owned_target(root: Path) -> Path:
    target = root / "target"
    target.mkdir()
    (target / "CACHEDIR.TAG").write_text(guard.CACHEDIR_TAG + "\n", encoding="utf-8")
    return target


class GuardBuildCacheTests(unittest.TestCase):
    def test_below_threshold_runs_without_cleanup(self):
        with tempfile.TemporaryDirectory() as directory:
            target = owned_target(Path(directory))
            (target / "artifact").write_bytes(b"x")
            with mock.patch.object(guard, "clean_targets") as clean:
                with mock.patch.object(guard, "active_build", return_value=False):
                    self.assertEqual(0, guard.guard(Path(directory), [target], [sys.executable, "-c", ""], 8192))
            clean.assert_not_called()

    def test_above_threshold_cleans_owned_target(self):
        with tempfile.TemporaryDirectory() as directory:
            target = owned_target(Path(directory))
            (target / "artifact").write_bytes(b"x" * 8)
            with mock.patch.object(guard, "clean_targets") as clean:
                with mock.patch.object(guard, "active_build", return_value=False):
                    self.assertEqual(0, guard.guard(Path(directory), [target], [sys.executable, "-c", ""], 1))
            clean.assert_called_once()

    def test_active_build_blocks_cleanup_and_command(self):
        with tempfile.TemporaryDirectory() as directory:
            target = owned_target(Path(directory))
            (target / "artifact").write_bytes(b"x" * 8)
            with mock.patch.object(guard, "active_build", return_value=True):
                with mock.patch.object(subprocess, "run") as run:
                    self.assertEqual(3, guard.guard(Path(directory), [target], [sys.executable, "-c", ""], 1))
            run.assert_not_called()

    def test_target_scope_rejects_sibling(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            with self.assertRaises(ValueError):
                guard.repo_targets(root, [root.parent / "sibling" / "target"])
            with self.assertRaises(ValueError):
                guard.repo_targets(root, [root])
            with tempfile.TemporaryDirectory() as outside:
                link = root / "external-link"
                link.symlink_to(Path(outside), target_is_directory=True)
                with self.assertRaises(ValueError):
                    guard.repo_targets(root, [link])

    def test_invalid_threshold_is_rejected(self):
        with self.assertRaises(ValueError):
            guard.parse_max_gib("not-a-number")
        with self.assertRaises(ValueError):
            guard.parse_max_gib("nan")
        with self.assertRaises(ValueError):
            guard.parse_max_gib("0")

    def test_environment_threshold_is_used(self):
        with mock.patch.dict("os.environ", {"KDV_BUILD_CACHE_MAX_GIB": "2.5"}):
            self.assertEqual(int(2.5 * 1024**3), guard.parse_max_gib())

    def test_hardlinked_artifacts_are_counted_once(self):
        with tempfile.TemporaryDirectory() as directory:
            target = owned_target(Path(directory))
            artifact = target / "artifact"
            artifact.write_bytes(b"build artifact")
            before = guard.target_bytes(target)
            os.link(artifact, target / "alias")
            self.assertEqual(before, guard.target_bytes(target))

    def test_exact_threshold_does_not_clean(self):
        with tempfile.TemporaryDirectory() as directory:
            target = owned_target(Path(directory))
            size = guard.target_bytes(target)
            with mock.patch.object(guard, "clean_targets") as clean:
                self.assertEqual(0, guard.guard(Path(directory), [target], [sys.executable, "-c", ""], size))
            clean.assert_not_called()

    def test_missing_tag_is_allowed_below_but_blocks_cleanup_above(self):
        with tempfile.TemporaryDirectory() as directory:
            target = Path(directory) / "target"
            target.mkdir()
            (target / "artifact").write_bytes(b"x" * 8)
            command = [sys.executable, "-c", ""]
            self.assertEqual(0, guard.guard(Path(directory), [target], command, 8192))
            with mock.patch.object(guard, "active_build", return_value=False):
                self.assertEqual(2, guard.guard(Path(directory), [target], command, 1))

    def test_parent_and_child_targets_are_deduplicated(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            parent = owned_target(root)
            child = parent / "nested" / "target"
            child.mkdir(parents=True)
            self.assertEqual([parent.resolve()], guard.repo_targets(root, [child, parent]))
            custom = root / "build-cache"
            custom.mkdir()
            self.assertIn(custom.resolve(), guard.repo_targets(root, [custom]))

    def test_probe_errors_fail_closed(self):
        with tempfile.TemporaryDirectory() as directory:
            target = owned_target(Path(directory))
            result = subprocess.CompletedProcess([], 1, "", "probe failed")
            with mock.patch.object(subprocess, "run", return_value=result):
                self.assertTrue(guard.active_build([target]))

    def test_windows_process_probe_checks_tasklist(self):
        result = subprocess.CompletedProcess([], 0, '"cargo.exe","123"\n', "")
        with mock.patch.object(guard.os, "name", "nt"):
            with mock.patch.object(subprocess, "run", return_value=result):
                self.assertTrue(guard.active_build([]))

    def test_unrelated_cargo_command_is_not_guarded(self):
        self.assertFalse(guard.should_guard(["cargo", "fmt"]))
        self.assertTrue(guard.should_guard(["cargo", "test", "--workspace"]))
        self.assertTrue(guard.should_guard(["cargo", "semver-checks"]))

    def test_toolchain_and_global_options_are_guarded(self):
        self.assertTrue(guard.should_guard(["cargo", "+stable", "test"]))
        self.assertTrue(guard.should_guard(["cargo", "--color", "always", "build"]))
        self.assertTrue(guard.should_guard(["cargo", "--config=net.offline=true", "check"]))
        self.assertFalse(guard.should_guard(["cargo", "+stable", "fmt"]))

    def test_wrapped_command_exit_status_is_preserved(self):
        with tempfile.TemporaryDirectory() as directory:
            target = owned_target(Path(directory))
            self.assertEqual(
                7,
                guard.guard(
                    Path(directory),
                    [target],
                    [sys.executable, "-c", "import sys; sys.exit(7)"],
                    8192,
                ),
            )

    def test_real_cargo_cleanup_removes_only_owned_target(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "Cargo.toml").write_text(
                '[package]\nname="cache-guard-smoke"\nversion="0.0.0"\nedition="2021"\n'
            )
            (root / "src").mkdir()
            source = root / "src" / "main.rs"
            source.write_text("fn main() {}\n")
            target = owned_target(root)
            (target / "old-artifact").write_bytes(b"old build")
            code = "import sys; from pathlib import Path; assert not Path(sys.argv[1]).exists()"
            result = guard.guard(
                root, [target], [sys.executable, "-c", code, str(target / "old-artifact")], 1
            )
            self.assertEqual(0, result)
            self.assertTrue(source.exists())

    @unittest.skipIf(guard.fcntl is None, "Unix flock contract")
    def test_lock_is_held_until_wrapped_command_finishes(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = owned_target(root)
            child_code = (
                "import importlib.util,sys; from pathlib import Path; sys.path.insert(0,str(Path(sys.argv[1]).parent)); "
                "s=importlib.util.spec_from_file_location('guard',sys.argv[1]); "
                "g=importlib.util.module_from_spec(s); s.loader.exec_module(g); "
                "g.guard(Path(sys.argv[2]),[Path(sys.argv[2])/'target'],"
                "[sys.executable,'-c',\"print('READY',flush=True); input()\"],100000)"
            )
            with subprocess.Popen(
                [sys.executable, "-c", child_code, str(SCRIPT.resolve()), str(root)],
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True,
            ) as child:
                try:
                    self.assertEqual("READY\n", child.stdout.readline())
                    with (root / "tmp/maintenance/build-cache.lock").open("a+") as lock:
                        with self.assertRaises(BlockingIOError):
                            guard.fcntl.flock(lock.fileno(), guard.fcntl.LOCK_EX | guard.fcntl.LOCK_NB)
                finally:
                    child.stdin.write("\n")
                    child.stdin.flush()
                    child.wait(timeout=10)


if __name__ == "__main__":
    unittest.main()
