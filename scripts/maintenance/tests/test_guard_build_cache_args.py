import sys
import unittest
from pathlib import Path


SCRIPT_DIR = Path(__file__).parents[1]
sys.path.insert(0, str(SCRIPT_DIR))
from cargo_target_args import command_directory, command_target_dirs


class CargoTargetArgsTests(unittest.TestCase):
    def test_reads_separated_and_equals_forms(self):
        self.assertEqual(
            [Path("build-cache"), Path("target/llvm-cov")],
            command_target_dirs(
                ["cargo", "build", "--target-dir", "build-cache", "--target-dir=target/llvm-cov"]
            ),
        )

    def test_skips_option_values_and_child_arguments(self):
        command = [
            "cargo", "build", "--manifest-path", "Cargo.toml", "--config", "net.offline=true",
            "--target-dir", "build-cache", "--", "--target-dir", "child-value",
        ]
        self.assertEqual([Path("build-cache")], command_target_dirs(command))

    def test_rejects_missing_target_directory(self):
        with self.assertRaises(ValueError):
            command_target_dirs(["cargo", "build", "--target-dir"])
        with self.assertRaises(ValueError):
            command_target_dirs(["cargo", "build", "--target-dir="])

    def test_cargo_directory_resolves_relative_target_dirs(self):
        root = Path("/tmp/cargo-command-root").resolve()
        command = ["cargo", "-C", "nested", "build", "--target-dir", "cache"]
        self.assertEqual(root / "nested", command_directory(command, root))
        self.assertEqual([root / "nested/cache"], command_target_dirs(command, root))

    def test_attached_cargo_directory_resolves_target_dir_from_cargo_directory(self):
        root = Path("/tmp/cargo-command-root").resolve()
        command = ["cargo", "-Cnested", "build", "--target-dir=cache"]
        self.assertEqual(root / "nested", command_directory(command, root))
        self.assertEqual([root / "nested/cache"], command_target_dirs(command, root))

    def test_multiple_cargo_directories_and_manifest_value(self):
        root = Path("/tmp/cargo-command-root").resolve()
        command = [
            "cargo", "+nightly", "-Z", "unstable-options", "-C", "nested",
            "-C/tmp/cargo-final-root", "build", "--manifest-path", "Cargo.toml",
            "--target-dir", "cache",
        ]
        final_root = Path("/tmp/cargo-final-root").resolve()
        self.assertEqual(final_root, command_directory(command, root))
        self.assertEqual([final_root / "cache"], command_target_dirs(command, root))


if __name__ == "__main__":
    unittest.main()
