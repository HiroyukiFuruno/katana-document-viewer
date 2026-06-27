#!/usr/bin/env bash

FORBIDDEN_DELEGATION_REGEXES=(
  '(sub[[:space:]_-]*agent|サブ[[:space:]_-]*エージェント).{0,24}(任意|必要なら|必要ならば|必要な場合|必要に応じ|使う場合|使うなら)'
  '(任意|必要なら|必要ならば|必要な場合|必要に応じ).{0,24}(sub[[:space:]_-]*agent|サブ[[:space:]_-]*エージェント)'
  '(if[[:space:]]+(needed|required)([[:space:]]+then)?|when[[:space:]]+(needed|required)|as[[:space:]]+needed|optional).{0,24}sub[[:space:]_-]*agent'
  'sub[[:space:]_-]*agent.{0,24}(if[[:space:]]+(needed|required)|when[[:space:]]+(needed|required)|as[[:space:]]+needed|optional)'
  '(Spark|gpt-5[.]3-codex-spark).{0,24}(任意|必要なら|必要ならば|必要な場合|必要に応じ|使う場合|使うなら|if[[:space:]]+(needed|required)|when[[:space:]]+(needed|required)|as[[:space:]]+needed|optional)'
  '(任意|必要なら|必要ならば|必要な場合|必要に応じ|使う場合|使うなら|if[[:space:]]+(needed|required)|when[[:space:]]+(needed|required)|as[[:space:]]+needed|optional).{0,24}(Spark|gpt-5[.]3-codex-spark)'
  'Spark.{0,24}(指定がある場合|未指定なら|環境やAGENTS)'
)

check_forbidden_regex_terms_absent() {
  local file="$1"
  local pattern
  local match
  local line

  for pattern in "${FORBIDDEN_DELEGATION_REGEXES[@]}"; do
    match="$(grep -Eni "$pattern" "$file" || true)"
    [[ -n "$match" ]] || continue

    while IFS= read -r line; do
      case "$line" in
        *禁止* | *拒否* | *"fail fast"* | *対象* | *検出*)
          continue
          ;;
      esac
      fail_fast "$file" \
        "forbidden optional subagent policy term is present: $line"
    done <<<"$match"
  done
}
