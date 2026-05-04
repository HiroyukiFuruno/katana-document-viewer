## Why

preview 実装を独立した crate として確立する。`katana-document-preview` は Markdown に限らず、あらゆるドキュメント形式のプレビューを扱う汎用 widget として設計する。v0.1.0 は Markdown 中心だが、PDF / Draw.io / Word / Excel / PPT / CSV 等を順次対応していくための neutral interface を最初から設計する。

## What Changes

- `katana-document-preview`（neutral interface、egui 非依存）に以下を定義する：
  - `DocumentPreview` trait（フォーマット非依存の preview widget 契約）
  - `PreviewConfig`（テーマ・フォントサイズ等の注入）
  - `PreviewSource`（Markdown / 画像 / PDF / Binary 等を統一的に扱う enum）
  - kcf の `Renderer` trait 経由で図表描画を呼び出す契約
- `katana-document-preview-egui` に以下を実装する（v0.1.0 は Markdown 中心）：
  - `egui_commonmark` ラップ、絵文字ハック（Twemoji 等アセット置換）
  - ダイアグラムブロックの kcf 呼び出し
  - 画像 preview（egui Image）
  - 将来フォーマットの拡張ポイント（`PreviewSource` variant 追加で対応）
- `vendor/egui_commonmark_upstream` を KatanA から除去し、この repo の dependency に移す
- `v0.1.0` として release tag を切る

## Capabilities

### New Capabilities（v0.1.0）

- `markdown-preview-component`: neutral interface + egui MVP 実装（Markdown 中心）
- `image-preview`: 画像ファイルの inline preview
- `diagram-preview`: kcf `Renderer` trait 経由の Mermaid / Draw.io 描画

### Planned（将来バージョン）

- `pdf-preview`: PDF レンダリング
- `office-preview`: Word / Excel / PPT / CSV

## Known Constraints（egui MVP 段階）

egui はカラー絵文字（Apple Color Emoji 等）を OS フォントフォールバック経由では描画できないため、Markdown 内の絵文字は Twemoji 等のアセット画像で代替する。根本解決は `katana-document-preview-floem` 実装時。

## Impact

- `crates/katana-document-preview/` — neutral interface crate（egui 非依存）
- `crates/katana-document-preview-egui/` — egui 実装 crate
