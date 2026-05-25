## ADDED Requirements

### Requirement: ViewerSource を拡張して PDF / CSV / Office / SVG viewerを提供しなければならない

システムは、`ViewerSource` enum に `Pdf` / `Csv` / `Office`（DOCX / XLSX / PPTX）/ `Svg` の variant を追加し、各フォーマットのviewerを `katana-document-viewer-kuc` で提供しなければならない（MUST）。

#### Scenario: PDF を preview する

- **WHEN** ホストが `ViewerSource::Pdf(path)` を渡す
- **THEN** PDF ページが画像として表示される
- **THEN** ページナビゲーション（前後・ページ番号入力）が利用できる

#### Scenario: CSV をテーブル表示する

- **WHEN** ホストが `ViewerSource::Csv(path)` を渡す
- **THEN** CSV がviewer上のtableとして表示される
- **THEN** ヘッダー行の自動検出と列幅の自動調整が適用される

#### Scenario: Office ドキュメントを表示する

- **WHEN** ホストが `ViewerSource::Office(path)`（DOCX / XLSX / PPTX）を渡す
- **THEN** ドキュメントの内容（テキスト・表・画像）が表示される
- **THEN** 完全なレイアウト再現は目標にせず、可読性を優先する

#### Scenario: SVG をネイティブ描画する

- **WHEN** ホストが `ViewerSource::Svg(path)` を渡す
- **THEN** SVG が `resvg` 等のネイティブライブラリで描画される
