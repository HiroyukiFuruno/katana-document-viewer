#!/usr/bin/env bash
set -euo pipefail

if command -v pbpaste >/dev/null 2>&1; then
  pbpaste
elif command -v xclip >/dev/null 2>&1; then
  xclip -selection clipboard -out
elif command -v wl-paste >/dev/null 2>&1; then
  wl-paste
else
  echo "clipboard read command not found" >&2
  exit 127
fi
