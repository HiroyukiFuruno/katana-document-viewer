## Why

viewer実装を独立したcrateとして確立する。`katana-document-preview` は未リリース・未取り込みのため、`katana-document-viewer`（KDV）へ改名してからv0.1.0を切る。v0.1.0はMarkdown viewerとexport pipeline方針を中心にし、KMM public DTOを正本入力にする。

## What Changes

- `katana-document-viewer`（neutral interface、egui非依存）に以下を定義する：
  - `DocumentViewer` trait（KMM DTOを入力にするviewer契約）
  - `ViewerConfig`（テーマ・フォントサイズ等の注入）
  - `ViewerSource`（KMM document、画像、PDF、Binary等を統一的に扱うenum）
  - `ExportConfig`（HTML/PDF/PNG/JPG export設定）
  - KCFの外部描画結果をviewer/export pipelineへ組み込む契約
- `katana-document-viewer-floem` に以下を実装する（v0.1.0 はMarkdown中心）：
  - KMM node rendering
  - hit-test metadata
  - unresolved metadata表示
  - viewer/export共通render pipelineの土台
- `katana-document-preview-egui` と `egui_commonmark` vendor patchを正規経路にしない
- `v0.1.0` として release tag を切る

## Capabilities

### New Capabilities（v0.1.0）

- `markdown-viewer-component`: KMM DTOを入力にするneutral interface + Floem viewer実装
- `markdown-viewer-export`: viewer表示と同じrender pipelineからHTML/PDF/PNG/JPG exportを行う方針
- `diagram-rendering-delegation`: KCF経由のMermaid / Draw.io / PlantUML / math外部描画

### Planned（将来バージョン）

- `pdf-viewer`: PDF表示
- `office-viewer`: Word / Excel / PPT / CSV表示
- `export-migration`: KCF既存exportのKDV移譲とKCF側削除

## Known Constraints

KDVはeditor-viewer同期制御を持たない。同期制御はKatanAが持ち、KatanAがviewerまたはeditorへ命令する。

## Impact

- `crates/katana-document-viewer/` — neutral interface crate（egui非依存）
- `crates/katana-document-viewer-floem/` — Floem viewer/export実装 crate
