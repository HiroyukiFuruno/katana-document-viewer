# Issue #58の入力検証（採点・採用完了ではない）

独立producerはKatanA commit `7b5224d4e6c452446329d7d8f2e4b3c4778e086c`。
原証跡`canonical-public-kdv058-2026-10-01.md`から宣言SHA・寸法・frameを固定し、mainが実ファイルを別途SHA照合した。
宣言は`current-katana-diagrams-declaration.json`。既存tracked PNG/referenceとproducerファイルは変更していない。

## 検証結果

- crop/full/geometry SHAは宣言と一致。
- crop1280×2400、full2565×4774、physical crop2374×4450 at88,268。
- producer schema1、同一completed/UI frame1223、font14、scale2、scroll0、viewport1187×2225。
- hover/selection/code/image/diagram controls0、pointer outside、active_editor_line null、clean state required。
- `rtk proxy just current-preview-crop-verifier-test`: 27/27 PASS。SHA/寸法/frame、欠落field、不正型、誤producer schema、各操作表示、重複JSON key、非有限数、SemVer、IHDR CRCを拒否し、入力不変を検証。
- 同verifierの4明示入力CLIに実producer artifactを指定: exit0、`provenance_verified=true`。
- `rtk proxy just current-preview-crop-score-check`へ同じ4入力を環境変数で渡し、既存Rust image::openでPNG全体decodeと独立visual採点を実行: 1pass/0fail、visual100/95、average99・content100・dimension100・row100・reference-to-candidate100・candidate-to-reference100。raw logは`issue59-rss.oBZVZI/current-crop-visual-score.log`。
- 最新exact registry KUC0.4.1/KRR0.4.22/office2pdf0.8.0のKDV0.5.9 scorerでも、同じ四つのimmutable外部artifactを別途再検証し、exit0・1pass/0fail・visual100/95・average99を維持した。raw: `issue59-rss.oBZVZI/current-crop-visual-score-kuc041-final.log`。これは旧producer7b5224d4の外部artifactを最新scorerで採点した証拠であり、root新HEADや公開予定KDV0.5.9で生成した新cropの証拠ではない。

## 境界

Pythonではsignature・IHDR CRC・寸法を検査し、完全decodeはRust scorerで成功した。
manifestのproducer/依存情報は原証跡の固定宣言であり、PNGから復元した情報ではない。
visualは当該immutable候補に限り100、semantic/interaction/performanceは`not_evaluated`。4項目各95点・採用・公開・下流受入を満たしたとは報告しない。
独立した意味・挙動・性能証拠と採用判断を3.2に残す。producerがその後更新したHEADの採点へ、7b5224d4の結果を転用しない。

## 実行形

```bash
rtk proxy python3 -B scripts/feasibility/verify-current-preview-crop.py \
  --manifest openspec/changes/v0-5-9-office-source-lifetime-and-independent-crop/evidence/current-katana-diagrams-declaration.json \
  --crop <producer-crop.png> --full <producer-full.png> --geometry <producer-geometry.json>
```

入力pathにはproducerの元ファイルを指定する。missing/mismatchは失敗し、KDV自己レンダーや旧referenceへfallbackしない。
