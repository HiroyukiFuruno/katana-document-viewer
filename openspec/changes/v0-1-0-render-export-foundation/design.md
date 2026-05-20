## Context

KDV v0.1.0 は、KUC / Floemの画面実装を待たずに、文書成果物（artifact）基盤と書き出し（export）基盤を先に固める。

KDVが最初に持つべき責務は、画面そのものではなく、同じ入力から同じ中間成果物、診断、書き出し結果を作れる契約である。ここを先に自動検証できる形にし、画面操作は後続 `v0.2.0` でKUC部品に接続する。

## Goals / Non-Goals

**Goals:**

- KDVのneutral crateを、UI非依存の文書成果物（artifact）/ forge / export契約として定義する。
- KMM公開データ型（public DTO）を入力にし、parser内部型やrenderer内部型をKDV stateへ漏らさない。
- CommonMark / GFM / 数式（math） / GitHub alert / KatanA互換Markdownを、描画評価の検証用入力（fixture）として棚卸しする。
- Mermaid、Draw.io、ZenUML、PlantUML、mathをKDR/KCF委譲として扱う。
- HTML/PDF/PNG/JPG書き出し（export）が、同じ中間成果物と診断情報を使うことを自動テストで確認できるようにする。

**Non-Goals:**

- KUC / Floem上の最終viewer画面を実装しない。
- hover、選択、目次（TOC）、toolbar、scrollbar、split paneなど画面操作を実装しない。
- KCF crate / KCF CLI / KCF public APIを縮小しない。
- KDV内でKMM parserや独自Mermaid / Draw.io / ZenUML / PlantUML / math rendererを再実装しない。

## Decisions

### v0.1.0はUI非依存の基盤に限定する

`v0.1.0` は `katana-document-viewer` neutral crate、文書成果物（artifact）型、forge API、export API、描画評価fixture、CLI APIを対象にする。`katana-document-viewer-floem` の画面実装は `v0.2.0` の対象にする。

### 文書入力と成果物を分ける

KDVは `DocumentSource`、`SourceUri`、`SourceKind`、`SourceRevision`、`DocumentId`、`DocumentKind`、`DocumentSnapshot` を持つ。入力元の種類と、評価・exportに渡す安定したsnapshotを分ける。

書き出し結果は `ArtifactId`、`ArtifactKind`、`ArtifactFormat`、`ArtifactBytes`、`ArtifactUri`、`ArtifactManifest`、`ArtifactDiagnostics` で表す。画面表示用preview、HTML、PDF、PNG、JPEGは同じartifact modelへ載せる。

### forgeをUIから切り離す

`forge` は `BuildRequest`、`BuildProfile`、`BuildGraph`、`TransformStep`、`ExportRequest`、`ExportFormat`、`ExportOutput`、`ForgeDiagnostics`、`ForgeError`、`ForgeBackend`、`ForgePipeline` を持つ。

`forge` moduleはUI frameworkに依存しない。`cargo tree -p katana-document-viewer` で `egui`、`floem`、`winit`、`vello` が入らないことを品質ゲートにする。

### 描画評価は中間成果物で行う

KUC完成前は、実画面のスクリーンショットではなく、KMM DTO、KDR/KCF結果、render tree相当の中間成果物、artifact manifest、diagnosticsを比較する。検証用入力（fixture）は次に分ける。

- CommonMark基準。
- GFM基準。
- 数式（math）基準。inline math、fenced math、`$$` 内側の半角スペースを許容するKatanA互換入力を含む。
- GitHub alert基準。`> [!NOTE]` / `> [!TIP]` / `> [!IMPORTANT]` / `> [!WARNING]` / `> [!CAUTION]` を含む。
- KatanA互換基準。
- 外部描画成功基準。
- 外部描画失敗時のraw保持基準。

KDVはMarkdown parserの正本を持たない。KMMが構造化したMarkdownを受け取り、KDR/KCFの外部描画結果を組み込み、その組み合わせが期待するartifact manifestとdiagnosticsになるかを評価する。

### exportは同じ中間成果物を使う

HTML/PDF/PNG/JPG書き出し（export）は、KMM DTOを独自に再parseしない。KDVは同じ `BuildRequest` と `BuildGraph` から `ExportOutput` を作る。

外部描画失敗時は、HTML/PDF/PNG/JPGでもraw本文、代表メッセージ、診断情報を失わない。tooltipなど画面固有の表現は `v0.2.0` へ送る。

Markdown標準の全記法、数式、GitHub alert、KatanA独自解釈の評価結果は、export形式ごとに再解釈しない。形式差分はartifact formatの変換だけに限定し、評価済みのdocument semanticsは `BuildGraph` 側に固定する。

### KCF/KDRはtransitional backendとして扱う

KDVはKCF/KDRを呼ぶためのadapterを持つ。KCFから返る結果はKDVのartifact modelへ変換する。KCF側のpublic API縮小やCLI delegate化はこのchangeでは行わない。

## Risks / Trade-offs

- 実画面がないため見た目の最終差分は検出できない。代わりに、中間成果物とartifact manifestの差分を固定し、後続viewer実装の回帰検出に使う。
- PDF/PNG/JPGは環境差分が出やすい。`v0.1.0` ではbyte完全一致よりも、manifest、page count、diagnostics、外部描画結果の保持を優先する。
- KCF/KDR側の未完成機能をKDVで補うと責務が崩れる。未対応backendはraw保持とdiagnosticsで返す。
