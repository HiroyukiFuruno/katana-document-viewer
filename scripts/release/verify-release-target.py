#!/usr/bin/env python3
"""Verify that the requested release version follows the published release line."""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path
from urllib import error, request


Version = tuple[int, int, int]


def parse_version(value: str) -> Version:
    match = re.fullmatch(r"v?(\d+)\.(\d+)\.(\d+)", value.strip())
    if match is None:
        raise ValueError(f"expected a stable version like v1.2.3, got {value!r}")
    return tuple(int(group) for group in match.groups())


def tag(version: Version) -> str:
    return f"v{version[0]}.{version[1]}.{version[2]}"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target-version", required=True, help="Release version such as v0.1.0")
    parser.add_argument("--latest-version", help="Override latest stable version for tests")
    parser.add_argument(
        "--repo",
        default="HiroyukiFuruno/katana-document-viewer",
        help="GitHub repository used to resolve published stable releases",
    )
    parser.add_argument("--github-releases-json", help="Read a GitHub Releases API fixture")
    parser.add_argument("--remote", default="origin", help="Git remote used when fetching tags")
    return parser.parse_args()


def github_headers() -> dict[str, str]:
    headers = {
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
        "User-Agent": "katana-document-viewer-release-target-check",
    }
    token = os.environ.get("GITHUB_TOKEN") or os.environ.get("GH_TOKEN")
    if token:
        headers["Authorization"] = f"Bearer {token}"
    return headers


def stable_from_release_payload(payload: object, target: Version) -> Version | None:
    releases = payload if isinstance(payload, list) else [payload]
    versions: list[Version] = []
    for release in releases:
        if not isinstance(release, dict):
            continue
        if release.get("draft") or release.get("prerelease"):
            continue
        tag_name = release.get("tag_name")
        if not isinstance(tag_name, str):
            continue
        try:
            version = parse_version(tag_name)
        except ValueError:
            continue
        if version != target:
            versions.append(version)
    return max(versions) if versions else None


def latest_from_fixture(path: str, target: Version) -> Version | None:
    payload = json.loads(Path(path).read_text(encoding="utf-8"))
    return stable_from_release_payload(payload, target)


def latest_from_github(repo: str, target: Version) -> Version | None:
    api_request = request.Request(
        f"https://api.github.com/repos/{repo}/releases/latest", headers=github_headers()
    )
    try:
        with request.urlopen(api_request, timeout=20) as response:
            payload = json.loads(response.read().decode("utf-8"))
    except error.HTTPError as release_error:
        if release_error.code == 404:
            return None
        message = f"could not read latest stable GitHub Release: {release_error}"
        raise RuntimeError(message) from release_error
    except (error.URLError, json.JSONDecodeError, UnicodeDecodeError, OSError) as release_error:
        message = f"could not read latest stable GitHub Release: {release_error}"
        raise RuntimeError(message) from release_error
    return stable_from_release_payload(payload, target)


def latest_from_tags(remote: str, target: Version) -> Version | None:
    subprocess.run(
        ["git", "fetch", "--quiet", "--tags", remote],
        check=False,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    result = subprocess.run(
        ["git", "tag", "--list", "v[0-9]*.[0-9]*.[0-9]*"],
        check=True,
        text=True,
        stdout=subprocess.PIPE,
    )
    versions = []
    for line in result.stdout.splitlines():
        try:
            version = parse_version(line)
        except ValueError:
            continue
        if version < target:
            versions.append(version)
    return max(versions) if versions else None


def latest_stable(args: argparse.Namespace, target: Version) -> Version | None:
    if args.latest_version:
        return parse_version(args.latest_version)
    if args.github_releases_json:
        return latest_from_fixture(args.github_releases_json, target)
    try:
        latest = latest_from_github(args.repo, target)
    except RuntimeError as release_error:
        latest = latest_from_tags(args.remote, target)
        if latest is None:
            raise release_error
        print(
            f"Fallback to local tags for release-line resolution: {tag(latest)}.",
            file=sys.stderr,
        )
    return latest or latest_from_tags(args.remote, target)


def fail(message: str) -> int:
    print(f"Release target sanity check failed: {message}", file=sys.stderr)
    print(
        "If this is intentional, get user confirmation and rerun with "
        "KDV_RELEASE_ALLOW_VERSION_LINE_OVERRIDE=1.",
        file=sys.stderr,
    )
    return 1


def verify(target: Version, latest: Version | None) -> int:
    if os.environ.get("KDV_RELEASE_ALLOW_VERSION_LINE_OVERRIDE") == "1":
        print("Release target sanity check override is enabled.")
        return 0
    if latest is None:
        print(f"No previous stable release tag found; accepting {tag(target)}.")
        return 0
    if target <= latest:
        return fail(f"{tag(target)} is not newer than latest stable release {tag(latest)}.")
    if target[:2] == latest[:2]:
        expected = (latest[0], latest[1], latest[2] + 1)
        return 0 if target == expected else fail(f"expected {tag(expected)}, got {tag(target)}.")
    if target[0] == latest[0] and target[1] == latest[1] + 1:
        expected = (latest[0], latest[1] + 1, 0)
        return 0 if target == expected else fail(f"expected {tag(expected)}, got {tag(target)}.")
    if target[0] == latest[0] + 1:
        expected = (latest[0] + 1, 0, 0)
        return 0 if target == expected else fail(f"expected {tag(expected)}, got {tag(target)}.")
    return fail(f"{tag(target)} skips over latest stable release {tag(latest)}.")


def main() -> int:
    args = parse_args()
    target = parse_version(args.target_version)
    return verify(target, latest_stable(args, target))


if __name__ == "__main__":
    raise SystemExit(main())
