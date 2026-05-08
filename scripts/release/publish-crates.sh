#!/usr/bin/env bash
set -euo pipefail

version="$(bash "$(dirname "$0")/verify-version.sh" "${1:-}" | awk -F= '$1 == "version_bare" { print $2 }')"

require_token() {
  local name="$1"
  if [[ -n "${!name:-}" ]]; then
    return
  fi
  echo "${name} is required." >&2
  exit 1
}

publish_if_needed() {
  local package="$1"
  local token="$2"
  if cargo info "${package}@${version}" --registry crates-io >/dev/null 2>&1; then
    echo "${package} ${version} already published; skipping."
    return
  fi
  cargo publish -p "${package}" --locked --token "${token}"
}

wait_for_crate() {
  local package="$1"
  for _ in {1..30}; do
    if cargo info "${package}@${version}" --registry crates-io >/dev/null 2>&1; then
      return
    fi
    sleep 10
  done
  echo "${package} ${version} did not become visible on crates.io in time." >&2
  exit 1
}

require_token CARGO_REGISTRY_TOKEN
publish_if_needed katana-document-preview "${CARGO_REGISTRY_TOKEN}"
wait_for_crate katana-document-preview
publish_if_needed katana-document-preview-egui "${CARGO_REGISTRY_TOKEN}"
