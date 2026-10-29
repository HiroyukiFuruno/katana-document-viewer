import errno
import importlib.util
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


ROOT = Path(__file__).resolve().parents[3]
SCRIPT = ROOT / "scripts" / "maintenance" / "guard-build-cache.py"
sys.path.insert(0, str(SCRIPT.parent))
SPEC = importlib.util.spec_from_file_location("guard_build_cache_maintenance", SCRIPT)
guard = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(guard)


def owned_target(root: Path) -> Path:
    target = root / "target"
    target.mkdir()
    (target / "CACHEDIR.TAG").write_text(guard.CACHEDIR_TAG + "\n", encoding="utf-8")
    (target / "artifact").write_text("cache\n", encoding="utf-8")
    return target


def release_child(child: subprocess.Popen) -> None:
    if child.poll() is None:
        child.stdin.write("\n")
        child.stdin.flush()
        child.stdin.close()
    child.wait(timeout=5)


@unittest.skipIf(guard.fcntl is None, "requires POSIX file locking")
class MaintenanceLockTests(unittest.TestCase):
    def test_child_holding_os_lock_rejects_maintenance_and_preserves_target(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = owned_target(root)
            lock_path = root / "tmp" / "maintenance" / "build-cache.lock"
            lock_path.parent.mkdir(parents=True)
            child_code = (
                "import fcntl,sys; lock=open(sys.argv[1],'a+'); "
                "fcntl.flock(lock.fileno(),fcntl.LOCK_EX); print('READY',flush=True); input()"
            )
            with subprocess.Popen([sys.executable, "-c", child_code, str(lock_path)],
                                  stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True) as child:
                try:
                    self.assertEqual("READY\n", child.stdout.readline())
                    marker = root / "ran"
                    result = guard.guard(root, [target], [sys.executable, "-c", f"open({str(marker)!r},'w')"], 1,
                                         maintenance=True)
                    self.assertEqual(3, result)
                    self.assertFalse(marker.exists())
                    self.assertTrue((target / "artifact").exists())
                    for subcommand in ("clean", "sweep"):
                        self.assertEqual(3, guard.main([
                            "--repo-root", str(root), "--", "cargo", "+stable", subcommand,
                        ]))
                finally:
                    release_child(child)
    def test_idle_maintenance_holds_lock_during_command_and_propagates_status(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            target = owned_target(root)
            lock_path = root / "tmp" / "maintenance" / "build-cache.lock"
            marker = root / "lock-state"
            child_code = (
                "import fcntl,sys; lock=open(sys.argv[1],'a+'); "
                "state='busy'; "
                "\ntry: fcntl.flock(lock.fileno(),fcntl.LOCK_EX|fcntl.LOCK_NB); state='free'\n"
                "except BlockingIOError: pass\nopen(sys.argv[2],'w').write(state); sys.exit(7)"
            )
            result = guard.guard(root, [target], [sys.executable, "-c", child_code, str(lock_path), str(marker)], 1,
                                 maintenance=True)
            self.assertEqual(7, result)
            self.assertEqual("busy", marker.read_text(encoding="utf-8"))


class MaintenanceProbeTests(unittest.TestCase):
    def test_lsof_output_remains_active_with_partial_failure_status(self):
        result = subprocess.CompletedProcess([], 1, "COMMAND PID NAME\npython 123 artifact\n", "")
        with tempfile.TemporaryDirectory() as directory:
            target = Path(directory)
            with mock.patch.object(guard.os, "name", "posix"):
                with mock.patch.object(guard.subprocess, "run", return_value=result):
                    self.assertTrue(guard.active_build([target]))

    @unittest.skipIf(guard.fcntl is None or os.name == "nt", "requires POSIX lsof")
    def test_open_target_file_rejects_maintenance_and_preserves_target(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            target = (root / "target").resolve()
            target.mkdir()
            (target / "CACHEDIR.TAG").write_text(guard.CACHEDIR_TAG + "\n", encoding="utf-8")
            artifact = target / "open-artifact"
            artifact.write_text("held\n", encoding="utf-8")
            code = "import sys; f=open(sys.argv[1]); print('READY',flush=True); input()"
            with subprocess.Popen([sys.executable, "-c", code, str(artifact)],
                                  stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True) as child:
                try:
                    self.assertEqual("READY\n", child.stdout.readline())
                    marker = root / "ran"
                    result = guard.guard(root, [target],
                                         [sys.executable, "-c", f"open({str(marker)!r},'w')"], 1,
                                         maintenance=True)
                    self.assertEqual(3, result)
                    self.assertFalse(marker.exists())
                    self.assertTrue(artifact.exists())
                finally:
                    release_child(child)
    def test_windows_nonblocking_busy_and_unknown_errno(self):
        api = mock.Mock()
        api.locking.side_effect = [OSError(errno.EACCES, "busy"), OSError(errno.EIO, "unknown")]
        with tempfile.TemporaryFile(mode="w+") as lock:
            with mock.patch.object(guard, "fcntl", None), mock.patch.object(guard, "msvcrt", api, create=True):
                self.assertFalse(guard.try_acquire_lock(lock))
                with self.assertRaises(OSError) as error:
                    guard.try_acquire_lock(lock)
                self.assertEqual(errno.EIO, error.exception.errno)


class MaintenanceContractTests(unittest.TestCase):
    def test_cargo_subcommand_and_maintenance_recipe_contract(self):
        self.assertEqual("clean", guard.cargo_subcommand(["cargo", "+stable", "clean"]))
        self.assertEqual("sweep", guard.cargo_subcommand(["cargo", "sweep", "--time", "7"]))
        justfile = (ROOT / "Justfile").read_text(encoding="utf-8")
        self.assertNotIn("clean: sweep", justfile)
        sweep = justfile.split("sweep:\n", 1)[1].split("\n\n", 1)[0]
        self.assertIn("{{CARGO}}", sweep)
        self.assertNotIn("|| true", sweep)


if __name__ == "__main__":
    unittest.main()
