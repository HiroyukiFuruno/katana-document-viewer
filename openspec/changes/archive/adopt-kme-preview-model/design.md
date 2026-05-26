## Context

> Status: このchangeは初期構想の整理用としてarchive済み。現在の実装順序とactive/archive状態は `openspec list` と `openspec/changes/archive/` を正とする。

`katana-document-preview` は未リリース・未取り込みのため、`katana-document-viewer`（KDV）へ改名する。

KDVはMarkdown以外のdocument viewerも担うが、KMM採用ではMarkdown viewerの文書構造、hit-test、metadata表示、HTML/PDF/PNG/JPG exportが中心になる。表示はKUCを前提にし、egui_commonmarkのvendor patchへ戻らない。

## Goals

- KMM文書モデルをKUCで表示する。
- viewer表示とHTML/PDF/PNG/JPG exportを同じrender pipelineに寄せる。
- KMM node idとsource rangeをhit-testに使う。
- hover、選択、AST単位コピーの入口を提供する。
- unresolved metadataを画面に表示できる。
- P0 `katana-ast-lint` を品質ゲートにする。

## Non-Goals

- KMM parserをkdp内に再実装すること。
- metadata schemaをkdp内で定義すること。
- egui rendererをKMM採用の正規経路にすること。
- editor-viewer同期制御をKDVが担当すること。

## Decisions

### KMM Model Input

KDVはKMM public DTOをviewer/export inputとして受け取る。KMM内部parser型やthird-party AST型は受け取らない。

### P3 Consumer Order

KDVのKMM採用はP3作業とする。P0 `katana-ast-lint`、P1 KMM文書モデル、P2 `katana-ui-core` の境界を受け、viewer固有のUI部品をKatanA本体へ増やしすぎない。

### Hit-test Metadata

表示されたnodeはKMM node id、source range、rendered rect identityを持つ。KatanA UIはこれを使ってhover、選択、copy/edit actionへ接続する。

KDVは同期制御を持たない。KatanAがKMMの位置情報を使い、viewerまたはeditorへscroll、selection、highlightなどの命令を送る。

### Unresolved Metadata Display

KMMまたはeditorがunresolved targetを返した場合、kdpは該当箇所またはdocument-level noticeとして表示できる入口を持つ。

### KUC Baseline

KMM viewer modelはKUC実装を前提にする。egui実装は移行前MVPとして扱い、KMM adoptionの完了条件にしない。

### Export Responsibility

KDVはHTML/PDF/PNG/JPG exportを担う。書き出し（export）はKDV内部のartifact/export契約へ集約する。
