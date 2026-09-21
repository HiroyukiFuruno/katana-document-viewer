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
    "needs.discover.outputs.status == 'eligible'",
    "--prepare-candidate",
    "cargo update -p office2pdf --precise",
    "just JOBS=2 check",
    "just coverage",
    "just release-contract-check",
    "cargo package -p katana-document-viewer --locked --allow-dirty",
    "cargo publish -p katana-document-viewer --dry-run --locked --allow-dirty",
    "gh pr create --draft",
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
    return []


def self_test() -> None:
    valid = "\n".join(REQUIRED) + "\ncargo update -p office2pdf --precise\njust JOBS=2 check\ngh pr create --draft\n"
    assert not errors(valid)
    assert errors(valid.replace("just coverage", ""))
    assert errors(valid.replace("cargo update -p office2pdf --precise", ""))


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
