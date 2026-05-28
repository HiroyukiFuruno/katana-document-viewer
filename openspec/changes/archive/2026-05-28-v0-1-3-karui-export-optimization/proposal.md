## Why

HTML/PDF/PNG/JPG exportの見た目互換を壊さずに、成果物サイズと生成後処理の効率を改善できる可能性がある。
KaruiはPDF生成本体の代替ではなく、KDV native export後の最適化候補として扱う。

## What Changes

- KaruiをPDF/画像exportの後段最適化候補として評価する。
- HTMLを正とする既存のexport互換契約は維持する。
- PDF生成、pagination、link annotation、SVG rasterize、Markdown評価の責務はKDV native surfaceから動かさない。
- Karui適用時も `ExportQualityGate` のscore、fatal failure、ファイルサイズ、生成時間を比較する。
- Karuiが見た目互換、リンク注釈、数式、図形、脚注、コードブロックのいずれかを壊す場合は採用しない。

## Capabilities

### New Capabilities

- `export-postprocess-optimization`: KDV export成果物に対する後段最適化の評価、採用条件、品質ゲートを定義する

### Modified Capabilities

- なし

## Impact

- `crates/katana-document-viewer` のexport payload後段処理
- `ExportQualityGate` とE2Eのscore出力
- CLI/APIの将来オプション。ただし初期評価では既定動作を変更しない
