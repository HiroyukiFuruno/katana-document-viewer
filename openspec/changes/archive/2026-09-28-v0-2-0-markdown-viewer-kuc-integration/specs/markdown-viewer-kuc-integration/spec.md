## ADDED Requirements

### Requirement: KUC viewerでMarkdownを表示しなければならない

システムは、`v0.1.0` の `DocumentSnapshot` とartifact / diagnosticsを入力にして、KUC上でMarkdown viewerを表示しなければならない（MUST）。

#### Scenario: neutral viewer inputをKUC実装へ渡す

- **WHEN** ホストがKMM documentのsnapshotとartifact / diagnosticsをviewerへ渡す
- **THEN** `katana-document-viewer` はKUC型を含まないviewer inputを構築する
- **THEN** `katana-document-viewer-kuc` はviewer inputをKUC部品へ変換する
- **THEN** KDV neutral crateは `katana-ui-core`、`egui`、`winit`、`vello` に依存しない

#### Scenario: Markdown本文を表示する

- **WHEN** ホストがKMM documentのsnapshotをviewerへ渡す
- **THEN** KDVはKUC viewerにMarkdown本文を表示する
- **THEN** KDVはKMM DTOを独自に再parseしない
- **THEN** `egui_commonmark` vendor patchを正規経路として使わない

#### Scenario: KUC adapterはKDV node treeを主描画経路として表示する

- **WHEN** `katana-document-viewer-kuc` がMarkdown viewerをKUC render modelへ変換する
- **THEN** KUC adapterはKDVが評価した `ViewerNodePlan` をKUC `PaintRequest` / `UiTree` へ変換する
- **THEN** KUC adapterはeguiや単一RGBA画像を本文描画の基盤にしない
- **THEN** HTML block、table、code block、math、diagramはKUC node tree上のviewer nodeとして表示される
- **THEN** diagram / image / mathの描画済みartifactは、semantic viewer node tree内のnode単位mediaとしてのみ `ImageSurface` を使える
- **THEN** diagramなどの詳細描画が未完了または失敗した場合はログを出す
- **THEN** 失敗時は別rendererへ切り替えず、該当KMM nodeのraw文字列を表示文字列として返す

#### Scenario: vendor-free KUC Storybookで同一viewer node treeを実runtime表示する

- **WHEN** release DoDの統合確認を行う
- **THEN** vendor-free KUC Storybook正規入口はKatana由来fixtureを同じKDV viewer node treeで表示する
- **THEN** KDV Storybook経路はKDVのrender logicを再実装しない
- **THEN** Storybook gateはKatana由来fixture、direct source、表示互換score 95点以上を検証する
- **THEN** Storybook gateは単一または全文 `ImageSurface` routeへ戻っていないことを検証する
- **THEN** adapter plan一致だけを実runtime表示として扱ってはならない

#### Scenario: HTML / PDF export互換をscoreで検証する

- **WHEN** viewer node treeとexport artifactを比較する
- **THEN** KDVはHTML / PDFと同じKMM評価結果からviewer node treeを生成する
- **THEN** KDVはPDF / PNG surfaceとの互換scoreを数値化する
- **THEN** 互換scoreが95点未満の場合、release DoDは未達とする

#### Scenario: Markdown切替をinteractive budget内に収める

- **WHEN** ホストが同じviewer instanceへ別のMarkdown document revisionを渡す
- **THEN** KDVは初期表示でKRR diagram renderを同期実行しない
- **THEN** KDVは初期表示で全文書PNG exportとPNG decodeを行わない
- **THEN** KDVは全文書の論理viewer node treeを同期生成する
- **THEN** 図形の詳細描画はviewport優先のlazy asset経路へ残し、初期表示でKRR diagram renderを同期実行しない
- **THEN** `just storybook-check` はrelease buildのdiagram fixture warm file switchを600ms以内として検証する

#### Scenario: themeとfontをKUC契約から受け取る

- **WHEN** ホストがviewer configを構築する
- **THEN** KDVはtheme、font、i18n、interaction設定を必須入力として受け取る
- **THEN** rendering codeは色literal、OS固有font path、preset直接参照を持たない

### Requirement: viewerはSPAではなく単一instanceとして扱われなければならない

システムは、KDV viewerをIDEシェルやStorybook全体として扱わず、ホストが所有する単一のviewer instanceとして扱わなければならない（MUST）。

#### Scenario: ホストがviewer instanceを所有する

- **WHEN** ホストがMarkdown viewerを表示する
- **THEN** KDVは単一のviewer instanceへdocument snapshot、artifact set、viewport、theme、interaction設定を受け取る
- **THEN** KDVはfile picker、side menu、tab routing、workspace layout、window生成をviewer runtimeに含めない
- **THEN** Storybookはviewer instanceへ入力を渡す検証用shellとしてだけ機能する

#### Scenario: document revisionでviewer stateを更新する

- **WHEN** ホストが同じviewer instanceへ新しいdocument revisionを渡す
- **THEN** KDVは古いlayout result、anchor map、asset load結果を破棄する
- **THEN** KDVはdocument revisionとconfig revisionを区別する
- **THEN** configだけが変わった場合、KDVは再parseではなく再layoutまたは表示state更新を行う

### Requirement: viewerはviewport駆動でlayoutしなければならない

システムは、固定viewportではなく現在のviewport sizeからlayout、visible range、anchor map、slideshow page rangeを更新しなければならない（MUST）。

#### Scenario: viewport sizeが変わる

- **WHEN** ホストまたはadapterが新しいviewport sizeをviewerへ渡す
- **THEN** KDVはlayout result、content height、visible range、anchor mapを更新する
- **THEN** Slideshow modeではcurrent page indexとmax page indexを新しいviewport heightで再計算する
- **THEN** KDVは固定値のviewport heightを正本にしない

#### Scenario: 末尾headingへjumpする

- **WHEN** ユーザーが末尾付近のTOC itemへjumpする
- **THEN** KDVはlayout resultにbottom spacerを含める
- **THEN** KDVは対象heading anchorを要求align位置へscrollできる
- **THEN** bottom spacerはtheme上の装飾ではなくscroll可能領域として扱われる

### Requirement: external artifactは並列load可能なassetとして扱われなければならない

システムは、HTML、PDF、PNG、JPG、diagram、mathの描画結果をviewer inputのassetとして扱い、load stateとstale result破棄条件を表現しなければならない（MUST）。

#### Scenario: 複数assetをloadする

- **WHEN** Markdown本文に複数の画像、diagram、外部artifactが含まれる
- **THEN** KDVはdocument revision、KMM node id、artifact id、URIからload requestを識別する
- **THEN** adapterは複数load requestを並列実行できる
- **THEN** viewport内またはviewport近傍のassetを優先できる

#### Scenario: 古いasset load結果を破棄する

- **WHEN** document revisionが変わった後に古いasset load結果が完了する
- **THEN** KDVはその結果をviewer stateへ反映しない
- **THEN** KDVは現在のdocument revisionに対応するload結果だけを表示する

### Requirement: Markdown viewer はDocument modeとSlideshow modeを切り替えられなければならない

システムは、Markdown viewerの表示モードとして、通常文書表示の `ViewerMode::Document` と、1枚ずつ表示する `ViewerMode::Slideshow` を提供しなければならない（MUST）。

#### Scenario: Document modeで表示する

- **WHEN** ホストが `ViewerMode::Document` を指定する
- **THEN** KDVはKMM documentを連続した文書として表示する
- **THEN** TOC、hit-test、hover、selection、media controlsは通常文書表示の座標系で動作する

#### Scenario: Slideshow modeを開く

- **WHEN** ホストがactive Markdown文書を `ViewerMode::Slideshow` で開く
- **THEN** KDVはMarkdown本文を全画面相当の表示領域に描画する
- **THEN** KDVは通常previewと同じrendered contentを使う
- **THEN** KDVは見出しや手動markerから別のslide deckを作らない
- **THEN** KDVはMarkdown本文を再parseしてslide境界を推測しない

#### Scenario: Slideshow modeで仮想ページを計算する

- **WHEN** Slideshow modeでlayout後のcontent heightとviewport heightが得られる
- **THEN** KDVは `1 viewport height = 1 slideshow page` として仮想ページを計算する
- **THEN** KDVはcurrent page indexとmax page indexをviewer stateに保持する
- **THEN** KDVはpage offsetを `current_page_index * viewport_height` として扱う
- **THEN** diagramを含む本文もdiagram専用の手動分割を作らず、同じ仮想ページングに従う

#### Scenario: Slideshow modeで次ページへ移動する

- **WHEN** ユーザーが `ArrowRight`、`PageDown`、`Space`、または下部next controlを操作する
- **THEN** KDVはcurrent page indexを1増やすviewer stateを返す
- **THEN** current page indexがmax page index以上の場合、KDVは末尾ページを維持する

#### Scenario: Slideshow modeで前ページへ移動する

- **WHEN** ユーザーが `ArrowLeft`、`PageUp`、または下部previous controlを操作する
- **THEN** KDVはcurrent page indexを1減らすviewer stateを返す
- **THEN** current page indexが0の場合、KDVは先頭ページを維持する

#### Scenario: Slideshow modeを終了する

- **WHEN** ユーザーが `Esc` または右上close controlを操作する
- **THEN** KDVはslideshow close commandをホストへ返す
- **THEN** KDVはwindow fullscreen解除やOS viewport操作を直接実行しない

#### Scenario: Slideshow settingsを切り替える

- **WHEN** ユーザーがsettings tabからhover highlightまたはdiagram controlsを切り替える
- **THEN** KDVは `slideshow_hover_highlight` と `slideshow_show_diagram_controls` をviewer stateへ反映する
- **THEN** hover highlightが有効な場合、Slideshow modeでも対象nodeをtheme由来のhover表現で示す
- **THEN** diagram controlsが有効な場合、Slideshow modeでもdiagram操作入口を表示できる

#### Scenario: Slideshow modeは現在themeを継承する

- **WHEN** ホストが現在themeを指定してSlideshow modeを開く
- **THEN** KDVは背景色、文字色、強調色を通常previewと同じtheme契約から決定する
- **THEN** KDVはslideshow専用themeを作らない

### Requirement: hit-test metadataでKMM位置へ戻れなければならない

システムは、画面上のrendered nodeまたは座標から、KMM node idとsource rangeへ戻れるmetadataを提供しなければならない（MUST）。

#### Scenario: rendered nodeをhit-testする

- **WHEN** ホストがrendered nodeまたは画面座標に対してhit-testを行う
- **THEN** KDVは対応するKMM node idとsource rangeを返す
- **THEN** 対象が存在しない場合は失敗結果を返し、viewer状態を壊さない

### Requirement: KMM AST由来の目次を表示しなければならない

システムは、KMM AST解析結果から得た見出し構造を正本にして目次（TOC）を表示しなければならない（MUST）。

#### Scenario: 見出し構造から目次を構築する

- **WHEN** KMM documentにheading nodeが含まれる
- **THEN** KDVはheading level、表示text、KMM node id、source range、heading anchor候補から目次itemを構築する
- **THEN** KDVはMarkdown本文を再parseして目次の正本を作らない

#### Scenario: 目次clickでviewer commandを返す

- **WHEN** ユーザーが目次itemをclickする
- **THEN** KDVは対応するrendered heading anchorへviewerをscrollする
- **THEN** KDVはKMM node id、source range、heading anchorを含むviewer commandをホストへ返す
- **THEN** KatanAがeditor scrollを実行する

### Requirement: 検索hitをhighlightしjumpできなければならない

システムは、Katanaで実現済みの検索hit highlight、current hit表示、next / previous jumpを提供しなければならない（MUST）。

#### Scenario: 検索hitをhighlightする

- **WHEN** ホストが検索queryとmatch listをviewerへ渡す
- **THEN** KDVはmatchをKMM node id、source range、match range、match identityで保持する
- **THEN** KDVはlayout後のrendered rectへmatchを解決する
- **THEN** KDVはtheme由来の表現で検索hitをhighlightする
- **THEN** current matchは通常hitと区別できるtheme tokenで表示する

#### Scenario: 次の検索hitへjumpする

- **WHEN** ユーザーがnext search hitを操作する
- **THEN** KDVはcurrent match indexを次のmatchへ更新する
- **THEN** current matchが末尾の場合、KDVは先頭matchへwrapする
- **THEN** KDVは対象matchのrendered rectへscrollするviewer commandを返す
- **THEN** 末尾matchでもbottom spacerにより要求align位置へscrollできる

#### Scenario: 前の検索hitへjumpする

- **WHEN** ユーザーがprevious search hitを操作する
- **THEN** KDVはcurrent match indexを前のmatchへ更新する
- **THEN** current matchが先頭の場合、KDVは末尾matchへwrapする
- **THEN** KDVは対象matchのrendered rectへscrollするviewer commandを返す

#### Scenario: artifact textを検索対象に含める

- **WHEN** HTML、PDF、diagram、image artifactがtext extraction結果を提供する
- **THEN** KDVはartifact idを含むmatchとして検索hitを表示できる
- **THEN** KDVはviewer内でOCRやPDF解析を新規実装しない

### Requirement: hover / selection / media controls をinteraction設定で制御しなければならない

システムは、hover highlight、選択、画像・図形操作を `ViewerInteractionConfig` で制御しなければならない（MUST）。

#### Scenario: hover highlightを切り替える

- **WHEN** `hover_highlight_enabled` がtrueで、pointerがrendered node上にある
- **THEN** KDVは対象nodeをtheme由来のhover表現で示す
- **WHEN** `hover_highlight_enabled` がfalseである
- **THEN** KDVはhover highlightを描画しない

#### Scenario: 画像と図形の操作をhost commandにする

- **WHEN** `image_controls_enabled` または `diagram_controls_enabled` がtrueで、対象がhoverまたはfocusされる
- **THEN** KDVは拡大/fit、open、copy相当の操作入口を表示する
- **THEN** 操作はKDV内で副作用を起こさず、viewer commandとしてホストへ返す
- **THEN** viewer commandはKMM node id、source range、artifact id、操作種別を含む

### Requirement: 表示テキストは選択とclipboard copyができなければならない

システムは、viewerまたはStorybook shell上でUIとして表示したテキストを、KUC Canvas text run由来の選択対象として扱い、clipboard copy可能にしなければならない（MUST）。

#### Scenario: 可視テキストを選択してcopyする

- **WHEN** `selection_enabled` がtrueで、ユーザーがviewer本文、file tree、settings、header、statusに表示されたテキスト範囲をdrag selectionする
- **THEN** KDV Storybook hostはKUC Canvas text runから選択範囲のclipboard payloadを作る
- **THEN** `Cmd+C` または `Ctrl+C` で選択テキストをclipboardへ書き込む
- **THEN** KDVはglyph座標判定や文字列parseを独自に再実装しない

#### Scenario: Special CharactersとemojiをOS依存glyphとして扱う

- **WHEN** viewer本文にspecial charactersまたはemojiが含まれる
- **THEN** KDV/KUCはKatanA/egui由来の白黒化や共通fallbackへ寄せない
- **THEN** KUC Canvas text runはOS依存の文字・絵文字を可視テキストとして保持する
- **THEN** selection rectとclipboard payloadは可視text flowと一致する
- **THEN** KDV Storybook hostはglyph hit-test、文字列parse、座標補正、独自selection widgetを追加しない

#### Scenario: selection設定で選択copyを無効化する

- **WHEN** `selection_enabled` がfalseである
- **THEN** KDV Storybook hostは既存の選択状態を破棄する
- **THEN** KDV Storybook hostはdrag selectionを開始しない
- **THEN** KDV Storybook hostはclipboard copy payloadを返さない

#### Scenario: artifact内テキストを扱う

- **WHEN** SVG、diagram、image、PDFなどのartifactが表示上の文字を含む
- **THEN** artifactがtext extractionまたはsemantic overlayを提供する場合だけ選択copy対象へ含める
- **THEN** KDV viewer内でOCRやartifact再解析を新規実装しない
- **THEN** raster化された文字がcopy不能なままの場合、viewer parity完了として扱わない

### Requirement: unresolved metadataを本文から消してはならない

システムは、KMM DTOに存在するがKDVが専用表示できないmetadataを、本文から削除してはならない（MUST NOT）。

#### Scenario: unresolved metadataを補助表示する

- **WHEN** KMM documentにKDVが専用表示できないmetadataが含まれる
- **THEN** KDVは本文を残したまま補助表示を出す
- **THEN** 代表メッセージと詳細情報はi18nとdiagnosticsから取得する
- **THEN** 色はthemeから取得する
