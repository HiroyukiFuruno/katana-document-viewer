#!/usr/bin/env bash
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly HARNESS_SCRIPT="${SCRIPT_DIR}/check-subagent-spark-harness.sh"
readonly AGENT_ID="019e7c36-0a98-79b2-8b2e-bebe22223877"
fail_test() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}
write_workspace() {
  local workspace="$1"
  local change_path="$2"
  local evidence="$3"
  mkdir -p \
    "${workspace}/.codex/workflows" \
    "${workspace}/.github/workflows" \
    "${workspace}/${change_path}"
  : >"${workspace}/dummy"
  cat >"${workspace}/Justfile" <<'JUSTFILE'
check: check-subagent-harness

check-subagent-harness:
    bash scripts/check-subagent-spark-harness-tests.sh
    bash scripts/check-subagent-spark-harness-edge-tests.sh
    bash scripts/check-subagent-spark-harness-policy-tests.sh
    bash scripts/check-subagent-spark-harness-change-tests.sh
    bash scripts/check-subagent-spark-harness-verify-tests.sh
    bash scripts/check-subagent-spark-harness-coverage-tests.sh
    bash scripts/check-subagent-spark-harness-ci-tests.sh
    bash scripts/check-subagent-spark-harness-diff-tests.sh
    bash scripts/check-subagent-spark-harness.sh
JUSTFILE
  cat >"${workspace}/.github/workflows/test-and-build.yml" <<'WORKFLOW'
name: test
jobs:
  test:
    steps:
      - name: Run subagent/Spark harness check
        run: just check-subagent-harness
WORKFLOW
  cat >"${workspace}/.codex/workflows/subagent-spark-policy.md" <<'POLICY'
- 許可済みの作業で、分離できる実装・調査は原則 subagent へ移譲する。
- 作業済みactive tasks.mdには少なくとも1件のsubagent / Spark証跡を残す。
- subagent は `gpt-5.3-codex-spark` / reasoning `medium` を明示する。
- OpenSpec証跡には `file:` と command: `multi_agent_v1.spawn_agent` と close: `multi_agent_v1.close_agent` を残す。
- OpenSpec証跡の検証コマンドは `verify:` に分ける。
- 移譲 / 委譲 / worker / explorer も subagent証跡の対象にする。
- 作業済みactive `tasks.md` は `<!-- subagent-spark-harness-strict-start -->` 以降の作業行に `証跡:` または `delegation-exception:` を必須化する。
- 親モデル継承や `fork_context` 制約を理由に、model / reasoning の明示を省略しない。
- 例外は、単純な一手作業、直列のクリティカルパス、書き込み範囲を明示できない作業、ユーザーがsubagent利用を禁止した場合に限る。
- `delegation-exception:` は許可済みの例外理由だけを受け付ける。
- main agent は設計、計画、レビュー、統合判断、ユーザー対話を担当する。
- subagent には分離できる実装・調査を渡し、同じファイルや同じ責務を重ねない。
- 完了済み subagent は速やかに閉じ、新しい並列作業の枠を塞がない。
- OpenSpec証跡には `model:` と `reasoning:` を残す。
POLICY
  printf '%s\n%s\n' '<!-- subagent-spark-harness-strict-start -->' "$evidence" >"${workspace}/${change_path}/tasks.md"
  printf -- '- [/] handoff証跡を残す。証跡: agent: `%s` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `dummy` / command: `multi_agent_v1.spawn_agent` / verify: `rtk just check-subagent-harness` / close: `multi_agent_v1.close_agent`\n' "$AGENT_ID" >"${workspace}/${change_path}/handoff.md"
}
run_harness() {
  local workspace="$1"
  KDV_SUBAGENT_HARNESS_WORKSPACE_DIR="$workspace" \
    KDV_SUBAGENT_HARNESS_CHECK_EXTERNAL=0 \
    bash "$HARNESS_SCRIPT" >"${workspace}/stdout" 2>"${workspace}/stderr"
}
expect_failure_contains() {
  local workspace="$1"
  local expected="$2"
  local label="$3"

  if run_harness "$workspace"; then fail_test "${label} should fail"; fi
  if ! grep -q "$expected" "${workspace}/stderr"; then
    fail_test "${label} should explain ${expected}"
  fi
}
expect_delegation_keyword_without_subagent_is_checked() {
  local workspace
  workspace="$(mktemp -d)"
  trap 'rm -rf "$workspace"' RETURN

  write_workspace "$workspace" \
    "openspec/changes/delegation-keyword" \
    "- [/] workerへ移譲するが証跡を省略する。"

  expect_failure_contains "$workspace" "証跡" "delegation keyword without evidence"
}

expect_deep_change_file_is_checked() {
  local workspace
  workspace="$(mktemp -d)"
  trap 'rm -rf "$workspace"' RETURN

  write_workspace "$workspace" \
    "openspec/changes/group/deep-change" \
    "- [/] subagent証跡を深い階層に残す。証跡: agent: \`${AGENT_ID}\` / model: \`gpt-5.3-codex-spark\` / file: \`dummy\` / command: \`multi_agent_v1.spawn_agent\` / verify: \`rtk just check-subagent-harness\`"

  expect_failure_contains "$workspace" "reasoningは medium" "deep change file"
}

expect_uppercase_done_state_is_checked() {
  local workspace
  workspace="$(mktemp -d)"
  trap 'rm -rf "$workspace"' RETURN

  write_workspace "$workspace" \
    "openspec/changes/uppercase-task-state" \
    "- [X] Spark証跡を大文字完了で残す。証跡: agent: \`${AGENT_ID}\` / model: \`gpt-5.3-codex-spark\` / file: \`dummy\` / command: \`multi_agent_v1.spawn_agent\` / verify: \`rtk just check-subagent-harness\`"

  expect_failure_contains "$workspace" "reasoningは medium" "uppercase task state"
}

expect_delegate_marker_is_checked() {
  local workspace
  workspace="$(mktemp -d)"
  trap 'rm -rf "$workspace"' RETURN

  write_workspace "$workspace" \
    "openspec/changes/delegate-marker" \
    "- [/] delegate=subagent の作業証跡を省略する。"

  expect_failure_contains "$workspace" "証跡" "delegate marker"
}

expect_suspended_task_state_is_checked() {
  local workspace
  workspace="$(mktemp -d)"
  trap 'rm -rf "$workspace"' RETURN

  write_workspace "$workspace" \
    "openspec/changes/suspended-task-state" \
    "- [-] workerへ移譲するが証跡を省略する。"

  expect_failure_contains "$workspace" "証跡" "suspended task state"
}

expect_mixed_spawn_and_verification_command_is_rejected() {
  local workspace
  workspace="$(mktemp -d)"
  trap 'rm -rf "$workspace"' RETURN

  write_workspace "$workspace" \
    "openspec/changes/mixed-command" \
    "- [/] subagent証跡に検証コマンドを混ぜる。証跡: agent: \`${AGENT_ID}\` / model: \`gpt-5.3-codex-spark\` / reasoning: \`medium\` / file: \`dummy\` / command: \`multi_agent_v1.spawn_agent\` / command: \`rtk just check-subagent-harness\` / verify: \`rtk just check-subagent-harness\` / close: \`multi_agent_v1.close_agent\`"

  expect_failure_contains "$workspace" "検証コマンドは verify:" "mixed command"
}

expect_optional_delegation_variants_are_rejected() {
  local workspace
  local variant
  workspace="$(mktemp -d)"
  trap 'rm -rf "$workspace"' RETURN

  for variant in \
    "subagentは任意運用" \
    "必要なら subagent を使う" \
    "必要に応じて subagent も使う" \
    "if needed, use subagent" \
    "サブエージェントが必要な場合は使う"; do
    write_workspace "$workspace" \
      "openspec/changes/optional-delegation-variants" \
      "- [/] ${variant}。証跡: agent: \`${AGENT_ID}\` / model: \`gpt-5.3-codex-spark\` / reasoning: \`medium\` / file: \`dummy\` / command: \`multi_agent_v1.spawn_agent\` / verify: \`rtk just check-subagent-harness\` / close: \`multi_agent_v1.close_agent\`"
    expect_failure_contains "$workspace" \
      "forbidden optional subagent policy term" \
      "optional delegation variant ${variant}"
  done
}

expect_forced_external_policy_requires_files() {
  local workspace
  workspace="$(mktemp -d)"
  trap 'rm -rf "$workspace"' RETURN

  write_workspace "$workspace" \
    "openspec/changes/missing-external-policy" \
    "- [/] subagent証跡を残す。証跡: agent: \`${AGENT_ID}\` / model: \`gpt-5.3-codex-spark\` / reasoning: \`medium\` / file: \`dummy\` / command: \`multi_agent_v1.spawn_agent\` / verify: \`rtk just check-subagent-harness\` / close: \`multi_agent_v1.close_agent\`"

  if HOME="${workspace}/home" \
    KDV_SUBAGENT_HARNESS_WORKSPACE_DIR="$workspace" \
    KDV_SUBAGENT_HARNESS_CHECK_EXTERNAL=1 \
    bash "$HARNESS_SCRIPT" >"${workspace}/stdout" 2>"${workspace}/stderr"; then
    fail_test "forced external policy without files should fail"
  fi

  if ! grep -q "missing external policy file" "${workspace}/stderr"; then
    fail_test "forced external policy should explain missing external file"
  fi
}

expect_delegation_keyword_without_subagent_is_checked
expect_deep_change_file_is_checked
expect_uppercase_done_state_is_checked
expect_delegate_marker_is_checked
expect_suspended_task_state_is_checked
expect_mixed_spawn_and_verification_command_is_rejected
expect_optional_delegation_variants_are_rejected
expect_forced_external_policy_requires_files

echo "check-subagent-spark-harness-edge-tests: ok"
