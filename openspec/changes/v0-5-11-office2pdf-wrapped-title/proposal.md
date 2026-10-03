## Why

Issue #59の原本PPTXで、autofitを要求しない26ptの中央寄せ折返し見出しがconverter PDF内で15.6ptへ縮小していた。owner Issue https://github.com/developer0hye/office2pdf/issues/1963 の修正を含む0.8.1がregistry公開されたため、公開依存を採用して元入力を再検証する。

Issue #58の外部crop評価では、PythonのIHDR/provenance検査後にRust scorerがcropだけをdecodeし、full PNGの画像データが検査されていなかった。明示入力の完全性に関する小さなowner-layer修正を同じ0.5.11へ統合する。

## What Changes

- office2pdfをexact registry0.8.1へ更新し、直接/推移依存・lockfileの最新互換版を調査・検証する。
- autofitなしの文字サイズと中央アンカー、明示autofitの維持を公開workerのPDF合成変換で回帰検証する。
- 元私有PPTXの原本/独立参照/SHA/既存制限を保持した新graphの再受入証跡を取得する。
- 外部crop/full PNGの実画素デコードを既存native評価入口で必須化し、宣言hashを更新したheader-only/IDAT CRC/zlib破損を拒否する回帰を追加する。
- 全標準gate、Draft current-HEAD review、required checks、通常merge、自動公開、fresh consumer、局所cleanupまで0.5.11として進める。
- KRR公開待ちのHTML、current canonicalの独立非visual評価、全配布受入をOffice局所成功から推定しない。

## Capabilities

### New Capabilities

- `office-wrapped-title-registry-regression`: 公開converter更新後の折返し見出しサイズとautofit契約をworker境界で検証する。
- `current-crop-png-integrity`: provenance検査だけを画像実体の完全性とみなさず、両PNGのデコード成功を採点の前提にする。

### Modified Capabilities

なし。既存受入条件を変更しない。

## Impact

Cargo.toml/Cargo.lock、Office worker回帰、Storybookの外部PNG検証と回帰、OpenSpec、公開依存/版番号。KDVのgeometry補償、sandbox/deadlineの変更、公開APIの変更は行わない。Issue #58/#59の全DoDは旧changeのまま残す。
