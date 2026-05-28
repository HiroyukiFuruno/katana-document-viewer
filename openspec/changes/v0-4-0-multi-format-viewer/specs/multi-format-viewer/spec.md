## ADDED Requirements

### Requirement: multi-format viewer は実装前にKRR/KDV/KUC境界を判断しなければならない

システムは、PDF / CSV / Office / SVG / WebP / AVIF対応を追加する前に、解析、描画、viewer統合、操作commandの責務をKRR / KDV / KUCへ分類しなければならない（MUST）。

#### Scenario: formatごとの責務を分類する

- **WHEN** `v0.4.0` の実装を開始する
- **THEN** KDVはPDF / CSV / Office / SVG / WebP / AVIFごとに、parse/extract owner、render owner、viewer owner、command ownerを記録する
- **THEN** 判断表には目標品質、入力契約、出力契約、採用owner、却下したownerと理由、KRR側handoff要否を含める
- **THEN** ownerが未確定のformatは実装へ進まない
- **THEN** KRR候補はrender-runtimeとして公開APIに乗せられる描画処理に限定する
- **THEN** KDV候補は `ViewerSource`、source identity、diagnostics、viewer state、host command、KUC bridgeに限定する
- **THEN** KUC候補は解釈済みdisplay modelの表示部品に限定する

#### Scenario: KRR責務に分類する

- **WHEN** format固有入力から描画成果物、layout geometry、raster imageのいずれかを生成する処理が必要である
- **THEN** KDVはその処理がviewer stateやhost commandなしで独立実行できるか確認する
- **THEN** KDVはその処理がKDV以外の利用者にも再利用可能か確認する
- **THEN** KDVはKRR public APIとして入力、出力、diagnosticsを安定化できるか確認する
- **THEN** すべて満たす場合だけKRR責務として扱う

#### Scenario: KDV責務に分類する

- **WHEN** 処理の主目的がsource identity、diagnostics、viewer state、host command、またはKUC bridgeである
- **THEN** KDVはその処理をKDV責務として扱う
- **THEN** KDVは忠実layout描画やrasterizationそのものをKDV責務にしない
- **THEN** KRRで汎用化できる処理をKDVの暫定rendererとして正規経路にしない

#### Scenario: KRR責務と判断した処理がある

- **WHEN** PDF page rendering、Office layout rendering、SVG/image rasterizationのいずれかをKRR責務と判断する
- **THEN** KDVはKRRのpublic APIまたは別changeへのhandoffを前提にする
- **THEN** KDVはKRR内部実装やCLIを直接呼び出す抜け道を持たない
- **THEN** KRR未対応の間はraw sourceとdiagnosticsを保持し、KDV内の暫定rendererを正規経路にしない

### Requirement: ViewerSource を拡張して PDF / CSV / Office / image viewerを提供しなければならない

システムは、責務境界の判断後に `ViewerSource` enum に `Pdf` / `Csv` / `Office`（DOCX / XLSX / PPTX）/ `Image` の variantを追加し、各フォーマットのviewerを提供しなければならない（MUST）。

#### Scenario: owner未確定のformatを扱う

- **WHEN** 責務判断表でownerが未確定のformatがある
- **THEN** KDVはそのformatの `ViewerSource` variantを追加しない
- **THEN** KDVは未対応formatとしてraw sourceとdiagnosticsを返す
- **THEN** KDVは候補実装を暫定正規経路にしない

#### Scenario: PDF を preview する

- **WHEN** ホストが `ViewerSource::Pdf(path)` を渡す
- **THEN** KDVはPDF source identity、page navigation state、diagnosticsを保持する
- **THEN** PDF page renderingの実体はboundary decisionで選んだKRR public APIまたはKDV adapterを使う
- **THEN** ページナビゲーション（前後・ページ番号入力）が利用できる

#### Scenario: CSV をテーブル表示する

- **WHEN** ホストが `ViewerSource::Csv(path)` を渡す
- **THEN** KDVはCSVを表データmodelへ変換する
- **THEN** KUC viewerは表データmodelをtableとして表示する
- **THEN** ヘッダー行の自動検出と列幅の自動調整が適用される
- **THEN** KDVはCSVをKRRへ渡す前提にしない

#### Scenario: Office ドキュメントを表示する

- **WHEN** ホストが `ViewerSource::Office(path)`（DOCX / XLSX / PPTX）を渡す
- **THEN** KDVはsemantic extractionとlayout-faithful renderingを別責務として扱う
- **THEN** 可読性優先の場合、KDVはテキスト・表・画像の抽出modelをKUC viewerへ渡す
- **THEN** layout-faithful renderingを目標にする場合、KRR責務として扱うかをboundary decisionで判断する

#### Scenario: Image sourceを表示する

- **WHEN** ホストが `ViewerSource::Image(path)`（SVG / WebP / AVIF等）を渡す
- **THEN** KDVは画像source identity、zoom、fit、open/copy commandを保持する
- **THEN** decode / rasterizationの実体はboundary decisionで選んだKRR public APIまたはKDV adapterを使う
