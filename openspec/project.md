# katana-document-preview OpenSpec

## Project

`katana-document-preview`（kdp）は、ドキュメントプレビュー（Markdown / PDF / 画像 / Draw.io / Word / Excel / PPT / CSV 等）の neutral interface と egui MVP 実装を提供する library。KatanA はこれを git dependency として consume する。

## Design Principles

- `katana-document-preview` crate（neutral interface）は `egui` に依存しない。
- `katana-document-preview-egui` crate が egui 実装を持つ。将来の独自 UI への差し替えはこの crate のみ変更すれば良い。
- KatanA は neutral interface と egui 実装の両方を dependency に取るが、interface 経由でしか呼ばない。

## Versioning

- `v0.1.x`: KatanA v0.26.0 で分離する preview 実装の移管。egui MVP 確立。
- `v0.2.x`: 独自 UI 実装への差し替え（egui 脱却）

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

### egui から脱却する理由（要約）

- カラー絵文字：epaint が SBIX/CBTF 非対応 → cosmic-text で解決
- IME 不完全：egui TextEdit の composition が壊れる → cosmic-text + winit で解決
- レイアウト拡張不可：vendor パッチなしに行間・マージンを変えられない → vello Scene への直接描画で解決
- immediate mode の再描画コスト → vello の retained 描画で解決

### この repo の責務

各 `-egui` impl crate を `-floem` impl crate に差し替える。neutral interface crate は変えない。
KatanA の `Cargo.toml` の impl crate 行を変えるだけで移行が完了する。

### katana-document-preview の移行

```
katana-document-preview         neutral interface（変わらない）
katana-document-preview-egui    MVP 実装（Phase 2 で置き換え対象）
katana-document-preview-floem   vello retained 描画実装（Phase 2 で新規作成）
```

Phase 2（editor/chat の後）。preview は vello Scene への直接描画で vendor パッチ問題が根本解決する。
PDF / 画像 / 図表もすべて同じ wgpu surface で統一できる。
