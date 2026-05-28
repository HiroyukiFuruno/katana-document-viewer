## Context

`v0.4.0` はMarkdown以外のviewer拡張を扱う。ただし、PDF / CSV / Office / SVG / WebP / AVIFを単純にKDVへ積むと、render-runtimeであるKRRと責務が重なる可能性がある。

KRRは図形専用ではなくrender-runtimeである。そのため、PDF page rendering、Office layout rendering、SVG/image rasterizationのような「入力から表示可能な描画成果物へ変換する処理」はKRR候補になる。一方で、KDVはdocument viewerとして、source identity、viewer state、diagnostics、host command、KUC bridgeを持つ。

## Goals / Non-Goals

**Goals:**

- PDF / CSV / Office / SVG / WebP / AVIFごとの責務境界を判断する。
- KRRへ置くべき処理とKDVへ置くべき処理を混ぜない。
- KRR未対応の処理をKDV内の暫定rendererで正規経路化しない。
- KDVの `ViewerSource` とKUC viewer modeを、責務判断後の最小範囲で追加する。

**Non-Goals:**

- KRRのpublic APIをこのchange内で勝手に拡張しない。
- KRR内部実装やCLIを直接呼び出さない。
- Officeの完全レイアウト再現をKDVだけで実装しない。
- CSVをrender-runtimeへ寄せる前提にしない。

## Boundary Principles

### 判断を先送りしない

`v0.4.0` の最初の成果物は、formatごとの責務判断表である。判断表は次の列を必須にする。

- format
- 目標品質（可読性優先 / layout faithful / pixel faithful）
- 入力契約
- 出力契約
- parse / extract owner
- render owner
- viewer / command owner
- KRRへ置く理由、またはKRRへ置かない理由
- KDVへ置く理由、またはKDVへ置かない理由
- KUCが受け取るdisplay model
- KRR側handoffが必要か

ownerが未確定のformatは実装へ進めない。`KRR候補` や `KDV候補` のまま実装タスクへ進むことを禁止する。

### KRR候補

KRRは、format固有入力を表示可能な描画成果物へ変換するruntime処理の候補である。

- PDF page rendering / text geometry extraction
- Office layout-faithful rendering
- SVG / WebP / AVIF decodeまたはrasterization
- diagramやmathなど既存KRR契約に近い描画処理

KRR候補にする場合でも、KDVはKRR public APIだけを使う。public APIがない場合は、KRR側の別changeへhandoffし、KDVではraw sourceとdiagnosticsを保持する。

KRR責務にする条件:

- viewer stateやhost commandを必要とせず、入力から描画成果物へ変換できる
- KDV以外の利用者も再利用する価値がある
- layout / raster / geometryなどrender-runtimeとして独立検証できる
- KRR public APIとして入力、出力、diagnosticsを安定化できる

KRR責務にしない条件:

- page navigation、selection、copy、openなどviewer操作が主である
- CSV table modelのように、描画よりデータ解釈が主である
- KUC部品へ渡すview stateを作るだけである
- KRR public APIへ載せる前にKDV固有要件へ強く依存している

### KDV責務

KDVはdocument viewer契約を持つ。

- `ViewerSource` とsource identity
- format detection後のviewer input
- diagnosticsとunsupported metadata保持
- page navigation、zoom、fit、copy、openなどのhost command
- KRR adapterとKUC bridge
- KatanAへ返す操作結果

KDV責務にする条件:

- `ViewerSource` からviewer inputを作る
- source identity、revision、MIME、diagnosticsを保持する
- host commandを返す
- KRR public APIを呼び、KDV artifactまたはdisplay modelへ変換する
- format固有の可読性優先抽出modelを作る

KDV責務にしない条件:

- 忠実なlayout描画やrasterizationそのものが主目的である
- KRRに汎用runtimeとして置ける処理をKDVだけで二重実装する
- KUCの部品描画を直接持つ

### KUC責務

KUCは、解釈済みdisplay modelを画面部品として表示する。

- page list / viewport / scroll
- table view
- image viewport
- toolbarやnavigation controls

KUCはPDF parser、Office parser、CSV parser、KRR adapterを持たない。

## Initial Classification

| Format | Parse / Extract | Render | Viewer / Command |
| --- | --- | --- | --- |
| PDF | KDV adapterまたはKRR public APIの判断対象 | KRR候補。未対応ならKDV adapter候補 | KDV + KUC |
| CSV | KDV | KUC table表示。KRR前提にしない | KDV + KUC |
| DOCX | KDV抽出model、またはlayout renderingをKRR候補 | 可読性優先はKDV/KUC。忠実layoutはKRR候補 | KDV + KUC |
| XLSX | KDV表model | KUC table表示。忠実layoutはKRR候補 | KDV + KUC |
| PPTX | KDV抽出model、またはslide renderingをKRR候補 | slide忠実描画はKRR候補 | KDV + KUC |
| SVG / WebP / AVIF | KDV source model | KRR候補。未対応ならKDV adapter候補 | KDV + KUC |

この分類は実装前の仮説であり、`v0.4.0` の最初の成果物としてKRR public API、KDV既存adapter、KUC表示部品を確認して確定する。

確定後の判断表でownerが未確定のformatは、`ViewerSource` variantを追加しない。未対応formatとしてraw sourceとdiagnosticsを返す。

## Risks / Trade-offs

- KDVにrenderingを抱え込むとKRRと二重実装になる。
- KRRへ寄せすぎると、viewer stateやhost commandまでruntimeへ漏れる。
- CSVはrenderingよりdata viewerの性質が強く、KRRに寄せると責務が膨らむ。
- Officeは「可読性優先」と「忠実layout」で責務が変わるため、最初に目標を固定する必要がある。
