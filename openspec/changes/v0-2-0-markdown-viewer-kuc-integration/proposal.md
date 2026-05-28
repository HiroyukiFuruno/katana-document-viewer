## Why

`v0.1.0` でKUC非依存のrender/export基盤を固めた後、Markdownを実際に画面へ表示し、操作できるviewerへ接続する必要がある。

このchangeは、KUC上でMarkdown viewerを実装する。画面上では、本文、スライドショー表示、目次（TOC）、hover、選択、画像・図形の操作が見える。操作結果はKDV内で副作用を完結させず、viewer commandとしてホストへ返す。

## What Changes

- `katana-document-viewer-kuc` をMarkdown viewer実装として追加する。
- `katana-document-viewer` neutral crateに、KUC実装へ渡すviewer input / viewer command / hit-test metadataの公開契約を追加する。
- Markdown本文を通常文書表示する `ViewerMode::Document` と、全画面スライドショー表示の `ViewerMode::Slideshow` を追加する。
- Slideshow modeはKatanA既存仕様に合わせ、連続したMarkdown本文をviewport height単位の仮想ページとして扱い、見出しや手動markerでslide deckを再構成しない。
- KUCのstyle/theme/font/state契約を受け取り、KDV内の色や表示文言を直接固定しない。
- KMM node id、source range、rendered rect identityをhit-test metadataへ接続する。
- 目次（TOC）はKMM AST由来の見出し構造を正本にし、本文再parseで作らない。
- hover highlight、選択、画像・図形操作を `ViewerInteractionConfig` で切り替える。
- Mermaid、Draw.io、ZenUML、PlantUML、mathは、`v0.1.0` のKRR委譲結果またはraw保持diagnosticsを画面へ表示する。
- unresolved metadataは本文を消さず、警告表示、代表メッセージ、詳細確認へ接続する。

## Capabilities

### New Capabilities

- `markdown-viewer-kuc-integration`: KUC上のMarkdown viewer、slideshow mode、hit-test、目次（TOC）、hover、選択、画像・図形操作を提供する。

### Modified Capabilities

- `render-export-foundation`: `v0.1.0` のartifact / diagnostics / export結果を、実画面viewerの入力として利用する。
- `complete-theme-contract`: KUC viewerも完全テーマとfont契約を受け取り、KDV側の固定色やOS font pathへ戻らない。

## Known Constraints

- KDVはeditor-viewer同期制御を持たない。KatanAがeditor scroll、preview-editor同期、workspace layoutを管理する。
- KDVはKUCの共通部品を直接再実装しない。
- `egui_commonmark` vendor patchを正規経路にしない。

## Impact

- `crates/katana-document-viewer-kuc/` — KUC viewer実装。
- `crates/katana-document-viewer/` — viewer command / hit-test metadata / interaction config。
- `crates/kdp-linter/` — KUC viewer境界、固定色、OS font path、preset直接参照の構造検査。
- KatanA — viewer commandを受け取り、editor scrollやcopy/openなどの副作用を実行する。
