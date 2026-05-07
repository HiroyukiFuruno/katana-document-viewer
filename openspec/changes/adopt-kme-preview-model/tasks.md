# Tasks: adopt-kme-viewer-model

## 0. Repository Rename

### Definition of Ready

- [ ] `katana-document-preview` は未リリース・未取り込みである
- [ ] viewer/exportを同一責務にする方針が親OpenSpecで確定している

### Tasks

- [ ] 0.1 repository計画名を `katana-document-viewer`（KDV）へ更新する
- [ ] 0.2 crate名とOpenSpec名の改名範囲を定義する
- [ ] 0.3 既存 `katana-document-preview` 表記を互換メモとして扱い、release前に消す

### Definition of Done

- [ ] `katana-document-viewer` が正式名称として使われている
- [ ] KatanA側のdependency計画がKDVを参照している

## 1. Viewer Model Contract

### Definition of Ready

- [ ] KME document model DTOが定義済みである
- [ ] P0 `katana-ast-lint` の共通品質ゲート方針が利用可能である
- [ ] P2 `katana-ui-widget` のmetadata表示境界が整理されている
- [ ] Floem viewer implementation方針が確定している

### Tasks

- [ ] 1.1 KME model inputをviewer/export neutral interfaceへ追加する
- [ ] 1.2 KME node id、source range、rendered rect identityをviewer metadataとして返す
- [ ] 1.3 parser/vendor internalsをviewer stateに入れない
- [ ] 1.4 共通AST lintをviewer repository adapterで実行する方針を決める
- [ ] 1.5 editor-viewer同期制御をKDVが持たないことをpublic contractへ明記する

### Definition of Done

- [ ] KME public DTOだけでviewer/export inputが成立する
- [ ] metadata schemaはKMEのpublic contractを使い、KDVで独自定義しない
- [ ] egui型がneutral interfaceに入っていない
- [ ] viewer固有のlint driftを品質ゲートにしていない

## 2. Floem Viewer

### Definition of Ready

- [ ] Task 1のcontractが確定している

### Tasks

- [ ] 2.1 見出し、段落、引用、リスト、table、code、HTML badgeをFloemで表示する
- [ ] 2.2 Mermaid/draw.io/PlantUML/math nodeをKCF renderer resultまたはraw fallbackで表示する
- [ ] 2.3 emojiを削除せず表示側rendererへ渡す

### Definition of Done

- [ ] `sample.md` とREADME badgeがKME model経由で表示できる
- [ ] egui_commonmark vendor patchへ依存していない

## 3. Export Pipeline

### Definition of Ready

- [ ] Task 2のviewer pipelineが確定している

### Tasks

- [ ] 3.1 HTML/PDF/PNG/JPG exportをKDV責務として設計する
- [ ] 3.2 viewer表示とexportが同じrender pipelineを使う方針を固定する
- [ ] 3.3 KCF既存exportからKDVへ移譲する対象と削除条件を定義する

### Definition of Done

- [ ] viewer表示とexport出力が同じKME fixtureで検証できる
- [ ] KCFに新規export責務を追加しない

## 4. Metadata and Interaction

### Definition of Ready

- [ ] KME metadata target resolutionが利用可能である

### Tasks

- [ ] 4.1 hoverと選択をKME node idへ対応させる
- [ ] 4.2 AST単位コピーの入口を提供する
- [ ] 4.3 unresolved metadata表示の入口を提供する

### Definition of Done

- [ ] KatanAがviewer selectionからpublic descriptorを取得できる
- [ ] unresolved metadataが画面上で確認できる

## 5. Final Verification

- [ ] 5.1 KME fixture viewer/export testsを実行する
- [ ] 5.2 共通AST lintのviewer adapterで検査できることを確認する
- [ ] 5.3 `npx -y @fission-ai/openspec validate "adopt-kme-preview-model" --strict` を実行する
