# v0.2.0 handoff

最終更新: 2026-06-03

## 直近の引き継ぎ要点

- Katana task checkbox stateはKDV node上の `interaction.value` とstyle classで `[ ]`、`[x]`、`[/]`、`[-]` を保持し、ローカルKUC `UiTreeCanvasRenderer` も generic `x` 固定ではなくKatana task marker別に実canvas描画する。KDV Storybook frame側も `katana/sample_basic.md` で4状態と実preview pixel到達を検証する。
- Storybook左下SettingsListはKUC `SettingsList` のまま、`Theme` と `Mode` をKUC Selectとして操作対象にした。`Mode` はクリック/設定actionで `ViewerMode::Document` / `ViewerMode::Slideshow` を切り替え、Storybook scene再構築にも届く。KDV側は独自select UIを作らず、KUC `SettingsListAction::UpdateField` の `FieldChanged` を確認してから状態へ反映する。
- `just storybook` はinteractive launcherのまま維持されている。`STORYBOOK_FRAMES=1 /opt/homebrew/bin/rtk just storybook` で `cargo run --release --locked -p kdv-storybook -- --interactive --frames 1` を実行し、正常終了する。
- Storybook通常表示はlazy初期表示へ変更済み。`PreviewRenderEngine::render_viewer_output` でKUC treeを即時作り、visible/near assetは別threadの `StorybookAssetJob` でロード済みsceneへ差し替える。同期ロード済み検証はheadless smoke / matrix testsで明示的に使う。
- Storybook左ペインはKUC `FileTree` / `SettingsList` 経由で、catalog smokeは `assets/fixtures` 配下のsupported fixture全件を対象にする。catalog順序はKatana fixtureを先頭にし、画面表示のdirect fixtureは冗長拡張子を落とした代表fixtureへ絞る。非表示代表を選択中にした場合もTreeViewのactive idは落とさない。SettingsListのStateにsurface寸法または `lazy` を表示する。
- Storybook左ペインは本文viewerとは別のscroll stateを持つ。ホイール入力はpointer位置で左上FileTree、左下SettingsList、本文viewerへ振り分ける。FileTreeのoffsetはローカルKUC `FileTree::render_with_offset` 経由で渡し、SettingsListのscroll範囲はKUC `UiNode` 構造から算出する。KDV側でTreeView行構造や設定UI部品を再実装しない。
- Storybook frame role pixel gateはpreview本文領域だけを集計する。sidebar/header/statusのpixelでrole描画が誤通過する状態は禁止する。text系HTML roleはanti-aliasingでexact text colorが不安定なため、HTML alignment専用テストで位置を検証し、role pixel gateでは本文領域内の非背景pixel到達を確認する。
- 図形コントロールのKDV/KUC node contractは、右上相当の `fullscreen` / `copy-source` と、右下相当のpan/zoom/reset gridへ分離済み。KDV-KUC adapter はKDV専用MediaControlを `Stack` の `UiPosition::Absolute` child と8px marginで表現し、KUC `UiTreeCanvasRenderer` / hit collector はKDV diagram style classではなくgeneric absolute overlayとして描画・hit-testする。`copy-source` は `ViewerCommand::Host(HostCommand::CopyText(target.source.raw.text))` へ正規化する。通常コードブロックの `copy-code` もKUC `kdv-code-control` nodeから `ViewerCommandFactory::code_control_from_action` 経由で `HostCommand::CopyText` へ正規化する。
- artifact検索はKDV coreの `ViewerArtifactSearchResolver` がHTML/SVG artifact bytesから検索可能テキストを抽出し、同じartifact_idを持つ `ViewerNode` のrectへ解決する。Storybookはこのcore resolver結果をKUC plan由来の検索targetへ併合するだけにする。
- GFM取り消し線はKMM DTOとしてではなくKDV viewer/export contractとして扱う。coverage matrixは `gfm-strikethrough` を `KdvExportContract` にし、Katana basic fixtureのstrikethrough spanが `ViewerNodePlan` とscore matrixへ届くことを検証する。
- 直接画像のStorybook scene gateはPNG単体ではなく `bmp/gif/jpeg/jpg/png/svg/webp` 全direct image fixtureを対象にし、各fixtureがKUC `ImageSurface`、asset load、image surface countへ届くことを検証する。
- 直接図形のStorybook scene gateを追加済み。`drawio/drowio`、`mermaid/mmd`、`plantuml/puml` は、KUC `ImageSurface`、種別別accessibility label、図形control、asset失敗0へ届くことを検証する。KDV側も同じdirect diagram群でviewer diagram kindとHTML/PDF/PNG/JPEG export score 95点以上を結び付ける。
- Katana full sample score gateを追加済み。`katana/sample.md`、`katana/sample.ja.md`、`katana/sample_html.md`、`katana/sample_html.ja.md` は、KDV runtimeでrendered diagram graphを作ったうえでHTML/PDF/PNG/JPEG export score 95点以上を要求する。
- KDV KUC adapterはroot `ScrollArea`。KDV側で表示用Panelを足さない。KUC `UiTreeCanvasRenderer` はColumn/Stack/Alignなどのlayout containerで不要indentを入れない。
- `ExportQualityGate` は `SurfaceEquivalenceArtifacts` を任意入力として受け、指定時はPNG/JPEG/PDF surface equivalence 95点未満を `Surface:` fatal failureにする。`export_artifacts_e2e` は `KdvPreviewSurfaceFactory` のpreview surfaceをreferenceとして渡す。
- HTML中央/右寄せは固定文字幅推定へ寄せず、export surface側もKUC Storybook側も実測text span幅で配置する。これにより `direct/html-alignment.html` のsurface parity 95点以上と、Storybook viewport resize時の中央/右寄せ追従を両立する。
- Storybook surface parityは、HTML/image/HTML sampleのfast gateと、direct/Katana図形を含むdiagram gateへ分離する。fast gateにdiagram-heavy fixtureを混ぜないことを `fast_surface_parity_fixtures_do_not_include_diagram_sources` で固定し、diagram gateも通常testとして実行する。
- Storybook surface parityは、reference/candidateがどちらも空の場合に100点へ誤通過しない。`SurfaceParityScorer` は双方blankのboundsを0点にし、空画面同士で「一致」と扱う偽陽性を禁止する。
- Export surfaceは `Option<SurfaceTextPainter>` を使わない。本文/コード/表/アラート/バッジ/リスト番号/図形pending labelは実 `SurfaceTextPainter` を必須にし、簡易矩形文字の `draw_fallback_text` は削除した。フォント描画欠落を「表示あり」としてscoreやsurface parityが通る状態は禁止する。
- Storybook KUC scene requirements matrixは、role存在だけでなくKUC `UiTextSpan` 上のsyntax-highlighted code spanとstrikethrough spanを検証する。KDV planやexport scoreで通っても、KUC表示ノードでspan styleが落ちる場合はfailする。
- ローカルKUC Storybook rendererは `UiTextSpanStyle::strikethrough` を実canvas上の取り消し線として描画する。KUC sceneにspanが残るだけで画面に線が出ない状態を禁止する。
- KDV Storybook frame role pixel gateは、Katana basic fixtureのsyntax-highlighted KUC span色が実frame pixelへ出ることも検証する。KUC sceneに色spanが残ってもcanvasが通常文字色へ潰れる状態は禁止する。
- Link hover cursorは、KUC `UiCursor::Pointer` を通常text linkとlist内linkへ付与し、Storybook windowがhover位置からKUC host action targetのcursorを解決してminifb `OpenHand` へ写像する。
- Hover highlightは、KUC adapter configの `hovered_node_id` から対象viewer nodeをKUC `UiVisualRole::HoverSurface` wrapperへ変換する。Storybook windowはpointer位置を `ViewerTarget` へ解決し、frame rendererはKUC hover surfaceを実frame pixelへ描く。KDV独自の矩形塗りつぶしだけで済ませる経路は禁止する。
- 最新確認:
  - `rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer -- --test-threads=1 --nocapture`: `2 passed`、`real 108.38`。fast + diagram surface parityはいずれも95点以上。
  - `rtk just storybook-check`: 通過。`kdv-storybook` fullは `97 passed`。entrypoint、KUC adapter boundary、content gate、interaction gate、window smoke、KUC smoke、performance gateを通しで確認した。
  - `KDV_STORYBOOK_SURFACE_PARITY_FIXTURE=katana/sample_html.md /usr/bin/time -p rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer -- --test-threads=1 --nocapture`: `1 passed`、`real 6.69`。
  - `rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer_diagrams -- --ignored --test-threads=1 --nocapture`: `1 passed`、`real 87.54`。この確認後にignoredを外し通常testへ昇格した。
  - `rtk cargo test -p kdv-storybook --locked html_alignment_uses_preview_area_positions -- --test-threads=1 --nocapture`: `1 passed`。HTML中央/右寄せはviewport幅変更へ追従する。
  - `rtk cargo test -p katana-document-viewer --locked line_text_x_switches_for_code_and_center_lines -- --test-threads=1 --nocapture`: `1 passed`。export surfaceの中央/右寄せはsystem fontで実測したspan幅を使う。
  - `rtk cargo test -p katana-ui-core-storybook --locked html_ -- --test-threads=1 --nocapture`: `2 passed`。KUC canvas rendererのHTML中央/右寄せは実測text幅を使う。
  - `rtk cargo fmt -p katana-document-viewer -p kdv-storybook -- --check`: 通過。
  - `rtk cargo fmt -p katana-ui-core-storybook -- --check`: 通過。
  - `rtk cargo clippy -p kdv-storybook --all-targets --locked -- -D warnings`: `No issues found`
  - `rtk cargo clippy -p katana-document-viewer -p kdv-storybook --all-targets --locked -- -D warnings`: `No issues found`
  - `rtk cargo test -p katana-document-viewer --locked export_surface_painter -- --test-threads=1`: `33 passed`。Export surfaceの本文/コード/表/アラート/バッジ/リスト番号/図形pending labelが実 `SurfaceTextPainter` 経由で描画される。
  - `rtk cargo test -p katana-document-viewer --locked export_surface_font -- --test-threads=1`: `20 passed`。
  - `rtk cargo clippy -p katana-document-viewer --all-targets --locked -- -D warnings`: `No issues found`
  - `rtk cargo test -p katana-document-viewer --locked export_quality -- --test-threads=1`: `215 passed`
  - `rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer -- --test-threads=1 --nocapture`: `2 passed`
  - `rtk cargo clippy -p katana-ui-core-storybook --all-targets --locked -- -D warnings`: `No issues found`
  - `rtk just storybook-entrypoint-check`: `storybook-entrypoint-check: ok`
  - `rtk just storybook-window-smoke`: `storybook-window-smoke: ok fixtures=42 checked=42`
  - `rtk just storybook-performance-check`: preview engine performance `1 passed`、Storybook lazy frame performance `1 passed`、frame側は `0.33s`
  - `rtk just storybook-content-check`: 通過。内訳は `preview_feature_matrix` 1 passed、`direct_diagram` 1 passed、`preview_requirement_matrix` 3 passed、`fixture_feature_matrix` 4 passed、`fixture_mermaid` 1 passed、`direct_diagrams_bind` 1 passed、`fixture_score_matrix` 4 passed、direct source 79 passed、export quality 215 passed、surface equivalence 21 passed。
  - `rtk just ast-lint`: `1 passed`。Export surfaceのfallback削除後も行数/構造lintを通過している。
  - `rtk cargo test -p kdv-storybook --locked preview_requirement_matrix -- --test-threads=1 --nocapture`: `3 passed`。KUC scene上のsyntax-highlighted code spanとstrikethrough span保持を含む。
  - `rtk cargo fmt -p kdv-storybook -- --check`: 通過。
  - `rtk cargo clippy -p kdv-storybook --all-targets --locked -- -D warnings`: `No issues found`
  - KUC側 `rtk cargo test -p katana-ui-core-storybook --locked strikethrough_span_draws_line_even_for_whitespace -- --test-threads=1 --nocapture`: `1 passed`。空白spanの取り消し線pixelを検証し、glyph描画だけで通る偽陽性を避ける。
  - KUC側 `rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas_text -- --test-threads=1`: `10 passed`
  - KUC側 `rtk cargo fmt -p katana-ui-core-storybook -- --check`: 通過。
  - KUC側 `rtk cargo clippy -p katana-ui-core-storybook --all-targets --locked -- -D warnings`: `No issues found`
  - `KDV_STORYBOOK_SURFACE_PARITY_FIXTURE=katana/sample_basic.md rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer -- --test-threads=1 --nocapture`: `2 passed`
  - `rtk cargo test -p kdv-storybook --locked required_kuc_roles_reach_fixture_frame_pixels -- --test-threads=1 --nocapture`: `1 passed`
  - `rtk cargo test -p kdv-storybook --locked frame -- --test-threads=1`: `25 passed`。syntax-highlighted frame pixel gateとhover highlight frame pixel gateを含む。
  - `rtk cargo test -p katana-document-viewer-kuc --locked markdown_link_node_carries_pointer_cursor -- --test-threads=1 --nocapture`: `1 passed`
  - `rtk cargo test -p katana-document-viewer-kuc --locked list_node_preserves_link_spans_as_kuc_host_targets -- --test-threads=1 --nocapture`: `1 passed`
  - `rtk cargo test -p kdv-storybook --locked link_hover_uses_pointer_cursor_from_kuc_node -- --test-threads=1 --nocapture`: `1 passed`
  - `rtk cargo test -p kdv-storybook --locked mouse -- --test-threads=1`: `10 passed`
  - `rtk cargo test -p kdv-storybook --locked window -- --test-threads=1`: `8 passed`
  - `rtk cargo test -p katana-document-viewer-kuc --locked hovered_node_uses_kuc_hover_surface_wrapper -- --test-threads=1 --nocapture`: `1 passed`
  - `rtk cargo test -p kdv-storybook --locked hover_highlight_reaches_storybook_frame_pixels -- --test-threads=1 --nocapture`: `1 passed`
  - `rtk cargo test -p kdv-storybook --locked frame -- --test-threads=1`: `25 passed`
  - `rtk cargo test -p katana-document-viewer-kuc --locked -- --test-threads=1`: `76 passed`
  - `rtk cargo clippy -p kdv-storybook -p katana-document-viewer-kuc --all-targets --locked -- -D warnings`: `No issues found`
  - `rtk just storybook-interaction-check`: 通過。mouse `10 passed`、frame `25 passed` を含む。
  - `rtk just storybook-check`: 通過。entrypoint、KUC adapter boundary、`kdv-storybook` full `97 passed`、content gate、interaction gate、window smoke、KUC smoke、performance gateを通しで確認した。
  - `rtk just storybook-content-check`: 通過。内訳は `preview_feature_matrix` 1 passed、`direct_diagram` 1 passed、`preview_requirement_matrix` 3 passed、`fixture_feature_matrix` 4 passed、`fixture_mermaid` 1 passed、`direct_diagrams_bind` 1 passed、`fixture_score_matrix` 4 passed、direct source 79 passed、export quality 215 passed、surface equivalence 21 passed。
  - `rtk just storybook-interaction-check`: 通過。内訳は commands `14 passed`、interaction matrix `6 passed`、interaction metadata `2 passed`、mouse `10 passed`、frame `25 passed`、search `10 passed`、slideshow `11 passed`、window `8 passed`。
  - `rtk cargo test -p kdv-storybook --locked sidebar -- --test-threads=1`: `11 passed`
  - `rtk cargo test -p kdv-storybook --locked window -- --test-threads=1`: `8 passed`
  - `rtk cargo test -p katana-ui-core --locked file_tree -- --test-threads=1`: `3 passed`
  - `rtk cargo clippy -p kdv-storybook --all-targets --locked -- -D warnings`: `No issues found`
  - `rtk cargo clippy -p katana-ui-core --all-targets --locked -- -D warnings`: `No issues found`
  - `rtk cargo test -p kdv-linter --locked`: `73 passed`
  - `STORYBOOK_FRAMES=1 rtk just storybook`: `cargo run --release --locked -p kdv-storybook -- --interactive --frames 1` を実行し正常終了。
  - `rtk cargo test -p kdv-storybook --locked preview_katana -- --test-threads=1`: `2 passed`。Katana markdownのKUC scene到達と、Katana drawio/basic 12 fixture全件のImageSurface/diagram control到達を検証。
  - `rtk cargo test -p katana-document-viewer --locked fixture_mermaid -- --test-threads=1`: `1 passed`。巨大Mermaid fixtureはStorybook同期ロードではなくKDV runtime planで50件以上のMermaid diagram node到達を検証。
  - `rtk cargo test -p kdv-storybook --locked catalog -- --test-threads=1`: `2 passed`。`direct/kdv-icon.bmp`、`direct/sample.mmd`、`katana/drawio/basic/03-basic-flow.drawio`、`katana/drawio/basic/11-japanese-labels.drawio` がcatalogから落ちないことを検証。
  - `rtk cargo test -p katana-document-viewer --locked evaluation -- --test-threads=1`: `10 passed`。`gfm-strikethrough` は `KdvExportContract` として分類する。
  - `rtk cargo test -p katana-document-viewer --locked fixture_feature_matrix -- --test-threads=1`: `4 passed`。Katana basic fixtureのstrikethrough spanがviewer planへ届くことを検証。
  - `rtk cargo test -p katana-document-viewer --locked fixture_score_matrix -- --test-threads=1`: `4 passed`。strikethrough要件もHTML/PDF/PNG/JPEG score gateと同じfixture上で検証。
  - `rtk cargo test -p kdv-storybook --locked preview_feature_matrix -- --test-threads=1`: `1 passed`。直接画像 `bmp/gif/jpeg/jpg/png/svg/webp` のKUC scene到達を検証。
  - `rtk cargo test -p kdv-storybook --locked direct_diagram -- --test-threads=1`: `1 passed`。直接図形 `drawio/drowio`、`mermaid/mmd`、`plantuml/puml` のKUC scene到達を検証。
  - `rtk cargo test -p katana-document-viewer --locked direct_diagrams_bind -- --test-threads=1`: `1 passed`。直接図形のviewer diagram kindとHTML/PDF/PNG/JPEG export score 95点以上を検証。
  - `rtk just storybook-content-check`: 通過。内訳は `preview_feature_matrix` 1 passed、`direct_diagram` 1 passed、`preview_requirement_matrix` 3 passed、`fixture_feature_matrix` 4 passed、`fixture_mermaid` 1 passed、`direct_diagrams_bind` 1 passed、`fixture_score_matrix` 4 passed、direct source 79 passed、export quality 215 passed、surface equivalence 21 passed。
  - `rtk just clean`: 正常終了。`cargo clean` で build artifacts を削除できる。
  - `rtk cargo test -p katana-document-viewer --locked direct_html_source -- --test-threads=1`: `10 passed`
  - `rtk cargo test -p katana-document-viewer --locked surface_equivalence -- --test-threads=1`: `21 passed`
  - `rtk cargo test -p katana-document-viewer --locked --test export_artifacts_e2e e2e_export_scores_evaluated_html_pdf_png_and_jpeg_without_sidecars -- --exact --test-threads=1`: `1 passed`
  - `rtk cargo test -p kdv-storybook --locked preview_ -- --test-threads=1`: `26 passed`
  - `rtk just storybook-window-smoke`: `storybook-window-smoke: ok fixtures=42 checked=42`
  - `rtk just storybook-performance-check`: preview engine performance `1 passed`、Storybook lazy frame performance `1 passed`、frame側は `0.30s`
  - `rtk cargo clippy -p katana-document-viewer -p katana-document-viewer-kuc -p kdv-storybook --all-targets --locked -- -D warnings`: `No issues found`
  - `rtk cargo clippy -p katana-ui-core-storybook --all-targets --locked -- -D warnings`: `No issues found`
  - `rtk cargo test -p katana-ui-core-storybook --locked checkbox_ -- --test-threads=1 --nocapture`: `18 passed`。KUC canvas rendererでKatana task marker `[ ]`、`[x]`、`[/]`、`[-]` を実state別に扱う。
  - `rtk cargo test -p kdv-storybook --locked katana_task_checkbox_states_reach_scene_and_frame -- --test-threads=1 --nocapture`: `1 passed`。KDV KUC sceneとStorybook frameへtask 4状態が到達する。
  - `rtk cargo test -p kdv-storybook --locked mouse_left_click_on_code_copy_returns_host_copy_command -- --test-threads=1 --nocapture`: `1 passed`。通常コードブロックのcopy control clickが `HostCommand::CopyText` へ届く。
  - `rtk cargo test -p kdv-storybook --locked settings_action -- --test-threads=1 --nocapture`: `2 passed`。KUC SettingsList action結果を通したDark/Mode反映を検証する。
  - `rtk cargo test -p kdv-storybook --locked sidebar -- --test-threads=1 --nocapture`: `14 passed`。KUC FileTree/SettingsList、Theme/Mode Select hit、別scroll state、Mode設定のSlideshow scene再構築を含む。
  - `rtk cargo test -p kdv-storybook --locked frame -- --test-threads=1 --nocapture`: `25 passed`。task state frame到達、search/slideshow/media/hover、blank surface parity reject等のframe gateを含む。
  - `rtk cargo test -p kdv-storybook --locked required_kuc_roles_reach_fixture_frame_pixels -- --test-threads=1 --nocapture`: `1 passed`。role pixel countはpreview本文領域だけを対象にし、sidebar/header/status pixelでは通らない。
  - `rtk just storybook-content-check`: 通過。内訳は `preview_feature_matrix` 1 passed、`direct_diagram` 1 passed、`preview_requirement_matrix` 3 passed、`fixture_feature_matrix` 4 passed、`fixture_mermaid` 1 passed、`direct_diagrams_bind` 1 passed、`fixture_score_matrix` 4 passed、direct source 79 passed、export quality 215 passed、surface equivalence 21 passed。
  - `rtk just storybook-interaction-check`: 通過。内訳は commands `14 passed`、interaction matrix `6 passed`、interaction metadata `2 passed`、mouse `10 passed`、frame `25 passed`、search `10 passed`、slideshow `11 passed`、window `8 passed`。
  - `rtk cargo test -p kdv-storybook --locked frame_surface_similarity -- --test-threads=1 --nocapture`: `4 passed`。Storybook surface parityはreference/candidate双方blankを0点へ落とす。
  - `rtk just kuc-adapter-boundary-check`: `kuc-adapter-boundary-check: ok`。KDV/KUC neutral implementationとvendor-free Storybookにvendor runtime依存、独自Tree UI、独自toggle反転が戻っていないことを検査する。
  - `rtk just storybook-window-smoke`: `storybook-window-smoke: ok fixtures=42 checked=42`
  - `STORYBOOK_FRAMES=1 rtk just storybook`: `cargo run --release --locked -p kdv-storybook -- --interactive --frames 1` を実行し正常終了。

## 現在の方針

- KDV core の解析、render、viewer node 生成は UI vendor 非依存に保つ。
- KDV の UI 表示は KUC を入口にする。KDV 側で無駄な独自 UI 部品を増やさない。
- KDV domain 固有の source normalizer、viewer runtime、export quality、direct visual handling は KDV 側で持つ。
- Storybook は検証 shell であり、`crates/` には置かない。
- 正しさは自動テスト、score gate、runtime smoke を根拠にする。画面確認は補助とする。
- vendor-free Storybook の実描画はローカルKUCの `UiTreeCanvasRenderer` を使う。KDV側の旧独自canvas renderer群は `tmp/archive/kdv-storybook-old-renderer-2026-06-02/` へ退避済みで、正規経路にしない。

## 現在の入口

- `just storybook`
  - vendor-free KUC preview window を起動する。
  - 実体は `cargo run --release --locked -p kdv-storybook -- --interactive --frames {{STORYBOOK_FRAMES}}`。
  - `scripts/check-storybook-entrypoint.sh` が test-only / smoke-only 入口への退化を検出する。
- `just storybook-check`
  - `storybook-entrypoint-check`
  - `storybook-tool-test`
  - `storybook-content-check`
  - `storybook-interaction-check`
  - `storybook-window-smoke`
  - `storybook-kuc-smoke`
  - `storybook-performance-check`

## 復元した gate

- `storybook-content-check` を `storybook-check` の必須 gate に戻した。
- `storybook-content-check` は次を実行する。
  - `cargo test -p kdv-storybook --locked preview_feature_matrix -- --test-threads=1`
  - `cargo test -p kdv-storybook --locked direct_diagram -- --test-threads=1`
  - `cargo test -p kdv-storybook --locked preview_requirement_matrix -- --test-threads=1`
  - `cargo test -p katana-document-viewer --locked fixture_feature_matrix -- --test-threads=1`
  - `cargo test -p katana-document-viewer --locked fixture_mermaid -- --test-threads=1`
  - `cargo test -p katana-document-viewer --locked direct_diagrams_bind -- --test-threads=1`
  - `cargo test -p katana-document-viewer --locked fixture_score_matrix -- --test-threads=1`
  - `cargo test -p katana-document-viewer --locked direct_ -- --test-threads=1`
  - `cargo test -p katana-document-viewer --locked export_quality -- --test-threads=1`
  - `cargo test -p katana-document-viewer --locked surface_equivalence -- --test-threads=1`
- `fixture_feature_matrix_tests.rs` は、Katana fixture / direct HTML fixture から `ViewerNodePlan` へ要件featureが届くことを検証する。
  - 対象: nested list、task states、syntax highlight、table、horizontal rule、accordion、math、footnote、alert、link、Mermaid、PlantUML、Draw.io、HTML alignment。
- `fixture_score_matrix_tests.rs` は、同じfixtureで `ViewerNodePlan` の要件到達とHTML/PDF/PNG/JPEG export score 95点以上を結び付ける。現時点では `direct/html-alignment.html` の中央/右/左寄せ、link、table、accordionと、`katana/sample_basic.md` のlist/task/code/table/rule/alert/accordion/math/footnote/linkを対象にする。
- `fixture_score_matrix_full_tests.rs` は、Katana top-level sampleの `sample` / `sample.ja` / `sample_html` / `sample_html.ja` について、rendered diagram graphを使ったHTML/PDF/PNG/JPEG export score 95点以上を検証する。
- `fixture_score_matrix_requirements_tests.rs` は、要件名ごとのmatrixでfixture到達とHTML/PDF/PNG/JPEG全format scoreの存在を同時に検証する。format score欠落もfailする。
- `preview_requirement_matrix_tests.rs` は、同じ要件featureがKUC Storybook sceneへ届くことを検証する。Katana basic Markdownではcode/table/alert/list/task/rule/link/mathに加え、KUC `UiTextSpan` 上のsyntax-highlighted code spanとstrikethrough spanも検証する。図形は単なるImageSurface数ではなく、`diagram:Mermaid`、`diagram:PlantUml`、`diagram:DrawIo` の種別別accessibility labelも検証する。
- `preview_direct_diagram_matrix_tests.rs` は、直接図形 `drawio/drowio`、`mermaid/mmd`、`plantuml/puml` がKUC Storybook scene上のImageSurface、種別別accessibility label、図形control action、asset失敗0へ届くことを検証する。
- `direct_diagram_score_matrix_tests.rs` は、同じ直接図形群がKDV viewer plan上の図形種別へ届き、rendered diagram graphを使ったHTML/PDF/PNG/JPEG export score 95点以上も満たすことを検証する。
- `preview_katana_markdown_matrix_tests.rs` は、Katana top-level Markdown fixtureのうち `sample` / `sample_basic` / `sample_diagrams` / `sample_html` の日英fixtureがKUC Storybook sceneへ届くことを検証する。巨大 `sample_mermaid*` は通常gateの同期ロード対象にせず、KDV runtime planの `fixture_mermaid_matrix_tests.rs` で50件以上のMermaid diagram nodeを検証する。
- `preview_katana_drawio_matrix_tests.rs` は、`assets/fixtures/katana/drawio/basic/*.drawio` 12件すべてがKUC Storybook scene上のImageSurface、asset失敗0、diagram control actionへ到達することを検証する。
- `frame_tests.rs` は、KUC feature matrixがPreviewScene上だけで止まらず、Storybook preview領域の実frame pixelまたはImageSurface数へ届くことを検証する。
  - 対象: `katana/sample.md`、`direct/html-alignment.html`、`katana/sample_diagrams.md`、`direct/sample.md`、`direct/kdv-icon.png`
  - pixel数はsidebar/header/statusを除いたpreview領域だけを `StorybookFramePixelGuard::preview_content_pixel_count` で数える。
- `frame_role_pixel_tests.rs` は、`katana/sample.md` と `direct/html-alignment.html` のKUC roleがStorybook frame上の期待色pixelへ到達することを検証する。Katana basic fixtureではKUC syntax span色が実frame pixelへ出ることも検証する。ノード存在だけで通る偽陽性を防ぐ。
- `frame_interaction_tests.rs` は、search highlight、Slideshow mode、media control toggle、hover highlight block背景がStorybook preview領域の実frame pixelへ届くことを検証する。
- `window_tests.rs` は、window resize検出だけでStorybook scene viewport更新対象になることを検証する。
- `window_tests.rs` は、resize後の `update_scene` が新viewportでSlideshow page数を再計算することも検証する。
- `ViewerCommandFactory` は、link clickをKMM node/source/rect metadata付き `LinkCommand` へ正規化し、task left clickとcontext menu markerを `TaskStateCommand` へ正規化する。
- `ViewerCommandFactory` は、KUC scene上のimage/diagram control action valueを `ImageControlCommand` / `DiagramControlCommand` へ正規化する。これは `katana-document-viewer` facade からも公開し、`preview_interaction_command_matrix_tests.rs` はKUC scene上のtask/link/image/diagram actionがpreview facade経由でKDV commandへ変換できることを検証する。
- `ViewerCommandFactory` は、diagram `copy-source` actionを `HostCommand::CopyText` へ正規化する。コピー対象は図形nodeのsource raw textであり、rendered SVGや表示ラベルではない。
- `ViewerCommandFactory` は、TOC item、search next/previous、Slideshow next/previous/close/settingsも統一入口から `ViewerCommand` へ正規化する。`preview_interaction_command_matrix_tests.rs` はTOC item、search target、Slideshow commandもpreview facade経由で検証する。
- `preview_interaction_command_metadata_tests.rs` は、KUC media control nodeの `state_id` とKUC link host action planから実 `ViewerNodePlan` 由来の `ViewerTarget` を復元し、合成targetではなくKMM node/source/rect metadata付きcommandになることを検証する。
- KUC list変換はraw行でtask markerを判定し、span行でlist内Markdown linkの `ViewerTextSpan.link_target` を保持する。これにより `https://github.com` / `mailto:test@example.com` のlist内linkもKUC host open actionへ到達する。
- `mouse_tests.rs` は、Storybook windowのminifb mouse edge相当入力からscroll込みdocument座標へ変換し、list内Markdown linkの実座標クリックがKMM node/source/rect付き `LinkCommand` になること、task checkbox左クリックが `TaskStateCommand` になること、task checkbox右クリックでKUC context menuを開きmenu item選択が `TaskStateCommand` になること、KUC image/diagram control clickが実 `ViewerTarget` 付き `ImageControlCommand` / `DiagramControlCommand` になることを検証する。
- `mouse_cursor_tests.rs` は、link hover cursorがKUC nodeの `UiCursor::Pointer` から解決されることを検証する。KDV側でlink hover UIを独自判定だけで作らない。
- `mouse_tests.rs` は、diagram `copy-source` のクリックが右上overlay controlのdocument bandで解決され、`HostCommand::CopyText` へ変換されることも検証する。
- `slideshow_keys_tests.rs` は、Katana仕様の `ArrowRight` / `PageDown` / `Space`、`ArrowLeft` / `PageUp`、`Esc` keymapを検証する。Slideshow中の `Esc` はStorybook window終了ではなくviewer closeへ流す。
- `search_keys_tests.rs` は、検索jumpのscroll位置をcontent heightとviewport heightでclampし、末尾hitで範囲外scrollしないことを検証する。
- `kuc-adapter-boundary-check` は、KDV core、KDV preview、KUC viewer、vendor-free Storybookのcargo treeと、ローカル `../katana-ui-core/crates/katana-ui-core` のcargo tree/sourceにegui/gpui/floem/winit/vello等が入った場合にfailする。
- `kuc-adapter-boundary-check` は、KDV core、KDV preview、KUC viewer、vendor-free Storybookの非テスト実装sourceへegui/gpui/floem/winit/vello等のvendor runtime参照が戻った場合もfailする。`build.rs`、`examples`、`benches` も存在すれば監査対象に含める。
- `kuc-adapter-boundary-check` は、vendor-free Storybook sidebarがKUC `FileTree::render` / `SettingsList::new` 経由であることも検査する。`tools/kdv-storybook/src` へ `CollapsingHeader` / `Button::selectable` / `TreeView::new` / `TreeNode::new` / vendor runtime参照が戻るとfailする。
- `kuc-adapter-boundary-check` は、KDV側へ `KucTreeCanvasRenderer` / `KucTextCanvasRenderer` / `KucControlCanvasRenderer` / `KucTreeViewCanvasRenderer` / `mod kuc_` の旧独自rendererが戻った場合もfailする。
- KUC側の `ui_tree_canvas_tests.rs` は、`UiTreeCanvasRenderer` がTree選択色、code背景色、SettingsListのToggle/Input/Select control、diagram media frame overlayを実canvas pixelへ描くことを検証する。KDV Storybook左ペインと図形コントローラーの実描画をKUC側でも退化検出するためのgateである。
- KUC側の `ui_tree_canvas_text_lines_tests.rs` は、HTML中央/右寄せの実測幅に加えて、`UiTextSpanStyle::strikethrough` が空白spanでも実canvas線になることを検証する。KDV StorybookはこのKUC rendererを使うため、KDV側で取り消し線描画を再実装しない。
- `just check` と `just storybook-check` は `kuc-adapter-boundary-check` を含む。

## 現在確認済み

```sh
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked frame -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked window -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked commands -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked viewer -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked artifact_search -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked preview_search -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked evaluation -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked fixture_feature_matrix -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked fixture_score_matrix -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked preview_feature_matrix -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked direct_diagram -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked direct_diagrams_bind -- --test-threads=1
/opt/homebrew/bin/rtk just storybook-interaction-check
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked preview_interaction_command_metadata -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked mouse -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked -- --test-threads=1
/opt/homebrew/bin/rtk cargo clippy -p kdv-storybook --all-targets --locked -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::todo -D clippy::unimplemented -D clippy::dbg_macro -D clippy::panic -D clippy::wildcard_imports
/opt/homebrew/bin/rtk just kuc-adapter-boundary-check
/opt/homebrew/bin/rtk bash -n scripts/kuc-adapter-boundary-check.sh
/opt/homebrew/bin/rtk just storybook-content-check
/opt/homebrew/bin/rtk just storybook-check
/opt/homebrew/bin/rtk just ast-lint
/opt/homebrew/bin/rtk cargo clippy -p katana-document-viewer --all-targets --locked -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::todo -D clippy::unimplemented -D clippy::dbg_macro -D clippy::panic -D clippy::wildcard_imports
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked direct_visual -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked source_text -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked markdown_blocks -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked table_count -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked asset_pipeline -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --release --locked export_quality -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --release --locked html_score -- --test-threads=1
/opt/homebrew/bin/rtk ./scripts/openspec validate v0-2-0-markdown-viewer-kuc-integration --strict --no-interactive
STORYBOOK_FRAMES=1 /opt/homebrew/bin/rtk just storybook
/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas -- --test-threads=1
/opt/homebrew/bin/rtk cargo clippy -p katana-ui-core-storybook --all-targets --locked -- -D warnings
/opt/homebrew/bin/rtk bash -lc 'git diff --check'
/opt/homebrew/bin/rtk bash -lc 'git -C /Users/hiroyuki_furuno/works/private/katana-ui-core diff --check'
```

結果:

- `frame`: 14 passed。
- `frame` filter current: 25 passed。syntax-highlighted frame pixel gate、hover highlight frame pixel gate、task state frame gate、preview本文領域限定のrole pixel gate、blank surface parity rejectを含む。
- `window`: 8 passed。
- `commands`: 14 passed。
- `viewer`: 145 passed。
- `kdv-storybook` full: 97 passed。Storybook上位gate内で通過。
- `artifact_search`: 6 passed。HTML/SVG artifact bytesから抽出した検索textが、matching artifact_idを持つViewerNode rectへ解決されることを検証。
- `preview_search`: 2 passed。Markdown解析、fake図形rendererのSVG artifactロード、KUC plan、Storybook検索target収集までartifact text検索が到達することを検証。
- `evaluation`: 10 passed。`gfm-strikethrough` をKDV viewer/export contractとしてcoverage matrixに含める。
- `fixture_feature_matrix`: 4 passed。取り消し線spanがviewer planへ届くことを検証。
- `fixture_score_matrix`: 4 passed。取り消し線要件もexport score 95点以上のgateへ接続する。
- `preview_feature_matrix`: 1 passed。直接画像全拡張子がStorybook KUC sceneへ到達することを検証。
- `direct_diagram`: 1 passed。直接図形全拡張子がStorybook KUC sceneへ到達することを検証。
- `direct_diagrams_bind`: 1 passed。直接図形全拡張子がviewer diagram kindとexport score 95点以上へ接続されることを検証。
- `kuc-adapter-boundary-check`: ok。
- `storybook-content-check`: `preview_feature_matrix` 1 passed、`direct_diagram` 1 passed、`preview_requirement_matrix` 3 passed、`fixture_feature_matrix` 4 passed、`fixture_mermaid` 1 passed、`direct_diagrams_bind` 1 passed、`fixture_score_matrix` 4 passed、direct source 79 passed、export quality 215 passed、surface equivalence 21 passed。`preview_requirement_matrix` はKUC scene上のsyntax-highlighted code spanとstrikethrough span保持を含む。
- `storybook-interaction-check`: commands 14 passed、`preview_interaction_command_matrix` 6 passed、`preview_interaction_command_metadata` 2 passed、mouse 10 passed、frame 25 passed、search 10 passed、slideshow 11 passed、window 8 passed。
- `preview_interaction_command_metadata`: 2 passed。action state idから復元したtarget/actionがcommand variant、action enum、target identityへroundtripすること、diagram複数nodeとaction+target重複なし、KUC link host action planから復元したlist内Markdown linkがKMM node/source/rect付き `LinkCommand` になることを検証。
- `mouse`: 10 passed。left clickがlink commandを返すこと、right clickがlink openにならないこと、task checkbox左クリックがtask commandを返すこと、task checkbox右クリックでcontext menuを開き `[-]` 選択がblocked task commandを返すこと、image control clickがimage commandを返すこと、diagram control clickがdiagram commandを返すこと、diagram copy-source clickがHost copy commandを返すこと、通常コードブロックcopy clickがHost copy commandを返すこと、mouse downのedgeだけをpressとして扱うこと、link hover cursorを検証。
- `clippy -p kdv-storybook --all-targets`: No issues found。
- `storybook-check`: 通過。entrypoint、KUC adapter boundary、`kdv-storybook` full 97 passed、content gate、interaction gate、window smoke、KUC smoke、performance checkを含む。
- `ast-lint`: 1 passed。
- `clippy -p katana-document-viewer --all-targets`: No issues found。
- `direct_visual`: 38 passed。
- `source_text`: 9 passed。
- `markdown_blocks`: 22 passed。
- `table_count`: 1 passed。
- `asset_pipeline`: 5 passed。current revisionの初回asset完了だけをacceptし、stale resultと重複完了はstate更新なしとして拒否する。
- `export_quality --release`: 213 passed。
- `html_score --release`: 155 passed。
- OpenSpec validate: valid。
- `STORYBOOK_FRAMES=1 just storybook`: `cargo run --release --locked -p kdv-storybook -- --interactive --frames 1` を実行する。
- KUC `ui_tree_canvas`: 2 passed。
- KUC `ui_tree_canvas_text`: 10 passed。
- `clippy -p katana-ui-core-storybook --all-targets`: No issues found。
- `git diff --check`: KDV/KUCともに問題なし。

## 分離作業証跡

- direct visual score gateの監査と修正は分離作業で実施済み。証跡: agent: `019e8377-49be-7e20-b0b3-8bfef655c636` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `crates/katana-document-viewer/src/export_quality/html_score_direct_visual.rs` / file: `crates/katana-document-viewer/src/export_quality/html_score_direct_visual_helper_tests.rs` / file: `crates/katana-document-viewer/src/export_quality/html_score_direct_visual_tests.rs` / file: `crates/katana-document-viewer/src/export_quality/html_score.rs` / file: `crates/katana-document-viewer/src/export_quality/html_score_tests.rs` / command: `multi_agent_v1.spawn_agent` / close: `multi_agent_v1.close_agent` / verify: `rtk cargo test -p katana-document-viewer --release --locked whitespace -- --test-threads=1`
- Score gate偽陽性監査は read-only subagent で追加実施した。証跡: agent: `019e8724-5376-7681-afc4-0437327876fc` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `crates/katana-document-viewer/src/export_quality/html_score_direct_visual.rs` / command: `multi_agent_v1.spawn_agent` / verify: `rtk just check-subagent-harness` / close: `multi_agent_v1.close_agent` / 指摘: direct image URI属性semantics、HTML source text順序、Markdown block count、Storybook frame shape、interaction metadata roundtrip / 対応済み: direct image URI属性semantics、HTML source text順序、Markdown heading/list/code/table count、interaction metadata roundtrip。
- 2026-06-03 追加: 利用上限で分離作業を起動できず、ローカル実装で継続した。対応内容は、KUC Accordion open overrideをStorybook window stateへ接続、code block copy controlをKUC nodeから `ViewerCommandFactory::code_control_from_action` 経由の `HostCommand::CopyText` へ接続、diagram `copy-source` の右上overlay control検証、KDV Storybook file selectionをKUC `FileTree::selected_item_id` action契約経由へ変更、mouse/preview/windowの肥大化ファイル分割、Storybook surface parity双方blank誤通過の禁止、Export surfaceの実 `SurfaceTextPainter` 必須化と簡易矩形文字fallback削除。検証: `rtk cargo test -p kdv-storybook --locked mouse -- --test-threads=1 --nocapture` は 10 passed、`rtk cargo test -p kdv-storybook --locked window -- --test-threads=1 --nocapture` は 8 passed、`rtk cargo test -p kdv-storybook --locked sidebar -- --test-threads=1 --nocapture` は 14 passed、`rtk cargo test -p katana-document-viewer-kuc --locked -- --test-threads=1` は 76 passed、`rtk cargo clippy -p kdv-storybook -p katana-document-viewer-kuc --all-targets --locked -- -D warnings` は No issues found、`rtk cargo clippy -p katana-document-viewer --all-targets --locked -- -D warnings` は No issues found、`rtk cargo test -p katana-document-viewer --locked export_surface_painter -- --test-threads=1` は 33 passed、`rtk cargo test -p katana-document-viewer --locked export_surface_font -- --test-threads=1` は 20 passed、`rtk cargo test -p katana-document-viewer --locked export_quality -- --test-threads=1` は 215 passed、`rtk cargo test -p kdv-storybook --locked frame_surface_similarity -- --test-threads=1 --nocapture` は 4 passed、`rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer -- --test-threads=1 --nocapture` は 2 passed、`rtk cargo test -p kdv-storybook --locked frame -- --test-threads=1 --nocapture` は 25 passed、`rtk just storybook-interaction-check` は commands 14 passed、command matrix 6 passed、metadata 2 passed、mouse 10 passed、frame 25 passed、search 10 passed、slideshow 11 passed、window 8 passed、`rtk just kuc-adapter-boundary-check` は ok、`rtk just storybook-content-check` は preview feature 1 passed、direct diagram 1 passed、requirement matrix 3 passed、fixture feature 4 passed、fixture mermaid 1 passed、direct diagrams bind 1 passed、fixture score 4 passed、direct source 79 passed、export quality 215 passed、surface equivalence 21 passed、`rtk just storybook-window-smoke` は `fixtures=42 checked=42`、`rtk just storybook-check` は `kdv-storybook` full 97 passedを含めて通過、`STORYBOOK_FRAMES=1 rtk just storybook` は interactive entrypointを起動して正常終了した。

## 注意点

- ローカル `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/examples` には、現時点で `kdv_storybook.rs` / `kdv_storybook_smoke.rs` が存在しない。
- そのため、過去の「Katana-hosted launcher」証跡は current evidence として使わない。
- Katana 由来の完全互換は、KDV/KUC の fixture、direct source、viewer feature matrix、export quality、surface equivalence、runtime smoke で継続して証明する。

## 残り

- `remaining-plan.md` の `- [ ]` は現時点で0件。次の作業は、current stateで要件とユーザー指摘を再監査し、抜けがあれば新しい未対応項目として物理ファイルへ追加すること。
- 完了判定は、要件ごとの実装箇所、fixture、score gate、runtime smoke または interaction test、実行コマンドの current evidence が揃い、かつ偽陽性経路が追加監査で見つからない時点で行う。
