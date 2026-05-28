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

#### Scenario: themeとfontをKUC契約から受け取る

- **WHEN** ホストがviewer configを構築する
- **THEN** KDVはtheme、font、i18n、interaction設定を必須入力として受け取る
- **THEN** rendering codeは色literal、OS固有font path、preset直接参照を持たない

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

### Requirement: unresolved metadataを本文から消してはならない

システムは、KMM DTOに存在するがKDVが専用表示できないmetadataを、本文から削除してはならない（MUST NOT）。

#### Scenario: unresolved metadataを補助表示する

- **WHEN** KMM documentにKDVが専用表示できないmetadataが含まれる
- **THEN** KDVは本文を残したまま補助表示を出す
- **THEN** 代表メッセージと詳細情報はi18nとdiagnosticsから取得する
- **THEN** 色はthemeから取得する
