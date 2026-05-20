# Tasks: katana-document-viewer v0.1.0 render/export foundation

## Definition of Ready (DoR)

- [x] proposal / design / spec / tasks の対象capability名が一致している
- [x] KMM / KDV / KDR / KCF / KUC / KatanA の責務境界がdesign.mdとspec.mdで明示されている
- [x] `v0.1.0` で実装するUI非依存範囲と、`v0.2.0` へ送る画面操作範囲がtasks.mdから判定できる

## Definition of Done (DoD)

- [ ] `npx -y @fission-ai/openspec validate v0-1-0-render-export-foundation --strict` が通る
- [ ] `cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace` が通る
- [ ] `katana-document-viewer` のUI非依存dependency guardが通る
- [ ] CommonMark / GFM / 数式（math） / GitHub alert / KatanA互換fixtureの描画評価テストが、spec.mdのScenarioと対応している
- [ ] HTML/PDF/PNG/JPG書き出し（export）のartifact manifestテストが、spec.mdのScenarioと対応している
- [ ] KCF/KDR backend smokeが、成功時と失敗時の両方を検証している

---

## Branch Rule

- **標準ブランチ**: `release/v0.1.0`
- **作業ブランチ**: `feature/v0.1.0-task-x`

---

## 1. v0.1.0 / v0.2.0 の境界を確定する

- [x] 1.1 `v0.1.0` はKUC非依存のrender/export foundationと明記する
- [x] 1.2 hover、選択、目次（TOC）、画像・図形操作、KUC / Floem viewerを `v0.2.0` へ送る
- [x] 1.3 `adopt-kme-preview-model` との重複を確認し、必要な内容をこのchangeまたは `v0.2.0` へ移す
- [x] 1.4 `openspec/project.md` と `docs/ui-separation-plan.md` のversion順序を同期する

---

## 2. neutral interface と document model を定義する

- [ ] 2.1 `crates/katana-document-viewer` を追加する
- [ ] 2.2 `DocumentSource` / `SourceUri` / `SourceKind` / `SourceRevision` を定義する
- [ ] 2.3 `DocumentId` / `DocumentKind` / `DocumentSnapshot` / `DocumentOutline` / `DocumentMetadataView` を定義する
- [ ] 2.4 KMM input conversion を追加する
- [ ] 2.5 KMM parse result conversion を追加する
- [ ] 2.6 source serialization test を追加する
- [ ] 2.7 document snapshot test を追加する

---

## 3. artifact model を定義する

- [ ] 3.1 `ArtifactId` / `ArtifactKind` / `ArtifactFormat` を定義する
- [ ] 3.2 `ArtifactBytes` / `ArtifactUri` / `ArtifactManifest` を定義する
- [ ] 3.3 `ArtifactDiagnostics` を定義する
- [ ] 3.4 preview artifact を定義する
- [ ] 3.5 export artifact を定義する
- [ ] 3.6 image artifact を定義する
- [ ] 3.7 PDF artifact を定義する
- [ ] 3.8 Office artifact placeholder を定義する
- [ ] 3.9 artifact manifest serialization test を追加する

---

## 4. forge / export API を定義する

- [ ] 4.1 `forge` module を作る
- [ ] 4.2 `BuildRequest` / `BuildProfile` / `BuildGraph` / `TransformStep` を定義する
- [ ] 4.3 `ExportRequest` / `ExportFormat` / `ExportOutput` を定義する
- [ ] 4.4 `ForgeDiagnostics` / `ForgeError` を定義する
- [ ] 4.5 `ForgeBackend` trait と `ForgePipeline` を定義する
- [ ] 4.6 no-UI dependency test を追加する

---

## 5. KCF / KDR backend integration を作る

- [ ] 5.1 `backend::canvas_forge` module を作る
- [ ] 5.2 KCF `RenderInput` への変換を作る
- [ ] 5.3 KCF `RenderOutput` から `Artifact` への変換を作る
- [ ] 5.4 KCF export output から `ExportOutput` への変換を作る
- [ ] 5.5 Mermaid / Draw.io / ZenUML / PlantUML / math の委譲境界を定義する
- [ ] 5.6 HTML / PDF / PNG / JPEG export path を接続する
- [ ] 5.7 KCF dependency をtransitionalとしてREADMEに記載する
- [ ] 5.8 KCF compatibility tests を追加する

---

## 6. rendering evaluation fixtures を作る

- [ ] 6.1 CommonMark fixture set を作る
- [ ] 6.2 GFM fixture set を作る
- [ ] 6.3 数式（math）fixture set を作る
- [ ] 6.4 GitHub alert fixture set を作る
- [ ] 6.5 KatanA互換fixture set を作る
- [ ] 6.6 外部描画成功fixture set を作る
- [ ] 6.7 外部描画失敗時のraw保持fixture set を作る
- [ ] 6.8 fixtureとspec Scenarioの対応表を追加する

---

## 7. CLI API と品質ゲートを作る

- [ ] 7.1 `cli_api` module を作る
- [ ] 7.2 `CliRequest` / `CliOutput` / `CliDiagnostics` を定義する
- [ ] 7.3 markdown preview build CLI entry を作る
- [ ] 7.4 export CLI entry を作る
- [ ] 7.5 diagram render CLI entry を作る
- [ ] 7.6 export-debug CLI entry を作る
- [ ] 7.7 KDV `just check` にforge no-UI dependency guardを追加する
- [ ] 7.8 KDV `just check` にKCF backend smokeを追加する
- [ ] 7.9 KDV `just check` にCLI API smokeを追加する
- [ ] 7.10 artifact fixture tests とexport fixture testsをCIへ追加する

---

## 8. Final Verification & Release Work

- [ ] 8.1 `cargo fmt` / `cargo clippy --workspace -- -D warnings` / `cargo test --workspace` が通る
- [ ] 8.2 KDV AST lintを実行し、UI非依存境界違反がないことを確認する
- [ ] 8.3 `npx -y @fission-ai/openspec validate v0-1-0-render-export-foundation --strict` が通る
- [ ] 8.4 release tag `v0.1.0` を切り GitHub Release を作成する
- [ ] 8.5 KatanA が `katana-document-viewer = { git = "...", tag = "v0.1.0" }` でビルドできることを確認する
