## Context

`v0.1.0` はUI非依存のrender/export基盤であり、実画面のMarkdown viewerはこのchangeで扱う。

画面上では、Markdown本文、スライドショー表示、目次（TOC）、hover highlight、選択、画像・図形の操作UI、unresolved metadataの表示が必要になる。これらはKUCの状態・見た目・部品契約と結び付くため、KUC完成前のfoundationから分離する。

## Goals / Non-Goals

**Goals:**

- KUC上でKMM documentを表示する。
- neutral crateからKUC実装へ渡すviewer input、viewer event、viewer command、hit-test metadataを定義する。
- 通常文書表示とスライドショー表示をviewer modeとして切り替える。
- rendered nodeからKMM node idとsource rangeへ戻るhit-test metadataを提供する。
- 目次（TOC）をKMM AST由来の見出し構造から表示する。
- hover、選択、画像・図形操作をhost commandへ変換する。
- theme、font、i18n、interaction設定をKUC/KDV境界で明示する。
- Katanaが現時点で実現しているpreview / slideshow / 図形操作をv0.2.0の最低完了条件として踏襲する。
- viewerをIDEシェルやStorybook画面そのものとして扱わず、ホストが所有する単一のviewer instanceとして扱う。
- HTML / PDF / PNG / JPG / diagramの既存解析・描画結果をviewer入力として再利用し、画面側で再解析しない。
- viewport変化、scroll、asset読込、anchor更新に追従できるviewer runtimeを定義する。
- Katanaで実現済みの検索hit highlight、current hit表示、next / previous jumpを踏襲する。

**Non-Goals:**

- KMM parserをKDV内で再実装しない。
- editor-viewer同期制御をKDV内に持たない。
- KUC共通部品をKDVで再実装しない。
- export基盤をこのchangeで作り直さない。
- ファイル選択、tab管理、side menu、workspace layoutなどのIDEシェルをKDV viewer本体へ含めない。

## Decisions

### viewer instanceはSPAではなく独立した単一instanceにする

KDV viewerは、IDEやStorybookのようなアプリケーション全体を内包するSPAとして扱わない。ホストまたはadapterが単一のviewer instanceを所有し、そのinstanceへ現在のdocument snapshot、artifact set、viewport、theme、interaction設定を渡す。

viewer instanceは、表示対象のdocument revision、scroll offset、selection、hover、anchor map、layout result、asset load状態、slideshow stateを保持する。ファイル選択、side menu、tab routing、workspace layout、window生成はホスト責務であり、KDVのruntime責務ではない。

Storybookや検証画面は、このviewer instanceへ入力を渡す外側の検証用shellとしてのみ存在する。検証用shellのファイル一覧や設定panelを、KDV viewer contractの正本にしない。

documentや設定が変わるときは、viewer instanceをアプリごと作り直すのではなく、明示的なdocument revisionとconfig revisionで内部状態を更新する。document revisionが変わった場合、古いlayout result、anchor map、asset load結果は破棄する。configだけが変わった場合は、再parseではなく再layoutまたは表示state更新に留める。

### KUCを画面部品の正本にする

KDVはKUCのstyle/theme/font/state契約を受け取り、viewer画面へ適用する。KDV内のrendering codeは色literal、font path、preset直接参照を持たない。

KUC componentは、defaultでは副作用を持たなくてよい。ただしButton、FormField、Toggle、SettingsList rowなど操作可能な部品は、HTML / Reactのevent handlerに相当する明示的なaction受け口をcomponent builder APIとして持つ。利用側が `UiCommonProps` を直接組み立ててactionを注入する形を正規経路にしない。

KUC は Cargo dependency として利用し、KDVが sibling KUC source を private patch surface として扱ってはいけない。現行 KDV は `katana-ui-core` / `katana-ui-core-storybook` を KUC `v0.1.1` git tag に pin する。KUC `v0.1.0` は ContextMenu item typed host action を含まず、採用するとKDV側で downstream action復元や座標補正へ戻るため使わない。KUC側に不足する汎用部品契約はKUC issueへ切り出し、KDV側では action復元や座標補正で埋めない。ContextMenu item の typed action / secondary-click target identity は ADR `adrs/2026-06-23-kuc-installed-boundary-and-context-menu-actions.md` と KUC issue #7 を正本にし、KDVはKUC typed item actionを消費して task context menu target drift を閉じる。TreeView / FileTree / SettingsList / Toggle / Button / link-like span / media control の hover、cursor、row action、typed action injection は ADR `adrs/2026-06-23-kuc-interaction-target-contract.md` と KUC issue #8 `https://github.com/HiroyukiFuruno/katana-ui-core/issues/8` を正本にし、KDV Storybook はKUCが返す interaction target をhostするだけにする。

KatanA Markdown viewer 固有の `document_viewer` harness は ADR `adrs/2026-06-24-document-viewer-harness-ownership.md` を正本にし、KDV が所有する。KDV は harness を `tools/kdv-storybook/src/document_viewer.rs` と `tools/kdv-storybook/src/document_viewer/*` に保持し、`../katana-ui-core` から `#[path]` で直接取り込まない。Cargo package としての `katana-ui-core-storybook` は `katana-document-viewer` に依存しないため、`cargo test -p katana-ui-core-storybook --locked document_viewer -- --test-threads=1` は release proof にしない。KDV release では `cargo test -p kdv-storybook --locked document_viewer -- --test-threads=1` を viewer-specific harness regression として扱う。

### Special Characters / emoji は OS glyph 維持を正本にする

Special Characters と emoji は KatanA/egui の表示仕様を踏襲対象にしない。egui 由来の白黒化、共通 fallback、platform 差の吸収へ寄せず、KUC/KDV 独自の text rendering 契約として OS 依存の文字・絵文字をそのまま表示する。

KUC は Canvas text run、glyph metrics、selection rect、clipboard payload を同じ text flow から生成する。KDV Storybook は glyph hit-test、文字列 parse、座標補正、独自 selection widget を持たず、KUC の text run / selection API を host するだけにする。multi-line selection は document text flow として扱い、first line は anchor から行末、middle line は全幅、last line は行頭から focus までを選択対象にする。

neutral crateはKUC型をpublic APIへ漏らさない。`katana-document-viewer` はviewer input、viewer state snapshot、viewer command、hit-test responseを公開し、`katana-document-viewer-kuc` がそれをKUC部品へ変換する。

egui利用中のKatana向け接続口はKUCまたはhost側adapterの責務で提供する。`katana-document-viewer` neutral crateはegui、KUC、winit、velloへ依存しない。vendor adapterはvendor入力と表示をneutral viewer command / stateへ変換するだけで、viewer contractの正本にはならない。

vendor実装への依存はadapter crateの最小範囲に閉じ込める。`katana-document-viewer`、`katana-document-viewer`、`katana-document-viewer-kuc`、`katana-ui-core` はegui / gpui / floem / winit / vello、および `katana-ui-core-egui` などのvendor adapter crateを参照しない。各vendor adapterは、自分のvendorとneutral KUC / KDV contractだけを表示へ変換し、Markdown評価、HTML/PDF/diagram評価、viewer node生成を再実装しない。この境界は `kdv-linter` のvendor boundary AST lintで検査する。

### KMM metadataを表示の正本にする

KDVはKMM node id、source range、heading anchor候補を使ってviewer metadataを作る。TOCはMarkdown本文を再parseせず、KMM AST由来のheading listから作る。

### Slideshow modeはKatanA既存仕様を正とする

KDVは `ViewerMode::Document` と `ViewerMode::Slideshow` をneutral contractに持つ。Slideshow modeでは、KatanA既存仕様に合わせ、通常previewと同じrendered contentを全画面相当領域に表示し、viewport height単位で仮想ページングする。

KatanA既存仕様では、スライドショーはアクティブMarkdownを全画面で表示し、連続したMarkdown本文を `1 viewport height = 1 page` として扱う。KDVはこの挙動を移植時の正本にする。見出し単位や明示marker単位で新しいslide deckを作らない。

Slideshow modeのviewer stateは、viewport size、content height、current page index、max page index、hover highlight設定、diagram controls設定を持つ。次へ、前へ、終了、設定変更はviewer commandとしてホストへ返す。fullscreen、window制御、active document選択はKDVでは実行せず、ホスト側の副作用として扱う。

KatanA既存仕様から引き継ぐ操作は次の通り。

- 起動: ホストがactive Markdown文書を `ViewerMode::Slideshow` で開く。
- 表示: 通常previewを背面に隠し、全画面相当の表示領域へMarkdown本文を描画する。
- ページング: `ArrowRight` / `PageDown` / `Space` または下部次ボタンで次ページへ進む。
- 逆方向: `ArrowLeft` / `PageUp` または下部前ボタンで前ページへ戻る。
- 端の挙動: 先頭より前、末尾より後へは進まず、現在ページを維持する。
- 終了: `Esc` または右上close buttonで終了commandを返す。
- 設定: 右側settings tabでhover highlightとdiagram controlsを切り替える。
- コントロール表示: 操作後は表示し、一定時間idleならfadeする。
- Theme: 現在のthemeを継承し、slideshow専用themeを作らない。

### Katana既存の図形コントロールを全て踏襲する

v0.2.0はMVP相当の一部実装では完了にしない。Katanaで現在使える図形・画像viewer操作をparity対象にする。

通常表示では、図形またはローカル画像に対してfullscreen開始、pan上、pan下、pan左、pan右、zoom in、zoom out、reset、trackpad操作説明を提供する。ローカル画像のOS表示操作は副作用としてhost commandへ返す。

fullscreen表示では、close、drag pan、smooth scroll pan、trackpad zoom、pan上下左右、zoom in、zoom out、resetを提供する。closeはviewer state更新またはhost commandとして表現し、adapter内でwindow制御を直接正本化しない。

slideshow表示では、通常previewと同じrendered contentを使い、hover highlightとdiagram controlsの表示設定を引き継ぐ。図形コントロールが無効のときは図形操作UIとfullscreen入口を出さない。有効のときは通常表示と同じ図形操作を使える。

図形コントロールの各操作は、KMM node id、source range、artifact id、対象rect identityを含むviewer commandまたはviewer state transitionとして検証する。見た目だけでなく、自動テストで全操作が失われていないことを確認する。

### 副作用はhost commandへ逃がす

copy、open、download、editor scrollなど副作用を伴う操作はKDV内で実行しない。KDVは操作内容をviewer commandとしてホストへ返す。

### KMM評価結果をKUC node treeへ変換する

外部描画結果やdiagnosticsは `v0.1.0` のartifact / diagnosticsから受け取る。viewerは同じsourceを再処理せず、表示用のlayoutとinteractionに集中する。

viewerはHTML/PDF/PNG/JPG exportの合否を判定しない。export品質は `v0.1.x` の `ExportQualityGate` と実artifact確認を正とし、このchangeは画面操作、metadata、command境界を検証対象にする。

`katana-document-viewer-kuc` は、KMM ASTからKDVが評価した `ViewerNodePlan` をKUC render modelへ渡す。通常経路は `Panel`、`ScrollArea`、`Column`、`Text` などのKUC node treeであり、eguiや単一RGBA画像を本文描画の基盤にしない。HTML block、table、code block、math、diagramはKMM node単位のviewer nodeとして保持する。描画済みartifactを持つdiagram / image / mathは、semantic node tree内のnode単位mediaとしてのみKUC `ImageSurface` を使える。詳細描画が未完了または失敗した場合はログを出したうえでraw文字列をそのnodeの表示文字列として返す。

egui側の手描き本文rendererや直接egui surface表示は正規経路にしない。通常操作で見えるStorybookやKatana接続口は、KUC adapter経由で得たnode treeを表示する。エラー時に別rendererへ切り替えない。adapterはエラーをログに出し、KMM raw textを表示可能な文字列として返す。

file switch時の初期表示では、export用PNGのencode / decodeやKRR diagram renderを同期実行しない。KDVはKMM ASTから論理viewer node treeを構築し、diagramの詳細描画はasset pipelineのviewport優先lazy loadへ残す。`just storybook-check` はrelease buildのdiagram fixture warm file switchが600ms以内に収まることを単一test threadで検証する。

検索hit highlightやcurrent hit表示は、viewer state由来のrect / node metadataとしてKUC adapterへ渡す。本文の再描画やHTML/PDF解析をegui adapter側でやり直さない。

現在のローカルKUCには、vendor adapterごとのrender plan検証経路がある。ただし、render plan検証は実viewer runtimeで開いた証跡ではない。egui製Storybookの中で `katana`、`gpui`、`floem` を選ばせるだけのUIは「4 vendorで開いた」扱いにしない。

Storybookはvendor-free KUC render modelを既定入口にする。既定入口の `just storybook` はKDV/KUCのviewer shellを開き、egui内route selectorだけを複数vendor検証として扱わない。adapter別runtime検証はKUC側または各adapter owner側の明示入口で扱い、KDV本体の正規Storybook入口へvendor runtimeを混ぜない。

release DoDでは、このKDV viewer node treeがvendor-free KUC Storybook、HTML / PDF / PNG / JPEG export、direct source smoke、interaction testで互換score 95点以上を満たすことを確認する。adapter plan一致だけでは完了にしない。

### HTML / PDF / 画像 / diagramはasset pipelineとして扱う

viewerは、HTML、PDF、PNG、JPG、diagram、mathなどの描画済みartifactをblock単位のassetとして扱う。KDV内で既存の解析・描画処理を再実装せず、artifact id、source range、diagnostics、rendered media情報をviewer inputとして受け取る。

画像や外部artifactは、document revision、KMM node id、artifact id、URIをkeyにしたload requestとして扱う。load requestは並列に実行でき、viewport内またはviewport近傍のblockを優先する。document revisionが進んだ場合、完了済みでも古いload結果はviewer stateへ反映しない。

adapterは実際のthread、task、GPU texture生成などを担当してよいが、KDV contractはload request、load state、cancel対象、stale result破棄条件を表現する。

### viewport駆動のlayoutとanchor mapを正本にする

viewerは固定の高さを前提にしない。ホストまたはadapterから渡される現在のviewport sizeを使い、layout result、content height、visible range、anchor map、slideshow page rangeを更新する。

TOC jumpは、KMM headingから作ったanchor identityをlayout後のrendered anchor mapへ解決し、adapterへscroll commandとして渡す。本文再parseや文字列検索でjump先を決めない。

末尾付近のheadingへjumpできるよう、document modeのscrollable contentは本文の実高さだけで終わらせない。VS Code型の末尾余白として、少なくとも最後のanchorをviewport上部または指定align位置へ移動できるbottom spacerをlayout resultに含める。

bottom spacerは見た目の余白ではなく、TOC jumpとanchor alignmentを成立させるためのviewer contractである。自動テストでは、最後のheadingへTOC jumpしたときにanchorが要求位置へalignできることを検証する。

### 検索hit highlightとjumpをviewer stateに含める

KDVは、Katanaで実現済みの検索hit highlightとjumpをv0.2.0のparity対象に含める。検索UIそのもの、検索queryの入力欄、IDE全体の検索panelはホスト責務とし、viewerは検索結果の表示と移動を担当する。

viewer inputは、検索query、match list、current match index、match identityを受け取れる。matchはKMM node id、source range、match range、match text、必要に応じてartifact idを持つ。KDV側で検索を実行する場合も、結果は同じmatch listへ正規化する。

layout後、viewerは検索matchをrendered rectへ解決し、theme由来のhighlightで表示する。current matchは通常hitと別のtheme tokenで示す。next / previous操作はcurrent match indexを更新し、対象matchのrendered rectへscrollするviewer commandを返す。Katana既存挙動に合わせ、nextは末尾から先頭へ、previousは先頭から末尾へwrapする。

検索jumpはTOC jumpと同じanchor/scroll基盤を使う。末尾のmatchへjumpできるよう、bottom spacerはheadingだけでなくsearch matchのalignにも効く必要がある。

検索対象はKMM由来の本文textを基本とする。HTML / PDF / diagram / image内textは、既存artifactがtext extraction結果を提供する場合だけmatch対象に含める。KDV viewer内でOCRやPDF解析を新規実装しない。

### 実装単位をruntime / layout / asset / search / adapterへ分ける

`katana-document-viewer` はUI非依存のviewer runtimeを持つ。runtimeは次の責務へ分割する。

- `ViewerSession`: ホスト所有の単一viewer instanceを表す。document revision、config revision、scroll state、selection、hover、slideshow stateを保持する。
- `ViewerLayoutEngine`: KMM nodeとartifact metadataからlayout block、rendered rect、anchor map、visible range、bottom spacerを作る。
- `ViewerAssetPipeline`: artifactごとのload request、load state、priority、stale result破棄条件を管理する。
- `ViewerSearchEngine`: 検索match list、current match、highlight rect、next / previous jump commandを管理する。
- `ViewerCommandFactory`: TOC jump、search jump、media operation、slideshow operationを副作用なしのviewer commandへ変換する。

`katana-document-viewer-kuc` は、KUCの `Panel`、`SearchControlStrip`、`VirtualizationConfig`、theme token、event modelへ変換するadapterである。KUC側にある検索UIや仮想化primitivesは使うが、Markdown viewerのdocument runtime、asset pipeline、bottom spacer、search match解決をKUCへ押し込まない。

vendor adapterはKUCまたはhost側の責務として扱う。adapterはviewport計測、pointer / keyboard入力、描画、scroll要求の実行だけを担当し、viewer contractや検索・asset・layoutの正本にならない。

検証用Storybookは `tools/` 配下に置く。Storybookはfile picker、side menu、settings panel、main paneを持てるが、それらはviewer instanceを操作する検証shellであり、viewer runtimeそのものではない。

## Risks / Trade-offs

- KUCのScrollArea、SplitPane、SearchControlStripなどの基礎部品が不足すると、このchangeの完了条件は満たせない。
- KUC境界が未確定のまま進めるとKDV内に暫定UI型が漏れるため、最初にviewer input / commandのneutral契約を固定する。
- slide deckを新規生成するとKatanA既存仕様とずれるため、viewport height単位の仮想ページングを正本にする。
- viewerをStorybookやSPAとして実装すると、Katanaの単一viewer instance利用とずれるため、検証shellとviewer runtimeを分離する。
- asset loadの並列化やstale result破棄を後回しにすると、リアルタイムpreviewで古い画像やdiagramが混入する。
- bottom spacerをlayout contractに含めないと、末尾headingへのTOC jumpが実質機能しない。
- 検索hit highlightを単なる描画だけにすると、current hitや末尾hitへのjumpがKatanaとずれる。
- 見た目の確認だけでは完了にしない。hit-test、TOC click、hover設定、command発火は自動テストで確認する。
- 画面操作とexportを同時に変えると原因が追いづらくなるため、このchangeはviewer操作に限定する。
