## Context

`katana-document-preview` は未リリース・未取り込みのため、`katana-document-viewer`（KDV）へ改名する。

KDVはMarkdown以外のdocument viewerも担うが、KME採用ではMarkdown viewerの文書構造、hit-test、metadata表示、HTML/PDF/PNG/JPG exportが中心になる。表示はFloem/velloを前提にし、egui_commonmarkのvendor patchへ戻らない。

## Goals

- KME文書モデルをFloemで高速表示する。
- viewer表示とHTML/PDF/PNG/JPG exportを同じrender pipelineに寄せる。
- KME node idとsource rangeをhit-testに使う。
- hover、選択、AST単位コピーの入口を提供する。
- unresolved metadataを画面に表示できる。
- P0 `katana-ast-lint` を品質ゲートにする。

## Non-Goals

- KME parserをkdp内に再実装すること。
- metadata schemaをkdp内で定義すること。
- egui rendererをKME採用の正規経路にすること。
- editor-viewer同期制御をKDVが担当すること。

## Decisions

### KME Model Input

KDVはKME public DTOをviewer/export inputとして受け取る。KME内部parser型やthird-party AST型は受け取らない。

### P3 Consumer Order

KDVのKME採用はP3作業とする。P0 `katana-ast-lint`、P1 KME文書モデル、P2 `katana-ui-widget` の境界を受け、viewer固有のUI部品をKatanA本体へ増やしすぎない。

### Hit-test Metadata

表示されたnodeはKME node id、source range、rendered rect identityを持つ。KatanA UIはこれを使ってhover、選択、copy/edit actionへ接続する。

KDVは同期制御を持たない。KatanAがKMEの位置情報を使い、viewerまたはeditorへscroll、selection、highlightなどの命令を送る。

### Unresolved Metadata Display

KMEまたはeditorがunresolved targetを返した場合、kdpは該当箇所またはdocument-level noticeとして表示できる入口を持つ。

### Floem Baseline

KME viewer modelはFloem実装を前提にする。egui実装は移行前MVPとして扱い、KME adoptionの完了条件にしない。

### Export Responsibility

KDVはHTML/PDF/PNG/JPG exportを担う。KCFの既存exportは、KDV側に同等機能が入るまで維持され、KDV実装後に移譲・削除する。
