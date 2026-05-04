# Tasks: katana-markdown-preview v0.1.0

## Branch Rule

- **標準ブランチ**: `release/v0.1.0`
- **作業ブランチ**: `feature/v0.1.0-task-x`

---

## 1. neutral interface を確定する

- [ ] 1.1 `MarkdownPreview` trait と DTO（`MarkdownSource` / `PreviewConfig` / `PreviewTheme` / `PreviewOutput` / `PreviewDiagnostics` / `RenderTarget`）を本実装向けに拡張する
- [ ] 1.2 `katana-markdown-preview` の `cargo tree` に `egui` が含まれないことを確認する
- [ ] 1.3 KatanA が interface crate のみを依存しても型エラーが出ないことを確認する

---

## 2. KatanA preview 実装を移管する

- [ ] 2.1 KatanA `katana-ui` の preview 描画コードを `katana-markdown-preview-egui` へ移管する
- [ ] 2.2 `vendor/egui_commonmark_upstream` を egui crate の dependency に移し、KatanA ルートの `[patch.crates-io]` から除去する
- [ ] 2.3 ダイアグラム描画を `katana-canvas-forge` 経由に統一する（egui crate 内で独自 Mermaid renderer を持たない）
- [ ] 2.4 絵文字ハック（`egui::Image` 置換）を egui crate へ移管する

---

## 3. v0.1.0 release

- [ ] 3.1 `cargo fmt` / `cargo clippy --workspace -- -D warnings` / `cargo test --workspace` が通る
- [ ] 3.2 release tag `v0.1.0` を切り GitHub Release を作成する
- [ ] 3.3 KatanA v0.26.0 が `katana-markdown-preview = { git = "...", tag = "v0.1.0" }` でビルドできることを確認する
