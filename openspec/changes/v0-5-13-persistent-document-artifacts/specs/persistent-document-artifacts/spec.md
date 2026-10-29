## ADDED Requirements

### Requirement: Host所有の永続artifact
KDV SHALL provide opt-in公開契約によってOffice変換PDFとPDF/Office描画をhost指定の専用directoryへ保存・復元する。既存非永続session APIと安全制約を維持する。

#### Scenario: 独立process restart
- **WHEN** 同じ入力/設定を独立processで再度開く
- **THEN** 変換と同一page/scaleのrasterを再実行せず、同一frameとcache hitを返す
- **AND** PDF geometry/outlineのdecodeを省いたと主張しない

### Requirement: 無効化と機密情報
KDV SHALL source内容/revision/MIME/format、KDV/schema版、静的リンクされた描画engineを含む実host imageの指紋、worker実体/config、host environment revision、page/scale/renderer limitsを再利用判定に含める。同じKDV版のdownstream再buildでも実imageが変われば旧artifactを再利用しない。host SHALL外部font/viewport/config・外部dynamic module等のimage外の環境変更に合わせてrevisionを更新し、機密入力の保存可否・保存期間を所有する。

#### Scenario: 内容または設定の変更
- **WHEN** 再利用判定の対象が変わる
- **THEN** 古いartifactを再利用せずmissする

### Requirement: bounded storageと明示error
KDV SHALL総容量/entry上限を維持し、排他・原子書込・keyに束縛したchecksum・clearを提供する。欠損はmiss、破損/I/O/容量超過は型付きerrorとして返す。

#### Scenario: 破損とclear
- **WHEN** artifactが破損し、その後hostがclearする
- **THEN** 最初に明示errorを返し、clear後はmissとして再生成する
- **AND** clear後も既に開いたsessionは有効で、drop後にsession memoryを保持しない
