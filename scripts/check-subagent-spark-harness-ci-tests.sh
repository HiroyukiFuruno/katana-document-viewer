#!/usr/bin/env bash
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/subagent-spark-harness-ci.sh"

readonly COMMAND="just check-subagent-harness"

fail_test() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

write_workflow() {
  local file="$1"
  local body="$2"

  printf '%s\n' "$body" >"$file"
}

expect_pass() {
  local label="$1"
  local body="$2"
  local workspace
  workspace="$(mktemp -d)"
  trap "rm -rf '$workspace'" RETURN

  write_workflow "${workspace}/workflow.yml" "$body"
  ci_runs_command "${workspace}/workflow.yml" "$COMMAND" ||
    fail_test "${label} should pass"
}

expect_failure() {
  local label="$1"
  local body="$2"
  local workspace
  workspace="$(mktemp -d)"
  trap "rm -rf '$workspace'" RETURN

  write_workflow "${workspace}/workflow.yml" "$body"
  if ci_runs_command "${workspace}/workflow.yml" "$COMMAND"; then
    fail_test "${label} should fail"
  fi
}

expect_pass "plain run" '
jobs:
  test:
    steps:
      - name: Run subagent/Spark harness check
        run: just check-subagent-harness'
expect_pass "job-level if does not hide step" '
jobs:
  test:
    if: github.event_name == "pull_request"
    steps:
      - name: Run subagent/Spark harness check
        run: just check-subagent-harness'
expect_pass "block run" '
jobs:
  test:
    steps:
      - name: Run subagent/Spark harness check
        run: |
          just check-subagent-harness'
expect_failure "step if false hides harness" '
jobs:
  test:
    steps:
      - name: Run subagent/Spark harness check
        if: false
        run: just check-subagent-harness'
expect_failure "step expression hides harness" '
jobs:
  test:
    steps:
      - name: Run subagent/Spark harness check
        if: matrix.os == "ubuntu-latest"
        run: just check-subagent-harness'
expect_failure "step if after run hides harness" '
jobs:
  test:
    steps:
      - name: Run subagent/Spark harness check
        run: just check-subagent-harness
        if: false'

echo "check-subagent-spark-harness-ci-tests: ok"
