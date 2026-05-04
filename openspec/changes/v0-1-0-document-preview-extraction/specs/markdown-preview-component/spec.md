## ADDED Requirements

### Requirement: DocumentPreview trait でフォーマット非依存の preview 契約を提供しなければならない

システムは、`DocumentPreview` trait、`PreviewConfig`、`PreviewSource`（Markdown / 画像 / PDF / Binary 等を統一的に扱う enum）を `katana-document-preview` neutral crate として提供しなければならない（MUST）。

#### Scenario: Markdown source を preview に渡す

- **WHEN** ホストが `PreviewSource::Markdown` を `DocumentPreview::render` に渡す
- **THEN** preview は Markdown を解析して描画する
- **THEN** ダイアグラムブロックは kcf の `Renderer` trait 経由で描画する

#### Scenario: katana-document-preview は egui / kcf 実装本体に依存しない

- **WHEN** `cargo tree -p katana-document-preview` を実行する
- **THEN** `egui` は含まれない
- **THEN** kcf の `Renderer` trait 定義のみ参照し、特定の Mermaid 実装に依存しない

### Requirement: katana-document-preview-egui が Markdown / 画像 / 図表 preview の egui MVP を提供しなければならない

システムは、`egui_commonmark` ラップ、絵文字ハック（Twemoji 等のアセット置換）、画像 preview、kcf 経由のダイアグラム描画を `katana-document-preview-egui` impl crate として提供しなければならない（MUST）。

#### Scenario: Markdown を egui で preview する

- **WHEN** ホストが `EguiDocumentPreview::show(ui, &PreviewSource::Markdown(...))` を呼ぶ
- **THEN** Markdown が egui ui に描画される

#### Scenario: 図形ブロックを kcf 経由で描画する

- **WHEN** preview に Mermaid / Draw.io block が含まれる
- **THEN** kcf の `Renderer` trait に図形を渡し、返された SVG を preview に組み込む
- **THEN** preview crate 内に独自 Mermaid / Draw.io 描画は含まれない

#### Scenario: egui MVP の既知制約

- **WHEN** preview にカラー絵文字が含まれる
- **THEN** Twemoji 等のアセット画像で代替する
- **THEN** 根本解決は `katana-document-preview-floem`（vello / cosmic-text）で行う
