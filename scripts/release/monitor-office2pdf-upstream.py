#!/usr/bin/env python3
"""Record whether KDV can safely evaluate a newer official office2pdf crate."""

from __future__ import annotations

import argparse
import json
import re
import tomllib
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
REGISTRY = "registry+https://github.com/rust-lang/crates.io-index"
CRATE_URL = "https://crates.io/api/v1/crates/office2pdf"
RELEASE_URL = "https://api.github.com/repos/developer0hye/office2pdf/releases/tags/v{version}"
VERSION = re.compile(r"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$")


def version_tuple(value: str) -> tuple[int, int, int]:
    match = VERSION.fullmatch(value)
    if match is None:
        raise ValueError(f"unsupported non-stable semver version: {value}")
    return tuple(int(part) for part in match.groups())


def exact_office2pdf(manifest: str) -> str:
    dependency = tomllib.loads(manifest)["workspace"]["dependencies"].get("office2pdf")
    if not isinstance(dependency, dict):
        raise ValueError("office2pdf must use an inline-table exact registry dependency")
    if dependency.get("package") != "office2pdf" or any(key in dependency for key in ("git", "path")):
        raise ValueError("office2pdf must not use a package alias or git/path override")
    value = dependency.get("version")
    if not isinstance(value, str) or not value.startswith("="):
        raise ValueError("office2pdf must use an exact version requirement")
    version_tuple(value[1:])
    return value[1:]


def lock_entry(lockfile: str) -> dict[str, Any]:
    entries = [item for item in tomllib.loads(lockfile).get("package", []) if item.get("name") == "office2pdf"]
    if len(entries) != 1:
        raise ValueError("Cargo.lock must contain exactly one office2pdf package")
    entry = entries[0]
    if entry.get("source") != REGISTRY or not isinstance(entry.get("checksum"), str):
        raise ValueError("office2pdf must resolve from crates.io with a checksum")
    return entry


def latest_unyanked(payload: dict[str, Any]) -> str:
    versions = payload.get("versions")
    if not isinstance(versions, list):
        raise ValueError("crates.io response has no versions list")
    available = [item.get("num") for item in versions if isinstance(item, dict) and not item.get("yanked")]
    stable = [value for value in available if isinstance(value, str) and VERSION.fullmatch(value)]
    if not stable:
        raise ValueError("crates.io has no unyanked stable office2pdf release")
    return max(stable, key=version_tuple)


def release_matches(payload: dict[str, Any], version: str) -> bool:
    return (
        payload.get("tag_name") == f"v{version}"
        and payload.get("draft") is False
        and payload.get("prerelease") is False
    )


def candidate_manifest(manifest: str, candidate: str) -> str:
    current = exact_office2pdf(manifest)
    version_tuple(candidate)
    if version_tuple(candidate) <= version_tuple(current):
        raise ValueError("candidate must be newer than the current exact version")
    pattern = re.compile(r'(?m)^(office2pdf\s*=\s*\{\s*package\s*=\s*"office2pdf",\s*version\s*=\s*)"=[^"]+"(\s*\}\s*)$')
    updated, count = pattern.subn(rf'\g<1>"={candidate}"\g<2>', manifest)
    if count != 1:
        raise ValueError("Cargo.toml must contain exactly one canonical office2pdf dependency line")
    return updated


def decide(current: str, locked: dict[str, Any], crate: dict[str, Any], release: dict[str, Any]) -> dict[str, Any]:
    candidate = latest_unyanked(crate)
    result: dict[str, Any] = {
        "schema_version": 1,
        "dependency": "office2pdf",
        "current": {"version": current, "source": locked["source"], "checksum": locked["checksum"]},
        "upstream": {"crates_io_version": candidate, "release_tag": release.get("tag_name"), "release_url": release.get("html_url")},
        "status": "deferred",
        "reasons": [],
    }
    if not release_matches(release, candidate):
        result["reasons"].append("crates.io candidate does not have a matching published upstream release tag")
        return result
    if version_tuple(candidate) == version_tuple(current):
        result["status"] = "current"
        return result
    if version_tuple(candidate) < version_tuple(current):
        result["reasons"].append("crates.io latest version is older than the pinned dependency")
        return result
    if version_tuple(candidate)[0] != version_tuple(current)[0]:
        result["reasons"].append("major version migration requires a separately reviewed change")
        return result
    result["status"] = "eligible"
    result["candidate"] = {"version": candidate, "release_url": release.get("html_url")}
    return result


def unavailable(current: str, locked: dict[str, Any], error: Exception) -> dict[str, Any]:
    return {"schema_version": 1, "dependency": "office2pdf", "status": "unavailable", "current": {"version": current, "source": locked["source"], "checksum": locked["checksum"]}, "reasons": [str(error)]}


def fetch_json(url: str) -> dict[str, Any]:
    request = urllib.request.Request(url, headers={"Accept": "application/json", "User-Agent": "kdv-office2pdf-monitor"})
    with urllib.request.urlopen(request, timeout=30) as response:
        payload = json.load(response)
    if not isinstance(payload, dict):
        raise ValueError(f"unexpected JSON object from {url}")
    return payload


def monitor(root: Path) -> dict[str, Any]:
    current = exact_office2pdf((root / "Cargo.toml").read_text(encoding="utf-8"))
    locked = lock_entry((root / "Cargo.lock").read_text(encoding="utf-8"))
    if locked.get("version") != current:
        raise ValueError("Cargo.toml and Cargo.lock office2pdf versions disagree")
    try:
        crate = fetch_json(CRATE_URL)
        candidate = latest_unyanked(crate)
        release = fetch_json(RELEASE_URL.format(version=candidate))
    except (OSError, urllib.error.URLError, urllib.error.HTTPError, ValueError) as error:
        return unavailable(current, locked, error)
    return decide(current, locked, crate, release)


def self_test() -> None:
    lock = {"version": "0.7.0", "source": REGISTRY, "checksum": "a" * 64}
    current = {"versions": [{"num": "0.7.0", "yanked": False}]}
    matching = {"tag_name": "v0.7.0", "draft": False, "prerelease": False, "html_url": "https://example.test/v0.7.0"}
    assert decide("0.7.0", lock, current, matching)["status"] == "current"
    eligible = {"versions": [{"num": "0.7.1", "yanked": False}, {"num": "0.7.0", "yanked": False}]}
    assert decide("0.7.0", lock, eligible, {**matching, "tag_name": "v0.7.1"})["status"] == "eligible"
    yanked = {"versions": [{"num": "0.7.1", "yanked": True}, {"num": "0.7.0", "yanked": False}]}
    assert decide("0.7.0", lock, yanked, matching)["status"] == "current"
    major = {"versions": [{"num": "1.0.0", "yanked": False}]}
    assert decide("0.7.0", lock, major, {**matching, "tag_name": "v1.0.0"})["status"] == "deferred"
    assert decide("0.7.0", lock, eligible, matching)["status"] == "deferred"
    assert unavailable("0.7.0", lock, OSError("offline"))["status"] == "unavailable"
    manifest = '[workspace]\n[workspace.dependencies]\noffice2pdf = { package = "office2pdf", version = "=0.7.0" }\n'
    assert exact_office2pdf(manifest) == "0.7.0"
    assert 'version = "=0.7.1"' in candidate_manifest(manifest, "0.7.1")
    try:
        exact_office2pdf('[workspace]\n[workspace.dependencies]\noffice2pdf = { version = "=0.7.0", git = "https://example.test" }\n')
    except ValueError:
        pass
    else:
        raise AssertionError("git override must be rejected")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, required=False)
    parser.add_argument("--github-output", type=Path, required=False)
    parser.add_argument("--prepare-candidate", type=str, required=False)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        print("office2pdf monitor self-test passed")
        return 0
    if args.prepare_candidate:
        try:
            manifest_path = ROOT / "Cargo.toml"
            manifest_path.write_text(candidate_manifest(manifest_path.read_text(encoding="utf-8"), args.prepare_candidate), encoding="utf-8")
            result = {"schema_version": 1, "dependency": "office2pdf", "status": "prepared", "candidate": {"version": args.prepare_candidate}}
        except (KeyError, ValueError, tomllib.TOMLDecodeError) as error:
            result = {"schema_version": 1, "dependency": "office2pdf", "status": "rejected", "reasons": [str(error)]}
    else:
        try:
            result = monitor(ROOT)
        except (KeyError, ValueError, tomllib.TOMLDecodeError) as error:
            result = {"schema_version": 1, "dependency": "office2pdf", "status": "rejected", "reasons": [str(error)]}
    rendered = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(rendered, encoding="utf-8")
    else:
        print(rendered, end="")
    if args.github_output:
        candidate = result.get("candidate", {})
        args.github_output.write_text(f"status={result['status']}\ncandidate_version={candidate.get('version', '')}\n", encoding="utf-8")
    return 0 if result["status"] in {"current", "eligible", "deferred", "prepared"} else 1


if __name__ == "__main__":
    raise SystemExit(main())
