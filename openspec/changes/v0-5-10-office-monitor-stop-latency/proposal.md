## Why

Issue #59のOffice段階診断で、公開0.5.9のmacOSメモリ監視終了が原本PPTXの各workerに9–104ms、代表DOCX/PPTXのrasterに77–82msかかることを確認した。監視threadの100ms sleepを終了通知で解除できないため、既に終わったworkerの監視待ちが表示とcloseに加算される。

## What Changes

- 監視終了を通知可能にし、既存100ms周期・RSS上限・超過時kill・joinを維持する。
- 待機前の通知保持と待機中の解除を回帰検証する。
- 同入力・同公開依存で待機段階とframe/metadata/資源をA/B検証し、0.5.10を通常リリースする。
- 監視修正だけでOffice全体改善とせず、XLSX filter/import/evaluate/streamingと親ZIP完全性の段階を既存DEBUG相関トレースで細分計測する。安全検査は省かない。
- HTML正常close、配布版受入、Issue #58の独立非visual採点を今回の局所改善から推定しない。既存未完DoDは変更しない。

## Capabilities

### New Capabilities

- `office-monitor-stop-notification`: macOS Office監視の終了通知と安全契約の両立。

### Modified Capabilities

なし。

## Impact

`office_worker_monitor.rs`と回帰test、workspace版番号、OpenSpec・公開依存検証。公開API・sandbox・deadline・描画内容・品質閾値は変更しない。XLSX共有監視への影響も検証する。
