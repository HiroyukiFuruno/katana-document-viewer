#!/usr/bin/env bash
set -euo pipefail

version="$(bash "$(dirname "$0")/verify-version.sh" "${1:-}" | awk -F= '$1 == "version_bare" { print $2 }')"

require_token() {
  if [[ -n "${CARGO_REGISTRY_TOKEN:-}" ]]; then
    return
  fi
  echo "CARGO_REGISTRY_TOKEN is required." >&2
  exit 1
}

if cargo info "kdv@${version}" --registry crates-io >/dev/null 2>&1; then
  echo "kdv ${version} already published; skipping."
  exit 0
fi

require_token
cargo publish -p kdv --locked --token "${CARGO_REGISTRY_TOKEN}"
for _ in {1..30}; do
  if cargo info "kdv@${version}" --registry crates-io >/dev/null 2>&1; then
    exit 0
  fi
  sleep 10
done

echo "kdv ${version} did not become visible on crates.io in time." >&2
exit 1
