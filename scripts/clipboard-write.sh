#!/usr/bin/env bash
set -euo pipefail

if command -v pbcopy >/dev/null 2>&1; then
  pbcopy
elif command -v xclip >/dev/null 2>&1; then
  xclip -selection clipboard
elif command -v wl-copy >/dev/null 2>&1; then
  wl-copy
else
  echo "clipboard write command not found" >&2
  exit 127
fi
