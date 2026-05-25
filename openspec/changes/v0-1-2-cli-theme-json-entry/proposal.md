## Why

CLIでも将来的に完全テーマ（complete theme object）をJSONで渡したいが、現在の主導線は `--light` / `--dark` で十分である。

このため、完全JSON入力は主線のtheme contractから分離して、別changeで扱う。

## What Changes

- 将来のCLI option名を `--theme <json>` に固定する。
- `--thema` は採用しない。
- JSONはapp/API向け `KdvThemeSnapshot` と同じ完全テーマを受け取る。
- 部分JSON、欠落field、暗黙fallbackは許可しない。

## Capabilities

### New Capabilities

- `cli-theme-json-entry`: CLIから完全テーマJSONを読み込む入口を定義する。

### Modified Capabilities

- `complete-theme-contract`: CLI簡易theme modeに加えて、将来の完全JSON入口を接続する。

## Impact

- export debug CLI。
- JSON schema / serde validation。
- `KdvThemeSnapshot` のCLI deserialization。
