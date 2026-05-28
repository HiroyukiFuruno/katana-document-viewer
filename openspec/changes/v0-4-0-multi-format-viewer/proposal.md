## Why

`v0.1.x` のrender/export foundation、`v0.2.0` のMarkdown viewer、`v0.3.0` のPDF改ページ書き出し（export）の後に、Markdown以外のドキュメント形式へ対応する。

ただし、PDF・CSV・Office（DOCX / XLSX / PPTX）・画像拡張（SVG / WebP 等）をそのままKDV責務として追加してよいかは未確定である。KRRは図形専用ではなくrender-runtimeであり、PDF page rendering、Office layout rendering、SVG/image rasterizationの一部はKRR側に置く方が自然な可能性がある。一方で、ファイル種別の受け口、viewer状態、diagnostics、KUC表示、host commandはKDV側の責務に見える。

このchangeは、先にKRR/KDV/KUCの責務境界を判断し、その判断に沿って最小のmulti-format viewer契約を追加する。

## What Changes

### Boundary decision gate

- PDF / CSV / Office / SVG / WebP / AVIFごとに、解析、描画、viewer統合、操作commandの責務をKRR / KDV / KUCへ分類する。
- KRRへ置く候補は、render-runtimeとしての公開APIに乗る描画処理に限定する。
- KDVへ置く候補は、`ViewerSource`、source identity、diagnostics、viewer state、host command、KUC bridgeに限定する。
- KUCへ置く候補は、既に解釈済みの表示modelを画面部品として表示する責務に限定する。

### PDF viewer

- PDF page rendering / text geometry抽出をKRRへ置くべきか、KDV adapterへ置くべきかを判断する
- KDVはPDF source identity、page navigation state、selection/copy/open commandを持つ
- ページナビゲーション（前後・ページ番号入力）を提供する
- テキストレイヤー抽出（コピー可能なテキスト選択）は将来対応として扱う

### CSV viewer

- CSVを表データとしてparseし、KUC table表示へ渡す
- ヘッダー行の自動検出、列幅の自動調整を行う
- CSVはrender-runtimeよりもdocument viewerのデータ解釈責務に近いが、表のsurface描画が必要な場合はKRR/KDV境界を別途判断する

### Office viewer（DOCX / XLSX / PPTX）

- Office package解析、semantic extraction、layout-faithful renderingの責務を分離して判断する
- 内容の可読性を優先する場合はKDVの抽出modelとして扱う
- layout-faithful renderingを目標にする場合はKRR側のrender-runtime候補として扱う

### 画像拡張

- SVG / WebP / AVIFのdecode / rasterizationをKRRへ置くべきか、KDV adapterへ置くべきかを判断する
- KDVは画像source identity、zoom、fit、open/copy commandを持つ

## Capabilities

### New Capabilities

- `multi-format-viewer-boundary`: PDF / CSV / Office / image系viewerの責務境界を判断する
- `pdf-viewer`: PDFページ表示・ナビゲーション
- `csv-viewer`: CSVテーブル表示
- `office-viewer`: DOCX / XLSX / PPTXの内容表示
- `image-viewer`: SVG / WebP / AVIFなどの画像表示

## Impact

- `crates/katana-document-viewer/` — `ViewerSource`、source identity、diagnostics、viewer command、KRR/KDV adapter境界
- `crates/katana-document-viewer-kuc/` — 各フォーマットのKUC viewer mode
- `katana-render-runtime` — boundary decisionでKRR責務と判断した描画runtimeだけ、別changeまたは別PRで拡張する
