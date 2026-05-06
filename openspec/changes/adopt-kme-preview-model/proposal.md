## Why

KMEが文書モデルを所有しても、preview側が別parserや別metadata解釈で表示すると、editor、export、previewの仕様が再び分かれる。

`katana-document-preview` はKMEモデルをFloemで高速に表示し、hover、選択、AST単位コピー、unresolved metadata表示の入口を提供する。

このchangeはP3として扱う。P0 `katana-ast-lint`、P1 KME文書モデル、P2 `katana-ui-widget` の境界を受けて進める。

## What Changes

- KME文書モデルをpreview inputとして受け取る
- Floem previewでKME nodeを表示する
- hit-test metadataをKME node id/source rangeへ対応させる
- unresolved metadataを表示できる入口を作る
- egui実装継続を前提にしない
- 共通AST lintをpreview側の品質ゲートとして使う

## Capabilities

### New Capabilities

- `kme-preview-model`: KME文書モデルをFloem previewとして表示する

## Impact

- `katana-document-preview` neutral interface: KME model inputとpreview metadata DTO
- `katana-document-preview-floem`: KME node rendering、hit-test、metadata display
- `katana-ast-lint`: P0品質ゲート
- `katana-ui-widget`: metadata表示やcopy/edit actionの共通UI部品
- KatanA: preview selectionとmetadata表示の接続
