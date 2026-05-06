# Tasks: adopt-kme-preview-model

## 1. Preview Model Contract

### Definition of Ready

- [ ] KME document model DTOが定義済みである
- [ ] P0 `katana-ast-lint` の共通品質ゲート方針が利用可能である
- [ ] P2 `katana-ui-widget` のmetadata表示境界が整理されている
- [ ] Floem preview implementation方針が確定している

### Tasks

- [ ] 1.1 KME model inputをpreview neutral interfaceへ追加する
- [ ] 1.2 KME node id、source range、rendered rect identityをpreview metadataとして返す
- [ ] 1.3 parser/vendor internalsをpreview stateに入れない
- [ ] 1.4 共通AST lintをpreview repository adapterで実行する方針を決める

### Definition of Done

- [ ] KME public DTOだけでpreview inputが成立する
- [ ] egui型がneutral interfaceに入っていない
- [ ] preview固有のlint driftを品質ゲートにしていない

## 2. Floem Rendering

### Definition of Ready

- [ ] Task 1のcontractが確定している

### Tasks

- [ ] 2.1 見出し、段落、引用、リスト、table、code、HTML badgeをFloemで表示する
- [ ] 2.2 Mermaid/draw.io/PlantUML/math nodeをrenderer resultまたはraw fallbackで表示する
- [ ] 2.3 emojiを削除せず表示側rendererへ渡す

### Definition of Done

- [ ] `sample.md` とREADME badgeがKME model経由で表示できる
- [ ] egui_commonmark vendor patchへ依存していない

## 3. Metadata and Interaction

### Definition of Ready

- [ ] KME metadata target resolutionが利用可能である

### Tasks

- [ ] 3.1 hoverと選択をKME node idへ対応させる
- [ ] 3.2 AST単位コピーの入口を提供する
- [ ] 3.3 unresolved metadata表示の入口を提供する

### Definition of Done

- [ ] KatanAがpreview selectionからpublic descriptorを取得できる
- [ ] unresolved metadataが画面上で確認できる

## 4. Final Verification

- [ ] 4.1 KME fixture preview testsを実行する
- [ ] 4.2 共通AST lintのpreview adapterで検査できることを確認する
- [ ] 4.3 `npx -y @fission-ai/openspec validate "adopt-kme-preview-model" --strict` を実行する
