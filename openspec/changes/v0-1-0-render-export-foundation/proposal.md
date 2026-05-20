## Why

Markdown描画評価と書き出し（export）の基盤は、KUC完成を待たずに進められる。ここを先に固めないと、後続のviewer画面実装が「見た目は出るが、同じ入力から同じ成果物を作れるか検証できない」状態になる。

`v0.1.0` は、KDVを文書成果物（artifact）基盤として成立させる最小版にする。画面上のhover、選択、目次（TOC）、toolbarなどは `v0.2.0` へ送る。

## What Changes

- `katana-document-viewer` のneutral契約を、UI非依存の文書成果物（artifact）/ forge / export中心へ組み替える。
- KMM公開データ型（public DTO）を受け取り、KMM内部parser型やrenderer内部型をKDV stateへ漏らさない。
- `DocumentSource`、`DocumentSnapshot`、`Artifact*`、`BuildRequest`、`ExportRequest`、`ExportOutput`、`ForgeDiagnostics` を定義する。
- Markdown描画評価の検証用入力（fixture）を、CommonMark / GFM / 数式（math） / GitHub alert / KatanA互換ごとに分類する。
- KMMがMarkdown構造を渡し、KDRが外部描画結果を返す前提で、KDVはMarkdownそのものの評価結果をartifact manifestとdiagnosticsへ固定する。
- viewer実画面ではなく、描画木（render tree）または中間成果物で評価できる自動テストを先に作る。
- Mermaid、Draw.io、ZenUML、PlantUML、mathはKDR/KCFへ委譲し、KDV内に独自rendererを持たない。
- HTML/PDF/PNG/JPG書き出し（export）は、同じ中間成果物と診断情報を使う契約にする。
- KCF既存exportは維持し、KDV側の同等機能と検証が揃うまでKCF側を縮小しない。

## Capabilities

### New Capabilities

- `render-export-foundation`: KUC非依存のKDV neutral契約、描画評価、自動検証、書き出し（export）基盤を提供する。

### Planned

- `markdown-viewer-kuc-integration`: KUC / Floem上でMarkdown viewerを表示し、hit-test、目次（TOC）、hover、選択、画像・図形操作を提供する。
- `pdf-export-pagination`: PDF書き出し（export）の改ページ制御と事前確認viewerを提供する。
- `multi-format-viewer`: PDF / CSV / Office / SVG などMarkdown以外のviewerを提供する。

## Known Constraints

- KDVはeditor-viewer同期制御を持たない。同期制御はKatanAが持つ。
- `v0.1.0` はKUC部品の完成を前提にしない。
- `v0.1.0` はKCF/KDR側のpublic API縮小をしない。縮小や移譲は後続phaseで扱う。

## Impact

- `crates/katana-document-viewer/` — neutral契約、artifact、forge、export、CLI API。
- `crates/katana-document-preview/` — compatibility facade。
- `crates/kdp-linter/` — UI非依存境界と描画評価fixtureを検査する規則。
- 後続 `v0.2.0` — KUC / Floem viewer実装。
