#!/usr/bin/env bash

validate_backtick_field_syntax() {
  local context="$1"
  local payload="$2"
  local field="$3"
  local pattern="(^[[:space:]]*|[[:space:]]/[[:space:]]*)${field}:[[:space:]]*"
  local rest="$payload"
  local match
  local after

  while [[ "$rest" =~ $pattern ]]; do
    match="${BASH_REMATCH[0]}"
    after="${rest#*"${match}"}"
    if [[ "$after" != \`* ]]; then
      fail_fast "$context" \
        "subagent / Spark証跡の${field}: は backtick付き exact value で記録してください。"
    fi
    rest="${after#\`}"
    if [[ "$rest" != *"\`"* ]]; then
      fail_fast "$context" \
        "subagent / Spark証跡の${field}: は backtickで閉じてください。"
    fi
    rest="${rest#*\`}"
  done
}

validate_backtick_fields() {
  local context="$1"
  local payload="$2"
  shift 2
  local field

  for field in "$@"; do
    validate_backtick_field_syntax "$context" "$payload" "$field"
  done
}

validate_agent_ids() {
  local context="$1"
  local payload="$2"
  local value
  local found=0
  local agent_pattern='^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'

  while IFS= read -r value; do
    [[ -n "$value" ]] || continue
    found=1
    if [[ ! "$value" =~ $agent_pattern ]]; then
      fail_fast "$context" \
        "subagent / Spark証跡の実行agent idはUUIDだけにしてください。agent: \`<uuid>\` を必須化してください。"
    fi
  done < <(extract_backtick_field_values "agent" "$payload")

  if [[ "$found" -ne 1 ]]; then
    fail_fast "$context" \
      "subagent / Spark証跡に実行agent idがありません。agent: \`<uuid>\` を必須化してください。"
  fi
}

validate_verify_commands() {
  local context="$1"
  local payload="$2"
  local value
  local found=0

  while IFS= read -r value; do
    [[ -n "$value" ]] || continue
    found=1

    if [[ ! "$value" =~ ^rtk[[:space:]]+[^[:space:]] ]]; then
      fail_fast "$context" \
        "subagent / Spark証跡のverifyは rtk で始まる再現可能な検証コマンドだけにしてください: ${value}"
    fi

    case "$value" in
      rtk\ just\ * | rtk\ bash\ * | rtk\ cargo\ * | rtk\ rustfmt\ * | rtk\ env\ * | rtk\ git\ * | rtk\ ./*)
        ;;
      *)
        fail_fast "$context" \
          "subagent / Spark証跡のverifyは許可済みrtkコマンド形にしてください: ${value}"
        ;;
    esac

    case "$value" in
      *TODO* | *todo* | *FIXME* | *placeholder* | *not-a-command* | *未確認* | *あとで*)
        fail_fast "$context" \
          "subagent / Spark証跡のverifyに未検証の仮置き文言を入れないでください: ${value}"
        ;;
    esac
  done < <(extract_backtick_field_values "verify" "$payload")

  if [[ "$found" -ne 1 ]]; then
    fail_fast "$context" \
      "subagent / Spark証跡には verify: \`<検証コマンド>\` を必須化してください。"
  fi
}
