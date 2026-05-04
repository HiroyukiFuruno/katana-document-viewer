## Why

KatanA v0.26.0 で `katana-ui` から preview 実装を切り出す。切り出し先をこの repo とし、KatanA は git dependency として consume するだけにする。KatanA の検証範囲から preview の実装詳細を除く。

## What Changes

- KatanA `katana-ui` の preview 描画コード（`egui_commonmark` ラップ、ダイアグラム呼び出し、絵文字ハック等）を `katana-document-preview-egui` へ移管する。
- `katana-document-preview`（neutral interface）の trait と DTO を確定する。
- `MarkdownPreviewWidget::show(ui, source, config)` を KatanA が呼ぶ唯一のエントリポイントにする。
- ダイアグラム描画は `katana-canvas-forge` 経由に統一する（preview crate 内で独自 Mermaid renderer を持たない）。
- `vendor/egui_commonmark_upstream` を katana-document-preview-egui の dependency に移し、KatanA ルートの `[patch.crates-io]` から除去する。
- `v0.1.0` として release tag を切る。

## Capabilities

### New Capabilities

- `markdown-preview-component`: neutral interface + egui MVP 実装。

## Impact

- `crates/katana-document-preview/` — neutral interface crate
- `crates/katana-document-preview-egui/` — egui 実装 crate
