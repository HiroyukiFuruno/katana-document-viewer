## Why

KDVのHTML書き出しと将来のviewer表示で、色の欠落や暗黙fallbackにより見た目が壊れる経路をなくす。

CLIは簡単に使える入口、appは完全なテーマ（theme）契約を必須にすることで、利用者ごとの責務を分ける。

## What Changes

- `katana-light` を light 側の基準presetにする。
- CLI入口はテーマ指定を必須にせず、`--light` / `--dark` の二択を主導線にする。
- CLI入口で省略した場合は `--light` と同じ扱いにする。
- app/API入口はテーマを必須にし、全ての配色を省略なしで受け取る。
- app/API入口では部分指定や暗黙fallbackで配色を補完しない。
- app/API入口から渡された完全テーマをKDR `RenderContext.theme` へ渡し、暗色表示かどうかはKDR `RenderThemeSnapshot.mode` に委譲する。
- `--theme <json>` はこのchangeに含めず、別changeで扱う。
- `--thema` は採用しない。実装前の表記確認結果として、CLIの将来拡張名は `--theme` に固定する。

## Capabilities

### New Capabilities

- `complete-theme-contract`: CLI向けtheme modeとapp向けcomplete theme objectの公開API契約を定義する。

### Modified Capabilities

- `render-export-foundation`: HTML/PDF/PNG/JPG export requestがapp向けcomplete theme objectを必須入力として受け取る。

## Impact

- `crates/katana-document-viewer` の公開API。
- `CliRequest` / `ExportRequest` / export debug example。
- HTML export stylesheetとKDR diagram theme mode。
- OpenSpec `render-export-foundation` のexport request契約。
