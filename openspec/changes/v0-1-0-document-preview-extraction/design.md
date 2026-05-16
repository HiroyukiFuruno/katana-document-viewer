## Context

KDV v0.1.0 はKMM公開データ型（public DTO）を正本入力にする。KDVがMarkdownを別解釈すると、KatanA、KMM、KDV、exportの表示差分が再発する。

現行OpenSpecは「Markdown documentを表示する」とだけ書いており、標準Markdown（CommonMark）、GitHub風Markdown（GitHub Flavored Markdown / GFM）、KatanA現行互換のどこまでを描画対象にするかが不足している。

## Goals / Non-Goals

**Goals:**

- CommonMark 0.31.2 のブロック要素・インライン要素を表示対象として棚卸しする。
- GFM 0.29-gfm の拡張要素を表示対象として棚卸しする。
- GitHub Docsで実運用されるfootnote、alert、emoji、relative link、heading anchorをKatanA互換対象として扱う。
- KMM v0.1.0の公開データ型で構造化済みのKatanA独自要素をFloem表示へ接続する。
- KMMがまだ専用データ型を持たない要素も、生テキスト断片（raw snippet）とsource rangeを失わず表示する。

**Non-Goals:**

- KDV内でKMM parserを再実装しない。
- KDV内で独自Mermaid、Draw.io、ZenUML、PlantUML、math rendererを持たない。
- GitHubの通知、issue参照、自動補完など、GitHubサービス側の後処理はKDV責務にしない。

## Decisions

### 標準Markdownの基準

標準MarkdownはCommonMark 0.31.2を基準にする。対象は、thematic break、ATX/setext heading、indented/fenced code block、HTML block、link reference definition、paragraph、blank line、blockquote、list item、list、backslash escape、entity/numeric character reference、code span、emphasis/strong、link、image、autolink、raw HTML、hard/soft line break、textual content。

### GFMとGitHub実運用拡張

GFM仕様のtable、task list item、strikethrough、autolink extension、disallowed raw HTMLを表示対象に含める。tableはviewer本文領域の横幅100%で表示し、列ごとの左寄せ、中央寄せ、右寄せを保持する。列幅はcontent-fitを基本にし、最小幅が本文領域を超える場合はtable領域だけ横スクロールにする。加えて、KatanA READMEとKMM fixtureが前提にしているfootnote、alert、emoji、relative link、heading anchorもKatanA互換対象として扱う。

### KMM公開データ型を正本にする

KDVはKMM node id、source range、生テキスト断片（raw snippet）を描画・hit-test・exportの正本にする。KMMが専用nodeを持つ要素はnode kindで描画し、専用nodeがないinline要素はraw snippetから表示用整形だけを行う。表示用整形で得た構造はKMMの正本データ型として扱わない。

この原則はCommonMark、GFM、KatanA独自互換の全てに適用する。KDVがtable cell、alert label、inline math、emoji shortcode、heading anchorを表示用に整形しても、その結果をKMM公開データ型の代替正本として返さない。

### ViewerOutput / ViewerDiagnostics / metadata

`ViewerOutput` はrendered document handle、render tree metadata、export result、`ViewerDiagnostics` を返す。`ViewerDiagnostics` はseverity、diagnostic code、KMM node id、source range、代表メッセージkey、詳細エラー本文を持つ。代表メッセージは `ViewerI18n` から解決し、詳細エラー本文はtooltipやlog向けに保持する。

hit-test metadataは画面上の座標またはrendered nodeからKMM node idとsource rangeへ戻るためのindexとして扱う。KDVはselectionやscroll同期の入力面を提供するが、KatanAのeditor-viewer同期状態やKLE状態は持たない。

unresolved metadata表示は、KMM DTOに存在するがKDVが専用表示できないmetadataを本文から消さずに示す補助表示である。previewでは小さな警告表示、代表メッセージ、tooltip詳細を使い、色と文言は `ViewerTheme` / `ViewerI18n` から取得する。

### 見た目テーマ（theme） / 多言語文言（i18n）を必須の境界型（interface）にする

KDVは見た目テーマ（theme）と多言語文言（i18n）を `ViewerConfig` の必須入力として受け取る。`ViewerTheme` と `ViewerI18n` はnull不可とし、`Option` や未指定時の暗黙defaultにしない。KDVは呼び出し側がそのまま渡せるdefault theme presetと英語（en）i18n presetを用意するが、利用側は必ず具体値として `ViewerConfig` に渡す。KDVが同梱するi18n presetは英語（en）のみとし、日本語（ja）を含む他言語はKatanAなど呼び出し側が `ViewerI18n` として渡す。

全ての色表現は `ViewerTheme` 経由に限定する。本文、背景、code block、table、alert、selection、focus、hover、error icon、error message、error border などの色はrendering codeで直接指定しない。KDV側のAST lintを拡張し、preset定義とtest fixtureを除くhard-coded color literalを検出して失敗させる。

KDV内の固定表示文言は `ViewerI18n` 経由に限定する。外部rendererから返る技術的な詳細エラーはtooltip本文に含めてよいが、代表メッセージやUI labelはi18n presetまたは呼び出し側が渡した文言を使う。

KDV AST lintは既存の `kdp-linter` を拡張する。対象は `crates/katana-document-viewer-floem/src` と、neutral crate内で描画・preset参照に関わるmoduleである。preset定義module、test fixture、lint自身の違反fixtureだけは色literalを許容する。rendering code内のhex color、RGB/RGBA constructor、framework固有named color直指定、preset定義moduleへの直接依存は違反にする。透明色やalpha値も必要な場合は `ViewerTheme` fieldとして定義する。

### KatanA現行互換の対象

KDV v0.1.0は次をKatanA現行互換として扱う。

- 中央寄せHTML block、README badge row、HTML inline、`details` accordion。
- 通常task marker `[x]` / `[ ]` に加え、KatanA fixtureの `[-]` / `[/]`。
- GFM alert `> [!NOTE]` / `> [!TIP]` / `> [!IMPORTANT]` / `> [!WARNING]` / `> [!CAUTION]` と、legacy note block `> **Note**` など。
- description list。
- Unicode emoji と shortcode emoji。
- Mermaid、Draw.io、ZenUML、PlantUML、math fenced block。
- `$$` 内側の半角スペースを許容する寛容なmath表示。
- Draw.ioの直接code block、`.drawio` / `.xml` 添付・参照先の先頭Draw.io判定、KDRによるSVG化表示。
- 外部描画失敗時のraw code block表示、theme由来のerror border、エラーアイコン、代表メッセージ、tooltip詳細。
- inline math、footnote、image、link、autolink、relative link、heading anchor。
- 空code block、長い行、日本語、HTML entity、連続する異種block。
- proseの長い行は折り返し、code blockとraw code blockは横スクロールを基本にする。
- emoji shortcodeはKDVの静的GitHub互換mapで解決し、未知shortcodeはrawのまま表示する。
- heading anchorはKMM metadataがあればそれを使い、ない場合は表示用のGitHub互換slugを生成する。
- relative linkはKDV内で解決・遷移せず、hrefとsource rangeをviewer commandとしてホストへ渡す。

### 外部描画の責務

Mermaid、Draw.io、ZenUML、PlantUML、mathはkatana-diagram-rendererへ委譲する。KDR側に未公開backendがある場合や描画に失敗した場合、KDVは独自実装へ逃げず、rawをそのままcode block枠で表示する。枠のborderは `ViewerTheme` のerror系カラーを使う。previewではraw表示にエラーアイコン、代表メッセージ、tooltipの詳細エラーを添える。

`.drawio` / `.xml` 添付・参照は、Markdown parsingではなく参照解決と軽量signature判定としてKDVが扱う。KDVはdocument root基準で参照先を読み、BOMと空白を除いた先頭がDraw.io XMLとして扱える候補（例: `<mxfile` または `<mxGraphModel`）で始まる場合だけKDRへ渡す。完全なDraw.io妥当性判定とSVG化はKDRの責務とし、path traversalやdocument root外参照は未解決としてraw code block表示に倒す。

### Export pipeline

v0.1.0では、HTML/PDF/PNG/JPG exportをKDV責務として定義し、viewer表示と同じrender pipelineを使う契約と土台を提供する。exportはKMM DTOを再parseせず、viewerと同じrender tree、KDR結果、`ViewerTheme`、`ViewerI18n`、diagnosticsを入力にする。外部描画失敗時は、exportでもraw code block、error border、代表メッセージを保持する。tooltip詳細はHTMLなど対応形式ではtooltipにし、PDF/PNG/JPGでは代表メッセージとraw本文を失わないことを優先する。

## Risks / Trade-offs

- KMM v0.1.0が全CommonMark要素を専用node化していない → raw snippetとsource rangeを維持し、専用node化はKMM側の後続changeで扱う。
- raw HTMLは表示自由度と安全性が衝突する → script実行や危険属性は実行せず、安全な表示またはrawそのまま表示に倒し、previewではエラーアイコン、代表メッセージ、tooltip詳細を添える。
- GitHubサービス側の自動参照はローカルviewerでは再現できない → Markdown本文の表示に限定し、通知や補完は対象外にする。
- Draw.io添付・参照は拡張子だけで判定すると誤検知する → `.drawio` / `.xml` の参照先を読み、実際にDraw.ioとして有効なXMLブロックで始まる場合だけKDRへ渡す。
- raw HTMLはGFM disallowed raw HTMLを最小基準にし、KatanA互換として明示した `details`、badge row、中央寄せHTML以外は原則inertなraw表示に倒す。
