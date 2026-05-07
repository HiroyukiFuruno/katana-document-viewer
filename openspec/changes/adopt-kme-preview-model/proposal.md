## Why

KMEが文書モデルを所有しても、viewer側が別parserや別metadata解釈で表示すると、editor、export、viewerの仕様が再び分かれる。

`katana-document-preview` は未リリース・未取り込みのため、`katana-document-viewer`（KDV）へ改名する。

KDVはKMEモデルをFloemで高速に表示し、hover、選択、AST単位コピー、unresolved metadata表示、HTML/PDF/PNG/JPG exportを提供する。

このchangeはP3として扱う。P0 `katana-ast-lint`、P1 KME文書モデル、P2 `katana-ui-widget` の境界を受けて進める。

## What Changes

- KME文書モデルをviewer/export inputとして受け取る
- Floem viewerでKME nodeを表示する
- viewer表示とexportを同じrender pipelineに寄せる
- hit-test metadataをKME node id/source rangeへ対応させる
- unresolved metadataを表示できる入口を作る
- egui実装継続を前提にしない
- 共通AST lintをviewer側の品質ゲートとして使う
- editor-viewer同期制御はKatanAが持つ前提にする
- Mermaid、Draw.io、PlantUML、mathなどの外部描画はKCFへ委譲する

## Capabilities

### New Capabilities

- `kme-viewer-model`: KME文書モデルをFloem viewerとして表示する
- `kme-viewer-export`: KME文書モデルからHTML/PDF/PNG/JPG exportを行う

## Impact

- `katana-document-viewer` neutral interface: KME model inputとviewer metadata DTO
- `katana-document-viewer-floem`: KME node rendering、hit-test、metadata display、export
- `katana-ast-lint`: P0品質ゲート
- `katana-ui-widget`: metadata表示やcopy/edit actionの共通UI部品
- KatanA: viewer selection、metadata表示、editor-viewer同期の接続
