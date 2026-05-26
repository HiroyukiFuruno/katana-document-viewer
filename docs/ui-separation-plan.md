# katana-document-viewer — UI 分離計画 抜粋

作成日: 2026-05-17  
canonical: [`katana/docs/architecture/ui-separation/detailed-design-and-tasks.md`](../../katana/docs/architecture/ui-separation/detailed-design-and-tasks.md)

## このファイルの位置付け

本ファイルは KatanA ecosystem の **UI 分離構想 master** から `katana-document-viewer` (KDV) 担当部分を抜粋したもの。task ID は master と同一。**master が単一情報源**であり、本ファイル単独で task を追加・修正してはならない。

## Repository の役割

`katana-document-viewer` (KDV) は **文書アーティファクト基盤**として位置付ける。

- 単なる preview crate ではない。document model / artifact model / viewer surface / forge (build/transform/export/artifact generation) / cli_api を所有する。
- preview は viewer surface の一機能。
- `forge` を内部 subsystem として持つ。文書成果物（artifact）と書き出し（export）のpublic API ownerになる。
- CLI から呼び出せる document pipeline API (`cli_api`) を提供する。
- UI に依存しない。`forge` は特に UI / framework 非依存を厳守。

詳細: master [`5.2 katana-document-viewer` 詳細設計](../../katana/docs/architecture/ui-separation/detailed-design-and-tasks.md#52-katana-document-viewer-詳細設計)

## 担当 Phase

- **Phase 2**: KDV 拡張 + forge 内包 (本 repo のメイン作業)
- **Phase 7**: KRR / KDV forge 再編 (KDV 側視点)
- **横断**: P0 (governance), P6 (KMM canonical 切替対応), P8 (documentation)

依存グラフ抜粋:

```
P0 → P2 (本 repo)
       ↓ provides: KDV facade, KRR backend adapter, cli_api skeleton
     P5 (KatanA で preview / export 移行に使用)
     P7 (本 repo の facade が完成している前提でbackend境界を整理)
```

## Phase 2 スコープ宣言 (重要)

Phase 2 は **KDV 側にKRR direct pathとexport契約を追加するだけ**にする。

- 触る対象: KDV のみ。
- KRR は Mermaid / Draw.io / PlantUML / MathJax の直接描画の正本として扱う。ZenUML はKatanA互換では Mermaid fence 内の `zenuml` content として扱われるため、KDVはKMM DTOとbackend結果の組み合わせで評価し、KMMに専用 enum を要求しない。
- CommonMark未構造化要素は、対応backendまたはKMM DTOが揃うまで raw source と diagnostics を保持する。数式（math）はKRRのMathJax SVG生成に委譲し、HTML/PDF/PNG/JPGで同じSVGを使う。KDVがMarkdownを独自parseして補完しない。
- 書き出し（export）はKDV内部の `BuildGraph` / `ArtifactManifest` / `ExportOutput` 契約として定義する。

この境界によって、Phase 2 では外部の削除予定crateに依存しない。

## 先行着手する version 分割

KUC完成前に進められる範囲を `v0.1.0` へ前倒しし、画面操作を伴うviewer実装は `v0.2.0` へ分ける。

- `v0.1.0`: KDV neutral契約、文書成果物（artifact）型、forge API、書き出し（export）型、描画評価の自動検証、KRR委譲窓口（facade）、KMM DTO coverage gapのdiagnostics、CLI API。UI非依存。
- `v0.2.0`: KUC上のMarkdown viewer、hit-test、目次（TOC）、hover、選択、画像・図形操作。KUCの基礎部品を前提にする。
- `v0.3.0`: PDF書き出し（export）の改ページ制御と事前確認viewer。
- `v0.4.0`: Markdown以外のviewer拡張。

## Task list (master 抜粋)

### P2-A. Workspace restructuring

- [ ] P2-A-001: root Cargo.toml の current members を確認する。
- [ ] P2-A-002: `crates/katana-document-viewer` を追加する。
- [ ] P2-A-003: `crates/katana-document-preview` を compatibility facade に変更する。
- [ ] P2-A-004: `crates/katana-document-preview-egui` を deprecated compatibility adapter として扱う。
- [ ] P2-A-005: `crates/katana-document-viewer-egui` を temporary adapter として追加するか判断する。判断基準: (a) KatanA 本体の preview が Phase 5 まで egui を保持する想定か / (b) `katana-document-preview-egui` の compatibility window で代替できるか。決定は ADR `docs/adr/kdv-egui-adapter.md` に記録する。
- [ ] P2-A-006: package descriptions を preview から viewer に更新する。
- [ ] P2-A-007: README の責務説明を document artifact subsystem に更新する。
- [ ] P2-A-008: OpenSpec を preview-only から viewer+forge に更新する。

### P2-B. Source and document model

- [ ] P2-B-001: `DocumentSource` を定義する。
- [ ] P2-B-002: `SourceUri` を定義する。
- [ ] P2-B-003: `SourceKind` を定義する。
- [ ] P2-B-004: `SourceRevision` を定義する。
- [ ] P2-B-005: `DocumentId` を定義する。
- [ ] P2-B-006: `DocumentKind` を定義する。
- [ ] P2-B-007: `DocumentSnapshot` を定義する。
- [ ] P2-B-008: `DocumentOutline` を定義する。
- [ ] P2-B-009: `DocumentMetadataView` を定義する。
- [ ] P2-B-010: KMM input conversion を作る。
- [ ] P2-B-011: KMM parse result conversion を作る。
- [ ] P2-B-012: source serialization test を作る。
- [ ] P2-B-013: document snapshot test を作る。

### P2-C. Artifact model

- [ ] P2-C-001: `ArtifactId` を定義する。
- [ ] P2-C-002: `ArtifactKind` を定義する。
- [ ] P2-C-003: `ArtifactFormat` を定義する。
- [ ] P2-C-004: `ArtifactBytes` を定義する。
- [ ] P2-C-005: `ArtifactUri` を定義する。
- [ ] P2-C-006: `ArtifactManifest` を定義する。
- [ ] P2-C-007: `ArtifactDiagnostics` を定義する。
- [ ] P2-C-008: preview artifact を定義する。
- [ ] P2-C-009: export artifact を定義する。
- [ ] P2-C-010: image artifact を定義する。
- [ ] P2-C-011: PDF artifact を定義する。
- [ ] P2-C-012: Office artifact placeholder を定義する。
- [ ] P2-C-013: artifact manifest serialization test を作る。

### P2-D. Forge API

- [ ] P2-D-001: `forge` module を作る。
- [ ] P2-D-002: `BuildRequest` を定義する。
- [ ] P2-D-003: `BuildProfile` を定義する。
- [ ] P2-D-004: `BuildGraph` を定義する。
- [ ] P2-D-005: `TransformStep` を定義する。
- [ ] P2-D-006: `ExportRequest` を定義する。
- [ ] P2-D-007: `ExportFormat` を定義する。
- [ ] P2-D-008: `ExportOutput` を定義する。
- [ ] P2-D-009: `ForgeDiagnostics` を定義する。
- [ ] P2-D-010: `ForgeError` を定義する。
- [ ] P2-D-011: `ForgeBackend` trait を定義する。
- [ ] P2-D-012: `ForgePipeline` を定義する。
- [ ] P2-D-013: no-UI dependency test を作る。

### P2-E. KRR backend integration

- [ ] P2-E-001: `backend::diagram` module を作る。
- [ ] P2-E-002: KRR `RenderInput` への変換を作る。
- [ ] P2-E-003: KRR `RenderOutput` から Artifact への変換を作る。
- [ ] P2-E-004: KDV export output から ExportOutput への変換を作る。
- [ ] P2-E-005: Mermaid render path を接続する。
- [ ] P2-E-006: Draw.io render path を接続する。
- [ ] P2-E-007: ZenUML render path を接続する。
- [ ] P2-E-008: HTML export path を接続する。
- [ ] P2-E-009: PDF export path を接続する。
- [ ] P2-E-010: PNG export path を接続する。
- [ ] P2-E-011: JPEG export path を接続する。
- [ ] P2-E-012: 削除予定crateに依存しないことを README に記載する。
- [ ] P2-E-013: KRR compatibility tests を作る。

### P2-F. Viewer surface

- [ ] P2-F-001: `ViewerState` を定義する。
- [ ] P2-F-002: `ViewerCommand` を定義する。
- [ ] P2-F-003: `ViewerEvent` を定義する。
- [ ] P2-F-004: `ViewerViewport` を定義する。
- [ ] P2-F-005: `PageModel` を定義する。
- [ ] P2-F-006: `ScrollAnchor` を定義する。
- [ ] P2-F-007: `HighlightRange` を定義する。
- [ ] P2-F-008: preview artifact display model を作る。
- [ ] P2-F-009: PDF page display model を作る。
- [ ] P2-F-010: Office display placeholder model を作る。
- [ ] P2-F-011: image display model を作る。
- [ ] P2-F-012: bundle manifest display model を作る。

### P2-G. Scroll sync

- [ ] P2-G-001: `SourceAnchor` を定義する。
- [ ] P2-G-002: `ArtifactAnchor` を定義する。
- [ ] P2-G-003: `ScrollSyncMap` を定義する。
- [ ] P2-G-004: `SyncResolution` を定義する。
- [ ] P2-G-005: KMM node id から viewer anchor への mapping を作る。
- [ ] P2-G-006: line-column から viewer anchor への mapping を作る。
- [ ] P2-G-007: fingerprint fallback mapping を作る。
- [ ] P2-G-008: unresolved anchor diagnostics を作る。
- [ ] P2-G-009: scroll sync fixture test を作る。
- [ ] P2-G-010: edited document re-resolution test を作る。

### P2-H. CLI API

- [ ] P2-H-001: `cli_api` module を作る。
- [ ] P2-H-002: `CliRequest` を定義する。
- [ ] P2-H-003: `CliOutput` を定義する。
- [ ] P2-H-004: `CliDiagnostics` を定義する。
- [ ] P2-H-005: markdown preview build CLI entry を作る。
- [ ] P2-H-006: export CLI entry を作る。
- [ ] P2-H-007: diagram render CLI entry を作る。
- [ ] P2-H-008: export-debug CLI entry を作る。
- [ ] P2-H-009: existing CLI command compatibility table を作る。
- [ ] P2-H-010: CLI delegate方針をKDV側で記録する。
- [ ] P2-H-011: CLI golden output fixtures を作る。

### P2-I. Adapter and compatibility

- [ ] P2-I-001: existing `MarkdownPreview` trait を KDV API へ adapter する。
- [ ] P2-I-002: `MarkdownSource` を `DocumentSource` に変換する。
- [ ] P2-I-003: `PreviewConfig` を `ViewerConfig` に変換する。
- [ ] P2-I-004: `PreviewOutput` を `ViewerOutput` に変換する。
- [ ] P2-I-005: `PreviewError` を `ViewerError` に変換する。
- [ ] P2-I-006: `katana-document-preview` に deprecated notice を入れる。
- [ ] P2-I-007: `katana-document-preview-egui` を temporary adapter として固定する。
- [ ] P2-I-008: KatanA から preview facade を使えるようにする。

### P2-J. Quality gate update

- [ ] P2-J-001: KDV `just check` に forge no-UI dependency guard を追加する。
- [ ] P2-J-002: KDV `just check` に KRR backend smoke を追加する。
- [ ] P2-J-003: KDV `just check` に CLI API smoke を追加する。
- [ ] P2-J-004: KDV release package に KDV crate を追加する。
- [ ] P2-J-005: KDV release package に preview facade を追加する。
- [ ] P2-J-006: KDV release package に adapter crate を追加する。
- [ ] P2-J-007: artifact fixture tests を CI に追加する。
- [ ] P2-J-008: export fixture tests を CI に追加する。

## Phase 7 (KDV 側視点)

Phase 7 は backend 境界とCLI delegate方針を整理するフェーズ。本 repo (KDV) では canonical schema 化を担当する。

### P7-B (KDV 側で完結する作業)

- [ ] P7-B-001: `RenderInput → BuildRequest` mapping を KDV 側 canonical schema として ADR に記録する。
- [ ] P7-B-002: `RenderConfig → BuildProfile` mapping を canonical schema として ADR に記録する。
- [ ] P7-B-003: `RenderOutput → Artifact` mapping を canonical schema として ADR に記録する。
- [ ] P7-B-004: `RenderDiagnostics → ForgeDiagnostics` mapping を canonical schema として ADR に記録する。

### P7-C (KDV docs に runtime asset policy を移管)

- [ ] P7-C-001: Mermaid runtime asset policy を KDV forge docs に移す。
- [ ] P7-C-002: Draw.io runtime asset policy を KDV forge docs に移す。
- [ ] P7-C-003: ZenUML runtime asset policy を KDV forge docs に移す。
- [ ] P7-C-004: checksum validation policy を KDV forge docs に移す。
- [ ] P7-C-005: reference comparison policy を KDV forge docs に移す。

## 前提 (depends on) / 出力 (provides)

- **前提 (P0 完了)**:
  - `katana-document-viewer` を document artifact subsystem とする ADR (P0-B-003)
  - `preview` を feature 名として残す (P0-B-004)
  - `forge` を KDV 内 subsystem とする (P0-B-005)
  - KRRをMermaid / Draw.io / PlantUML / MathJax direct backendとする (P0-B-006)

- **出力 (Phase 2 完了で他 Phase に提供するもの)**:
  - `katana-document-viewer` package の viewer / forge / cli_api facade
  - `DocumentSource` / `DocumentSnapshot` / `Artifact*` / `ViewerState` / `BuildRequest` / `ExportRequest` 等の DTO
  - KRR backend adapter (Phase 7 で canonical 化される mapping の実装)
  - `katana-document-preview` compatibility facade
  - KDV `cli_api` (Phase 7 でCLI delegate先として使うAPI)

## Done criteria

本 repo に関する master 9 章 Done criteria のうち、該当項目:

- [ ] `katana_document_viewer::forge` が UI なしで compile できる (P2-J-001 で検査)
- [ ] forge は KDV subsystem として public API owner になる
- [ ] 削除予定crateに依存しないことが README に記載されている
- [ ] CLI API が `katana_document_viewer::cli_api` から呼べる
- [ ] preview facade (`katana-document-preview`) に deprecated notice が入っている

## drift 検出

- 本ファイルの task ID は master と完全一致する。
- P8-A-001 の CI script が master と本ファイルの task ID 一致を検査する。

## 参照リンク

- [master detailed-design-and-tasks.md](../../katana/docs/architecture/ui-separation/detailed-design-and-tasks.md)
- [master principles.md](../../katana/docs/architecture/ui-separation/principles.md)
- [overview README](../../katana/docs/architecture/ui-separation/README.md)
- [既存 docs/release.md](release.md)
- [KRR repo の Phase 7 抜粋](../../katana-render-runtime/docs/ui-separation-plan.md)
