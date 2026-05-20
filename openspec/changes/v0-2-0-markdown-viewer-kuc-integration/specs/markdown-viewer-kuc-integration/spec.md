## ADDED Requirements

### Requirement: KUC / Floem viewerでMarkdownを表示しなければならない

システムは、`v0.1.0` の `DocumentSnapshot` とartifact / diagnosticsを入力にして、KUC / Floem上でMarkdown viewerを表示しなければならない（MUST）。

#### Scenario: Markdown本文を表示する

- **WHEN** ホストがKMM documentのsnapshotをviewerへ渡す
- **THEN** KDVはKUC / Floem viewerにMarkdown本文を表示する
- **THEN** KDVはKMM DTOを独自に再parseしない
- **THEN** `egui_commonmark` vendor patchを正規経路として使わない

#### Scenario: themeとfontをKUC契約から受け取る

- **WHEN** ホストがviewer configを構築する
- **THEN** KDVはtheme、font、i18n、interaction設定を必須入力として受け取る
- **THEN** rendering codeは色literal、OS固有font path、preset直接参照を持たない

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

### Requirement: unresolved metadataを本文から消してはならない

システムは、KMM DTOに存在するがKDVが専用表示できないmetadataを、本文から削除してはならない（MUST NOT）。

#### Scenario: unresolved metadataを補助表示する

- **WHEN** KMM documentにKDVが専用表示できないmetadataが含まれる
- **THEN** KDVは本文を残したまま補助表示を出す
- **THEN** 代表メッセージと詳細情報はi18nとdiagnosticsから取得する
- **THEN** 色はthemeから取得する
