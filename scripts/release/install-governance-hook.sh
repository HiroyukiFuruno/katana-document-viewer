#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
managed_path="$root/.githooks"
previous_path="$(git config --get core.hooksPath || true)"
existing_hook="$(git rev-parse --path-format=absolute --git-path hooks/pre-push)"
delegate=""

if [[ "$previous_path" != "$managed_path" && -x "$existing_hook" ]]; then
  delegate="$existing_hook"
fi

if [[ -n "$delegate" ]]; then
  git config kdv.pre-push-delegate "$delegate"
  printf 'KDV governance: delegated existing pre-push hook: %s\n' "$delegate"
fi
git config core.hooksPath "$managed_path"
printf 'KDV governance: installed managed hook path: %s\n' "$managed_path"
