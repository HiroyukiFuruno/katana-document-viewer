## ADDED Requirements

### Requirement: DocumentViewer trait でフォーマット非依存のviewer契約を提供しなければならない

システムは、`DocumentViewer` trait、`ViewerConfig`、`ViewerTheme`、`ViewerI18n`、`ViewerInteractionConfig`、`ViewerSource`（KMM document / 画像 / PDF / Binary 等を統一的に扱う enum）、`ViewerOutput`、`ViewerDiagnostics` を `katana-document-viewer` neutral crate として提供しなければならない（MUST）。

#### Scenario: KMM documentをviewerに渡す

- **WHEN** ホストが `ViewerSource::KmeDocument` を `DocumentViewer::render` に渡す
- **THEN** viewer はKMM public DTOを描画する
- **THEN** ダイアグラムブロックはkatana-diagram-rendererの外部描画結果を組み込む

#### Scenario: 見た目テーマ（theme）と多言語文言（i18n）とinteraction設定を必須入力として渡す

- **WHEN** ホストが `ViewerConfig` を構築する
- **THEN** `ViewerTheme`、`ViewerI18n`、`ViewerInteractionConfig` はnull不可の必須fieldとして渡される
- **THEN** KDVはdefault theme preset、英語（en）i18n preset、default interaction presetを提供する
- **THEN** presetを使う場合でも、ホストはpreset値を明示的に `ViewerConfig` へ渡す
- **THEN** KDVは未指定時に内部defaultへ暗黙fallbackしない

#### Scenario: interaction設定で表示機能を切り替える

- **WHEN** ホストが `ViewerInteractionConfig` に `hover_highlight_enabled`、`image_controls_enabled`、`diagram_controls_enabled` を指定する
- **THEN** KDVはhover highlight、画像制御群、図形制御群の表示有無をその値だけで決定する
- **THEN** rendering code内にこれらの表示有無を固定するhard-coded conditionを持たない

#### Scenario: 色と表示文言をinterfaceから解決する

- **WHEN** Floem viewerが本文、背景、code block、table、alert、selection、focus、hover、image controls、diagram controls、error icon、error message、error borderを描画する
- **THEN** 全ての色は `ViewerTheme` から取得する
- **THEN** rendering code内にhard-coded color literalを持たない
- **THEN** 代表メッセージ、tooltip label、空状態などの固定表示文言は `ViewerI18n` から取得する
- **THEN** 外部renderer由来の詳細エラーはtooltip本文へ渡せるが、代表メッセージは `ViewerI18n` の文言を使う

#### Scenario: ViewerOutputとViewerDiagnosticsを返す

- **WHEN** viewerまたはexport pipelineがKMM documentを処理する
- **THEN** `ViewerOutput` はrendered document handle、render tree metadata、export result、`ViewerDiagnostics` を返す
- **THEN** `ViewerDiagnostics` はseverity、diagnostic code、KMM node id、source range、代表メッセージkey、詳細エラー本文を持つ
- **THEN** 代表メッセージkeyは `ViewerI18n` で解決される

#### Scenario: katana-document-viewer は egui / KDR 実装本体に依存しない

- **WHEN** `cargo tree -p katana-document-viewer` を実行する
- **THEN** `egui` は含まれない
- **THEN** katana-diagram-renderer の公開契約のみ参照し、特定の Mermaid 実装に依存しない

### Requirement: katana-document-viewer-floem がMarkdown previewとexport pipeline土台を提供しなければならない

システムは、KMM node rendering、hit-test metadata、unresolved metadata表示、katana-diagram-renderer経由の外部描画組み込み、HTML/PDF/PNG/JPG export pipeline契約と土台を `katana-document-viewer-floem` impl crate として提供しなければならない（MUST）。

#### Scenario: MarkdownをFloem viewerで表示する

- **WHEN** ホストが `FloemDocumentViewer` に `ViewerSource::KmeDocument` を渡す
- **THEN** Markdown documentがFloem viewerに描画される
- **THEN** rendered node はKMM node idとsource rangeへ戻れる

#### Scenario: hit-test metadataでKMM位置へ戻る

- **WHEN** ホストがrendered nodeまたは画面座標に対してhit-testを行う
- **THEN** KDVは対応するKMM node idとsource rangeを返す
- **THEN** KDVはselectionやscroll同期の入力面を提供する
- **THEN** editor-viewer同期状態はKatanAが保持する

#### Scenario: unresolved metadataを補助表示する

- **WHEN** KMM documentにKDVが専用表示できないmetadataが含まれる
- **THEN** Floem viewerは本文を削除せず、小さな警告表示、代表メッセージ、tooltip詳細でmetadata未解決を示す
- **THEN** warning colorと文言は `ViewerTheme` / `ViewerI18n` から取得する

#### Scenario: 図形ブロックをKDR経由で描画する

- **WHEN** viewer に Mermaid / Draw.io / ZenUML / PlantUML / math block が含まれる
- **THEN** katana-diagram-rendererの外部描画結果をviewer/exportへ組み込む
- **THEN** viewer crate 内に独自 Mermaid / Draw.io / ZenUML / PlantUML / math 描画は含まれない

#### Scenario: viewerとexportが同じrender pipelineを使う

- **WHEN** ホストがHTML/PDF/PNG/JPG exportを要求する
- **THEN** exportはviewer表示と同じrender tree、KDR結果、`ViewerTheme`、`ViewerI18n`、`ViewerDiagnostics` を使う
- **THEN** exportはKMM DTOを独自に再parseしない
- **THEN** 外部描画失敗時もraw code block、error border、代表メッセージを失わない

#### Scenario: KatanAがeditor-viewer同期を制御する

- **WHEN** viewer上のnode selectionやscrollが必要になる
- **THEN** KDVはhit-test metadataとviewer command surfaceを提供する
- **THEN** KatanAがviewerまたはeditorへ命令する
- **THEN** KDVはKLEやKatanA統合状態を知らない

#### Scenario: 外部からscrollを制御する

- **WHEN** ホストがKMM node id、source range、heading anchor、またはscroll fractionを指定してKDVへscroll commandを送る
- **THEN** KDVは対象位置へviewerをscrollする
- **THEN** 対象が存在しない場合は失敗結果を返し、viewer状態を壊さない
- **THEN** KDVはscroll同期状態を保持せず、ホストが同期方針を管理する

#### Scenario: hover highlightを表示する

- **WHEN** `ViewerInteractionConfig.hover_highlight_enabled` がtrueで、pointerがrendered node上にある
- **THEN** Floem viewerは対象nodeをtheme由来のhover背景またはborderでhighlightする
- **THEN** hover highlightはKMM node idとsource rangeへ戻れる
- **WHEN** `hover_highlight_enabled` がfalseである
- **THEN** Floem viewerはhover highlightを描画しない

#### Scenario: 画像と図形に制御群を表示する

- **WHEN** `ViewerInteractionConfig.image_controls_enabled` がtrueで、pointerまたはfocusが画像上にある
- **THEN** Floem viewerは画像上に制御群overlayを表示する
- **WHEN** `ViewerInteractionConfig.diagram_controls_enabled` がtrueで、pointerまたはfocusが外部描画結果上にある
- **THEN** Floem viewerは図形上に制御群overlayを表示する
- **THEN** 制御群は拡大/fit、元画像またはrendered assetを開く、source参照をcopyする操作をviewer commandとしてホストへ渡せる
- **THEN** 対応する設定がfalseの場合、制御群overlayを表示しない

### Requirement: KMM AST由来の目次（TOC）をKDVが表示し、KatanAがeditor同期を制御しなければならない

システムは、KMM AST解析結果から得た見出し構造を正本にして目次（TOC）を表示しなければならない（MUST）。KDVは目次view、プレビュー側anchor解決、TOC click commandを提供しなければならない（MUST）。KatanAは目次panel配置、表示/非表示、editor scroll、preview-editor同期方針を保持しなければならない（MUST）。

#### Scenario: KMM AST由来の見出し構造から目次を構築する

- **WHEN** KMM documentにheading nodeが含まれる
- **THEN** KDVはKMM AST由来の見出しlevel、表示text、KMM node id、source range、heading anchor候補から目次itemを構築する
- **THEN** KDVはMarkdown本文を再parseして目次の正本を作らない
- **THEN** 目次itemはrendered heading anchor mapと対応できるmetadataを持つ

#### Scenario: 目次clickでプレビューをscrollしホストへeditor命令を通知する

- **WHEN** ユーザーが目次itemをclickする
- **THEN** KDVは対応するrendered heading anchorへプレビューをscrollする
- **THEN** KDVはKMM node id、source range、heading anchorを含むviewer commandをホストへ通知する
- **THEN** KatanAはviewer commandを受けてeditor scrollを実行する
- **THEN** KDVはKLE、editor buffer、editor scroll stateを直接参照しない

#### Scenario: active headingをpreview/editorと連動する

- **WHEN** プレビューのscroll位置が変わる
- **THEN** KDVはlayout後に確定したrendered heading anchor mapからactive headingを決定する
- **THEN** KDVは固定offsetや行番号だけの推測でactive headingを決定しない
- **WHEN** editorのscroll位置が変わる
- **THEN** KatanAはeditor側のactive headingをKMM node idまたはsource rangeとしてKDVへ渡す
- **THEN** KDVは渡されたactive headingに対応する目次itemをhighlightできる

#### Scenario: 目次panelの配置と表示状態はKatanAが管理する

- **WHEN** KatanAがworkspace layoutを構築する
- **THEN** KatanAは目次panelの表示位置、表示/非表示、初期表示を管理する
- **THEN** KDVは目次componentと命令面を提供し、KatanAのlayout policyやworkspace設定を持たない

### Requirement: Markdown標準記法の描画対象を明示しなければならない

システムは、標準Markdown（CommonMark）0.31.2 の全ブロック要素と全インライン要素を、KDVの描画対象として棚卸ししなければならない（MUST）。KMMが専用nodeをまだ持たない記法でも、生テキスト断片（raw snippet）とsource rangeを失ってはならない（MUST NOT）。

#### Scenario: CommonMarkのブロック要素を表示する

- **WHEN** KMM documentにthematic break、ATX heading、setext heading、indented code block、fenced code block、HTML block、link reference definition、paragraph、blank line、blockquote、list item、listが含まれる
- **THEN** Floem viewerは各要素を表示する
- **THEN** rendered node はKMM node idまたはKMM source rangeへ戻れる
- **THEN** KMMが専用nodeを持たない要素はrawをそのまま表示し、削除されない
- **THEN** 表示用整形で得た構造はKMM公開データ型の代替正本にならない

#### Scenario: CommonMarkのインライン要素を表示する

- **WHEN** KMM documentにbackslash escape、entity reference、numeric character reference、code span、emphasis、strong emphasis、link、image、autolink、raw HTML、hard line break、soft line break、textual contentが含まれる
- **THEN** Floem viewerは本文として表示する
- **THEN** link、image、raw HTML、line breakはraw textのまま潰れず、表示またはrawそのまま表示になる
- **THEN** inline表示用の追加解釈はKMM公開データ型の代替正本にならない

### Requirement: GFMとGitHub実運用拡張をKatanA互換対象として扱わなければならない

システムは、GitHub風Markdown（GitHub Flavored Markdown / GFM）のtable、task list item、strikethrough、autolink extension、disallowed raw HTMLを表示対象に含めなければならない（MUST）。tableは列の左寄せ、中央寄せ、右寄せを保持し、表示横幅を100%固定にしなければならない（MUST）。また、KatanA READMEとKMM fixtureが前提にしているfootnote、alert、emoji、relative link、heading anchorもKatanA互換対象として扱わなければならない（MUST）。

#### Scenario: GFM主要要素を表示する

- **WHEN** KMM documentにtable、task list item、strikethrough、GFM autolinkが含まれる
- **THEN** Floem viewerはtableの列ごとの左寄せ、中央寄せ、右寄せ、本文領域の横幅100%固定、task marker、取り消し線、autolinkを表示する
- **THEN** tableの最小幅が本文領域を超える場合はtable領域だけ横スクロールする
- **THEN** raw HTMLはKatanA互換として明示した要素以外をinertなraw表示に倒し、script実行や危険属性実行を行わない
- **THEN** 表示用整形で得た構造はKMM公開データ型の代替正本にならない

#### Scenario: GitHub実運用拡張を表示する

- **WHEN** KMM documentにfootnote、GFM alert `> [!NOTE]` / `> [!TIP]` / `> [!IMPORTANT]` / `> [!WARNING]` / `> [!CAUTION]`、emoji shortcode、relative link、heading anchorが含まれる
- **THEN** Floem viewerはKatanA README互換の見え方を維持する
- **THEN** KMMが専用nodeを持たないfootnoteやlinkはraw snippetとsource rangeを保持する
- **THEN** emoji shortcodeはKDVの静的GitHub互換mapで解決し、未知shortcodeはrawのまま表示する
- **THEN** heading anchorはKMM metadataがあればそれを使い、ない場合は表示用のGitHub互換slugを生成する
- **THEN** relative link clickはhrefとsource rangeをviewer commandとしてホストへ渡す
- **THEN** GitHubサービス側の通知、issue参照、自動補完はKDV責務にしない

### Requirement: KatanA独自のMarkdown表示仕様を取り込まなければならない

システムは、KMM canonical fixtures とKatanA現行previewで固定されている独自表示仕様を、KDV v0.1.0 の互換対象に含めなければならない（MUST）。

#### Scenario: KatanA固有ブロックを表示する

- **WHEN** KMM documentに中央寄せHTML block、README badge row、`details` accordion、description list、legacy note block、GFM alert blockが含まれる
- **THEN** Floem viewerは中央寄せ、同一行badge、折りたたみ相当表示、description list、note/alert種別を保持して表示する
- **THEN** unsupported metadataはunresolved metadata表示へ接続できる

#### Scenario: KatanA固有task markerと文字要素を表示する

- **WHEN** KMM documentに `[x]`、`[ ]`、`[-]`、`[/]` のtask marker、Unicode emoji、shortcode emoji、日本語、HTML entity、長い行が含まれる
- **THEN** Floem viewerは各markerと文字を欠落させず表示する
- **THEN** proseの長い行は折り返し、code blockとraw code blockは横スクロールし、layoutを破壊しない

#### Scenario: 寛容なmathを表示する

- **WHEN** KMM documentに `$$ E = mc^2 $$` のように区切り内側へ半角スペースを含むmathが含まれる
- **THEN** Floem viewerは厳密エラー扱いにせず、KatanA現行preview互換で数式として表示する
- **THEN** 元のraw snippetとsource rangeは保持する

#### Scenario: inline mathを表示する

- **WHEN** KMM documentに `$ E = mc^2 $` のようなinline mathが含まれる
- **THEN** Floem viewerはKatanA現行preview互換でinline数式として表示する
- **THEN** 誤判定を避けるため、KMMがmathとして構造化していない金額表記や通常textはKDVが独自にmath化しない
- **THEN** 元のraw snippetとsource rangeは保持する

#### Scenario: Draw.ioの直接入力と拡張子付き添付を図形として扱う

- **WHEN** KMM documentに `drawio` / `draw.io` code block、または `.drawio` / `.xml` 拡張子の添付・Markdown画像参照・HTML `img`・local pathが含まれる
- **THEN** KDVはdocument root基準で参照先を解決し、root外参照やpath traversalを拒否する
- **THEN** KDVはBOMと空白を除いた先頭がDraw.io XML候補（例: `<mxfile` または `<mxGraphModel`）で始まることを確認してからkatana-diagram-rendererへ渡す
- **THEN** 有効なDraw.io入力はkatana-diagram-rendererでSVG化し、viewer/exportへ組み込む
- **THEN** 拡張子だけ一致する入力、取得不能な入力、解析不能な入力はDraw.io図形として扱わず、rawをそのまま表示し、本文から削除しない

#### Scenario: 外部描画失敗時にrawとエラー補助UIを表示する

- **WHEN** Draw.io、ZenUML、Mermaid、PlantUML、mathの外部描画が取得不能、解析不能、またはKDR未対応で失敗する
- **THEN** Floem viewerは元のrawをcode block枠の中にそのまま表示する
- **THEN** code block枠のborderは `ViewerTheme` のerror系カラーを使う
- **THEN** preview上ではraw表示にエラーアイコンと代表メッセージを添える
- **THEN** エラーアイコンまたは代表メッセージのtooltipで詳細エラーを確認できる
- **THEN** エラーアイコン、代表メッセージ、tooltip labelの表示文言は `ViewerI18n` から取得する

#### Scenario: ZenUMLをMermaid互換図形として扱う

- **WHEN** KMM documentにZenUML図形が含まれる
- **THEN** KDVはkatana-diagram-rendererのZenUMLまたはMermaid互換描画結果をviewer/exportへ組み込む
- **THEN** ZenUML依存関係やbackendが未対応の場合もsourceを失わず、rawをcode block枠でそのまま表示し、previewではtheme由来のerror border、エラーアイコン、代表メッセージ、tooltip詳細を添える

#### Scenario: 外部描画blockをKDRへ委譲する

- **WHEN** KMM documentにMermaid、Draw.io、ZenUML、PlantUML、math fenced blockが含まれる
- **THEN** KDVはkatana-diagram-rendererの外部描画結果をviewer/exportへ組み込む
- **THEN** KDV内に独自Mermaid、Draw.io、ZenUML、PlantUML、math rendererを持たない
- **THEN** KDRが未対応のbackendはrawをcode block枠でそのまま表示し、previewではtheme由来のerror border、エラーアイコン、代表メッセージ、tooltip詳細を添える

### Requirement: KDV AST lintで色ハードコードを禁止しなければならない

システムは、KDV rendering codeにhard-coded color literalが入らないよう、KDV側のAST lintを拡張しなければならない（MUST）。

#### Scenario: rendering codeに色literalがある

- **WHEN** `crates/katana-document-viewer-floem/src` またはneutral crateの描画関連moduleにhex color、RGB/RGBA literal、framework固有のnamed color直指定が含まれる
- **THEN** AST lintは違反として失敗する
- **THEN** 修正は `ViewerTheme` field参照へ置き換える

#### Scenario: preset定義で色を持つ

- **WHEN** KDVがdefault theme presetを定義する
- **THEN** preset定義moduleは色値を持てる
- **THEN** preset以外のrendering codeはpreset値へ直接依存せず、`ViewerConfig` で渡された `ViewerTheme` を参照する
- **THEN** test fixtureとAST lint自身の違反fixtureは色literalを持てる

#### Scenario: rendering codeがpresetへ直接依存する

- **WHEN** rendering codeがdefault theme preset moduleや英語（en）i18n preset moduleを直接参照する
- **THEN** AST lintは違反として失敗する
- **THEN** rendering codeは `ViewerConfig` で渡された `ViewerTheme` / `ViewerI18n` のみを参照する
