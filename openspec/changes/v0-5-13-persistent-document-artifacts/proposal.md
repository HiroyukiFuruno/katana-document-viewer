# Office/PDF永続artifactとロード計測

Issue #65/#66/#67のKDV内責務を解消し、進行中KRR修正版を公開registryから取り込んでリリースする。

## 要求

Office変換PDFとPDF描画ページを、host所有の専用directoryへ保存・復元できるopt-in公開APIを追加する。既存session APIは維持する。独立プロセス再起動でconversion/raster抑制を証明し、cold/warm/restartの測定範囲を明示する。合法PPTX ZIPのdescriptor処理は実workerで切り分ける。

## 完了条件

一次台帳は `../../issue-release-tasks.md`。元の品質閾値/coverage/reference、安全制約を維持する。KRR公開前にKDVだけをリリースしない。下流アプリ・配布版受入はKDV完了条件にしない。
