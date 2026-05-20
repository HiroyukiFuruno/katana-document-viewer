# Tasks: katana-document-viewer v0.2.0 markdown viewer KUC integration

## Definition of Ready (DoR)

- [ ] `v0.1.0` のrender/export foundationが完了している
- [ ] KUCのstyle/theme/font/state契約が利用可能である
- [ ] KUCのScrollArea、SplitPane、基本操作部品の完了条件が確認できている
- [ ] KMM document model DTOが利用可能である

## Definition of Done (DoD)

- [ ] `npx -y @fission-ai/openspec validate v0-2-0-markdown-viewer-kuc-integration --strict` が通る
- [ ] `cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace` が通る
- [ ] hit-test、目次（TOC）click、hover設定、画像・図形操作commandの自動テストが通る
- [ ] 画面確認は補助として実施し、正しさの根拠は自動テストに置く

---

## 1. KUC / Floem viewer crate を追加する

- [ ] 1.1 `crates/katana-document-viewer-floem` を追加する
- [ ] 1.2 `floem = { workspace = true }` だけでFloemへ依存する
- [ ] 1.3 KUC theme / font / state契約をviewer configへ接続する
- [ ] 1.4 `egui_commonmark` vendor patchを正規経路にしないことを検査する

---

## 2. Markdown本文を表示する

- [ ] 2.1 `DocumentSnapshot` とartifact / diagnosticsをviewer inputとして受け取る
- [ ] 2.2 CommonMark fixtureを表示する
- [ ] 2.3 GFM fixtureを表示する
- [ ] 2.4 KatanA互換fixtureを表示する
- [ ] 2.5 外部描画成功 / 失敗artifactを表示する
- [ ] 2.6 raw保持情報を本文から削除しない

---

## 3. hit-test metadata を実装する

- [ ] 3.1 rendered node identityを保持する
- [ ] 3.2 rendered nodeからKMM node idへ戻る
- [ ] 3.3 rendered nodeからsource rangeへ戻る
- [ ] 3.4 画面座標からrendered nodeへ戻る
- [ ] 3.5 対象なしの場合に失敗結果を返す

---

## 4. 目次（TOC）とscroll commandを実装する

- [ ] 4.1 KMM AST由来のheading listを受け取る
- [ ] 4.2 heading listからTOC itemを作る
- [ ] 4.3 Markdown本文を再parseしてTOCを作っていないことをテストする
- [ ] 4.4 TOC clickでviewerをrendered heading anchorへscrollする
- [ ] 4.5 TOC clickでKMM node id / source range / heading anchorを含むviewer commandを返す
- [ ] 4.6 active headingをlayout後のrendered heading anchor mapから決定する

---

## 5. interaction設定を実装する

- [ ] 5.1 `hover_highlight_enabled` を実装する
- [ ] 5.2 `image_controls_enabled` を実装する
- [ ] 5.3 `diagram_controls_enabled` を実装する
- [ ] 5.4 copy / open / fit 操作をviewer commandへ変換する
- [ ] 5.5 KDV内で副作用を起こしていないことをテストする

---

## 6. 品質ゲートを追加する

- [ ] 6.1 rendering codeの色literalを禁止するAST lintを追加する
- [ ] 6.2 OS固有font pathの直接参照を禁止するAST lintを追加する
- [ ] 6.3 preset直接参照を禁止するAST lintを追加する
- [ ] 6.4 viewer regression testsをCIへ追加する
