#!/usr/bin/env python3
"""Verify real Git self-test fixtures preserve inherited caller metadata."""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

from git_fixture_environment import isolated_git_fixture


ROOT = Path(__file__).resolve().parents[2]


def run_git(arguments: list[str], environment: dict[str, str]) -> None:
    subprocess.run(
        ["git", *arguments], env=environment, check=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
    )


def snapshot(directory: Path) -> dict[str, bytes]:
    return {
        str(path.relative_to(directory)): path.read_bytes()
        for path in directory.rglob("*") if path.is_file()
    }


class GitFixtureEnvironmentTests(unittest.TestCase):
    def test_self_tests_preserve_inherited_caller(self) -> None:
        names = subprocess.run(
            ["git", "rev-parse", "--local-env-vars"], check=True,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
        ).stdout.splitlines()
        environment = {key: value for key, value in os.environ.items() if key not in names}
        environment.update(GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull)
        for script, full_environment in (
            ("post-release-cleanup.py", False), ("post-release-cleanup.py", True),
            ("verify-issue-governance.py", False), ("verify-issue-governance.py", True),
        ):
            with self.subTest(script=script, full_environment=full_environment), tempfile.TemporaryDirectory(prefix="kdv-git-env-") as directory:
                fixture = Path(directory)
                metadata = fixture / "caller.git"
                work_tree = fixture / "caller-files"
                work_tree.mkdir()
                run_git(["init", "--bare", "--initial-branch=master", str(metadata)], environment)
                run_git(["--git-dir", str(metadata), "config", "core.bare", "false"], environment)
                before = snapshot(metadata)
                inherited = {
                    **environment,
                    "GIT_DIR": str(metadata), "GIT_COMMON_DIR": str(metadata),
                    "GIT_WORK_TREE": str(work_tree), "GIT_INDEX_FILE": str(metadata / "index"),
                    "GIT_OBJECT_DIRECTORY": str(metadata / "objects"),
                    "GIT_ALTERNATE_OBJECT_DIRECTORIES": str(metadata / "objects"),
                    "GIT_CONFIG": str(metadata / "config"), "GIT_PREFIX": "caller-files/",
                    "GIT_CONFIG_COUNT": "1", "GIT_CONFIG_KEY_0": "user.name",
                    "GIT_CONFIG_VALUE_0": "Inherited Caller",
                }
                if not full_environment:
                    inherited = {**environment, "GIT_DIR": str(metadata)}
                completed = subprocess.run(
                    [sys.executable, "-B", str(ROOT / "scripts/release" / script), "--self-test"],
                    cwd=ROOT, env=inherited, check=False, stdout=subprocess.PIPE,
                    stderr=subprocess.STDOUT, text=True,
                )
                self.assertTrue(snapshot(metadata) == before, f"{script} changed caller Git metadata")
                self.assertEqual(completed.returncode, 0, completed.stdout)
                self.assertEqual(list(work_tree.iterdir()), [])

    def test_environment_is_restored_on_failure(self) -> None:
        previous = os.environ.copy()
        try:
            os.environ["GIT_PREFIX"] = "caller-prefix/"
            os.environ["KDV_FIXTURE_ENV_MARKER"] = "preserved"
            with self.assertRaisesRegex(ValueError, "fixture failed"):
                with isolated_git_fixture():
                    self.assertNotIn("GIT_PREFIX", os.environ)
                    self.assertEqual(os.environ["KDV_FIXTURE_ENV_MARKER"], "preserved")
                    raise ValueError("fixture failed")
            self.assertEqual(os.environ["GIT_PREFIX"], "caller-prefix/")
        finally:
            os.environ.clear()
            os.environ.update(previous)


if __name__ == "__main__":
    unittest.main()
