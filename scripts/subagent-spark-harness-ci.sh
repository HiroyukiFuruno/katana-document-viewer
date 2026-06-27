#!/usr/bin/env bash

ci_runs_command() {
  local file="$1"
  local command="$2"

  awk -v command="$command" '
    function trim(value) {
      sub(/^[[:space:]]+/, "", value)
      sub(/[[:space:]]+$/, "", value)
      return value
    }

    function unquote(value) {
      value = trim(value)
      if (substr(value, 1, 1) == "\"" && substr(value, length(value), 1) == "\"") {
        return substr(value, 2, length(value) - 2)
      }
      if (substr(value, 1, 1) == "'"'"'" && substr(value, length(value), 1) == "'"'"'") {
        return substr(value, 2, length(value) - 2)
      }
      return value
    }

    function finish_step() {
      if (step_has_run && !step_has_if) { found = 1; exit 0 }
      step_has_run = 0
      step_has_if = 0
    }

    {
      raw = $0
      line = trim($0)
      if (line ~ /^#/) next
      if (raw ~ /^[[:space:]]*-[[:space:]]+/) {
        if (in_step) finish_step()
        in_step = 1
        step_has_if = 0
        step_has_run = 0
        step_indent = match(raw, /[^[:space:]]/)
        in_run_block = 0
        sub(/^-[[:space:]]+/, "", line)
      }

      if (in_step && line ~ /^if:[[:space:]]*/) {
        step_has_if = 1
      }

      if (in_run_block) {
        if (line == "") next
        current_indent = match($0, /[^[:space:]]/)
        if (current_indent > run_block_indent) {
          if (line == command) { step_has_run = 1 }
          next
        }
        in_run_block = 0
      }

      if (line ~ /^run:[[:space:]]*[|>]/) {
        in_run_block = 1
        run_block_indent = match($0, /[^[:space:]]/)
        next
      }

      if (line ~ /^run:[[:space:]]*/) {
        value = line
        sub(/^run:[[:space:]]*/, "", value)
        if (unquote(value) == command) { step_has_run = 1 }
      }
    }

    END {
      if (!found && in_step && step_has_run && !step_has_if) { found = 1 }
      if (!found) exit 1
    }
  ' "$file"
}
