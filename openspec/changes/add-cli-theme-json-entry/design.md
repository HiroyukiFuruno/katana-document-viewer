## Context

`add-complete-theme-contract` ではCLIの主導線を `--light` / `--dark` にする。完全テーマJSONは必要になる可能性があるが、v0.1.0の最短経路ではない。

## Goals / Non-Goals

**Goals:**

- CLI option名を `--theme` に固定する。
- JSON入力は完全な `KdvThemeSnapshot` と同じ構造にする。
- 欠落fieldを拒否する。

**Non-Goals:**

- `--light` / `--dark` の実装はこのchangeで扱わない。
- partial themeやpreset差分JSONは扱わない。
- `--thema` aliasは追加しない。

## Decisions

### option名は `--theme` にする

会話中に `--thema` 表記があったが、実装名としては一般的な `--theme` を採用する。

### JSONもcomplete theme objectにする

CLI JSONだけ部分指定を許すと、app/API側で禁止した欠落補完がCLIから再侵入する。そのためJSONも完全テーマだけを受け付ける。

## Risks / Trade-offs

- JSONが長くなるが、壊れ方を明確にするため完全指定を優先する。
- JSON validationは別changeで扱うため、現在のCLI実装は `--light` / `--dark` に限定される。
