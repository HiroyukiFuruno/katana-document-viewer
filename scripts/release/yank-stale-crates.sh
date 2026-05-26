#!/usr/bin/env bash
set -euo pipefail

require_token() {
  if [[ -n "${CARGO_REGISTRY_TOKEN:-}" ]]; then
    return
  fi
  echo "CARGO_REGISTRY_TOKEN is required." >&2
  exit 1
}

yank_if_exists() {
  local package="$1"
  local version="$2"
  if ! cargo info "${package}@${version}" --registry crates-io >/dev/null 2>&1; then
    echo "${package} ${version} is not published; skipping."
    return
  fi
  if cargo yank "${package}" --version "${version}" --registry crates-io --token "${CARGO_REGISTRY_TOKEN}"; then
    echo "${package} ${version} yanked."
    return
  fi
  echo "failed to yank ${package} ${version}" >&2
  failures=$((failures + 1))
}

require_token
failures=0
yank_if_exists katana-document-preview 0.1.1
yank_if_exists katana-document-preview-egui 0.1.1
yank_if_exists katana-document-viewer 0.1.0
yank_if_exists katana-document-viewer-cli 0.1.0

if [[ "${failures}" != "0" ]]; then
  echo "${failures} yank operation(s) failed." >&2
  exit 1
fi
