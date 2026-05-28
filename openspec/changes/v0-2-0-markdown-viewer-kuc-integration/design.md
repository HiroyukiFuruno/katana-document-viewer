## Context

`v0.1.0` はUI非依存のrender/export基盤であり、実画面のMarkdown viewerはこのchangeで扱う。

画面上では、Markdown本文、スライドショー表示、目次（TOC）、hover highlight、選択、画像・図形の操作UI、unresolved metadataの表示が必要になる。これらはKUCの状態・見た目・部品契約と結び付くため、KUC完成前のfoundationから分離する。

## Goals / Non-Goals

**Goals:**

- KUC上でKMM documentを表示する。
- neutral crateからKUC実装へ渡すviewer input、viewer event、viewer command、hit-test metadataを定義する。
- 通常文書表示とスライドショー表示をviewer modeとして切り替える。
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

neutral crateはKUC型をpublic APIへ漏らさない。`katana-document-viewer` はviewer input、viewer state snapshot、viewer command、hit-test responseを公開し、`katana-document-viewer-kuc` がそれをKUC部品へ変換する。

### KMM metadataを表示の正本にする

KDVはKMM node id、source range、heading anchor候補を使ってviewer metadataを作る。TOCはMarkdown本文を再parseせず、KMM AST由来のheading listから作る。

### Slideshow modeはKatanA既存仕様を正とする

KDVは `ViewerMode::Document` と `ViewerMode::Slideshow` をneutral contractに持つ。Slideshow modeでは、KatanA既存仕様に合わせ、通常previewと同じrendered contentを全画面相当領域に表示し、viewport height単位で仮想ページングする。

KatanA既存仕様では、スライドショーはアクティブMarkdownを全画面で表示し、連続したMarkdown本文を `1 viewport height = 1 page` として扱う。KDVはこの挙動を移植時の正本にする。見出し単位や明示marker単位で新しいslide deckを作らない。

Slideshow modeのviewer stateは、viewport size、content height、current page index、max page index、hover highlight設定、diagram controls設定を持つ。次へ、前へ、終了、設定変更はviewer commandとしてホストへ返す。fullscreen、window制御、active document選択はKDVでは実行せず、ホスト側の副作用として扱う。

KatanA既存仕様から引き継ぐ操作は次の通り。

- 起動: ホストがactive Markdown文書を `ViewerMode::Slideshow` で開く。
- 表示: 通常previewを背面に隠し、全画面相当の表示領域へMarkdown本文を描画する。
- ページング: `ArrowRight` / `PageDown` / `Space` または下部次ボタンで次ページへ進む。
- 逆方向: `ArrowLeft` / `PageUp` または下部前ボタンで前ページへ戻る。
- 端の挙動: 先頭より前、末尾より後へは進まず、現在ページを維持する。
- 終了: `Esc` または右上close buttonで終了commandを返す。
- 設定: 右側settings tabでhover highlightとdiagram controlsを切り替える。
- コントロール表示: 操作後は表示し、一定時間idleならfadeする。
- Theme: 現在のthemeを継承し、slideshow専用themeを作らない。

### 副作用はhost commandへ逃がす

copy、open、download、editor scrollなど副作用を伴う操作はKDV内で実行しない。KDVは操作内容をviewer commandとしてホストへ返す。

### export結果を画面へ流用する

外部描画結果やdiagnosticsは `v0.1.0` のartifact / diagnosticsから受け取る。viewerは同じsourceを再処理せず、表示用のlayoutとinteractionに集中する。

viewerはHTML/PDF/PNG/JPG exportの合否を判定しない。export品質は `v0.1.x` の `ExportQualityGate` と実artifact確認を正とし、このchangeは画面操作、metadata、command境界を検証対象にする。

## Risks / Trade-offs

- KUCのScrollArea、SplitPane、SearchControlStripなどの基礎部品が不足すると、このchangeの完了条件は満たせない。
- KUC境界が未確定のまま進めるとKDV内に暫定UI型が漏れるため、最初にviewer input / commandのneutral契約を固定する。
- slide deckを新規生成するとKatanA既存仕様とずれるため、viewport height単位の仮想ページングを正本にする。
- 見た目の確認だけでは完了にしない。hit-test、TOC click、hover設定、command発火は自動テストで確認する。
- 画面操作とexportを同時に変えると原因が追いづらくなるため、このchangeはviewer操作に限定する。
