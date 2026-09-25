#!/usr/bin/env python3
"""Check the scheduled office2pdf monitor cannot create an unverified PR."""

from __future__ import annotations

import argparse
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github/workflows/office2pdf-upstream-monitor.yml"
REQUIRED = (
    "schedule:",
    "workflow_dispatch:",
    "contents: write",
    "pull-requests: write",
    "monitor-office2pdf-upstream.py --output monitor-result.json --github-output",
    "actions/upload-artifact@v4",
    "if: always()",
    "needs.discover.outputs.status == 'eligible'",
    "--prepare-candidate",
    "cargo update -p office2pdf --precise",
    "just JOBS=2 check",
    "just coverage",
    "just release-contract-check",
    "cargo package -p katana-document-viewer --locked --allow-dirty",
    "cargo publish -p katana-document-viewer --dry-run --locked --allow-dirty",
    "gh pr create --draft",
    "--reject-stage",
    "if: failure() && steps.checkout.outcome == 'success'",
    "Refs #45",
    "no path/git override",
    "KDV publication and KatanA adoption remain required post-merge release steps.",
)


def errors(source: str) -> list[str]:
    missing = [token for token in REQUIRED if token not in source]
    if missing:
        return [f"office2pdf monitor workflow is missing: {token}" for token in missing]
    ordered = (
        source.find("cargo update -p office2pdf --precise"),
        source.find("just JOBS=2 check"),
        source.find("gh pr create --draft"),
    )
    if not ordered[0] < ordered[1] < ordered[2]:
        return ["office2pdf monitor must update, gate, then create the draft PR in that order"]
    if "name: Upload monitor result\n        if: always()" not in source:
        return ["office2pdf monitor must upload the unavailable decision after discovery failure"]
    rejected = source.find("name: Mark failed candidate as rejected")
    uploaded = source.find("name: Upload candidate evidence on failure")
    if not ordered[2] < rejected < uploaded:
        return ["office2pdf monitor must mark failed candidates rejected before uploading diagnostics"]
    return []


def self_test() -> None:
    valid = WORKFLOW.read_text(encoding="utf-8")
    assert not errors(valid)
    assert errors(valid.replace("just coverage", ""))
    assert errors(valid.replace("cargo update -p office2pdf --precise", ""))
    assert errors(valid.replace("name: Upload monitor result\n        if: always()", "name: Upload monitor result"))
    assert errors(valid.replace("--reject-stage", "--ignore-stage"))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        print("office2pdf monitor contract self-test passed")
        return 0
    failures = errors(WORKFLOW.read_text(encoding="utf-8"))
    if failures:
        print("\n".join(f"office2pdf monitor contract: {failure}" for failure in failures))
        return 1
    print("office2pdf monitor contract passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
