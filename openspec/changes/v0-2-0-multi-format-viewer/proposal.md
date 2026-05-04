## Why

v0.1.0 で確立した neutral interface（`PreviewSource`）の上に、Markdown 以外のドキュメント形式への対応を追加する。PDF・CSV・Office（DOCX / XLSX / PPTX）・画像拡張（SVG / WebP 等）のプレビューを `katana-document-preview-egui` に実装し、`PreviewSource` variant として追加する。

## What Changes

### PDF preview

- Rust ネイティブの PDF ライブラリ（`pdf` / `pdf-extract` 等）でページ画像を生成し egui Image で表示する
- ページナビゲーション（前後・ページ番号入力）を提供する
- テキストレイヤー抽出（コピー可能なテキスト選択）は将来対応として扱う

### CSV preview

- CSV を paresしてテーブルとして表示する（`egui` table widget）
- ヘッダー行の自動検出、列幅の自動調整を行う

### Office preview（DOCX / XLSX / PPTX）

- `docx-rs` / `calamine` 等の Rust ライブラリで内容を抽出し、テキスト・表・画像として表示する
- 完全なレイアウト再現は目標にしない。内容の可読性を優先する

### 画像拡張

- SVG のネイティブ描画（`resvg` 等）
- WebP / AVIF 等の追加フォーマット対応

## Capabilities

### New Capabilities

- `pdf-preview`: PDF ページ表示・ナビゲーション
- `csv-preview`: CSV テーブル表示
- `office-preview`: DOCX / XLSX / PPTX の内容表示
- `svg-preview`: SVG ネイティブ描画

## Impact

- `crates/katana-document-preview-egui/` — 各フォーマットの preview 実装追加
- `crates/katana-document-preview/src/` — `PreviewSource` enum に variant 追加
