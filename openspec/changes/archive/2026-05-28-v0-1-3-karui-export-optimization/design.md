## Context

KDV v0.1.0ではHTMLを正とし、PDF/PNG/JPGはKMM/KRR評価済みsemanticsをnative surfaceへ投影する方針にした。
Karuiはこの方針を置き換えず、生成済み成果物の後段最適化としてだけ扱う。

## Goals / Non-Goals

### Goals

- Karui適用前後のPDF file size、生成時間、品質scoreを同じfixtureで比較できるようにする。
- Karui適用がPDF link annotation、ページ構造、画像、図形、数式、脚注、コードブロックを壊さないことを自動検査する。
- 採用する場合も既定では無効にし、明示的な最適化モードから開始する。

### Non-Goals

- KaruiをPDF生成backendとして使わない。
- HTMLからPDFへ変換する外部pipelineへ戻さない。
- PNG/JPG生成の正本をKaruiにしない。
- 見た目差分を圧縮効果で正当化しない。

## Design

KDVは通常どおりHTML/PDF/PNG/JPGを生成する。
KaruiはPDF生成後の任意postprocessとして評価し、最適化後PDFを追加成果物として扱う。

評価は次の順で行う。

1. 通常exportを生成し、既存の `ExportQualityGate` を通す。
2. Karui postprocess候補をPDFへ適用する。
3. 最適化後PDFに対して同じ品質ゲートを実行する。
4. file size、処理時間、品質score、fatal failureを比較する。
5. 1件でもfatal failureが増える場合は不採用にする。

PNG/JPG最適化は、PDF最適化が品質契約を満たしたあとに別途評価する。
PDFからPNG/JPGへ展開する経路は、このchangeでは変更しない。

## Risks

- PDF内link annotationやdestinationが削除される可能性がある。
- SVG由来の図形・数式がrasterize後に劣化する可能性がある。
- 最適化時間が短縮効果を上回る可能性がある。
- Karuiの依存追加がbuild/releaseを重くする可能性がある。

## Open Questions

- Karuiをlibraryとして直接呼ぶか、CLIとして呼ぶか。
- 最適化後PDFを既定成果物に置き換えるか、別artifactとして返すか。
- PNG/JPGにも同じpostprocess概念を広げるか。
