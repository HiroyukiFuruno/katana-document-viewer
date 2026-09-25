#!/usr/bin/env python3
"""Reject untracked or unevidenced non-default-branch pushes."""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
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
OVERRIDE = re.compile(r"(?m)^\s*[A-Za-z0-9_-]+\s*=\s*\{[^}]*\b(?:path|git)\s*=")
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
        command = ["git", "diff-tree", "--no-commit-id", "--name-only", "-r", local_sha]
    else:
        command = ["git", "diff", "--name-only", f"{remote_sha}..{local_sha}"]
    return {line for line in run(command).splitlines() if line}


def commit_text(local_sha: str, remote_sha: str) -> str:
    revision = local_sha if remote_sha == ZERO_SHA else f"{remote_sha}..{local_sha}"
    return run(["git", "log", "--format=%B", revision])


def dependency_change(files: Iterable[str]) -> bool:
    return any(Path(path).name in DEPENDENCY_FILES for path in files)


def manifest_override_errors(files: Iterable[str]) -> list[str]:
    errors: list[str] = []
    for filename in files:
        if Path(filename).name != "Cargo.toml":
            continue
        manifest = Path(filename)
        if manifest.is_file() and OVERRIDE.search(manifest.read_text(encoding="utf-8")):
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
    if local_sha == ZERO_SHA or remote_ref == f"refs/heads/{default}":
        return []
    files = changed_files(local_sha, remote_sha)
    errors = manifest_override_errors(files)
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
    with tempfile.TemporaryDirectory() as directory:
        manifest = Path(directory) / "Cargo.toml"
        manifest.write_text('dependency = { path = "../dependency" }\n', encoding="utf-8")
        previous = Path.cwd()
        try:
            os.chdir(directory)
            assert manifest_override_errors({"Cargo.toml"}) == [
                "Cargo.toml declares a path or git dependency override."
            ]
        finally:
            os.chdir(previous)
    assert parse_push_updates([f"refs/heads/release/v0.5.6 a refs/heads/release/v0.5.6 {ZERO_SHA}\n"])
    try:
        parse_push_updates(["invalid\n"])
    except ValueError:
        pass
    else:
        raise AssertionError("malformed pre-push input must be rejected")
    _self_test_validate_update(issue)
    _self_test_delegate_failure()


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
        globals()["manifest_override_errors"] = lambda _files: []
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
        globals()["manifest_override_errors"] = lambda _files: ["Cargo.toml declares a path or git dependency override."]
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
