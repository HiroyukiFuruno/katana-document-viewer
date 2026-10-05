## Why

Issue #58 の current KatanA Typography 候補に対し、Python provenance 検証は sample.md を扱えるが、native visual 採点入口は diagrams に固定されている。新 producer の実入力を既存 reference 非上書きで評価するため、この局所的な導線不足を解消する。

## What Changes

- Typography と Diagrams を明示的な fixture に結合する native 外部 crop 採点入口を提供する。
- 実 PNG decode・四つの外部入力・入力非変更・既存 visual 95 点を維持し、取り違えを回帰検査する。
- 部分再開のworkspace検査を既存Justfileの専用recipeとして併設し、標準RUSTFLAGS・ジョブ数を継承する。通常testの既存本体・対象・順序は維持する。
- 単一 release/v0.5.12 で依存監査・完全品質検査・Draft review・通常公開・fresh consumer・後処理を行う。
- 公開済みKRR0.4.23をregistry依存として採用し、caret最低版・lock・consumer公開契約を一致させる。新graphで全gateを再検証し、上流公開を元HTML正常close/性能受入へ読み替えない。
- 作業証跡ハーネスの機械入力に表示用省略を入れない。完全パス/archive除外/全証跡行を実回帰で保護し、既存の拒否条件を維持する。
- 未承認の非visual数値式は実装せず、#58/#59 の元受入不足を今回の公開だけで Close しない。

## Capabilities

### New Capabilities

- `current-typography-crop-evaluation`: immutable な current Typography/Diagrams 外部入力を正しい native fixture へ結合して visual 採点する。

### Modified Capabilities

なし。

## Impact

Storybook の test-only helper、Justfile、評価導線の文書、workspace 版番号と release 契約、公開KRR依存、作業証跡ハーネスの機械入力と回帰。KDV自身の製品runtimeソース/public API/reference/画像geometry/採点閾値は変更しない。Issue: https://github.com/HiroyukiFuruno/katana-document-viewer/issues/58
