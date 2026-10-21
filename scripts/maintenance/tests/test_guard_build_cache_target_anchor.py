import importlib.util
import contextlib
import ctypes
import io
import os
import struct
from ctypes import wintypes
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[3]
SCRIPT = ROOT / "scripts/maintenance/guard-build-cache.py"
sys.path.insert(0, str(SCRIPT.parent))
SPEC = importlib.util.spec_from_file_location("guard_target_anchor", SCRIPT)
guard = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(guard)


def fixture(root):
    (root / "Cargo.toml").write_text('[package]\nname="anchor-fixture"\nversion="0.0.0"\n')
    target = root / "target"
    target.mkdir()
    (target / "CACHEDIR.TAG").write_text(guard.CACHEDIR_TAG + "\n")
    (target / "artifact").write_bytes(b"old")
    return target


def target_fixture(root, name):
    target = root / name
    target.mkdir()
    (target / "CACHEDIR.TAG").write_text(guard.CACHEDIR_TAG + "\n")
    (target / "sentinel").write_bytes(name.encode())
    return target


def clean_command(root, *arguments):
    return ["--repo-root", str(root), "--", "cargo", "clean", *arguments]


def mountpoint_buffer(external):
    substitute = ("\\??\\" + str(external)).encode("utf-16-le")
    payload = struct.pack("<HHHH", 0, len(substitute), len(substitute), 0) + substitute
    return struct.pack("<LHH", 0xA0000003, len(payload), 0) + payload


def device_io(kernel, handle, code, buffer):
    kernel.DeviceIoControl.restype = wintypes.BOOL
    kernel.DeviceIoControl.argtypes = [wintypes.HANDLE, wintypes.DWORD, ctypes.c_void_p,
                                       wintypes.DWORD, ctypes.c_void_p, wintypes.DWORD,
                                       ctypes.POINTER(wintypes.DWORD), ctypes.c_void_p]
    data = ctypes.create_string_buffer(buffer)
    returned = wintypes.DWORD()
    return kernel.DeviceIoControl(handle, code, data, len(buffer), None, 0,
                                  ctypes.byref(returned), None)


class TargetAnchorTests(unittest.TestCase):
    @unittest.skipIf(os.name == "nt", "requires POSIX process inspection")
    def test_missing_lsof_reports_required_tool_and_preserves_artifacts(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            target = fixture(root)
            errors = io.StringIO()
            with patch.dict(os.environ, {"PATH": ""}), contextlib.redirect_stderr(errors):
                result = guard.main(clean_command(root))
            self.assertEqual(3, result)
            self.assertIn("install lsof on POSIX", errors.getvalue())
            self.assertTrue((target / "artifact").is_file())
            self.assertTrue((target / "CACHEDIR.TAG").is_file())

    @unittest.skipIf(os.name == "nt", "requires POSIX directory descriptors")
    def test_posix_refuses_replaced_ancestor_before_opening_external_owned_target(self):
        from posix_target_cleanup import PosixTargetCleanup

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            parent = root / "parent"
            parent.mkdir()
            target = fixture(parent)
            external = root / "external"
            external.mkdir()
            external_target = fixture(external)
            parent.rename(root / "original-parent")
            parent.symlink_to(external, target_is_directory=True)
            with self.assertRaises(OSError):
                with PosixTargetCleanup(target, guard.CACHEDIR_TAG) as anchor:
                    anchor.purge()
            self.assertEqual(b"old", (external_target / "artifact").read_bytes())
            self.assertEqual(b"old", (root / "original-parent/target/artifact").read_bytes())

    @unittest.skipIf(os.name == "nt", "POSIX permits renaming open directories")
    def test_full_clean_uses_original_directory_after_target_replacement(self):
        from posix_target_cleanup import PosixTargetCleanup

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            target = fixture(root)
            original = root / "original-target"
            purge = PosixTargetCleanup.purge

            def replace_then_purge(anchor):
                target.rename(original)
                target.mkdir()
                (target / "unowned-payload").write_bytes(b"keep")
                purge(anchor)

            with patch.object(PosixTargetCleanup, "purge", replace_then_purge):
                result = guard.main(["--repo-root", str(root), "--", "cargo", "clean"])
            self.assertEqual(0, result)
            self.assertEqual(b"keep", (target / "unowned-payload").read_bytes())
            self.assertFalse((original / "artifact").exists())
            self.assertTrue((original / "CACHEDIR.TAG").is_file())

    def test_full_clean_removes_nested_readonly_output_and_preserves_external_link(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            target = fixture(root)
            external = root / "external"
            external.mkdir()
            (external / "payload").write_bytes(b"keep")
            nested = target / "nested"
            nested.mkdir()
            artifact = nested / "readonly"
            artifact.write_bytes(b"old")
            artifact.chmod(0o444)
            (target / "external-link").symlink_to(external, target_is_directory=True)
            result = guard.main(["--repo-root", str(root), "--", "cargo", "clean"])
            self.assertEqual(0, result)
            self.assertEqual(["CACHEDIR.TAG"], [p.name for p in target.iterdir()])
            self.assertEqual(b"keep", (external / "payload").read_bytes())

    def test_selective_clean_and_sweep_refuse_without_deleting_any_output(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            target = fixture(root)
            for arguments in (["clean", "-p", "anchor-fixture"], ["clean", "--release"],
                              ["clean", "--doc"], ["sweep", "--time", "7"]):
                self.assertEqual(2, guard.main(["--repo-root", str(root), "--", "cargo", *arguments]))
                self.assertEqual(b"old", (target / "artifact").read_bytes())

    def test_unsupported_clean_does_not_create_missing_target(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            (root / "Cargo.toml").write_text('[package]\nname="anchor-fixture"\nversion="0.0.0"\n')
            for arguments in (["clean", "-p", "anchor-fixture"], ["sweep", "--time", "7"]):
                self.assertEqual(2, guard.main(["--repo-root", str(root), "--", "cargo", *arguments]))
                self.assertFalse((root / "target").exists())

    def test_cli_target_dir_only_cleans_selected_target(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            manifest = root / "Cargo.toml"
            manifest.write_text('[package]\nname="anchor-fixture"\nversion="0.0.0"\n')
            default = target_fixture(root, "target")
            selected = target_fixture(root, "custom")
            result = guard.main(clean_command(root, "--target-dir", str(selected)))
            self.assertEqual(0, result)
            self.assertTrue((default / "sentinel").is_file())
            self.assertFalse((selected / "sentinel").exists())

    def test_coverage_refresh_preserves_default_target_and_both_ownership_tags(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            default = fixture(root)
            selected = target_fixture(default, "llvm-cov-target")
            default_tag = (default / "CACHEDIR.TAG").read_bytes()
            selected_tag = (selected / "CACHEDIR.TAG").read_bytes()
            result = guard.main(clean_command(root, "--target-dir", str(selected)))
            self.assertEqual(0, result)
            self.assertTrue((default / "artifact").is_file())
            self.assertEqual(default_tag, (default / "CACHEDIR.TAG").read_bytes())
            self.assertEqual(selected_tag, (selected / "CACHEDIR.TAG").read_bytes())
            self.assertFalse((selected / "sentinel").exists())

    def test_environment_target_and_cli_precedence_preserve_other_targets(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            manifest = root / "Cargo.toml"
            manifest.write_text('[package]\nname="anchor-fixture"\nversion="0.0.0"\n')
            default = target_fixture(root, "target")
            env_target = target_fixture(root, "env-target")
            cli_target = target_fixture(root, "cli-target")
            with patch.dict(os.environ, {"CARGO_TARGET_DIR": str(env_target)}):
                result = guard.main(clean_command(root, "--target-dir", str(cli_target)))
            self.assertEqual(0, result)
            self.assertTrue((default / "sentinel").is_file())
            self.assertTrue((env_target / "sentinel").is_file())
            self.assertFalse((cli_target / "sentinel").exists())

    def test_environment_only_cleans_environment_target(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            default = fixture(root)
            env_target = target_fixture(root, "env-target")
            with patch.dict(os.environ, {"CARGO_TARGET_DIR": str(env_target)}):
                result = guard.main(clean_command(root))
            self.assertEqual(0, result)
            self.assertEqual(b"old", (default / "artifact").read_bytes())
            self.assertFalse((env_target / "sentinel").exists())

    def test_manifest_position_and_repeated_target_dir_are_rejected(self):
        for manifest_position in ("before", "after", "equal"):
            with self.subTest(manifest_position=manifest_position), tempfile.TemporaryDirectory() as directory:
                root = Path(directory).resolve()
                manifest = root / "Cargo.toml"
                manifest.write_text('[package]\nname="anchor-fixture"\nversion="0.0.0"\n')
                default = target_fixture(root, "target")
                selected = target_fixture(root, "selected")
                target_args = ["--target-dir", str(selected)]
                if manifest_position == "before":
                    command = ["--manifest-path", str(manifest), *target_args]
                elif manifest_position == "equal":
                    command = [f"--manifest-path={manifest}", *target_args]
                else:
                    command = [*target_args, "--manifest-path", str(manifest)]
                result = guard.main(["--repo-root", str(root), "--", "cargo", "clean", *command])
                self.assertEqual(2, result)
                self.assertTrue((default / "sentinel").is_file())
                self.assertTrue((selected / "sentinel").is_file())
                repeated = guard.main(["--repo-root", str(root), "--", "cargo", "clean",
                                       *target_args, *target_args])
                self.assertEqual(2, repeated)
                self.assertTrue((default / "sentinel").is_file())
                self.assertTrue((selected / "sentinel").is_file())

    def test_global_prefixes_are_rejected_before_deletion(self):
        for prefix in (("+stable",), ("--config", "build.target-dir=selected"),
                       ("-C", "."), ("--manifest-path", "Cargo.toml")):
            with self.subTest(prefix=prefix), tempfile.TemporaryDirectory() as directory:
                root = Path(directory).resolve()
                manifest = root / "Cargo.toml"
                manifest.write_text('[package]\nname="anchor-fixture"\nversion="0.0.0"\n')
                default = target_fixture(root, "target")
                selected = target_fixture(root, "selected")
                result = guard.main(["--repo-root", str(root), "--", "cargo", *prefix,
                                     "clean", "--target-dir", str(selected)])
                self.assertEqual(2, result)
                self.assertTrue((default / "sentinel").is_file())
                self.assertTrue((selected / "sentinel").is_file())

    def test_guard_only_target_dir_is_rejected_before_deletion(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            manifest = root / "Cargo.toml"
            manifest.write_text('[package]\nname="anchor-fixture"\nversion="0.0.0"\n')
            selected = target_fixture(root, "selected")
            result = guard.main(["--repo-root", str(root), "--target-dir", str(selected), "--",
                                 "cargo", "clean"])
            self.assertEqual(2, result)
            self.assertEqual(b"selected", (selected / "sentinel").read_bytes())

    def test_help_does_not_create_target_or_tag(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            manifest = root / "Cargo.toml"
            manifest.write_text('[package]\nname="anchor-fixture"\nversion="0.0.0"\n')
            result = guard.main(clean_command(root, "--help", "--manifest-path", str(manifest)))
            self.assertEqual(0, result)
            self.assertFalse((root / "target").exists())

    @unittest.skipUnless(os.name == "nt", "requires native Windows handles")
    def test_windows_holds_directory_names_against_rename_until_cleanup_finishes(self):
        from windows_target_cleanup import WindowsTargetCleanup

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            target = fixture(root)
            with WindowsTargetCleanup(target, guard.CACHEDIR_TAG) as anchor:
                with self.assertRaises(OSError):
                    target.rename(root / "moved")
                anchor.purge()
            self.assertFalse((target / "artifact").exists())
            target.rename(root / "moved")

    @unittest.skipUnless(os.name == "nt", "requires native Windows handles")
    def test_windows_anchor_blocks_existing_and_new_write_handles(self):
        from windows_tag_handle import kernel_api
        from windows_target_cleanup import WindowsTargetCleanup

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            target = fixture(root)
            kernel = kernel_api()
            access = 0x40000000
            sharing = 7
            flags = 0x02200000
            writer = kernel.CreateFileW(str(target), access, sharing, None, 3, flags, None)
            self.assertNotEqual(ctypes.c_void_p(-1).value, writer)
            try:
                with self.assertRaises(OSError):
                    WindowsTargetCleanup(target, guard.CACHEDIR_TAG).__enter__()
            finally:
                kernel.CloseHandle(writer)
            with WindowsTargetCleanup(target, guard.CACHEDIR_TAG):
                with self.assertRaises(OSError):
                    (target / "CACHEDIR.TAG").rename(target / "moved-tag")
                for path in (target, target / "CACHEDIR.TAG"):
                    denied = kernel.CreateFileW(str(path), access, sharing, None, 3, flags, None)
                    if denied != ctypes.c_void_p(-1).value:
                        kernel.CloseHandle(denied)
                    self.assertEqual(ctypes.c_void_p(-1).value, denied)

    @unittest.skipUnless(os.name == "nt", "requires native Windows reparse points")
    def test_windows_anchor_blocks_mountpoint_update_while_external_succeeds(self):
        from windows_target_cleanup import WindowsTargetCleanup
        from windows_tag_handle import kernel_api

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            target = fixture(root)
            child = target / "junction"
            child.mkdir()
            external = root / "external"
            external.mkdir()
            (external / "sentinel").write_bytes(b"keep")
            kernel = kernel_api()
            flags = 0x02200000
            attributes = kernel.CreateFileW(str(child), 0x100, 7, None, 3, flags, None)
            self.assertNotEqual(ctypes.c_void_p(-1).value, attributes)
            buffer = mountpoint_buffer(external)
            try:
                with WindowsTargetCleanup(target, guard.CACHEDIR_TAG) as anchor:
                    with anchor._open(child):
                        self.assertFalse(device_io(kernel, attributes, 0x000900A4, buffer))
                        self.assertEqual(5, ctypes.get_last_error())
            finally:
                kernel.CloseHandle(attributes)
            writer = kernel.CreateFileW(str(child), 0x40000000, 7, None, 3, flags, None)
            self.assertNotEqual(ctypes.c_void_p(-1).value, writer)
            try:
                self.assertTrue(device_io(kernel, writer, 0x000900A4, buffer))
            finally:
                kernel.CloseHandle(writer)
            self.assertEqual(b"keep", (external / "sentinel").read_bytes())
            deleter = kernel.CreateFileW(str(child), 0x40000000, 7, None, 3, flags, None)
            self.assertNotEqual(ctypes.c_void_p(-1).value, deleter)
            try:
                self.assertTrue(device_io(kernel, deleter, 0x000900AC,
                                          struct.pack("<LHH", 0xA0000003, 0, 0)))
            finally:
                kernel.CloseHandle(deleter)
            child.rmdir()


if __name__ == "__main__":
    unittest.main()
