#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
managed_path="$root/.githooks"
managed_hook="$managed_path/pre-push"
existing_hook="$(git rev-parse --path-format=absolute --git-path hooks/pre-push)"
configured_delegate="$(git config --get kdv.pre-push-delegate || true)"
delegate=""

if [[ -n "$configured_delegate" && "$configured_delegate" -ef "$managed_hook" ]]; then
  git config --unset kdv.pre-push-delegate
fi

if [[ -x "$existing_hook" && ! "$existing_hook" -ef "$managed_hook" ]]; then
  delegate="$existing_hook"
fi

if [[ -n "$delegate" ]]; then
  git config kdv.pre-push-delegate "$delegate"
  printf 'KDV governance: delegated existing pre-push hook: %s\n' "$delegate"
fi
git config core.hooksPath "$managed_path"
printf 'KDV governance: installed managed hook path: %s\n' "$managed_path"
