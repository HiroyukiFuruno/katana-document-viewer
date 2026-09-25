#!/usr/bin/env python3
"""Audit or safely clean release branches only after a GitHub Release exists."""

from __future__ import annotations

import argparse
import contextlib
import io
import json
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Worktree:
    path: Path
    branch: str | None
    bare: bool


def run(command: list[str], *, cwd: Path | None = None) -> str:
    completed = subprocess.run(
        command,
        check=False,
        cwd=cwd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    if completed.returncode != 0:
        raise RuntimeError(f"{' '.join(command)} failed:\n{completed.stdout}")
    return completed.stdout


def release_errors(payload: dict[str, object] | None, version: str) -> list[str]:
    if payload is None:
        return [f"GitHub Release {version} does not exist."]
    if payload.get("isDraft") is True:
        return [f"GitHub Release {version} is still a draft."]
    if payload.get("isPrerelease") is True:
        return [f"GitHub Release {version} is a prerelease."]
    return []


def published_release(repository: str, version: str) -> list[str]:
    completed = subprocess.run(
        ["gh", "release", "view", version, "--repo", repository, "--json", "isDraft,isPrerelease"],
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    if completed.returncode != 0:
        return release_errors(None, version)
    try:
        payload = json.loads(completed.stdout)
    except json.JSONDecodeError:
        return [f"GitHub Release {version} returned invalid JSON."]
    if not isinstance(payload, dict):
        return [f"GitHub Release {version} returned an invalid response."]
    return release_errors(payload, version)


def default_branch(repository: str) -> str:
    payload = json.loads(run(["gh", "repo", "view", repository, "--json", "defaultBranchRef"]))
    branch = payload.get("defaultBranchRef") if isinstance(payload, dict) else None
    if not isinstance(branch, dict) or not isinstance(branch.get("name"), str) or not branch["name"]:
        raise ValueError("GitHub repository did not return a default branch")
    return branch["name"]


def worktrees() -> list[Worktree]:
    entries: list[Worktree] = []
    current_path: Path | None = None
    current_branch: str | None = None
    current_bare = False
    for line in run(["git", "worktree", "list", "--porcelain"]).splitlines() + [""]:
        if not line:
            if current_path is not None:
                entries.append(Worktree(current_path, current_branch, current_bare))
            current_path = None
            current_branch = None
            current_bare = False
            continue
        key, _, value = line.partition(" ")
        if key == "worktree":
            current_path = Path(value)
        elif key == "branch":
            current_branch = value.removeprefix("refs/heads/")
        elif key == "bare":
            current_bare = True
    return entries


def is_merged(branch: str, default: str) -> bool:
    return subprocess.run(
        ["git", "merge-base", "--is-ancestor", branch, f"origin/{default}"],
        check=False,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    ).returncode == 0


def dirty(path: Path) -> bool:
    return bool(run(["git", "status", "--porcelain"], cwd=path).strip())


def retain_reasons(
    *,
    branch: str,
    default: str,
    merged: bool,
    checked_out: bool,
    is_dirty: bool,
) -> list[str]:
    reasons: list[str] = []
    if branch == default:
        reasons.append("default branch")
    if not merged:
        reasons.append("not merged into refreshed default branch")
    if checked_out:
        reasons.append("checked out by a worktree")
    if is_dirty:
        reasons.append("worktree has uncommitted changes")
    return reasons


def refresh_default(default: str, apply: bool) -> None:
    if not apply:
        print(f"audit: would switch to {default} and fast-forward from origin/{default}")
        return
    run(["git", "switch", default])
    run(["git", "pull", "--ff-only", "origin", default])


def local_cleanup(default: str, apply: bool) -> int:
    refresh_default(default, apply)
    if apply:
        run(["git", "fetch", "origin", default])
    entries = [entry for entry in worktrees() if not entry.bare]
    current = Path.cwd().resolve()
    branch_worktrees = {entry.branch: entry for entry in entries if entry.branch is not None}
    failures = 0
    for entry in entries:
        if entry.path.resolve() == current or entry.branch is None or entry.branch == default:
            continue
        reasons = retain_reasons(
            branch=entry.branch,
            default=default,
            merged=is_merged(entry.branch, default),
            checked_out=True,
            is_dirty=dirty(entry.path),
        )
        unsafe_reasons = [reason for reason in reasons if reason != "checked out by a worktree"]
        if unsafe_reasons:
            print(f"retain worktree {entry.path}: {', '.join(unsafe_reasons)}")
            failures += 1
            continue
        if not apply:
            print(f"audit: would remove clean merged worktree {entry.path}")
            continue
        run(["git", "worktree", "remove", str(entry.path)])
        print(f"removed worktree {entry.path}")
    if apply:
        run(["git", "worktree", "prune"])
    for branch in run(["git", "for-each-ref", "--format=%(refname:short)", "refs/heads"]).splitlines():
        entry = branch_worktrees.get(branch)
        reasons = retain_reasons(
            branch=branch,
            default=default,
            merged=is_merged(branch, default),
            checked_out=entry is not None and entry.path.exists(),
            is_dirty=entry is not None and entry.path.exists() and dirty(entry.path),
        )
        if reasons:
            print(f"retain branch {branch}: {', '.join(reasons)}")
            if branch != default:
                failures += 1
            continue
        if not apply:
            print(f"audit: would delete local branch {branch}")
            continue
        run(["git", "branch", "-d", branch])
        print(f"deleted local branch {branch}")
    return failures


def remote_branch_exists(branch: str) -> bool:
    completed = subprocess.run(
        ["git", "ls-remote", "--exit-code", "--heads", "origin", f"refs/heads/{branch}"],
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    if completed.returncode == 0:
        return True
    if completed.returncode == 2:
        return False
    raise RuntimeError(f"remote branch lookup failed: {completed.stdout}")


def remote_cleanup(default: str, branch: str, apply: bool) -> int:
    if branch == default:
        print(f"retain remote branch {branch}: default branch")
        return 1
    run(["git", "fetch", "origin", default])
    if not remote_branch_exists(branch):
        print(f"remote branch {branch} already deleted")
        return 0
    run(["git", "fetch", "origin", branch])
    if not is_merged(f"origin/{branch}", default):
        print(f"retain remote branch {branch}: not merged into origin/{default}")
        return 1
    if not apply:
        print(f"audit: would delete remote branch {branch}")
        return 0
    run(["git", "push", "origin", "--delete", branch])
    print(f"deleted remote branch {branch}")
    return 0


def self_test() -> None:
    assert release_errors(None, "v0.5.6") == ["GitHub Release v0.5.6 does not exist."]
    assert release_errors({"isDraft": False, "isPrerelease": False}, "v0.5.6") == []
    assert release_errors({"isDraft": True, "isPrerelease": False}, "v0.5.6")
    assert retain_reasons(
        branch="release/v0.5.6",
        default="master",
        merged=True,
        checked_out=False,
        is_dirty=False,
    ) == []
    assert retain_reasons(
        branch="master",
        default="master",
        merged=False,
        checked_out=True,
        is_dirty=True,
    ) == [
        "default branch",
        "not merged into refreshed default branch",
        "checked out by a worktree",
        "worktree has uncommitted changes",
    ]
    _self_test_remote_cleanup()
    _self_test_remote_branch_lookup()
    _self_test_failure_diagnostic()


def _self_test_remote_cleanup() -> None:
    original_run = run
    original_merged = is_merged
    original_exists = remote_branch_exists
    commands: list[list[str]] = []
    try:
        globals()["run"] = lambda command, cwd=None: commands.append(command) or ""
        globals()["remote_branch_exists"] = lambda _branch: False
        assert remote_cleanup("master", "release/v0.5.6", apply=True) == 0
        assert commands == [["git", "fetch", "origin", "master"]]
        commands.clear()
        globals()["remote_branch_exists"] = lambda _branch: True
        globals()["is_merged"] = lambda _branch, _default: False
        assert remote_cleanup("master", "release/v0.5.6", apply=False) == 1
        assert commands == [
            ["git", "fetch", "origin", "master"],
            ["git", "fetch", "origin", "release/v0.5.6"],
        ]
        commands.clear()
        globals()["is_merged"] = lambda _branch, _default: True
        assert remote_cleanup("master", "release/v0.5.6", apply=False) == 0
        assert commands == [
            ["git", "fetch", "origin", "master"],
            ["git", "fetch", "origin", "release/v0.5.6"],
        ]
        commands.clear()
        assert remote_cleanup("master", "release/v0.5.6", apply=True) == 0
        assert commands == [
            ["git", "fetch", "origin", "master"],
            ["git", "fetch", "origin", "release/v0.5.6"],
            ["git", "push", "origin", "--delete", "release/v0.5.6"],
        ]
        assert all("--force" not in command for command in commands)
    finally:
        globals()["run"] = original_run
        globals()["is_merged"] = original_merged
        globals()["remote_branch_exists"] = original_exists


def _self_test_remote_branch_lookup() -> None:
    original_run = subprocess.run
    status = 0
    commands: list[list[str]] = []

    def fake_run(command: list[str], **_kwargs: object) -> subprocess.CompletedProcess[str]:
        commands.append(command)
        return subprocess.CompletedProcess(command, status, stdout="lookup failed")

    try:
        subprocess.run = fake_run
        assert remote_branch_exists("release/v0.5.6")
        status = 2
        assert not remote_branch_exists("release/v0.5.6")
        status = 128
        try:
            remote_branch_exists("release/v0.5.6")
        except RuntimeError as error:
            assert "lookup failed" in str(error)
        else:
            raise AssertionError("remote lookup failure was treated as a deleted branch")
        assert commands == [
            ["git", "ls-remote", "--exit-code", "--heads", "origin", "refs/heads/release/v0.5.6"]
        ] * 3
    finally:
        subprocess.run = original_run


def _self_test_failure_diagnostic() -> None:
    original_argv = sys.argv
    original_release = published_release
    stderr = io.StringIO()
    try:
        sys.argv = [
            "post-release-cleanup.py", "--repo", "owner/repo", "--version", "v0.5.6",
            "--scope", "remote", "--branch", "release/v0.5.6",
        ]
        globals()["published_release"] = lambda _repository, _version: [
            "GitHub Release v0.5.6 does not exist."
        ]
        with contextlib.redirect_stderr(stderr):
            assert main() == 1
        assert "GitHub Release v0.5.6 does not exist." in stderr.getvalue()
    finally:
        sys.argv = original_argv
        globals()["published_release"] = original_release


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=False)
    parser.add_argument("--version", required=False)
    parser.add_argument("--scope", choices=("local", "remote"), required=False)
    parser.add_argument("--branch", required=False)
    parser.add_argument("--apply", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        print("post-release cleanup self-test passed")
        return 0
    if not args.repo or not args.version or not args.scope:
        parser.error("--repo, --version, and --scope are required unless --self-test is used")
    if args.scope == "remote" and not args.branch:
        parser.error("--branch is required for remote cleanup")
    try:
        errors = published_release(args.repo, args.version)
        if errors:
            for error in errors:
                print(f"post-release cleanup: {error}", file=sys.stderr)
            return 1
        default = default_branch(args.repo)
        return (
            remote_cleanup(default, args.branch, args.apply)
            if args.scope == "remote"
            else local_cleanup(default, args.apply)
        )
    except (RuntimeError, ValueError, json.JSONDecodeError) as error:
        print(f"post-release cleanup: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
