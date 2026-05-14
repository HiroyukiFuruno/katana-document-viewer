# katana-document-viewer OpenSpec

## Project

`katana-document-preview` は未リリース・未取り込みのため、計画上は `katana-document-viewer`（KDV）へ改名する。

KDVは、KMM public DTOを入力にしたMarkdown viewer、hit-test、node選択、HTML/PDF/PNG/JPG exportを担うlibraryである。KatanA はこれを git dependency として consume する。

## Design Principles

- `katana-document-viewer` crate（neutral interface）は `egui` に依存しない。
- `katana-document-viewer-floem` crate がFloem実装を持つ。
- KatanA は neutral interface と Floem 実装の両方を dependency に取るが、interface 経由でしか呼ばない。
- viewer表示とHTML/PDF/PNG/JPG exportは同じrender pipelineを使う。
- Mermaid、Draw.io、PlantUML、mathなどの外部描画はKCFへ委譲する。
- KDVはeditor-viewer同期制御を持たない。同期制御はKatanAが担い、KatanAがviewerまたはeditorへ命令する。

## Versioning

- `v0.1.x`: KDV改名、KMM model input、Floem viewer、viewer/export pipeline方針の確立。
- `v0.2.x`: HTML/PDF/PNG/JPG exportの実装とKCF export移譲。

## Consumers

- [KatanA](https://github.com/HiroyukiFuruno/KatanA) — git tag pinned（v0.26.0 で取り込み）

---

## UI フレームワーク移行方針（egui → Floem）

このセクションはエコシステム全体で共通の方針。詳細は [KatanA openspec/project.md](https://github.com/HiroyukiFuruno/KatanA/blob/master/openspec/project.md) を正とする。

### 技術選定（確定）

| 層 | 採用 |
|----|------|
| UI フレームワーク | **Floem**（Rust 純正・クロスプラットフォーム） |
| 文字描画 | **cosmic-text**（IME 完全対応・カラー絵文字 SBIX/CBTF） |
| 2D レンダリング | **vello + wgpu**（compute-shader・Metal/DX12/Vulkan） |
| レイアウト | **taffy**（flexbox + CSS Grid） |
| アーキテクチャ参考 | **GPUI / Zed**（設計の教材として活用） |

React / TypeScript / WebView は使用しない。Rust 純正のみ。

### eguiから脱却する理由（要約）

- カラー絵文字：epaint が SBIX/CBTF 非対応 → cosmic-text で解決
- IME 不完全：egui TextEdit の composition が壊れる → cosmic-text + winit で解決
- レイアウト拡張不可：vendor パッチなしに行間・マージンを変えられない → vello Scene への直接描画で解決
- immediate mode の再描画コスト → vello の retained 描画で解決

### このrepoの責務

`katana-document-preview` は未リリース・未取り込みのため、`-egui` implを正規路線にせず、`katana-document-viewer-floem` を初期実装の前提にする。neutral interface crate は `katana-document-viewer` とする。
KatanA の `Cargo.toml` の impl crate 行を変えるだけで移行が完了する。

### katana-document-viewer の移行

```
katana-document-viewer          neutral interface
katana-document-viewer-floem    vello retained viewer/export実装
```

viewer は vello Scene への直接描画で vendor パッチ問題を根本解決する。PDF / 画像 / 図表もすべて同じ wgpu surface で統一できる。

---

## KMM構想での扱い

KMM構想ではP3として、P0 `katana-ast-lint`、P1 `katana-markdown-model`、P2 `katana-ui-widget` の境界を受けて、KMM文書モデルをFloemで表示し、同じpipelineでexportする。

- KMM文書モデルを再実装しない。
- parser内部型やrenderer内部型をviewer stateへ漏らさない。
- Floemネイティブ表示を前提にする。
- unresolved metadataを画面上で確認できる入口を持つ。
- HTML/PDF/PNG/JPG exportを担う。
- editor-viewer同期制御は持たない。
- 共通AST lintを品質ゲートにする。
