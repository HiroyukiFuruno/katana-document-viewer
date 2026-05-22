## Why

`v0.1.0` でKUC非依存のrender/export基盤を固めた後、Markdownを実際に画面へ表示し、操作できるviewerへ接続する必要がある。

このchangeは、KUC上でMarkdown viewerを実装する。画面上では、本文、目次（TOC）、hover、選択、画像・図形の操作が見える。操作結果はKDV内で副作用を完結させず、viewer commandとしてホストへ返す。

## What Changes

- `katana-document-viewer-kuc` をMarkdown viewer実装として追加する。
- KUCのstyle/theme/font/state契約を受け取り、KDV内の色や表示文言を直接固定しない。
- KMM node id、source range、rendered rect identityをhit-test metadataへ接続する。
- 目次（TOC）はKMM AST由来の見出し構造を正本にし、本文再parseで作らない。
- hover highlight、選択、画像・図形操作を `ViewerInteractionConfig` で切り替える。
- Mermaid、Draw.io、ZenUML、PlantUML、mathは、`v0.1.0` のKDR委譲結果またはraw保持diagnosticsを画面へ表示する。
- unresolved metadataは本文を消さず、警告表示、代表メッセージ、詳細確認へ接続する。

## Capabilities

### New Capabilities

- `markdown-viewer-kuc-integration`: KUC上のMarkdown viewer、hit-test、目次（TOC）、hover、選択、画像・図形操作を提供する。

### Modified Capabilities

- `render-export-foundation`: `v0.1.0` のartifact / diagnostics / export結果を、実画面viewerの入力として利用する。

## Known Constraints

- KDVはeditor-viewer同期制御を持たない。KatanAがeditor scroll、preview-editor同期、workspace layoutを管理する。
- KDVはKUCの共通部品を直接再実装しない。
- `egui_commonmark` vendor patchを正規経路にしない。

## Impact

- `crates/katana-document-viewer-kuc/` — KUC viewer実装。
- `crates/katana-document-viewer/` — viewer command / hit-test metadata / interaction config。
- KatanA — viewer commandを受け取り、editor scrollやcopy/openなどの副作用を実行する。
