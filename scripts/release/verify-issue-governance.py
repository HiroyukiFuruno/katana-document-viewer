#!/usr/bin/env python3
"""Reject untracked or unevidenced non-default-branch pushes."""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib
from collections.abc import Iterable
from pathlib import Path


ISSUE_URL = re.compile(
    r"https://github\.com/(?P<repository>[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+)/issues/(?P<number>[1-9][0-9]*)"
)
DEPENDENCY_FILES = {"Cargo.toml", "Cargo.lock"}
REQUIRED_EVIDENCE = {
    "upstream public registry version": re.compile(
        r"(?im)^\s*(?:upstream\s+(?:public\s+)?registry|registry\s+version)\s*:\s*\S+"
    ),
    "API migration note": re.compile(r"(?im)^\s*(?:API\s+)?migration(?:\s+note)?\s*:\s*\S+"),
    "dependency manifest acknowledgement": re.compile(
        r"(?im)^\s*(?:dependency\s+)?manifest\s*:\s*\S+"
    ),
    "lockfile acknowledgement": re.compile(r"(?im)^\s*lockfile\s*:\s*\S+"),
    "verification evidence": re.compile(r"(?im)^\s*(?:verification|validation)\s*:\s*\S+"),
}
ZERO_SHA = "0" * 40


def run(command: list[str], *, input_text: str | None = None) -> str:
    completed = subprocess.run(
        command,
        check=False,
        input=input_text,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    if completed.returncode != 0:
        raise RuntimeError(f"{' '.join(command)} failed:\n{completed.stdout}")
    return completed.stdout


def repository_from_origin(origin: str) -> str:
    value = origin.strip()
    if value.endswith(".git"):
        value = value[:-4]
    for prefix in ("https://github.com/", "git@github.com:"):
        if value.startswith(prefix):
            repository = value.removeprefix(prefix)
            if re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", repository):
                return repository
    raise ValueError("origin must be a GitHub owner/repository URL")


def default_branch() -> str:
    try:
        reference = run(["git", "symbolic-ref", "--short", "refs/remotes/origin/HEAD"]).strip()
        return reference.removeprefix("origin/")
    except RuntimeError:
        payload = json.loads(run(["gh", "repo", "view", "--json", "defaultBranchRef"]))
        branch = payload.get("defaultBranchRef")
        if not isinstance(branch, dict) or not isinstance(branch.get("name"), str):
            raise ValueError("unable to determine the repository default branch")
        return branch["name"]


def changed_files(local_sha: str, remote_sha: str) -> set[str]:
    if remote_sha == ZERO_SHA:
        commits = run(["git", "rev-list", local_sha, "--not", "--remotes=origin"]).splitlines()
        return {
            line
            for commit in commits
            for line in run(
                ["git", "diff-tree", "--root", "--no-commit-id", "--name-only", "-r", "-m", commit]
            ).splitlines()
            if line
        }
    return {line for line in run(["git", "diff", "--name-only", f"{remote_sha}..{local_sha}"]).splitlines() if line}


def commit_text(local_sha: str, remote_sha: str) -> str:
    if remote_sha == ZERO_SHA:
        return run(["git", "log", "--format=%B", local_sha, "--not", "--remotes=origin"])
    return run(["git", "log", "--format=%B", f"{remote_sha}..{local_sha}"])


def dependency_change(files: Iterable[str]) -> bool:
    return any(Path(path).name in DEPENDENCY_FILES for path in files)


def manifest_at_commit(local_sha: str, filename: str) -> str | None:
    if filename not in run(["git", "ls-tree", "-r", "--name-only", local_sha, "--", filename]).splitlines():
        return None
    return run(["git", "show", f"{local_sha}:{filename}"])


def has_source_override(value: object) -> bool:
    if isinstance(value, dict):
        return "path" in value or "git" in value or any(has_source_override(item) for item in value.values())
    if isinstance(value, list):
        return any(has_source_override(item) for item in value)
    return False


def dependency_sections(manifest: dict[str, object]) -> list[object]:
    names = ("dependencies", "dev-dependencies", "build-dependencies")
    sections = [manifest.get(name) for name in names]
    workspace = manifest.get("workspace")
    if isinstance(workspace, dict):
        sections.append(workspace.get("dependencies"))
    targets = manifest.get("target")
    if isinstance(targets, dict):
        sections.extend(target.get(name) for target in targets.values() if isinstance(target, dict) for name in names)
    sections.extend((manifest.get("patch"), manifest.get("replace")))
    return sections


def manifest_override_errors(files: Iterable[str], local_sha: str) -> list[str]:
    errors: list[str] = []
    for filename in files:
        if Path(filename).name != "Cargo.toml":
            continue
        content = manifest_at_commit(local_sha, filename)
        if content is None:
            continue
        manifest = tomllib.loads(content)
        if any(has_source_override(section) for section in dependency_sections(manifest)):
            errors.append(f"{filename} declares a path or git dependency override.")
    return errors


def issue_reference_errors(commit_messages: str, repository: str) -> tuple[list[str], int | None]:
    matches = [match for match in ISSUE_URL.finditer(commit_messages) if match["repository"] == repository]
    if not matches:
        return [f"non-default branch pushes require an Open Issue URL for {repository}."], None
    return [], int(matches[-1]["number"])


def issue_evidence_errors(issue: dict[str, object], dependency_updated: bool) -> list[str]:
    if issue.get("state") != "OPEN":
        return ["referenced Issue must remain open while the branch is pushed."]
    if not dependency_updated:
        return []
    body = issue.get("body", "")
    comments = issue.get("comments", [])
    if not isinstance(body, str) or not isinstance(comments, list):
        return ["referenced Issue response has invalid evidence fields."]
    evidence = "\n".join(
        [body, *[comment.get("body", "") for comment in comments if isinstance(comment, dict)]]
    )
    return [
        f"referenced Issue lacks {name}."
        for name, expression in REQUIRED_EVIDENCE.items()
        if expression.search(evidence) is None
    ]


def issue_payload(repository: str, number: int) -> dict[str, object]:
    payload = json.loads(
        run(
            [
                "gh",
                "issue",
                "view",
                str(number),
                "--repo",
                repository,
                "--json",
                "state,body,comments",
            ]
        )
    )
    if not isinstance(payload, dict):
        raise ValueError("referenced Issue response must be an object")
    return payload


def validate_update(
    *,
    local_ref: str,
    local_sha: str,
    remote_ref: str,
    remote_sha: str,
    repository: str,
    default: str,
) -> list[str]:
    if local_sha == ZERO_SHA or not remote_ref.startswith("refs/heads/") or remote_ref == f"refs/heads/{default}":
        return []
    files = changed_files(local_sha, remote_sha)
    errors = manifest_override_errors(files, local_sha)
    reference_errors, number = issue_reference_errors(commit_text(local_sha, remote_sha), repository)
    errors.extend(reference_errors)
    if number is not None:
        errors.extend(issue_evidence_errors(issue_payload(repository, number), dependency_change(files)))
    return errors


def parse_push_updates(stream: Iterable[str]) -> list[tuple[str, str, str, str]]:
    updates: list[tuple[str, str, str, str]] = []
    for line in stream:
        fields = line.strip().split()
        if not fields:
            continue
        if len(fields) != 4:
            raise ValueError("pre-push input must contain local-ref local-sha remote-ref remote-sha")
        updates.append((fields[0], fields[1], fields[2], fields[3]))
    return updates


def validate_push() -> list[str]:
    repository = repository_from_origin(run(["git", "remote", "get-url", "origin"]))
    default = default_branch()
    errors: list[str] = []
    for local_ref, local_sha, remote_ref, remote_sha in parse_push_updates(sys.stdin):
        update_errors = validate_update(
            local_ref=local_ref,
            local_sha=local_sha,
            remote_ref=remote_ref,
            remote_sha=remote_sha,
            repository=repository,
            default=default,
        )
        errors.extend(f"{remote_ref}: {error}" for error in update_errors)
    return errors


def self_test() -> None:
    assert repository_from_origin("https://github.com/HiroyukiFuruno/katana-document-viewer.git") == (
        "HiroyukiFuruno/katana-document-viewer"
    )
    assert repository_from_origin("git@github.com:HiroyukiFuruno/katana-document-viewer.git") == (
        "HiroyukiFuruno/katana-document-viewer"
    )
    try:
        repository_from_origin("https://example.invalid/repository")
    except ValueError:
        pass
    else:
        raise AssertionError("non-GitHub remotes must be rejected")
    reference_errors, number = issue_reference_errors(
        "Refs https://github.com/HiroyukiFuruno/katana-document-viewer/issues/43",
        "HiroyukiFuruno/katana-document-viewer",
    )
    assert reference_errors == [] and number == 43
    assert issue_reference_errors("Refs #43", "HiroyukiFuruno/katana-document-viewer")[0]
    issue = {
        "state": "OPEN",
        "body": "\n".join(
            (
                "Upstream registry: katana-ui-core 0.3.15",
                "Migration: no API changes",
                "Manifest: Cargo.toml",
                "Lockfile: Cargo.lock",
                "Verification: just release-check",
            )
        ),
        "comments": [],
    }
    assert issue_evidence_errors(issue, True) == []
    assert issue_evidence_errors({**issue, "state": "CLOSED"}, False)
    assert "lockfile acknowledgement" in issue_evidence_errors(
        {**issue, "body": issue["body"].replace("Lockfile: Cargo.lock\n", "")}, True
    )[0]
    assert dependency_change({"Cargo.lock"})
    assert not dependency_change({"README.md"})
    override_manifests = (
        '[dependencies]\nlocal = { path = "../local" }\n',
        '[dependencies.local]\npath = "../local"\n',
        '[target."cfg(windows)".build-dependencies.tool]\ngit = "https://example.invalid/tool"\n',
        '[workspace.dependencies.local]\npath = "../local"\n',
        '[patch.crates-io]\nlocal = { path = "../local" }\n',
        '[replace]\n"local:1.0.0" = { git = "https://example.invalid/local" }\n',
    )
    assert all(
        any(has_source_override(section) for section in dependency_sections(tomllib.loads(content)))
        for content in override_manifests
    )
    assert not any(
        has_source_override(section)
        for section in dependency_sections(tomllib.loads('[dependencies]\nregistry = "1.0"\n'))
    )
    assert parse_push_updates([f"refs/heads/release/v0.5.6 a refs/heads/release/v0.5.6 {ZERO_SHA}\n"])
    try:
        parse_push_updates(["invalid\n"])
    except ValueError:
        pass
    else:
        raise AssertionError("malformed pre-push input must be rejected")
    _self_test_validate_update(issue)
    _self_test_new_branch_range()
    _self_test_delegate_failure()
    _self_test_delegate_replays_updates()
    _self_test_hook_installer()


def _self_test_validate_update(issue: dict[str, object]) -> None:
    original_files = changed_files
    original_messages = commit_text
    original_issue = issue_payload
    original_overrides = manifest_override_errors
    try:
        globals()["changed_files"] = lambda _local, _remote: {"README.md"}
        globals()["commit_text"] = lambda _local, _remote: (
            "Refs https://github.com/HiroyukiFuruno/katana-document-viewer/issues/43"
        )
        globals()["issue_payload"] = lambda _repository, _number: issue
        globals()["manifest_override_errors"] = lambda _files, _sha: []
        assert not validate_update(
            local_ref="refs/tags/v0.5.6",
            local_sha="a" * 40,
            remote_ref="refs/tags/v0.5.6",
            remote_sha=ZERO_SHA,
            repository="HiroyukiFuruno/katana-document-viewer",
            default="master",
        )
        assert not validate_update(
            local_ref="refs/heads/release/v0.5.6",
            local_sha="a" * 40,
            remote_ref="refs/heads/release/v0.5.6",
            remote_sha="b" * 40,
            repository="HiroyukiFuruno/katana-document-viewer",
            default="master",
        )
        globals()["changed_files"] = lambda _local, _remote: {"Cargo.lock"}
        globals()["issue_payload"] = lambda _repository, _number: {
            **issue,
            "body": str(issue["body"]).replace("Lockfile: Cargo.lock\n", ""),
        }
        assert "lockfile acknowledgement" in validate_update(
            local_ref="refs/heads/release/v0.5.6",
            local_sha="a" * 40,
            remote_ref="refs/heads/release/v0.5.6",
            remote_sha="b" * 40,
            repository="HiroyukiFuruno/katana-document-viewer",
            default="master",
        )[0]
        globals()["issue_payload"] = lambda _repository, _number: issue
        globals()["manifest_override_errors"] = lambda _files, _sha: [
            "Cargo.toml declares a path or git dependency override."
        ]
        assert "path or git dependency override" in validate_update(
            local_ref="refs/heads/release/v0.5.6",
            local_sha="a" * 40,
            remote_ref="refs/heads/release/v0.5.6",
            remote_sha="b" * 40,
            repository="HiroyukiFuruno/katana-document-viewer",
            default="master",
        )[0]
        globals()["commit_text"] = lambda _local, _remote: "no issue reference"
        assert "Open Issue URL" in validate_update(
            local_ref="refs/heads/release/v0.5.6",
            local_sha="a" * 40,
            remote_ref="refs/heads/release/v0.5.6",
            remote_sha="b" * 40,
            repository="HiroyukiFuruno/katana-document-viewer",
            default="master",
        )[1]
    finally:
        globals()["changed_files"] = original_files
        globals()["commit_text"] = original_messages
        globals()["issue_payload"] = original_issue
        globals()["manifest_override_errors"] = original_overrides


def _self_test_new_branch_range() -> None:
    with tempfile.TemporaryDirectory() as directory:
        previous = Path.cwd()
        try:
            os.chdir(directory)
            run(["git", "init", "--initial-branch=master", "--quiet"])
            run(["git", "config", "user.name", "KDV Self Test"])
            run(["git", "config", "user.email", "kdv-self-test@example.invalid"])
            Path("README.md").write_text("base\n", encoding="utf-8")
            run(["git", "add", "README.md"])
            run(["git", "commit", "--quiet", "-m", "old unrelated issue"])
            base = run(["git", "rev-parse", "HEAD"]).strip()
            run(["git", "update-ref", "refs/remotes/origin/master", base])
            Path("Cargo.toml").write_text('[dependencies.local]\npath = "../local"\n', encoding="utf-8")
            Path("Cargo.lock").write_text("lock update\n", encoding="utf-8")
            run(["git", "add", "Cargo.toml", "Cargo.lock"])
            run(["git", "commit", "--quiet", "-m", "update dependencies"])
            Path("README.md").write_text("tip\n", encoding="utf-8")
            run(["git", "add", "README.md"])
            run(["git", "commit", "--quiet", "-m", "Refs https://github.com/owner/repo/issues/43"])
            tip = run(["git", "rev-parse", "HEAD"]).strip()
            assert changed_files(tip, ZERO_SHA) == {"Cargo.toml", "Cargo.lock", "README.md"}
            messages = commit_text(tip, ZERO_SHA)
            assert "update dependencies" in messages and "old unrelated issue" not in messages
            run(["git", "switch", "--detach", "--quiet", base])
            assert manifest_override_errors({"Cargo.toml"}, tip) == [
                "Cargo.toml declares a path or git dependency override."
            ]
            assert manifest_override_errors({"Cargo.toml"}, base) == []
        finally:
            os.chdir(previous)


def _self_test_delegate_failure() -> None:
    root = Path(__file__).resolve().parents[2]
    environment = os.environ.copy()
    environment["KDV_PRE_PUSH_DELEGATE"] = "/usr/bin/false"
    completed = subprocess.run(
        [str(root / ".githooks/pre-push"), "origin", "https://example.invalid/repository"],
        check=False,
        input="",
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        env=environment,
        cwd=root,
    )
    if completed.returncode == 0:
        raise AssertionError("a failing existing pre-push delegate must reject the push")


def _self_test_delegate_replays_updates() -> None:
    root = Path(__file__).resolve().parents[2]
    with tempfile.TemporaryDirectory() as directory:
        fixture = Path(directory)
        previous = Path.cwd()
        try:
            os.chdir(fixture)
            run(["git", "init", "--initial-branch=master", "--quiet"])
            run(["git", "config", "user.name", "KDV Self Test"])
            run(["git", "-c", "user.email=kdv-self-test@example.invalid", "commit", "--allow-empty", "--quiet", "-m", "base"])
            run(["git", "remote", "add", "origin", "https://github.com/HiroyukiFuruno/katana-document-viewer.git"])
            base = run(["git", "rev-parse", "HEAD"]).strip()
            run(["git", "update-ref", "refs/remotes/origin/master", base])
            run(["git", "symbolic-ref", "refs/remotes/origin/HEAD", "refs/remotes/origin/master"])
        finally:
            os.chdir(previous)
        hook = fixture / ".githooks/pre-push"
        validator = fixture / "scripts/release/verify-issue-governance.py"
        hook.parent.mkdir(parents=True)
        validator.parent.mkdir(parents=True)
        shutil.copyfile(root / ".githooks/pre-push", hook)
        shutil.copyfile(Path(__file__), validator)
        hook.chmod(0o700)
        delegate = fixture / "consume-stdin"
        delegate.write_text("#!/bin/sh\ncat >/dev/null\n", encoding="utf-8")
        delegate.chmod(0o700)
        environment = os.environ.copy()
        environment["KDV_PRE_PUSH_DELEGATE"] = str(delegate)
        completed = subprocess.run(
            [str(hook), "origin", "https://github.com/HiroyukiFuruno/katana-document-viewer.git"],
            check=False,
            input="invalid update\n",
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            env=environment,
            cwd=fixture,
        )
        assert completed.returncode != 0
        assert "pre-push input must contain" in completed.stdout


def _self_test_hook_installer() -> None:
    source = Path(__file__).resolve().parents[2]
    installer = source / "scripts/release/install-governance-hook.sh"
    with tempfile.TemporaryDirectory(prefix="kdv-hook-install-") as directory:
        fixture = Path(directory)
        managed_hook = fixture / ".githooks/pre-push"
        managed_hook.parent.mkdir()
        shutil.copy2(source / ".githooks/pre-push", managed_hook)
        previous = Path.cwd()
        try:
            os.chdir(fixture)
            run(["git", "init", "--initial-branch=master", "--quiet"])
            run(["git", "config", "core.hooksPath", ".githooks"])
            for _ in range(2):
                run(["bash", str(installer)])
                configured_path = Path(run(["git", "config", "--get", "core.hooksPath"]).strip())
                assert configured_path.resolve() == managed_hook.parent.resolve()
                assert subprocess.run(
                    ["git", "config", "--get", "kdv.pre-push-delegate"],
                    check=False,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.DEVNULL,
                ).returncode == 1, "managed hook delegated to itself"

            run(["git", "config", "kdv.pre-push-delegate", str(managed_hook.resolve())])
            run(["bash", str(installer)])
            assert subprocess.run(
                ["git", "config", "--get", "kdv.pre-push-delegate"],
                check=False,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            ).returncode == 1, "an existing self-delegate was not cleared"

            legacy_hook = fixture / "legacy-hooks/pre-push"
            legacy_hook.parent.mkdir()
            shutil.copy2(source / ".githooks/pre-push", legacy_hook)
            run(["git", "config", "core.hooksPath", "legacy-hooks"])
            run(["bash", str(installer)])
            configured_delegate = Path(run(["git", "config", "--get", "kdv.pre-push-delegate"]).strip())
            assert configured_delegate.resolve() == legacy_hook.resolve()
        finally:
            os.chdir(previous)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        print("issue governance self-test passed")
        return 0
    try:
        errors = validate_push()
    except (RuntimeError, ValueError, json.JSONDecodeError) as error:
        print(f"issue governance: {error}", file=sys.stderr)
        return 1
    if errors:
        for error in errors:
            print(f"issue governance: {error}", file=sys.stderr)
        return 1
    print("issue governance: passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
