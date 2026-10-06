# 設計

## 永続artifactの責任

`PersistentDocumentCache` はhostが指定する専用保存先・総容量・environment revisionを受ける。hostは機密入力の保存可否、保存先の所有/アクセス制御、clear/UI方針を決める。KDVはentry単位のチェックサム・容量制限・原子書込・他プロセスとの排他・破損/欠損の明示・clearを提供する。欠損はmiss、破損やI/O/容量超過は型付きerror。暗黙fallbackや安全ゲート緩和は行わない。

内容SHA256、source URI/revision/MIME/format、KDV/schema/engine版、worker executableのSHA256、worker全config、host environment revisionで変換を無効化する。renderer limits・page・scaleも描画keyに含む。hostは外部font/viewport等の描画環境が変わればenvironment revisionを更新する。決定的Office fontはworker実体のhashで束縛する。

`PersistentOfficeViewerSession` / `PersistentPdfViewerSession` は既存sessionを包み、元のsource/ページ/描画制約をcache hit時にも検査する。PDFはgeometry/outline確定のため再decodeするが、変換・rasterを抑制できる。close/dropでsession内memoryを解放し、disk artifactだけをhostが保持する。

## 計測

既存DebugTraceを維持し、直接PDFのdecode/raster/encode/frame decodeを独立計測する。独立producer processで取得、open、初回frame、同一process close/reopen、独立restartを区別する。実workerのtraceからconversion/raster再実行の有無を証明する。原本未取得のZIP問題は匿名合法ZIPによる検証と分けて報告する。
