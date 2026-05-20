## Context

`v0.1.0` はUI非依存のrender/export基盤であり、実画面のMarkdown viewerはこのchangeで扱う。

画面上では、Markdown本文、目次（TOC）、hover highlight、選択、画像・図形の操作UI、unresolved metadataの表示が必要になる。これらはKUCの状態・見た目・部品契約と結び付くため、KUC完成前のfoundationから分離する。

## Goals / Non-Goals

**Goals:**

- KUC / Floem上でKMM documentを表示する。
- rendered nodeからKMM node idとsource rangeへ戻るhit-test metadataを提供する。
- 目次（TOC）をKMM AST由来の見出し構造から表示する。
- hover、選択、画像・図形操作をhost commandへ変換する。
- theme、font、i18n、interaction設定をKUC/KDV境界で明示する。

**Non-Goals:**

- KMM parserをKDV内で再実装しない。
- editor-viewer同期制御をKDV内に持たない。
- KUC共通部品をKDVで再実装しない。
- export基盤をこのchangeで作り直さない。

## Decisions

### KUCを画面部品の正本にする

KDVはKUCのstyle/theme/font/state契約を受け取り、viewer画面へ適用する。KDV内のrendering codeは色literal、font path、preset直接参照を持たない。

### KMM metadataを表示の正本にする

KDVはKMM node id、source range、heading anchor候補を使ってviewer metadataを作る。TOCはMarkdown本文を再parseせず、KMM AST由来のheading listから作る。

### 副作用はhost commandへ逃がす

copy、open、download、editor scrollなど副作用を伴う操作はKDV内で実行しない。KDVは操作内容をviewer commandとしてホストへ返す。

### export結果を画面へ流用する

外部描画結果やdiagnosticsは `v0.1.0` のartifact / diagnosticsから受け取る。viewerは同じsourceを再処理せず、表示用のlayoutとinteractionに集中する。

## Risks / Trade-offs

- KUCのScrollArea、SplitPane、SearchControlStripなどの基礎部品が不足すると、このchangeの完了条件は満たせない。
- 見た目の確認だけでは完了にしない。hit-test、TOC click、hover設定、command発火は自動テストで確認する。
- 画面操作とexportを同時に変えると原因が追いづらくなるため、このchangeはviewer操作に限定する。
