## ADDED Requirements

### Requirement: DocumentViewer trait でフォーマット非依存のviewer契約を提供しなければならない

システムは、`DocumentViewer` trait、`ViewerConfig`、`ViewerSource`（KME document / 画像 / PDF / Binary 等を統一的に扱う enum）を `katana-document-viewer` neutral crate として提供しなければならない（MUST）。

#### Scenario: KME documentをviewerに渡す

- **WHEN** ホストが `ViewerSource::KmeDocument` を `DocumentViewer::render` に渡す
- **THEN** viewer はKME public DTOを描画する
- **THEN** ダイアグラムブロックはKCFの外部描画結果を組み込む

#### Scenario: katana-document-viewer は egui / kcf 実装本体に依存しない

- **WHEN** `cargo tree -p katana-document-viewer` を実行する
- **THEN** `egui` は含まれない
- **THEN** kcf の `Renderer` trait 定義のみ参照し、特定の Mermaid 実装に依存しない

### Requirement: katana-document-viewer-floem がMarkdown viewerとexport pipelineを提供しなければならない

システムは、KME node rendering、hit-test metadata、unresolved metadata表示、KCF経由の外部描画組み込み、HTML/PDF/PNG/JPG export pipelineを `katana-document-viewer-floem` impl crate として提供しなければならない（MUST）。

#### Scenario: MarkdownをFloem viewerで表示する

- **WHEN** ホストが `FloemDocumentViewer` に `ViewerSource::KmeDocument` を渡す
- **THEN** Markdown documentがFloem viewerに描画される
- **THEN** rendered node はKME node idとsource rangeへ戻れる

#### Scenario: 図形ブロックを kcf 経由で描画する

- **WHEN** viewer に Mermaid / Draw.io / PlantUML / math block が含まれる
- **THEN** KCFの外部描画結果をviewer/exportへ組み込む
- **THEN** viewer crate 内に独自 Mermaid / Draw.io 描画は含まれない

#### Scenario: KatanAがeditor-viewer同期を制御する

- **WHEN** viewer上のnode selectionやscrollが必要になる
- **THEN** KDVはhit-test metadataとviewer command surfaceを提供する
- **THEN** KatanAがviewerまたはeditorへ命令する
- **THEN** KDVはKLEやKatanA統合状態を知らない
