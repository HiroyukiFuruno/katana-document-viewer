import sys
import unittest
from pathlib import Path


SCRIPT_DIR = Path(__file__).parents[1]
sys.path.insert(0, str(SCRIPT_DIR))
from cargo_target_args import command_target_dirs


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


if __name__ == "__main__":
    unittest.main()
