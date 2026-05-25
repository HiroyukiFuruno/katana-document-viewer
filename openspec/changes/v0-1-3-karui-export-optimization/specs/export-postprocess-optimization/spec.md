## ADDED Requirements

### Requirement: export postprocess は既存の見た目互換を壊してはならない

システムは、Karuiなどの後段最適化を適用する場合でも、HTMLを正とするPDF/PNG/JPGの見た目互換を壊してはならない（MUST）。

#### Scenario: KaruiをPDF後段最適化として評価する

- **GIVEN** 通常のKDV native exportでHTML/PDF/PNG/JPGが生成されている
- **WHEN** Karui postprocessをPDFへ適用する
- **THEN** 最適化後PDFは `ExportQualityGate` を通過しなければならない
- **THEN** link annotation、footnote destination、page tree、page image、図形、数式、コードブロック、GitHub alertの検査が通常PDFより劣化してはならない
- **THEN** file sizeと生成時間を通常PDFと比較できなければならない
- **THEN** fatal failureが1件でも増える場合、Karui適用結果を正式成果物として採用してはならない

#### Scenario: KaruiはPDF生成backendを置き換えない

- **WHEN** KDVがPDFを生成する
- **THEN** PDF生成、pagination、link annotation、surface描画はKDV native exportで行う
- **THEN** Karuiは生成済みPDFのpostprocessとしてのみ呼び出される
- **THEN** Karuiが未導入または失敗した場合でも、通常PDF exportは成功または既存のdiagnosticsで失敗しなければならない

#### Scenario: 最適化は既定では無効である

- **WHEN** 利用者が既存のCLI/APIでexportを実行する
- **THEN** Karui postprocessは暗黙に有効化されない
- **THEN** 既定の出力bytes、manifest、diagnosticsは既存のKDV native export契約に従う
- **THEN** Karui適用は明示的な将来オプションまたは実験APIからだけ選択できる
