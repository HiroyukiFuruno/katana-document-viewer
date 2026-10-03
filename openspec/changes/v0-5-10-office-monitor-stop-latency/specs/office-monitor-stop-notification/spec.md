## ADDED Requirements

### Requirement: 停止通知が監視待機を解除する

macOS Office監視 SHALL 停止通知を待機前にも保持し、停止時にpoll interval完了を不要に待たずthreadをjoinする。

#### Scenario: 待機中に停止する
- **WHEN** 監視threadがpoll待機中にfinishを呼ぶ
- **THEN** 待機を通知で解除しthread終了をjoinする

#### Scenario: 待機前に停止する
- **WHEN** finishの通知が次のparkより先に届く
- **THEN** 通知を失わずstop状態を確認してjoinを完了する

### Requirement: 安全監視と公開動作を維持する

監視 SHALL 既存100ms周期・RSS上限・超過時kill・exceeded結果を維持する。変更 MUST 同入力のmetadata/frame/cache/close資源に影響しない。

#### Scenario: メモリ上限を超える
- **WHEN** 実子processのresidentが設定上限を超える
- **THEN** killを実施しfinishはexceeded=trueを返す

#### Scenario: 同入力OfficeとXLSXを操作する
- **WHEN** 同公開graphの変更前後でframe生成・resize・zoomまたはscroll・closeを実行する
- **THEN** 描画/metadata契約とclose後全8資源0を維持し監視終了待ちだけを改善対象とする

### Requirement: Officeの段階をDEBUG相関で独立追跡する

診断 SHALL 既存DEBUG/session/source相関を使い、ZIP完全性検査、XLSX filter/import/init/evaluate/streaming/artifact/persisted filterの各段階を区別する。未計測段階を他段階の差引だけで原因確定しない。

#### Scenario: XLSXが重い場合
- **WHEN** DEBUGを明示して同原本を開く
- **THEN** 親preflightとworker内parseの細分を同session/sourceで追跡し安全検査を省略しない
