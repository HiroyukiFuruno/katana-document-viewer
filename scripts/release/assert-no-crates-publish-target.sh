#!/usr/bin/env bash
set -euo pipefail

if grep -R "cargo publish" .github/workflows scripts/release Justfile | grep -v -- "--dry-run" | grep -v "publish target is disabled" >/dev/null; then
  echo "crates.io publish command must stay disabled until crate naming is fixed." >&2
  exit 1
fi

echo "crates.io publish target is disabled"
