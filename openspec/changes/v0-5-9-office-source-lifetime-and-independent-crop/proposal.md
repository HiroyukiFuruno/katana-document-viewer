## Why

Issue #59の実Office受入では資源counterが0でもRSS基準を超え、KDVの原本寿命とPDF backendの解放後residentを分離する必要がある。Issue #58では現行KatanA artifactを明示入力にした独立評価がなく、旧referenceの成功だけでは受入条件を満たせない。

## What Changes

- Office変換成功後、原本bytesをPDF解析前に解放し、metadata・conversion key・診断・返却frameを保持する。
- 原本寿命だけでは解放後RSSを減らせない実測に基づき、高水準DocumentSessionのOffice rasterを既存sandbox/deadline/上限付きworkerへ隔離する。親のmetadata・cacheと低水準公開APIの互換動作は維持する。
- 同実入力・同worker・同倍率で製品変更前後を比較し、KDVによる寄与とbackend/allocatorによる保持を別々に記録する。
- 現行KatanA crop/provenanceを非上書きの明示入力として検証し、visual/semantic/interaction/performance各95点の評価を可能にする。
- 必要な修正のみを単一release branchへ統合し、通常公開・fresh consumer・元下流受入・Issue終了・安全な後処理まで追跡する。

## Capabilities

### New Capabilities

- `office-source-lifetime`: 原本bytesの不要な寿命重複をなくし、製品A/BとRSS責任境界を測定する。
- `independent-current-crop-evaluation`: 外部artifactのprovenanceと四項目の独立採点を既存reference非上書きで評価する。

### Modified Capabilities

なし。既存の画質・上限・deadline・公開型・95点閾値を変更しない。

## Impact

- 対象: Office static/DocumentSession/PDF cacheのprivate描画経路、既存worker dispatch/processのprivate PDF raster mode、回帰・診断、Storybook評価入口、release/OpenSpec。
- Issue: https://github.com/HiroyukiFuruno/katana-document-viewer/issues/59 と https://github.com/HiroyukiFuruno/katana-document-viewer/issues/58。
- sibling repo/registry sourceは編集しない。公開registry依存の調査・更新と再検証をリリース工程に含める。
