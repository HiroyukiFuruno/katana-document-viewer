## Context

KDVには2種類の入口がある。

CLI入口は人がコマンドで使うため、簡単な指定が必要になる。app/API入口はKatanAなどのホストが使うため、欠落した配色をKDVが補完しない明示契約が必要になる。

## Goals / Non-Goals

**Goals:**

- CLI入口で `Light` / `Dark` を選べる。
- CLI入口で未指定なら `katana-light` を使う。
- app/API入口で完全なテーマ（complete theme object）を必須にする。
- app/API入口のテーマは全配色をnot null fieldとして受ける。
- HTML exportとdiagram themeが同じtheme modeと配色から決まる。

**Non-Goals:**

- `--theme <json>` の読み込みはこのchangeで実装しない。
- 部分テーマ、差分テーマ、fallback chainは実装しない。
- KUC viewer本体の表示実装はこのchangeで扱わない。

## Decisions

### app/API入口は完全テーマを必須にする

`ExportRequest` は `KdvThemeSnapshot` を必須fieldとして持つ。`KdvThemeSnapshot` の配色fieldは全て `String` で、`Option` を使わない。

これにより、app側は全配色を明示してからKDVを呼ぶ。KDVは欠落値を推測しない。

### CLI入口はtheme modeだけを受ける

CLI用には `CliThemeMode::Light` / `CliThemeMode::Dark` を用意する。CLI requestではtheme modeを省略可能にし、省略時は `Light` に正規化する。

CLIからapp/API側の `ExportRequest` へ渡す時点で、`CliThemeMode` を完全な `KdvThemeSnapshot` へ変換する。

### diagram renderingへ完全テーマを渡す

KDVは `KdvThemeSnapshot` からKRR `RenderThemeSnapshot` を生成し、`RenderContext.theme` としてKRRへ渡す。暗色表示かどうかはKRR側で `RenderThemeSnapshot.mode` から判断するため、KDVは別の `dark-mode` fallbackを持たない。

### `katana-light` をlight presetの正本にする

`katana-light` はCLI省略時と `--light` の出力基準になる。`katana-dark` は `--dark` の出力基準になる。

presetはCLI入口の簡易指定を完全テーマへ変換するためにだけ使う。app/API入口ではpreset名だけを受け取らない。

### 将来の完全JSON入口は `--theme` にする

会話中に `--thema` 表記があったが、実装名としては採用しない。将来の完全JSON入力は `--theme <json>` とし、別change `v0-1-2-cli-theme-json-entry` で扱う。

## Risks / Trade-offs

- `ExportRequest` にtheme必須fieldを追加するため、既存呼び出し側の修正が必要になる。
- CLIとapp/APIの入口が分かれるため型は増える。ただし、欠落配色の暗黙補完を防ぐ効果を優先する。
- theme fieldの数は増えるが、部分指定よりも壊れ方が明確になる。
