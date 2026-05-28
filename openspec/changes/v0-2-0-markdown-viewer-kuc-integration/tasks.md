# Tasks: katana-document-viewer v0.2.0 markdown viewer KUC integration

## Definition of Ready (DoR)

- [ ] `v0.1.0` のrender/export foundationが完了している
- [ ] KUCのstyle/theme/font/state契約が利用可能である
- [ ] KUCのScrollArea、SplitPane、基本操作部品の完了条件が確認できている
- [ ] KMM document model DTOが利用可能である
- [ ] `katana-document-viewer` neutral crateへKUC型を露出しない方針が確認できている

## Definition of Done (DoD)

- [ ] `rtk ./scripts/openspec validate v0-2-0-markdown-viewer-kuc-integration --strict --no-interactive` が通る
- [ ] `rtk just check` が通る
- [ ] `rtk cargo tree -p katana-document-viewer --locked` に `katana-ui-core`、`egui`、`winit`、`vello` が含まれない
- [ ] hit-test、目次（TOC）click、hover設定、画像・図形操作commandの自動テストが通る
- [ ] Document modeとSlideshow modeの切り替え、viewport height仮想ページング、slideshow navigation commandの自動テストが通る
- [ ] 画面確認は補助として実施し、正しさの根拠は自動テストに置く

---

## 1. neutral viewer contract を固定する

- [ ] 1.1 viewer input / viewer state snapshot / viewer command / hit-test responseの型を `katana-document-viewer` 側に定義する
- [ ] 1.2 `ViewerMode::Document`、`ViewerMode::Slideshow`、`SlideshowState`、`SlideshowCommand` を定義する
- [ ] 1.3 viewer commandにKMM node id、source range、artifact id、操作種別を含める
- [ ] 1.4 slideshow commandにcurrent page index、max page index、close request、settings updateを含める
- [ ] 1.5 `katana-document-viewer` public APIへKUC型を露出しないことをテストする
- [ ] 1.6 `cargo tree -p katana-document-viewer --locked` でUI依存が入っていないことを確認する

---

## 2. KUC viewer crate を追加する

- [ ] 2.1 `crates/katana-document-viewer-kuc` を追加する
- [ ] 2.2 `katana-ui-core = { workspace = true }` だけでKUCへ依存する
- [ ] 2.3 KUC theme / font / state契約をviewer configへ接続する
- [ ] 2.4 `egui_commonmark` vendor patchを正規経路にしないことを検査する

---

## 3. Markdown本文を表示する

- [ ] 3.1 `DocumentSnapshot` とartifact / diagnosticsをviewer inputとして受け取る
- [ ] 3.2 CommonMark fixtureを表示する
- [ ] 3.3 GFM fixtureを表示する
- [ ] 3.4 KatanA互換fixtureを表示する
- [ ] 3.5 外部描画成功 / 失敗artifactを表示する
- [ ] 3.6 raw保持情報を本文から削除しない

---

## 4. hit-test metadata を実装する

- [ ] 4.1 rendered node identityを保持する
- [ ] 4.2 rendered nodeからKMM node idへ戻る
- [ ] 4.3 rendered nodeからsource rangeへ戻る
- [ ] 4.4 画面座標からrendered nodeへ戻る
- [ ] 4.5 対象なしの場合に失敗結果を返す

---

## 5. Slideshow modeを実装する

- [ ] 5.1 KatanA既存のMarkdown slideshow仕様をテストfixtureへ落とす
- [ ] 5.2 Slideshow modeで通常previewと同じrendered contentを全画面相当領域へ表示する
- [ ] 5.3 `1 viewport height = 1 slideshow page` としてcurrent page index / max page index / page offsetを計算する
- [ ] 5.4 `ArrowRight`、`PageDown`、`Space`、next controlを次ページcommandへ変換する
- [ ] 5.5 `ArrowLeft`、`PageUp`、previous controlを前ページcommandへ変換する
- [ ] 5.6 先頭・末尾でpage indexが範囲外へ出ないことをテストする
- [ ] 5.7 `Esc` と右上close controlをclose commandへ変換する
- [ ] 5.8 hover highlightとdiagram controlsのSlideshow settings stateを実装する
- [ ] 5.9 操作時にcontrolsを表示し、idle時にfadeできるstateを実装する
- [ ] 5.10 現在themeを継承し、slideshow専用themeを作っていないことをテストする
- [ ] 5.11 fullscreenやwindow制御をKDV内で実行していないことをテストする

---

## 6. 目次（TOC）とscroll commandを実装する

- [ ] 6.1 KMM AST由来のheading listを受け取る
- [ ] 6.2 heading listからTOC itemを作る
- [ ] 6.3 Markdown本文を再parseしてTOCを作っていないことをテストする
- [ ] 6.4 TOC clickでviewerをrendered heading anchorへscrollする
- [ ] 6.5 TOC clickでKMM node id / source range / heading anchorを含むviewer commandを返す
- [ ] 6.6 active headingをlayout後のrendered heading anchor mapから決定する

---

## 7. interaction設定を実装する

- [ ] 7.1 `hover_highlight_enabled` を実装する
- [ ] 7.2 `image_controls_enabled` を実装する
- [ ] 7.3 `diagram_controls_enabled` を実装する
- [ ] 7.4 copy / open / fit 操作をviewer commandへ変換する
- [ ] 7.5 KDV内で副作用を起こしていないことをテストする

---

## 8. 品質ゲートを追加する

- [ ] 8.1 rendering codeの色literalを禁止するAST lintを追加する
- [ ] 8.2 OS固有font pathの直接参照を禁止するAST lintを追加する
- [ ] 8.3 preset直接参照を禁止するAST lintを追加する
- [ ] 8.4 viewer regression testsをCIへ追加する
- [ ] 8.5 `rtk just check` を実行する
