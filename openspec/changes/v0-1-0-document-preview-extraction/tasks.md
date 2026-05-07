# Tasks: katana-document-viewer v0.1.0

## Branch Rule

- **標準ブランチ**: `release/v0.1.0`
- **作業ブランチ**: `feature/v0.1.0-task-x`

---

## 1. neutral interface を確定する

- [ ] 1.1 `DocumentViewer` trait と DTO（`ViewerSource` / `ViewerConfig` / `ViewerTheme` / `ViewerOutput` / `ViewerDiagnostics` / `ExportConfig`）を本実装向けに拡張する
- [ ] 1.2 `katana-document-viewer` の `cargo tree` に `egui` が含まれないことを確認する
- [ ] 1.3 KatanA が interface crate のみを依存しても型エラーが出ないことを確認する
- [ ] 1.4 editor-viewer同期制御をKDVが持たず、KatanAがviewerまたはeditorへ命令する契約を明記する

---

## 2. KatanA viewer 実装を移管する

- [ ] 2.1 KME public DTOをviewer inputとして受け取る
- [ ] 2.2 `katana-document-viewer-floem` にKME node renderingの土台を追加する
- [ ] 2.3 ダイアグラム描画を `katana-canvas-forge` 経由に統一する（KDV内で独自 Mermaid renderer を持たない）
- [ ] 2.4 `egui_commonmark` vendor patchを正規経路にしないことを確認する

---

## 3. Export pipeline

- [ ] 3.1 HTML/PDF/PNG/JPG exportをKDV責務として定義する
- [ ] 3.2 viewer表示とexportが同じrender pipelineを使う方針を固定する
- [ ] 3.3 KCF既存exportの維持期間、移譲条件、削除条件を文書化する

---

## 4. v0.1.0 release

- [ ] 4.1 `cargo fmt` / `cargo clippy --workspace -- -D warnings` / `cargo test --workspace` が通る
- [ ] 4.2 release tag `v0.1.0` を切り GitHub Release を作成する
- [ ] 4.3 KatanA が `katana-document-viewer = { git = "...", tag = "v0.1.0" }` でビルドできることを確認する
