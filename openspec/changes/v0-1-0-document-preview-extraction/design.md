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

GFM仕様のtable、task list item、strikethrough、autolink extension、disallowed raw HTMLを表示対象に含める。tableは列の左寄せ、中央寄せ、右寄せを保持し、表示横幅は100%固定にする。加えて、KatanA READMEとKMM fixtureが前提にしているfootnote、alert、emoji、relative link、heading anchorもKatanA互換対象として扱う。

### KMM公開データ型を正本にする

KDVはKMM node id、source range、生テキスト断片（raw snippet）を描画・hit-test・exportの正本にする。KMMが専用nodeを持つ要素はnode kindで描画し、専用nodeがないinline要素はraw snippetから表示用整形だけを行う。表示用整形で得た構造はKMMの正本データ型として扱わない。

### 見た目テーマ（theme） / 多言語文言（i18n）を必須の境界型（interface）にする

KDVは見た目テーマ（theme）と多言語文言（i18n）を `ViewerConfig` の必須入力として受け取る。`ViewerTheme` と `ViewerI18n` はnull不可とし、`Option` や未指定時の暗黙defaultにしない。KDVは呼び出し側がそのまま渡せるdefault theme presetと英語（en）i18n presetを用意するが、利用側は必ず具体値として `ViewerConfig` に渡す。

全ての色表現は `ViewerTheme` 経由に限定する。本文、背景、code block、table、alert、selection、focus、hover、error icon、error message、error border などの色はrendering codeで直接指定しない。KDV側のAST lintを拡張し、preset定義とtest fixtureを除くhard-coded color literalを検出して失敗させる。

KDV内の固定表示文言は `ViewerI18n` 経由に限定する。外部rendererから返る技術的な詳細エラーはtooltip本文に含めてよいが、代表メッセージやUI labelはi18n presetまたは呼び出し側が渡した文言を使う。

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

### 外部描画の責務

Mermaid、Draw.io、ZenUML、PlantUML、mathはkatana-diagram-rendererへ委譲する。KDR側に未公開backendがある場合や描画に失敗した場合、KDVは独自実装へ逃げず、rawをそのままcode block枠で表示する。枠のborderは `ViewerTheme` のerror系カラーを使う。previewではraw表示にエラーアイコン、代表メッセージ、tooltipの詳細エラーを添える。

## Risks / Trade-offs

- KMM v0.1.0が全CommonMark要素を専用node化していない → raw snippetとsource rangeを維持し、専用node化はKMM側の後続changeで扱う。
- raw HTMLは表示自由度と安全性が衝突する → script実行や危険属性は実行せず、安全な表示またはrawそのまま表示に倒し、previewではエラーアイコン、代表メッセージ、tooltip詳細を添える。
- GitHubサービス側の自動参照はローカルviewerでは再現できない → Markdown本文の表示に限定し、通知や補完は対象外にする。
- Draw.io添付・参照は拡張子だけで判定すると誤検知する → `.drawio` / `.xml` の参照先を読み、実際にDraw.ioとして有効なXMLブロックで始まる場合だけKDRへ渡す。
