## Context

kdpはMarkdown以外のdocument previewも担うが、KME採用ではMarkdown previewの文書構造、hit-test、metadata表示が中心になる。表示はFloem/velloを前提にし、egui_commonmarkのvendor patchへ戻らない。

## Goals

- KME文書モデルをFloemで高速表示する。
- KME node idとsource rangeをhit-testに使う。
- hover、選択、AST単位コピーの入口を提供する。
- unresolved metadataを画面に表示できる。
- P0 `katana-ast-lint` を品質ゲートにする。

## Non-Goals

- KME parserをkdp内に再実装すること。
- PDF/PNG/JPG exportをkdpが担当すること。
- egui rendererをKME採用の正規経路にすること。

## Decisions

### KME Model Input

kdpはKME public DTOをpreview inputとして受け取る。KME内部parser型やthird-party AST型は受け取らない。

### P3 Consumer Order

`katana-document-preview` のKME採用はP3作業とする。P0 `katana-ast-lint`、P1 KME文書モデル、P2 `katana-ui-widget` の境界を受け、preview固有のUI部品をKatanA本体へ増やしすぎない。

### Hit-test Metadata

表示されたnodeはKME node id、source range、rendered rect identityを持つ。KatanA UIはこれを使ってhover、選択、copy/edit actionへ接続する。

### Unresolved Metadata Display

KMEまたはeditorがunresolved targetを返した場合、kdpは該当箇所またはdocument-level noticeとして表示できる入口を持つ。

### Floem Baseline

KME preview modelはFloem実装を前提にする。egui実装は移行前MVPとして扱い、KME adoptionの完了条件にしない。
