#!/usr/bin/env bash
set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly HARNESS_SCRIPT="${SCRIPT_DIR}/check-subagent-spark-harness.sh"
readonly AGENT_ID="019e7c36-0a98-79b2-8b2e-bebe22223877"

fail_test() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

line_with_payload() {
  local marker="$1"
  local payload="$2"

  printf -- '- [%s] subagent証跡を残す。証跡: %s\n' "$marker" "$payload"
}

valid_payload() {
  printf 'agent: `%s` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `dummy` / command: `multi_agent_v1.spawn_agent` / verify: `rtk just check-subagent-harness` / close: `multi_agent_v1.close_agent`' "$AGENT_ID"
}

write_workspace() {
  local workspace="$1"
  local evidence="$2"

  mkdir -p \
    "${workspace}/.codex/workflows" \
    "${workspace}/.github/workflows" \
    "${workspace}/openspec/changes/subagent-harness"
  : >"${workspace}/dummy"
  cat >"${workspace}/Justfile" <<'JUSTFILE'
check: check-subagent-harness

check-subagent-harness:
    bash scripts/check-subagent-spark-harness.sh
    bash scripts/check-subagent-spark-harness-tests.sh
    bash scripts/check-subagent-spark-harness-edge-tests.sh
    bash scripts/check-subagent-spark-harness-policy-tests.sh
    bash scripts/check-subagent-spark-harness-change-tests.sh
    bash scripts/check-subagent-spark-harness-verify-tests.sh
    bash scripts/check-subagent-spark-harness-coverage-tests.sh
    bash scripts/check-subagent-spark-harness-ci-tests.sh
    bash scripts/check-subagent-spark-harness-diff-tests.sh
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
- OpenSpec証跡には `file:` と command: `multi_agent_v1.spawn_agent` と close: `multi_agent_v1.close_agent` を残し、検証コマンドは `verify:` に分ける。
- 移譲 / 委譲 / worker / explorer も subagent証跡の対象にする。
- 作業済みactive `tasks.md` は `<!-- subagent-spark-harness-strict-start -->` 以降の作業行に `証跡:` または `delegation-exception:` を必須化する。
- 親モデル継承や `fork_context` 制約を理由に、model / reasoning の明示を省略しない。
- 例外は、単純な一手作業、直列のクリティカルパス、書き込み範囲を明示できない作業、ユーザーがsubagent利用を禁止した場合に限る。main agent は設計、計画、レビュー、統合判断、ユーザー対話を担当する。subagent には分離できる実装・調査を渡し、同じファイルや同じ責務を重ねない。完了済み subagent は速やかに閉じ、新しい並列作業の枠を塞がない。
- `delegation-exception:` は許可済みの例外理由だけを受け付ける。
- OpenSpec証跡には `model:` と `reasoning:` を残す。
POLICY

  printf '%s\n%s\n' '<!-- subagent-spark-harness-strict-start -->' "$evidence" >"${workspace}/openspec/changes/subagent-harness/tasks.md"
  printf -- '- [/] handoff証跡を残す。証跡: %s\n' "$(valid_payload)" >"${workspace}/openspec/changes/subagent-harness/handoff.md"
}

run_harness() {
  local workspace="$1"
  KDV_SUBAGENT_HARNESS_WORKSPACE_DIR="$workspace" \
    KDV_SUBAGENT_HARNESS_CHECK_EXTERNAL=0 \
    bash "$HARNESS_SCRIPT" >"${workspace}/stdout" 2>"${workspace}/stderr"
}

expect_failure() {
  local label="$1"
  local evidence="$2"
  local expected="$3"
  local workspace
  workspace="$(mktemp -d)"
  trap 'rm -rf "$workspace"' RETURN

  write_workspace "$workspace" "$evidence"

  if run_harness "$workspace"; then
    fail_test "${label} should fail"
  fi

  if ! grep -Fq -- "$expected" "${workspace}/stderr"; then
    fail_test "${label} should explain ${expected}"
  fi
}

apply_mutation() {
  local workspace="$1"
  local mutation="$2"

  case "$mutation" in
    drop-check-dependency)
      sed -i.bak 's/check: check-subagent-harness/check: fmt-check/' \
    "${workspace}/Justfile"
      ;;
    hide-main-command-failure)
      sed -i.bak 's/^    bash scripts\/check-subagent-spark-harness.sh/    bash scripts\/check-subagent-spark-harness.sh || true/' "${workspace}/Justfile"
      ;;
    skip-ci-command)
      sed -i.bak 's/run: just check-subagent-harness/run: echo skipped/' "${workspace}/.github/workflows/test-and-build.yml"
      ;;
    optional-policy-term)
      printf '%s\n' '- Sparkを使うならmodel指定すればよい。' \
        >>"${workspace}/.codex/workflows/subagent-spark-policy.md"
      ;;
    *)
      fail_test "unknown mutation: ${mutation}"
      ;;
  esac
}

expect_mutated_failure() {
  local label="$1"
  local expected="$2"
  local mutation="$3"
  local workspace
  workspace="$(mktemp -d)"
  trap 'rm -rf "$workspace"' RETURN

  write_workspace "$workspace" "$(line_with_payload "/" "$(valid_payload)")"
  apply_mutation "$workspace" "$mutation"

  if run_harness "$workspace"; then
    fail_test "${label} should fail"
  fi

  if ! grep -Fq -- "$expected" "${workspace}/stderr"; then
    fail_test "${label} should explain ${expected}"
  fi
}

expect_pass() {
  local workspace
  workspace="$(mktemp -d)"
  trap 'rm -rf "$workspace"' RETURN

  write_workspace "$workspace" "$(line_with_payload "/" "$(valid_payload)")"
  run_harness "$workspace" || fail_test "valid subagent evidence should pass"
}

expect_pass
expect_failure "missing agent id" \
  "$(line_with_payload "/" 'model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `dummy` / command: `multi_agent_v1.spawn_agent`')" \
  "実行agent id"
expect_failure "missing Spark model" \
  "$(line_with_payload "/" "$(printf 'agent: `%s` / note: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `dummy` / command: `multi_agent_v1.spawn_agent`' "$AGENT_ID")")" \
  "modelは gpt-5.3-codex-spark"
expect_failure "contradictory Spark model" \
  "$(line_with_payload "/" "$(printf 'agent: `%s` / model: `gpt-5.3-codex` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `dummy` / command: `multi_agent_v1.spawn_agent`' "$AGENT_ID")")" \
  "modelは gpt-5.3-codex-spark"
expect_failure "missing reasoning" \
  "$(line_with_payload "/" "$(printf 'agent: `%s` / model: `gpt-5.3-codex-spark` / note: reasoning `medium` / file: `dummy` / command: `multi_agent_v1.spawn_agent`' "$AGENT_ID")")" \
  "reasoningは medium"
expect_failure "contradictory reasoning" \
  "$(line_with_payload "/" "$(printf 'agent: `%s` / model: `gpt-5.3-codex-spark` / reasoning: `high` / reasoning: `medium` / file: `dummy` / command: `multi_agent_v1.spawn_agent`' "$AGENT_ID")")" \
  "reasoningは medium"
expect_failure "missing file" \
  "$(line_with_payload "/" "$(printf 'agent: `%s` / model: `gpt-5.3-codex-spark` / reasoning: `medium`' "$AGENT_ID")")" \
  "file: フィールドは1件以上必須です。"
expect_failure "missing spawn command" \
  "$(line_with_payload "/" "$(printf 'agent: `%s` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `dummy`' "$AGENT_ID")")" \
  "commandは"
expect_failure "wrong spawn command" \
  "$(line_with_payload "/" "$(printf 'agent: `%s` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `dummy` / command: `rtk just check`' "$AGENT_ID")")" \
  "commandは"
expect_failure "spawn token outside command field" \
  "$(line_with_payload "/" "$(printf 'agent: `%s` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `dummy` / note: `multi_agent_v1.spawn_agent` / command: `rtk just check`' "$AGENT_ID")")" \
  "commandは"
expect_failure "missing file reference" \
  "$(line_with_payload "/" "$(printf 'agent: `%s` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `missing-file` / command: `multi_agent_v1.spawn_agent` / verify: `rtk just check-subagent-harness`' "$AGENT_ID")")" \
  "file参照が存在しません"
expect_failure "unfinished subagent task without reasoning" \
  "$(line_with_payload " " "$(printf 'agent: `%s` / model: `gpt-5.3-codex-spark` / file: `dummy` / command: `multi_agent_v1.spawn_agent`' "$AGENT_ID")")" \
  "reasoningは medium"
expect_failure "optional OpenSpec task term" \
  "$(printf '%s\n%s' "$(line_with_payload "/" "$(valid_payload)")" '- [/] subagentを使う場合は任意でよい。')" \
  "forbidden optional subagent policy term"
expect_failure "optional OpenSpec task term with Japanese comma" \
  "$(printf '%s\n%s' "$(line_with_payload "/" "$(valid_payload)")" '- [/] サブエージェントは、必要なら使う。')" \
  "forbidden optional subagent policy term"
expect_mutated_failure "just check without harness dependency" "just check must include check-subagent-harness" "drop-check-dependency"
expect_mutated_failure "failure-hidden harness command" "check-subagent-harness must run scripts/check-subagent-spark-harness.sh" "hide-main-command-failure"
expect_mutated_failure "CI step with wrong command" "CI must run just check-subagent-harness" "skip-ci-command"
expect_mutated_failure "conditional Spark policy term" "forbidden optional subagent policy term" "optional-policy-term"
echo "check-subagent-spark-harness-tests: ok"
