## Why

`v0.1.0` のrender/export foundation、`v0.2.0` のMarkdown viewer、`v0.3.0` のPDF改ページ書き出し（export）の後に、Markdown以外のドキュメント形式へ対応する。PDF・CSV・Office（DOCX / XLSX / PPTX）・画像拡張（SVG / WebP 等）のviewerを `katana-document-viewer-kuc` に実装し、`ViewerSource` variant として追加する。

## What Changes

### PDF viewer

- Rust ネイティブの PDF ライブラリ（`pdf` / `pdf-extract` 等）でページ画像を生成しKUC viewerで表示する
- ページナビゲーション（前後・ページ番号入力）を提供する
- テキストレイヤー抽出（コピー可能なテキスト選択）は将来対応として扱う

### CSV viewer

- CSV をparseしてテーブルとして表示する
- ヘッダー行の自動検出、列幅の自動調整を行う

### Office viewer（DOCX / XLSX / PPTX）

- `docx-rs` / `calamine` 等の Rust ライブラリで内容を抽出し、テキスト・表・画像として表示する
- 完全なレイアウト再現は目標にしない。内容の可読性を優先する

### 画像拡張

- SVG のネイティブ描画（`resvg` 等）
- WebP / AVIF 等の追加フォーマット対応

## Capabilities

### New Capabilities

- `pdf-viewer`: PDF ページ表示・ナビゲーション
- `csv-viewer`: CSV テーブル表示
- `office-viewer`: DOCX / XLSX / PPTX の内容表示
- `svg-preview`: SVG ネイティブ描画

## Impact

- `crates/katana-document-viewer-kuc/` — 各フォーマットのviewer実装追加
- `crates/katana-document-viewer/src/` — `ViewerSource` enum に variant 追加
