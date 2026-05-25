## Context

KDV v0.1.0 は、KUCの画面実装を待たずに、文書成果物（artifact）基盤と書き出し（export）基盤を先に固める。

KDVが最初に持つべき責務は、画面そのものではなく、同じ入力から同じ中間成果物、診断、書き出し結果を作れる契約である。ここを先に自動検証できる形にし、画面操作は後続 `v0.2.0` でKUC部品に接続する。

## Goals / Non-Goals

**Goals:**

- KDVのneutral crateを、UI非依存の文書成果物（artifact）/ forge / export契約として定義する。
- KMM公開データ型（public DTO）を入力にし、parser内部型やrenderer内部型をKDV stateへ漏らさない。
- CommonMark / GFM / 数式（math） / GitHub alert / KatanA互換Markdownを、描画評価の検証用入力（fixture）として棚卸しする。KMM v0で未構造化の記法は、KDVが独自parseせずcoverage gapとしてdiagnosticsへ残す。
- Mermaid / Draw.io / PlantUML / ZenUML互換入力 / 数式（math）はKRR（katana-render-runtime）0.3.2境界へ委譲し、KRR未対応経路はraw sourceとdiagnosticsを保持する。同じSVG生成契約をHTML/PDF/PNG/JPGで使う。
- HTML/PDF/PNG/JPG書き出し（export）が、同じ中間成果物と診断情報を使うことを自動テストで確認できるようにする。

**Non-Goals:**

- KUC上の最終viewer画面を実装しない。
- hover、選択、目次（TOC）、toolbar、scrollbar、split paneなど画面操作を実装しない。
- KRR crate / KRR CLI / KRR public APIを縮小しない。
- KDV内でKMM parserや独自Mermaid / Draw.io / ZenUML / PlantUML rendererを再実装しない。図形と数式（math）はKRR public APIへ委譲する。KRR adapterは、受け取ったTeX文字列をSVG化するだけで、Markdown AST解析を行わない。

## Decisions

### v0.1.0はUI非依存の基盤に限定する

`v0.1.0` は `katana-document-viewer` neutral crate、文書成果物（artifact）型、forge API、export API、描画評価fixture、CLI APIを対象にする。`katana-document-viewer-kuc` の画面実装は `v0.2.0` の対象にする。

### 文書入力と成果物を分ける

KDVは `DocumentSource`、`SourceUri`、`SourceKind`、`SourceRevision`、`DocumentId`、`DocumentKind`、`DocumentSnapshot` を持つ。入力元の種類と、評価・exportに渡す安定したsnapshotを分ける。

書き出し結果は `ArtifactId`、`ArtifactKind`、`ArtifactFormat`、`ArtifactBytes`、`ArtifactUri`、`ArtifactManifest`、`ArtifactDiagnostics` で表す。画面表示用preview、HTML、PDF、PNG、JPEGは同じartifact modelへ載せる。

### forgeをUIから切り離す

`forge` は `BuildRequest`、`BuildProfile`、`BuildGraph`、`TransformStep`、`ExportRequest`、`ExportFormat`、`ExportOutput`、`ForgeDiagnostics`、`ForgeError`、`ForgeBackend`、`ForgePipeline` を持つ。

`forge` moduleはUI frameworkに依存しない。`cargo tree -p katana-document-viewer` で `egui`、`katana-ui-core`、`winit`、`vello` が入らないことを品質ゲートにする。

### 描画評価は中間成果物で行う

KUC完成前は、実画面のスクリーンショットではなく、KMM DTO、KRR結果、render tree相当の中間成果物、artifact manifest、diagnosticsを比較する。検証用入力（fixture）は次に分ける。

- CommonMark基準。全記法をfixture matrixで棚卸しし、KMM DTOで構造化済みのもの、raw source保持になるもの、未対応diagnosticsになるものを分ける。
- GFM基準。GitHub alertのようにKMM DTOがあるものと、未構造化でraw保持になるものを分ける。
- 数式（math）基準。inline math、fenced math、`$$` 内側の半角スペースを許容するKatanA互換入力を含む。
- GitHub alert基準。`> [!NOTE]` / `> [!TIP]` / `> [!IMPORTANT]` / `> [!WARNING]` / `> [!CAUTION]` を含む。
- KatanA互換基準。
- 外部描画成功基準。
- 外部描画失敗時のraw保持基準。

KDVはMarkdown parserの正本を持たない。KMMが構造化したMarkdownを受け取り、KRRの外部描画結果またはKDV diagnosticsを組み込み、その組み合わせが期待するartifact manifestとdiagnosticsになるかを評価する。

KMM v0はKatanA互換を優先し、CommonMark完全coverageを直接保証しない。したがってKDV v0.1.0の「全記法」は、KDVが全記法を独自解釈する意味ではなく、KMM DTO coverage gapを含めて検証用入力（fixture）と診断キーで追跡する意味に限定する。

### exportは同じ中間成果物を使う

HTML/PDF/PNG/JPG書き出し（export）は、KMM DTOを独自に再parseしない。KDVは同じ `BuildRequest` と `BuildGraph` から `ExportOutput` を作る。

PDF/PNG/JPGはHTMLを外部ブラウザで印刷せず、Rust側のnative surfaceから生成する。KRRが返したSVG図形はraw textへ変換せず、surface上の図形blockとしてrasterizeして貼り込む。コードブロックは本文行ではなく、テーマ由来の背景と枠を持つblockとして描く。

数式（math）はHTMLとnative surfaceで別実装にしない。KRR境界がLaTeX互換入力をSVG化し、HTMLではそのSVGを埋め込み、PDF/PNG/JPGでは同じSVGをrasterizeしてsurfaceへ貼り込む。KDVは `katana-render-runtime` 0.3.0 の `MathJaxRenderer` を使い、KRRが描画失敗を返した場合はraw文字列と診断を保持する。

外部描画失敗時は、HTML/PDF/PNG/JPGでもraw本文、代表メッセージ、診断情報を失わない。tooltipなど画面固有の表現は `v0.2.0` へ送る。

fixture matrixで評価対象にしたMarkdown標準記法、数式、GitHub alert、KatanA独自解釈の評価結果は、export形式ごとに再解釈しない。形式差分はartifact formatの変換だけに限定し、評価済みのdocument semanticsは `BuildGraph` 側に固定する。

HTML/PDF/PNG/JPGの完了条件は、同じ `BuildGraph` から生成した成果物が同じ文書意味と見た目を持つことである。HTMLをまず検査基準として固定し、HTMLがMarkdown標準記法、数式、図形、table、code block、GitHub alert、KatanA独自解釈を満たすまで、PDF/PNG/JPG側を完了扱いしない。HTMLの合格判定は自動テストだけで確定せず、利用者が実artifactを確認して明示OKを出した時点で合格とする。PDF/PNG/JPGはHTMLで成立した評価済みsemanticsを別surfaceへ投影するだけにし、surfaceごとにMarkdown ASTやinline/list/table/alert/footnoteを再解釈しない。

形式差分の例外は、accordionの開閉操作はHTMLだけが保持し、PDF/PNG/JPGでは開いた状態の静的本文として出すこと、PDFだけがクリック可能なリンク注釈を保持し、PNG/JPGではクリック可能なリンク注釈を持たなくてよいことに限定する。PDF/PNG/JPGはHTMLの見え方を正とし、accordion開閉操作とリンククリック可否を除くすべての視覚表現をHTMLと同等にする。PNG/JPGでもリンクの色、下線、本文位置はHTML/PDFと互換にする。

4形式の成果物検証は `ExportQualityGate` に集約する。`ExportQualityGate` はHTML、PDF、PNG、JPGを同じE2E fixtureから受け取り、形式ごとの検査項目をscore化する。0 byte、壊れたsignature、decode不能、HTMLのraw Markdown漏れ、PDFのpage tree / page image / link annotation欠落、PNG/JPGの非文書サイズまたはblank画像はfatal failureとして扱い、合計scoreが満点でも合格にしない。

このscore gateは、HTML構造UTやPDF surface UTを置き換えない。記法ごとのUTで失敗原因を狭く固定し、E2Eのscore gateで4形式の成果物として成り立つかを最後に検査する。

### KRR direct backend境界を固定する

KRRはMermaid / Draw.io / PlantUML / MathJaxの直接描画の正本であり、KRR public API上の `DiagramKind` は `Mermaid` / `Drawio` / `PlantUml` / `MathJax` を持つ。KatanA互換のZenUMLはMermaid fence内の `zenuml` contentとして評価し、KDVはKMMにZenUML専用enumを要求しない。

KDVはKRRを呼ぶためのadapterを持ち、返る結果をKDVのartifact modelへ変換する。上位からKDVへ渡された完全テーマ（theme）は、KRRの `RenderThemeSnapshot` に変換して `RenderContext.theme` へ渡す。KRR側の暗色表示判定は `RenderThemeSnapshot.mode` を正とし、KDVが別の暗黙fallbackを持たない。

## Risks / Trade-offs

- 実画面がないため見た目の最終差分は検出できない。代わりに、中間成果物とartifact manifestの差分を固定し、後続viewer実装の回帰検出に使う。
- PDF/PNG/JPGは環境差分が出やすい。`v0.1.0` ではbyte完全一致よりも、manifest、page count、diagnostics、外部描画結果の保持を優先する。
- KRR側の未完成機能をKDVで補うと責務が崩れる。未対応backendはraw保持とdiagnosticsで返す。
