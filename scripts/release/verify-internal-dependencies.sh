#!/usr/bin/env bash
set -euo pipefail

version="$(bash "$(dirname "$0")/verify-version.sh" "${1:-}" | awk -F= '$1 == "version_bare" { print $2 }')"

dependency_line="$(grep '^katana-document-preview = ' crates/katana-document-preview-egui/Cargo.toml)"
if [[ "${dependency_line}" != *"version = \"${version}\""* ]]; then
  echo "katana-document-preview-egui must depend on katana-document-preview version ${version}" >&2
  exit 1
fi

echo "internal dependency versions match ${version}"
