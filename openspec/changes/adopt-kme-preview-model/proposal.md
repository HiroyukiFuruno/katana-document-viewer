## Why

> Status: このchangeは初期構想の整理用。実装順序は `v0-1-0-render-export-foundation`、`v0-2-0-markdown-viewer-kuc-integration`、`v0-3-0-pdf-export-pagination`、`v0-4-0-multi-format-viewer` を正とする。

KMMが文書モデルを所有しても、viewer側が別parserや別metadata解釈で表示すると、editor、export、viewerの仕様が再び分かれる。

`katana-document-preview` は未リリース・未取り込みのため、`katana-document-viewer`（KDV）へ改名する。

KDVはKMMモデルをKUCで表示し、hover、選択、AST単位コピー、unresolved metadata表示、HTML/PDF/PNG/JPG exportを提供する。

このchangeはP3として扱う。P0 `katana-ast-lint`、P1 KMM文書モデル、P2 `katana-ui-core` の境界を受けて進める。

## What Changes

- KMM文書モデルをviewer/export inputとして受け取る
- KUC viewerでKMM nodeを表示する
- viewer表示とexportを同じrender pipelineに寄せる
- hit-test metadataをKMM node id/source rangeへ対応させる
- unresolved metadataを表示できる入口を作る
- egui実装継続を前提にしない
- 共通AST lintをviewer側の品質ゲートとして使う
- editor-viewer同期制御はKatanAが持つ前提にする
- Mermaid、Draw.ioの外部描画はKDRへ委譲し、PlantUML、mathなど未対応の外部描画はraw fallbackとdiagnosticsで保持する

## Capabilities

### New Capabilities

- `kme-viewer-model`: KMM文書モデルをKUC viewerとして表示する
- `kme-viewer-export`: KMM文書モデルからHTML/PDF/PNG/JPG exportを行う

## Impact

- `katana-document-viewer` neutral interface: KMM model inputとviewer metadata DTO
- `katana-document-viewer-kuc`: KMM node rendering、hit-test、metadata display、export
- `katana-ast-lint`: P0品質ゲート
- `katana-ui-core`: metadata表示やcopy/edit actionの共通UI部品
- KatanA: viewer selection、metadata表示、editor-viewer同期の接続
