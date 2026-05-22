## Why

Markdown を PDF export する際、現在はページ境界の制御がなく、見出し・コードブロック・ダイアグラムがページ途中で分断される。印刷・配布用途では読みやすいページ分割が必要。

## What Changes

- Markdown の AST を解析し、ページ境界での分断を避けるための改ページルールを実装する：
  - 見出し（h1 / h2）の前に改ページを挿入する（設定可能）
  - コードブロック・ダイアグラム・テーブルが分断される場合に前ページに留めるか次ページへ送る
  - 強制改ページ記法（`---` 等）のサポート
- PDF export 前にページ分割viewerを表示する（KDV内部のviewer/export共通pipelineを使う）
- `katana-document-viewer-kuc` にページ分割viewer modeを追加する
- export 設定（用紙サイズ・余白・改ページルール）を `ExportConfig` として注入できる設計にする

## Capabilities

### New Capabilities

- `pdf-export-pagination`: ページ境界制御付きPDF exportと事前viewer確認

### Modified Capabilities

- `markdown-export`: 改ページルール・用紙設定を `ExportConfig` 経由で制御できるようにする

## Impact

- DoR: `v0.1.0` のrender/export foundationと `v0.2.0` のKUC viewer integrationが完了していること
- `crates/katana-document-viewer/` — `ExportConfig`、改ページルール定義の追加
- `crates/katana-document-viewer-kuc/` — pagination viewer modeの追加
