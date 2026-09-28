# KDV v0.2.0 未達引き継ぎ資料

最終更新: 2026-06-13

絶対パス:

```text
/Users/hiroyuki_furuno/works/private/katana-document-viewer/openspec/changes/v0-2-0-markdown-viewer-kuc-integration/handoff-unresolved-2026-06-12.md
```

## 結論

KDV v0.2.0 の DoD は未達。`just storybook-score-check` は 2026-06-12 の直近修正後に通過したが、実画面の TreeView / Toggle / link / accordion / diagram control / task checkbox / emoji / scroll などの未達が残っており、KatanA viewer と同等とは言えない。

最新の入口は `handoff-current-2026-06-13.md` とする。このファイルは詳細履歴として保持し、最初に読む短い入口は `handoff-current-2026-06-13.md`、計画書は `kdv-v0.2.0-viewer-recovery-plan.md` を参照する。

この資料は、次担当者が「何が出来ていないか」を先に把握し、見た目だけの局所修正や通過済みテストの誤解を避けるための引き継ぎ正本にする。

既存の `handoff.md` は 2026-06-03 時点の記録で、現在の実画面指摘と食い違う内容がある。このファイルを優先して読むこと。

## 最初に読む要約

この引き継ぎ時点で、v0.2.0 viewer parity は完了していない。直近でやりきった範囲は、重複 scene 構築の削減、direct image の重複 smoke 縮小、diagram scroll reload guard、interaction gate の再実行証跡、task checkbox / accordion / diagram load / slideshow の専用 gate を `storybook-check` と `storybook-entrypoint-check` へ接続したところまでである。

完了扱いにしてはいけない理由:

- `user-feedback-todo.md` に未対応 `[ ]` が残っている。
- `just storybook-score-check` は通るが、実画面の TreeView / Toggle / link / accordion / diagram control / task checkbox / emoji / scroll の全未達を捕捉できる gate にはまだなっていない。
- KatanA reference HTML/PDF/screenshots との全カテゴリ 95 点以上を、実画面指摘込みで証明できていない。
- KDV 側に UI hit-test / action / hover / cursor の過渡実装が残っており、KUC 共通 UI 契約への寄せ切りが終わっていない。

現時点で信用してよい最新の検証値:

- `/opt/homebrew/bin/rtk cargo fmt --all --check`: passed
- `/opt/homebrew/bin/rtk /usr/bin/time -p cargo test -p kdv-storybook --locked mouse -- --test-threads=1`: 28 passed、process real `18.53s`
- `/opt/homebrew/bin/rtk /usr/bin/time -p just storybook-interaction-check`: passed、process real `225.33s`
- `storybook-interaction-check` 内訳の最新値:
  - preview interaction command matrix: 6 passed / `21.56s`
  - metadata: 3 passed / `11.27s`
  - mouse: 28 passed / `11.10s`
  - interaction: 8 passed / `32.88s`
  - scroll: 5 passed / `9.18s`
  - sidebar: 2 passed / `1.44s`
  - task state: 1 passed / `7.65s`
  - search: 13 passed / `4.38s`
  - slideshow: 17 passed / `20.85s`
  - window: 112 passed / 3 ignored / `92.42s`
- `/opt/homebrew/bin/rtk just storybook-task-checkbox-check`: passed。20本通過。
- `/opt/homebrew/bin/rtk just storybook-accordion-check`: passed。17本通過。
- `/opt/homebrew/bin/rtk just storybook-diagram-load-check`: passed。30本通過。
- `/opt/homebrew/bin/rtk just storybook-slideshow-check`: passed。26本通過。
- `/opt/homebrew/bin/rtk just storybook-coordinate-contract-check`: passed。KUC core normalizer 1件、KUC Storybook hit 23件、row layout 1件、KDV window coordinate 3件、normalizer source guard 1件、hit rect center 5件、sidebar/settings/link/accordion/task/media/window regression 10件、manual action contract guard 1件が通過。
- `/opt/homebrew/bin/rtk just storybook-hover-contract-check`: passed。KUC core hover state 2件、KUC Storybook hover visual 8件、KDV hover / cursor / block hover / media hover / guard 21件が通過。
- `/opt/homebrew/bin/rtk just storybook-image-control-check`: passed。direct image 全種 surface/control hit、Window direct raster image render、hover、transparent base、click command、scene refresh、asset job loaded scene の10本が通過。
- `/opt/homebrew/bin/rtk just storybook-scroll-resize-contract-check`: passed。frame scroll、bottom spacer、last target top alignment、window bottom scroll、resize bottom anchor、scroll座標normalizer、diagram scroll/resize cache reuse の14本が通過。
- `/opt/homebrew/bin/rtk just storybook-entrypoint-check`: passed。上記 gate が `storybook-check` から外れた場合に fail する状態。

ただし、上記は DoD 完了証拠ではない。いずれも regression gate への接続であり、実 OS window screenshot、KatanA viewer / slideshow との visual parity、KatanA export HTML/PDF/screenshots とのカテゴリ別 95 点以上をまだ証明していない。

## 2026-06-13 時点の最重要未達

次担当者は、以下を未完了として扱うこと。ここを完了扱いにすると、過去と同じく broken UI を gate が見逃す。

1. `SS-002` / `UF-008`: click / hover / action の座標契約がまだ全 UI で信用できない。
   - TreeView、Toggle、Link、Accordion、Task、MediaControl の代表 gate は増えたが、全 node が同じ layout result / hit result を共有する証明と実 OS window 証跡が不足している。
2. `UF-031` / `UF-036` / `UF-038`: KUC interactive preset と KDV/KUC/Storybook 境界の証拠階層がまだ足りない。
   - static lint と一部 window gate はあるが、Contract / Static guard / Roundtrip / OS window score gate の全セルを埋め切っていない。
3. `UF-001` から `UF-007`: 左ペインの TreeView / SettingsList は KUC 経由の自動テストがあるが、KatanA Explorer 相当の実画面 visual parity と全操作証跡が未完。
4. `UF-011` / `UF-012` / `UF-015` / `UF-016`: Markdown 基本描画は、KatanA / HTML / PDF とまだ一致していない。
   - code、task、accordion、emoji は専用 gate を接続済みだが、KatanA reference screenshot との section parity は未証明。
5. `UF-019` から `UF-023` / `UF-039`: diagram / image control は lazy load、cache、hover、click の gate があるが、配置、透明 base、SVG品質、dark theme、png/jpg表示、KatanA control visual parity が未完。
6. `UF-027` / `UF-028` / `UF-029`: scroll / resize / score gate がまだ DoD 判定として弱い。
   - score は通ることがあるが、ユーザーが実画面で見つけた破綻を全て fail できる状態ではない。

## 2026-06-13 Hover Contract Gate 引き継ぎ更新

この更新では、UF-009 / UF-010 の途中作業を通常 gate に接続した。

変更:

- `justfile` に `storybook-hover-contract-check` を追加した。
- `storybook-check` に `storybook-hover-contract-check` を追加し、通常検証から外れないようにした。
- `scripts/check-storybook-entrypoint.sh` に hover gate の代表テスト名を追加し、entrypoint から外れたら fail するようにした。
- `user-feedback-todo.md` の UF-009 / UF-010 に、gate 化と検証結果を追記した。

検証:

- `/opt/homebrew/bin/rtk just storybook-hover-contract-check`: passed。
- `/opt/homebrew/bin/rtk just storybook-entrypoint-check`: `storybook-entrypoint-check: ok`。

この gate で固定した範囲:

- KUC `UiTree::with_hovered_node_id()` の hover state contract。
- KUC Storybook の TreeView hover row、TreeView hover border、generic button / toggle / checkbox / SettingsList / text hover visual。
- KDV Storybook の hover target 解決、block hover surface、scroll 後 hover highlight、cursor、image / code / diagram controls hover、window frame 上の TreeView / SettingsList / Link / Accordion / code copy / MediaControl hover pixel。
- `no_reintroduced_manual_storybook_action_contracts` による手戻り guard。

残:

- 実 OS cursor の screenshot / 操作証跡は未完。
- KatanA screenshot との hover visual parity は未完。
- 全操作部品の hover visual matrix は未完。
- そのため UF-009 / UF-010 は `[ ]` のまま維持する。

## 2026-06-13 Direct Image Window Gate 引き継ぎ更新

この更新では、UF-039 の direct raster image 表示検証を frame helper だけでなく `StorybookWindow` の loaded scene 経路へ広げた。

変更:

- `tools/kdv-storybook/src/window/image_fixture_window_tests.rs` を追加した。
- `tools/kdv-storybook/src/window.rs` へ test module として接続した。
- `Justfile` の `storybook-image-control-check` に `direct_raster_image_window_loaded_scenes_render_visible_image_surfaces` を追加した。
- `scripts/check-storybook-entrypoint.sh` に同テスト名を追加し、image gate から外れたら fail するようにした。
- `user-feedback-todo.md` の UF-039 に、Window 経路の direct raster image 表示証跡を追記した。

検証:

- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked direct_raster_image_window_loaded_scenes_render_visible_image_surfaces -- --test-threads=1`: passed。
- `/opt/homebrew/bin/rtk just storybook-image-control-check`: passed。
- `/opt/homebrew/bin/rtk just storybook-entrypoint-check`: `storybook-entrypoint-check: ok`。

この gate で固定した範囲:

- `.bmp/.gif/.jpeg/.jpg/.png/.webp` が `StorybookWindow::update_scene_loaded()` 後に pending / failed asset なしで `ImageSurface` と loaded asset を持つ。
- 同じ Window state の `render_canvas()` 上に、image control ではなく preview 側の KDV icon blue pixel が十分に描かれる。

残:

- 実 OS window screenshot による png/jpg 表示証跡は未完。
- KatanA reference との image visual parity は未完。
- image control の配置・見た目の人間確認と score gate 接続は未完。
- そのため UF-039 は `[/]` のまま維持する。

## 2026-06-13 Scroll / Resize Contract Gate 引き継ぎ更新

この更新では、UF-027 の scroll / resize / bottom spacer / diagram reload 抑止の既存検査を通常 gate にまとめた。

変更:

- `Justfile` に `storybook-scroll-resize-contract-check` を追加した。
- `storybook-check` に `storybook-scroll-resize-contract-check` を追加し、通常検証から外れないようにした。
- `scripts/check-storybook-entrypoint.sh` に scroll / resize gate の代表テスト名を追加し、entrypoint から外れたら fail するようにした。
- `user-feedback-todo.md` の UF-027 に、gate 化と検証結果を追記した。

検証:

- `/opt/homebrew/bin/rtk just storybook-scroll-resize-contract-check`: passed。
- `/opt/homebrew/bin/rtk just storybook-entrypoint-check`: `storybook-entrypoint-check: ok`。

この gate で固定した範囲:

- frame scroll で preview content が動き、shell を上書きしない。
- bottom scroll で KatanA-style tail space が出る。
- bottom scroll で last target top が viewport top に揃う。
- 同じ scroll offset を再構築しても描画が同一になる。
- window scroll 入力が KUC core mouse normalizer と同じ座標系を使う。
- preview scroll が document scroll を更新する。
- resize 後に過大 scroll が clamp される。
- scene viewport が実 rendered preview content area と一致する。
- scroll 後も scene / asset job scope が保たれる。
- bottom scroll と resize bottom anchor が `StorybookWindow` state で保たれる。
- diagram loaded scene の scroll / resize が asset job restart や rerender を起こさない。

残:

- 実 OS window での手動/自動 resize 操作証跡は未完。
- 実 screenshot での末尾到達証跡は未完。
- KatanA reference との scroll/resize visual score は未完。
- そのため UF-027 は `[ ]` のまま維持する。

次に着手する順序:

1. `user-feedback-todo.md` の未対応 `[ ]` を、実装済みだが証跡不足の項目と、本当に未実装の項目に分ける。
2. 実装済み扱いにする項目は、実 OS window または KatanA reference 比較まで証跡を追加してから `[x]` または `[/]` に変える。証跡不足なら `[ ]` のままにする。
3. まず KUC 側で cursor / hover / click / hit-test の共通契約を固め、KDV 側の個別補正を削る。
4. 次に左ペイン、Markdown 基本描画、diagram/image、scroll/resize、score gate の順に潰す。
5. `just storybook-score-check` の通過を完了根拠にせず、実画面指摘を fail できる score / interaction / performance gate へ昇格してから DoD 判定する。

このファイル内の履歴は長い。次担当者は、まずこの要約、`現在の未達一覧`、`次担当者の最短手順`、`完了条件` の4箇所を読むこと。

## 守る前提

- KDV は viewer engine / document model / asset pipeline を担当する。
- KUC は UI 共通部品、描画、hit-test、action、cursor、preset behavior を担当する。
- KDV Storybook は KUC 実部品を host する検証画面であり、独自 TreeView、独自 toggle、独自 media button、独自 hit-test を増やさない。
- egui / gpui / floem は現時点では復帰しない。まず Katana 由来の vendor-free KUC viewer を完成させる。
- `just storybook` は smoke ではなく interactive viewer 起動である。
- score は自己比較ではなく KatanA reference HTML/PDF/screenshots と比較し、visual / semantic / interaction / performance の全カテゴリ 95 点以上が必要。

## 直近で入ったが完了ではない修正

### Direct image control test de-duplication

変更対象:

- `tools/kdv-storybook/src/frame_media_control_tests/image_code_tests.rs`
- `tools/kdv-storybook/src/frame_media_control_tests/support.rs`
- `tools/kdv-storybook/src/frame_media_control_tests/hover_tests.rs`
- `tools/kdv-storybook/src/window_command/tests/media.rs`

内容:

- direct raster image の「画像本体 pixel が描かれている」検査を、別テストではなく `image_controls_have_rendered_frame_hits_for_all_actions` の同一 canvas 検査へ統合した。
- `.bmp/.gif/.jpeg/.jpg/.png/.webp` の raster image pixel 検査と、image control 6種の hit / visible pixel 検査を同じ scene/render で見る。
- scene cache 追加も試したが、`media_control` suite は明確に改善しなかったため採用しなかった。
- media / image control の test support は `PreviewBuilder` を共有し、fixture scene build の parser / loader 初期化を使い回す。ただし scene 自体の cache は入れていない。
- diagram / image / code hover tests は、検査対象ではない sidebar frame を各 hover render で作り直さず、同一 sidebar canvas を再利用する。
- image control command tests は、zoom / fit / copy の3テストで同じ direct image scene を作っていたため、`image_control_command_refreshes_scene` に統合して同じ contract を1つの scene flow で確認する。
- image control command tests は、`open` / `copy` / `reveal-in-os` の非再描画 command と、`zoom-out` の再描画 command も KUC host action 経由で確認する。
- 2026-06-12 追補: `frame_rect_for_hit` と `preview_frame_bounds` を test support に追加し、control 内部検査で hit 中心点を左上として扱う弱い検証をやめた。
- 2026-06-12 追補: direct raster image pixel 検査は preview 領域内かつ image control 矩形外だけを数える。右側の control 群や sidebar 選択色だけでは画像表示検査を通せない。

検証:

- `/opt/homebrew/bin/rtk cargo fmt --all --check` は通過。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked image_controls_have_rendered_frame_hits_for_all_actions -- --test-threads=1` は `1 passed`, `6.00s`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked image_ -- --test-threads=1` は `18 passed`, `60.69s`。直前の `19 passed`, `70.22s` から重複 test 1本分を削減した。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked media_control -- --test-threads=1` は `10 passed`, `65.70s`。直前の `11 passed`, `68.44s` から重複 test 1本分を削減した。
- `PreviewBuilder` 共有後の再実行では `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked media_control -- --test-threads=1` が `10 passed`, `58.93s`、`/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked image_ -- --test-threads=1` が `18 passed`, `61.08s`。
- `/opt/homebrew/bin/rtk just storybook-media-control-clickability-check` は通過。KDV Storybook `media` は `19 passed`, `97.10s`、`media_control` は `10 passed`, `59.46s`。
- sidebar 再利用と image command test 統合後の再実行では `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked media -- --test-threads=1` が `17 passed`, `93.13s`、`/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked media_control -- --test-threads=1` が `10 passed`, `55.34s`。
- `/opt/homebrew/bin/rtk just storybook-media-control-clickability-check` は再実行で通過。KDV Storybook `media` は `17 passed`, `93.07s`、`media_control` は `10 passed`, `54.17s`。
- 2026-06-12 追補後に `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked image_controls_have_rendered_frame_hits_for_all_actions -- --test-threads=1` は `1 passed`。
- 2026-06-12 追補後に `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked image_control_buttons_keep_transparent_base_in_dark_and_light -- --test-threads=1` は `1 passed`。
- 2026-06-12 追補後に `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked media_control -- --test-threads=1` は `10 passed`, `57.54s`。
- 2026-06-12 追補後に `/opt/homebrew/bin/rtk cargo fmt --all --check` は通過。
- 2026-06-12 追補後に `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked image_control -- --test-threads=1` は `6 passed`。
- 2026-06-12 追補後に `/opt/homebrew/bin/rtk just storybook-media-control-clickability-check` は通過。KUC `ui_tree_canvas_hit` 23 passed、KUC icon variant 1 passed、KUC document_viewer 77 passed、KDV core media_control 5 passed、KDV Storybook `media` 17 passed / `98.07s`、KDV Storybook `media_control` 10 passed / `59.15s`。
- 2026-06-12 追補後に `/opt/homebrew/bin/rtk just storybook-media-control-clickability-check` を再実行し、KUC `ui_tree_canvas_hit` 23 passed、KUC icon variant 1 passed、KUC document_viewer 79 passed、KDV core media_control 5 passed、KDV Storybook `media` 19 passed / `94.41s`、KDV Storybook `media_control` 10 passed / `55.24s` で通過した。

未達:

- `media` / `media_control` はまだ 59〜98 秒級で、実用性能としては未達。
- 実 OS window screenshot、png/jpg の KatanA reference との見た目同等性、画像 control の配置の人間確認は未達。
- DoD 完了扱いにしない。

### Storybook diagram physical SVG cache evidence

変更対象:

- `tools/kdv-storybook/src/preview.rs`
- `tools/kdv-storybook/src/preview_diagram_disk_cache_tests.rs`
- `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/handoff-unresolved-2026-06-12.md`
- `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/user-feedback-todo.md`

内容:

- KDV core の diagram disk cache は `crates/katana-document-viewer/src/preview_runtime/asset_loader_cache.rs` と `asset_loader_disk_cache_tests.rs` で既に物理 SVG cache を持つ。
- Storybook 側でも `PreviewBuilder` が `KDV_DIAGRAM_CACHE_DIR` または `target/kdv-diagram-cache` を `PreviewAssetLoader::with_diagram_cache_root()` へ渡している。
- ただし、Storybook 経由で実際に物理 cache root へ SVG が書かれる回帰が薄かったため、`storybook_loaded_diagram_scene_writes_svg_to_physical_cache_root` を追加した。
- KatanA platform cache の raw key は実装上 `doc_path + kind + theme + source_hash` 系で、AST position を含めていない。KDV core の `diagram_disk_cache_ignores_source_position_when_content_is_unchanged` はこの実装に近い契約として維持する。

検証:

- `/opt/homebrew/bin/rtk cargo fmt --all --check` は通過。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_loaded_diagram_scene_writes_svg_to_physical_cache_root -- --test-threads=1` は `1 passed`。
- `/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked diagram_disk_cache -- --test-threads=1` は `4 passed`。
- `/opt/homebrew/bin/rtk just storybook-performance-check` は初回 `cached_round_trip_storybook_switch_stays_inside_interactive_budget` が `726.510708ms / 600ms` で失敗した。単発再実行は通過した。その後 sidebar frame cache round-trip reuse 修正を追加し、`just storybook-performance-check` は通過した。

未達:

- 物理 SVG cache への書き込み証跡は追加したが、renderer boundary の cancellation、timeout、RGBA、高解像度 rasterize、実 OS window の連続 scroll / resize で cache が効いている証跡は未達。
- DoD 完了扱いにしない。

### Sidebar frame cache round-trip reuse

変更対象:

- `tools/kdv-storybook/src/window_sidebar_frame_cache.rs`
- `tools/kdv-storybook/src/window.rs`
- `tools/kdv-storybook/src/window_render.rs`
- `tools/kdv-storybook/src/window_loop.rs`
- `tools/kdv-storybook/src/window_keyboard.rs`
- `tools/kdv-storybook/src/window_mouse.rs`
- `tools/kdv-storybook/src/window_scene.rs`
- `tools/kdv-storybook/src/window_tests.rs`
- `tools/kdv-storybook/src/frame_performance_tests.rs`
- `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/user-feedback-todo.md`

内容:

- 実 window の sidebar frame cache は 1 件だけで、fixture A -> B -> A の戻りで A の sidebar frame を捨てていた。
- `StorybookSidebarFrameCacheStore` を追加し、最大 4 件の sidebar frame を key ごとに保持するようにした。
- resize / hover / settings section toggle / slideshow page change のように見た目状態が変わる場合は `clear()` する。fixture切替では scene と selected index を含む key mismatch に任せ、過去 frame を保持する。
- `cached_round_trip_storybook_switch_stays_inside_interactive_budget` は古い手動 `FrameRenderer` 経路ではなく、実 `StorybookWindow::update_scene_for_tests()` と `render_canvas_for_tests()` 経路で測るようにした。

検証:

- `/opt/homebrew/bin/rtk cargo fmt --all --check` は通過。
- `/opt/homebrew/bin/rtk cargo check -p kdv-storybook --locked` は通過。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked round_trip_fixture_switch_reuses_previous_sidebar_frame_cache -- --test-threads=1` は `1 passed`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked sidebar_frame_cache -- --test-threads=1` は `4 passed`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked asset_worker -- --test-threads=1` は `5 passed`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --release --locked cached_round_trip_storybook_switch_stays_inside_interactive_budget -- --ignored --test-threads=1` は `1 passed`。
- `/opt/homebrew/bin/rtk just storybook-performance-check` は通過。

未達:

- TreeView の実 OS hover/click 体感、全 fixture traversal、KatanA visual parity、score 95 は未達。
- DoD 完了扱いにしない。

### Task checkbox action target fallback

変更対象:

- `tools/kdv-storybook/src/mouse_task.rs`
- `tools/kdv-storybook/src/mouse_task/tests.rs`

内容:

- KUC `UiTaskControlAction` が取れているのに、KDV が `row_index` で元 source 行を再検索できない場合に task command 全体を落とす経路を削った。
- `row_source()` は行が取れる場合はその行を保持し、取れない場合は KUC action の `current_marker` を source snapshot として残す。
- Storybook は markdown source を編集せず、task command を外部 state override へ伝播する責務に限定する。
- 回帰テストは `mouse_task/tests.rs` に分離した。`mouse_task.rs` 本体は境界 lint により `UiTaskMarker` などの低レベル task marker 依存を置けない。

検証:

- `/opt/homebrew/bin/rtk cargo fmt --all --check` は通過。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked row_source_ -- --test-threads=1` は `2 passed`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked mouse_left_click_on_task -- --test-threads=1` は `2 passed`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked mouse_task_context_menu_selection_sets_task_state -- --test-threads=1` は `1 passed`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_window_task_ -- --test-threads=1` は `3 passed`。
- 2026-06-12 追補で `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked task -- --test-threads=1` は `19 passed`。KUC host action 経由の task click、context menu 全 marker `[ ]` / `[x]` / `[/]` / `[-]`、外部 state override、scene 再生成後の checkbox node value 反映を確認済み。
- `/opt/homebrew/bin/rtk just kuc-adapter-boundary-check` は通過。
- `/opt/homebrew/bin/rtk just storybook-interaction-check` は通過。

未達:

- task の command 生成、外部 state override、scene 反映は自動検証済み。
- checkbox の見た目、Retina 上の list marker / checkbox alignment、右クリック context menu の実機操作証跡、KatanA visual parity は未達。
- これは task action 接続の安全化であり、DoD 完了扱いにしない。

### KUC icon button transparent base

変更対象:

- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/ui_tree_canvas_control.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/ui_tree_canvas_tests.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/ui_tree_canvas_hit_tests.rs`

内容:

- KUC `UiVariant::Icon` button の通常状態から fill / muted border を外し、透明 base にした。
- `Outline` button は従来どおり outline を残す。
- hover border は既存 `UiTreeCanvasHover::draw_node_border` の shared hover border 契約で維持する。
- media overlay 用 `surface-overlay-button` は KatanA 参照の図形/画像コントロールに合わせ、通常時も surface + border を描く。通常 `Icon` button の透明 base とは分離する。
- 目的は、code block copy button、diagram/image control button の見た目を KDV 個別補正ではなく KUC 側の button contract として扱うこと。

検証:

- 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked button_keeps -- --test-threads=1` は `4 passed`。
- 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked stack_absolute_children_overlay_top_and_bottom_controls -- --test-threads=1` は `1 passed`。
- 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked row_respects_explicit_stack_slot_width_for_diagram_toolbar -- --test-threads=1` は `1 passed`。
- 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked icon_variant_button_keeps_transparent_base_on_tree_canvas -- --test-threads=1` は `1 passed`。
- 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked code_block_has_copy_control_without_losing_code_body -- --test-threads=1` は `1 passed`。
- 外部KUC `/opt/homebrew/bin/rtk cargo fmt --all --check` と `/opt/homebrew/bin/rtk cargo check -p katana-ui-core-storybook --locked` は通過。
- KDV `/opt/homebrew/bin/rtk just storybook-media-control-clickability-check` は通過。
- KDV `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked code_copy_control_does_not_render_blue_filled_button_in_storybook_frame -- --test-threads=1` は `1 passed`。code copy control が旧青塗りボタンへ退行しないことだけを固定する。
- KDV `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked mouse_left_click_on_code_copy_returns_host_copy_command -- --test-threads=1` は `1 passed`。
- KDV `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked required_kuc_roles_reach_fixture_frame_pixels -- --test-threads=1` は `1 passed`。
- KDV `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked preview_interaction_command_metadata -- --test-threads=1` は `3 passed`。
- KDV `/opt/homebrew/bin/rtk cargo fmt --all --check` は通過。
- KDV `/opt/homebrew/bin/rtk just kuc-adapter-boundary-check` は通過。

未達:

- 実 OS window 上の hover cursor / all media controls click / code copy click の目視および操作証跡はまだ不足している。
- KatanA visual parity、code block syntax / clipping、code block 全体の box metrics、diagram lazy completion、score 95 は未達。
- DoD 完了扱いにしない。

### Storybook interaction gate hardening

変更対象:

- `justfile`
- `scripts/check-storybook-entrypoint.sh`

内容:

- `storybook-settings-contract-check` に SettingsList / Toggle のクリック中心・端の明示ゲートを追加した。
- `storybook-entrypoint-check` が `storybook-check` の dry-run を見て、FileTree hover、SettingsList toggle click、media control、command metadata、mouse、frame interaction、scroll、sidebar、task state、search、slideshow、window、score parity gate を必須として確認するようにした。
- 目的は、実装が未達なのに recipe から相互作用ゲートだけが落ち、entrypoint check が通ってしまう再発を防ぐこと。

検証:

- `/opt/homebrew/bin/rtk bash scripts/check-storybook-entrypoint.sh` は `storybook-entrypoint-check: ok`。
- `/opt/homebrew/bin/rtk just storybook-entrypoint-check` は通過。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_window_settings_toggle_click_uses_kuc_action_target -- --test-threads=1` は `2 passed`。
- `/opt/homebrew/bin/rtk just storybook-settings-contract-check` は通過。
- `/opt/homebrew/bin/rtk just storybook-interaction-check` は通過。
- `/opt/homebrew/bin/rtk just storybook-media-control-clickability-check` は通過。

未達:

- この修正は gate の抜け落ちを防ぐだけで、実 OS window 上の全 click / hover / cursor、KatanA visual parity、score 95 を満たした証明ではない。
- DoD 完了扱いにしない。

### Slideshow keyboard page height drift

変更対象:

- `tools/kdv-storybook/src/slideshow_keys.rs`
- `tools/kdv-storybook/src/slideshow_keys_tests.rs`
- `tools/kdv-storybook/src/window_keyboard.rs`

内容:

- `window_keyboard.rs` の slideshow next / previous page は `window height - header` を page height にしていたため、実 preview inset 32px 分だけ page scroll がズレていた。
- `StorybookSlideshowKeys::viewport_height_for_window()` を追加し、keyboard 経由の slideshow page 移動量を `preview_viewport_height(window_height)` に統一した。
- `slideshow_page_height_uses_preview_viewport_not_header_only_window_height` と `slideshow_next_page_scrolls_by_storybook_preview_viewport_height` を追加し、旧 header-only page height へ戻ると落ちるようにした。

検証:

- `/opt/homebrew/bin/rtk cargo fmt --all --check` は通過。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked slideshow -- --test-threads=1` は `14 passed`。
- `/opt/homebrew/bin/rtk just storybook-interaction-check` は通過。commands `14 passed`、preview command matrix `6 passed`、metadata `3 passed`、mouse `28 passed`、interaction `8 passed`、scroll `4 passed`、sidebar `2 passed`、task state `1 passed`、search `13 passed`、slideshow `14 passed`、window `103 passed / 3 ignored`。

未達:

- keyboard page drift は解消したが、KatanA slideshow modal の visual parity、実 OS window での連続キー操作証跡、slide control UI の完全同等性は未達。
- DoD 完了扱いにしない。

### Accordion action contract

変更対象:

- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/src/render_model/host_action_text.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/src/render_model/mod.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/tests/host_action_plan_contract.rs`
- `tools/kdv-storybook/src/mouse_accordion.rs`
- `tools/kdv-storybook/src/storybook_contract_regression_tests.rs`

内容:

- 外部KUC `UiTextSpanAction::accordion_toggle_action()` に requested open state を作る責務を移した。
- KDV Storybook `mouse_accordion.rs` から `ToggleAccordion { open }` の分解と `!open` 合成を削除した。
- Storybook 側へ `ToggleAccordion` 分解や `!open` 合成が戻ったら regression test で fail するようにした。

検証:

- 外部KUC `/opt/homebrew/bin/rtk cargo fmt --all --check` は通過。
- KDV `/opt/homebrew/bin/rtk cargo fmt --all --check` は通過。
- 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core --locked accordion_text_action_exposes_requested_open_without_consumer_inversion -- --test-threads=1` は `1 passed`。
- 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core --locked --test host_action_plan_contract -- --test-threads=1` は `9 passed`。
- KDV `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked accordion -- --test-threads=1` は `8 passed`。
- KDV `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_contract_regression_tests -- --test-threads=1` は `1 passed`。
- KDV `/opt/homebrew/bin/rtk cargo test -p kdv-linter --locked storybook_contract -- --test-threads=1` は `3 passed`。
- KDV `/opt/homebrew/bin/rtk just kdv-lint` は通過。
- KDV `/opt/homebrew/bin/rtk just kuc-adapter-boundary-check` は通過。

未達:

- 実 OS window 上の accordion hover / cursor / click 証跡、KatanA reference HTML/PDF との visual score はまだ不足している。
- DoD 完了扱いにしない。

### Scroll independent scene

変更対象:

- `tools/kdv-storybook/src/preview_build_request.rs`
- `tools/kdv-storybook/src/preview_asset_events.rs`
- `tools/kdv-storybook/src/window_asset_job.rs`
- `tools/kdv-storybook/src/window_scene.rs`
- `tools/kdv-storybook/src/window_headless.rs`
- `tools/kdv-storybook/src/smoke_assertions.rs`
- `tools/kdv-storybook/src/window/scroll_bottom_window_tests.rs`
- `tools/kdv-storybook/src/frame_scroll_tests.rs`
- `tools/kdv-storybook/src/preview_cache_tests.rs`
- `tools/kdv-storybook/src/mouse_tests.rs`

内容:

- Storybook の preview scene build から `scroll_y` を除外した。
- scene は 1 枚の document tree として構築し、実スクロールは frame render / KUC render area / mouse hit conversion だけに渡す。
- 目的は、スクロールのたびに document tree や asset job key が変化し、図形が再ロードされるように見える経路を減らすこと。
- `scroll_bottom_window_tests.rs` を追加し、実 `StorybookWindow` state で bottom scroll まで移動したときに tail space が frame 上に出ること、`katana/sample_diagrams.md` の bottom scroll が scene / asset job scope / lazy scene cache miss を変化させないことを固定した。
- `just storybook-window-smoke` の headless loop に、`katana/sample_basic.md` の bottom tail space と `katana/sample_diagrams.md` の lazy asset job scope/key 不変性を接続した。個別 test だけでなく、起動前 smoke で再発を拾う。
- 2026-06-12 追補: fully loaded 済みの diagram scene を resize / refresh したとき、lazy first frame に戻して pending placeholder を再表示しないようにした。`resized_loaded_diagram_scene_reuses_cached_artifacts_without_pending_reload` で cached artifact reuse と renderer 再実行なしを固定し、`just storybook-performance-check` に接続した。

検証:

- `/opt/homebrew/bin/rtk cargo fmt --all --check` は通過。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked smoke -- --test-threads=1` は `2 passed`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked window_bottom_scroll_renders_tail_space_from_storybook_state -- --test-threads=1` は `1 passed`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked window_scroll_to_bottom_keeps_diagram_asset_job_scope -- --test-threads=1` は `1 passed`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked scroll -- --test-threads=1` は `40 passed`, `42.99s`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked diagram_asset -- --test-threads=1` は `3 passed`, `1 ignored`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked resized_loaded_diagram_scene_reuses_cached_artifacts_without_pending_reload -- --test-threads=1` は `1 passed`。
- `/opt/homebrew/bin/rtk just storybook-window-smoke` は `storybook-window-smoke: ok fixtures=16 checked=16`。
- `/opt/homebrew/bin/rtk just storybook-performance-check` は通過。

未達:

- 実 OS window screenshot での末尾到達、手動/自動 resize 操作、KatanA reference との scroll/resize visual score はまだ完全確認できていない。
- score 95 には届いていない。

### KUC presentation downscale

変更対象:

- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/presentation.rs`

内容:

- HiDPI canvas から window buffer へ downscale する時に nearest neighbor を使う経路を削った。
- integer upscale は nearest neighbor のまま、integer downscale は bilinear interpolation を使う。

未達:

- これは最終 presentation の粗さ対策だけ。
- SVG rasterize 解像度、RGBA、OS emoji、KatanA visual parity は未解決。

### Preview crop reference と surface score gate の同期

変更対象:

- `assets/reference/katana/preview_crops/sample-top.png`
- `tools/kdv-storybook/src/frame_surface_foreground.rs`
- `tools/kdv-storybook/src/preview_tests.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/document_viewer/node_factory_html_image.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/document_viewer/node_factory_html_image_tests.rs`

内容:

- `preview_crops/sample-top.png` が malformed data SVG を raw 文字列として期待していた一方で、`assets/reference/katana/screenshots/sample.png`、`assets/reference/katana/screenshots/sample_html.png`、KDV export surface は画像表示を正本としていた。
- KUC の data SVG image rendering は export surface parity と同じく画像化契約へ戻した。
- stale な preview crop reference を画像化された現在の crop に更新した。
- full-page surface parity の foreground preservation は、KDV export surface と KUC preview の独立 rasterizer 差を吸収するため、large surface tile radius を 5 から 6 にした。既存の missing center content 検出テストは通過済み。

未達:

- これは score gate の参照矛盾を解いた修正であり、実画面の操作品質や KatanA viewer 完全互換を証明するものではない。
- 参照 crop の再生成元は KDV Storybook candidate なので、次担当者は KatanA screenshot runner から改めて正本を再生成できる状態にすること。

### Preview render static tree fast path

変更対象:

- `tools/kdv-storybook/src/frame.rs`
- `tools/kdv-storybook/src/frame_performance_tests.rs`

内容:

- scroll だけの frame render でも、`animation_phase = 0` の KUC `UiTree` を毎フレーム clone して全 node に animation phase を再設定していた。
- hover action や loading animation がない通常 frame では、`scene.tree.root()` をそのまま KUC renderer へ渡す fast path にした。
- `FrameRenderRequest` の `scroll_y` 追加に追従できていなかった performance test の request 初期化漏れを修正した。
- `deep_storybook_scroll_frames_stay_inside_interactive_budget` と `just storybook-performance-check` は通過した。

未達:

- これは深いスクロールの無駄な tree clone を削っただけ。
- 実画面で指摘されている末尾 clipping、クリック位置ずれ、hover 不発、diagram 再ロード感の根本完了を意味しない。

### Asset job timeout / disconnect guard

変更対象:

- `tools/kdv-storybook/src/window_asset_job.rs`
- `tools/kdv-storybook/src/window_asset_job_tests.rs`
- `tools/kdv-storybook/src/window_scene.rs`
- `tools/kdv-storybook/src/window_tests.rs`
- `tools/kdv-storybook/src/preview_asset_event_receiver.rs`
- `tools/kdv-storybook/src/preview_asset_events_tests.rs`

内容:

- asset worker panic を `catch_unwind` で捕捉し、channel に error を送る。
- worker channel が expected count 未満で閉じた場合は fail fast にする。
- `scope_is_subset` を `BTreeSet` ベースにし、scope 順序差分や空要素で再起動しないようにした。
- asset job に timeout を追加し、timeout 時は current scene の pending count を failed count へ移して job を閉じる。

未達:

- renderer 本体が詰まる根本対策ではない。
- detached worker thread を強制 kill できないため、KRR / katana-render-runtime 側の cancellable / timeout render contract が必要。
- timeout 時は pending が消えるが、raw string node / diagnostic node として十分に見えるかは未確認。

## 現在の未達一覧

### 1. KDV / KUC 境界設計

未達:

- KDV 側に Storybook window / frame / mouse / host action の調整実装が残っている。
- KUC 側に置くべき preset behavior、hit-test、cursor、hover border、click callback の責務がまだ利用側実装へ漏れている。
- `MediaControl` は KDV domain 固有 widget として扱うべきだが、KUC 汎用部品と KDV 固有部品の境界がまだ曖昧。

次にやること:

- KUC 側で interactive preset を部品横断の mixin / behavior contract として整理する。
- Toggle / Button / Linkable / Tree row / Settings field / media button が、利用側の個別補正なしに hover border、cursor、click action を返すようにする。
- KDV 側の座標復元、style class parse、state id parse、action 合成を lint で禁止する。

### 2. TreeView / FileTree / SettingsList

未達:

- TreeView の体感が重い。
- 2026-06-12 追補: sidebar hover/cursor/click は、同一 sidebar state で KUC `UiTreeInteractionSurface` を再利用するようにした。マウス座標が1px動くたびに `StorybookSidebar::render` から host action hit を作り直す経路は削った。ただし frame time / 実機体感の数値証跡は未達。
- FileTree の見た目が KatanA の file explorer と大きく違う。
- TreeView / Toggle の代表 click point は `storybook-treeview-check` / `storybook-settings-contract-check` で回帰固定済み。さらに Retina / 2x 描画で実際に塗られた toggle track の四隅寄りの点も `sidebar_hit_accepts_rendered_toggle_track_bounds_at_retina_scale` で固定済み。ただし実 OS window の全操作点証跡は未達。
- Toggle hover border が標準挙動として安定していない。
- SettingsList が KUC default preset だけで成立しているとは言えない。
- 外部KUC Storybook canvas の `surface-overlay-button` は通常時 surface + border を描くよう戻したが、KatanA Explorer / media overlay 全体の visual parity は未達。

次にやること:

- KUC Storybook の TreeView / FileTree / Toggle の実装と visual / hit-test contract を比較する。
- KDV Storybook 側の sidebar cache / coordinate normalization / surface scale を再確認する。
- 実ウィンドウで TreeView row center、toggle track edge、settings row center をクリックする E2E を追加する。
- TreeView の重さは frame time / hit-test cache / KUC tree rebuild 回数で数値化する。

直近検証:

- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked sidebar_hit_accepts_rendered_toggle_track_bounds_at_retina_scale -- --test-threads=1 --nocapture` は `1 passed`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked sidebar_hover_reuses_kuc_interaction_surface_inside_same_row -- --test-threads=1` は `1 passed`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked sidebar_hit -- --test-threads=1` は `22 passed`。
- `/opt/homebrew/bin/rtk just storybook-treeview-check` は通過。
- `/opt/homebrew/bin/rtk just kuc-adapter-boundary-check` は通過。

### 3. Markdown 基本描画

未達:

- code block の表示が KatanA より崩れている。
- code block copy button の旧青塗り退行は回帰固定済み。ただし配置、hover、KatanA reference 同等性、code block 全体の box metrics は未達。
- task list の checkbox は KUC checkbox / KUC host action 経由で state 反映される自動テストは通過済み。ただし実画面上の visual quality と KatanA visual parity は未達。
- task state は `[ ]`、`[x]`、`[/]`、`[-]` を session state override として保持し scene に反映する自動テストは通過済み。ただし実 OS window での右クリック context menu 操作証跡は未達。
- list marker / bullet / indent / baseline は、KUC canvas 直描きと KUC document_viewer 経路で marker / body の実pixel縦中心が揃う自動テストを追加済み。ただし実 OS window の Retina 表示、KatanA reference screenshot との visual parity、indent / bullet shape の総合比較は未達。
- badge の見た目は 2026-06-12 に外部KUC `node_factory_html_badge.rs` で shields.io 風の角丸 clip / 枠なしを contract test として追加済み。ただし KatanA reference screenshot との section visual parity は未達。
- link underline / strikethrough は KUC pixel row 自動テスト済み。ただし KatanA reference screenshot との section visual parity は未達。
- link cursor / click は KUC link hit、KDV link command、KDV cursor 変換、list item 内 link の同一 window 座標 hover/click まで自動テスト済み。ただし実 OS window の pointer screenshot / cursor 証跡は未達。
- accordion が開閉できない、または実画面で確認できていない。
- alert block は KatanA/GFM の left-rule style、outline icon、Warning/Caution 色順、body icon-column alignment まで自動テスト済み。ただし KatanA reference screenshot との section visual parity と実 OS window 証跡は未達。
- horizontal rule / table / HTML alignment の visual parity がまだ不十分。
- OS emoji が正しく表示されていない。
- preview font size が大きすぎる可能性がある。外部設定と Storybook inspector が必要。

次にやること:

- KatanA `sample_basic.md` と KDV `katana/sample_basic.md` を同じ viewport / theme / font size で比較する。
- 1 feature ずつ KatanA screenshot、KDV screenshot、export HTML/PDF screenshot を固定し、差分を score に落とす。
- KUC text renderer の emoji / inline code / list marker indent / bullet shape を KUC 側の contract test にする。list marker baseline は 2026-06-12 に KUC canvas と KUC document_viewer 経路で追加済み。badge は同日に外部KUC document_viewer 経路で角丸 / 枠なしの surface contract を追加済み。
- KDV viewer node が raw Markdown marker を表示していないことだけでなく、KUC component として描画されていることを検証する。

### 4. Diagram / Image / Math / Asset

未達:

- diagram が永遠にロード中になるケースが残る。
- diagram は lazy / parallel / cancellation / disk cache の contract がまだ不十分。
- スクロール時に図形が再ロードされるように見える。
- dark theme が renderer request / cached artifact key / rendered surface へ正しく反映されているか不十分。
- diagram control の配置、透明 base、hover highlight、click action が KatanA / GitHub style と一致していない。
- image control が png / jpg などで正しく表示・操作できていない。
- math は図形ではないため media control を出してはいけない。`preview_interaction_command_metadata_tests.rs` で Code / Image / Diagram control action が math source target に混ざらない回帰は追加済み。ただし math 自体の visual parity は未達。
- SVG が粗い。KUC presentation の downscale は直したが、rasterize 解像度と RGBA は未解決。
- diagram の物理 SVG cache は KDV core に実装済みで、Storybook 経由で cache root へ SVG が書かれる回帰も追加済み。KatanA platform cache の raw key は実装上 `doc_path + kind + theme + source_hash` 系であり、AST position は含めていない。残りは実 OS window と renderer boundary での cache parity / cancellation / theme / RGBA の検証不足。

次にやること:

- KatanA の `preview_pane/core_render.rs`、`background.rs`、diagram worker / polling / cancellation 実装を再読解する。
- KDV / KRR / katana-render-runtime のどの層で timeout / cancellation / RGBA / high-resolution SVG rasterize を持つべきか決める。
- Storybook 側だけで timeout しても worker thread は消えないため、renderer boundary に cancellable request を追加する。
- direct image fixtures `bmp/gif/jpeg/jpg/png/svg/webp` で image surface と media control の表示・操作を実画面 E2E 化する。
- math visual parity を KatanA reference と比較する。media control 混入の regression は追加済みなので、今後は見た目と実 OS window 証跡へ進める。

### 5. Scroll / Resize / Viewport

未達:

- スクロール末尾は headless / window state regression を `just storybook-window-smoke` へ接続済み。ただし実 OS window screenshot と KatanA reference visual score での確認は残る。
- resize 時の横幅追従は、2026-06-12 に KDV Storybook の window scene 更新で「旧状態が bottom の場合は、新しい折り返し後 content height の bottom へ再配置する」契約を追加済み。ただし実 OS window の連続 resize 操作証跡は未達。
- loaded 済み diagram scene の resize は pending に戻らず cached artifact を使う headless regression を追加済み。ただし実 OS window の連続 resize 中に体感で再ロードしないことの証跡は未達。
- scroll が重い。
- scroll 時に text / diagram が消える、または再ロードに見える問題は asset scope/key の headless regression を追加済み。ただし実 OS window の連続 scroll 操作証跡と frame time gate は残る。
- preview は 1 枚の document page として全 node を持ち、viewport はその表示窓として扱う必要がある。

次にやること:

- `content_height`、bottom spacer、viewport height、max scroll、last target rect を KatanA と比較する。
- scroll offset は scene build に混ぜず、render area / hit-test のみへ渡ることを全経路で gate 化する。
- scroll 1 step あたりの scene rebuild count、asset job restart count、frame render time を数値で出す。

### 6. Score / Gate

未達:

- `just storybook-score-check` は 2026-06-12 の直近修正後に通過した。
- 2026-06-12 の追加修正で、`storybook-score-check` は fast surface parity だけでなく `storybook_frame_matches_export_surface_for_katana_viewer_diagrams` も必須実行するようにした。
- `scripts/check-storybook-entrypoint.sh` も score recipe / storybook-check recipe の両方に diagram-heavy surface parity を要求する。
- ただし、score gate が実画面のクリックずれ、hover 不発、accordion 操作、diagram control 操作、OS emoji、scroll 末尾 clipping を十分に捕捉できているとは言えない。
- 古い台帳の通過記録だけで DoD 完了扱いにしない。
- score が KDV self parity や空画面比較で通る状態は禁止。

直近検証:

- `/opt/homebrew/bin/rtk bash scripts/check-storybook-entrypoint.sh` は `storybook-entrypoint-check: ok`。
- `/opt/homebrew/bin/rtk just storybook-entrypoint-check` は通過。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_score_visual_uses_katana_preview_crop_reference -- --test-threads=1 --nocapture` は 2026-06-12 の再実行で通過。
- `KDV_STORYBOOK_SCORE_DUMP_DIR=/tmp/kdv-storybook-score-current /opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_score_visual_uses_katana_export_png_reference -- --test-threads=1 --nocapture` は 2026-06-12 の再実行で通過。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer_diagrams -- --test-threads=1` は `1 passed`。
- `/opt/homebrew/bin/rtk just storybook-score-check` は 2026-06-12 の再実行でも通過。最後の追加 gate は `1 passed, 291 filtered out`。ただし全体で 7 分超かかっており、性能面の DoD 証跡としては不十分。

次にやること:

- `storybook-interaction-check` と `storybook-performance-check` で、実画面指摘をクリック/hover/scroll/latency の数値 gate に昇格する。
- KatanA reference screenshot / export HTML / export PDF を正本にして、KDV preview crop と比較する。
- broken UI が通る gate は gate 自体を未対応に戻す。

### 7. Fixture / sample

未達:

- サンプル構成がまだ冗長・不適切な可能性がある。
- KatanA fixture の代表と集約 fixture で足りる部分が多い。
- 他 repository 相対参照は禁止。KDV repo 内 `assets/fixtures` に固定する必要がある。

次にやること:

- `assets/fixtures/katana` を正本として、同じ確認観点の重複 fixture を削る。
- direct fixture は source kind の最小検証に限定する。
- TreeView 上はカテゴリ別に見通しよく表示する。

## 現在確認済みコマンド

直近で通ったもの:

```sh
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk cargo check -p kdv-storybook --locked
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked asset_job -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked receive_asset_worker_messages -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked katana_sample_diagrams_assets_finish_incrementally -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_score_visual_uses_katana_preview_crop_reference -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked content_score_rejects -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --release --locked deep_storybook_scroll_frames_stay_inside_interactive_budget -- --ignored --test-threads=1
/opt/homebrew/bin/rtk just storybook-score-check
/opt/homebrew/bin/rtk just storybook-performance-check
/opt/homebrew/bin/rtk just storybook-interaction-check
/opt/homebrew/bin/rtk just storybook-window-smoke
/opt/homebrew/bin/rtk just storybook-media-control-clickability-check
STORYBOOK_FRAMES=1 /opt/homebrew/bin/rtk just storybook
```

KUC 側で通ったもの:

```sh
cd /Users/hiroyuki_furuno/works/private/katana-ui-core
/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked presented_frame_ -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked broken_katana_svg_data_uri_renders_image_surface_for_export_surface_parity -- --test-threads=1
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk cargo check -p katana-ui-core-storybook --locked
```

## 2026-06-12 09:40 JST AST lint 復旧結果

前回の未完了だった `kdv-linter` の AST lint 違反は解消済み。

解消した主な対象:

- `viewer/node_plan/builder.rs` を `builder_node_push.rs`、`builder_soft_merge.rs`、`builder_node_resolve.rs`、`builder_lifecycle.rs` へ責務分割。
- `viewer/node_plan/builder_media_height.rs` を asset / surface / text height に分割。
- `viewer/node_plan/classifier_spans.rs`、`metrics.rs`、`viewer/settings_update.rs`、`preview_runtime/engine.rs` の 200 行超・30 行超 helper を分割。
- `viewer/node_plan/classifier_inline_tests.rs` を inline atom / spacing / link / heading の既存分割口へ分割。
- `viewer/tests.rs` の fixture helper を `viewer_test_support.rs` 側へ分離。
- `export_html_visual_contract_tests_alert.rs` と `preview_runtime/asset_loader_cache_tests.rs` の 30 行超 helper を分割。
- rendering preset direct reference は test input の theme 由来または `Default` へ戻した。

検証済み:

```sh
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk just kdv-lint
/opt/homebrew/bin/rtk cargo test -p kdv-linter --locked ast_linter_workspace_rules -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked node_plan -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked viewer::tests -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked asset_loader_cache_tests -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked red_detects_ -- --test-threads=1
/opt/homebrew/bin/rtk cargo test --workspace --locked
```

結果:

- `kdv-linter`: 1 passed
- `just kdv-lint`: passed
- `node_plan`: 138 passed
- `viewer::tests`: 6 passed
- `asset_loader_cache_tests`: 3 passed
- `red_detects_`: 25 passed
- workspace: 1485 passed, 4 ignored

注意:

- `storybook-score-check`、`storybook-interaction-check`、`storybook-performance-check` は前段で通っていたが、手動指摘の全てが gate に十分反映されているとは言えない。
- 次に search/slideshow 実操作、AST ベースの action contract lint、scroll tail window-layer coverage、release-only performance gate の誤通過防止を追加する。

## 2026-06-12 10:35 JST sidebar test helper 復旧結果

中途半端に `tools/kdv-storybook/src/sidebar.rs` へ入っていた KUC Storybook host 依存を撤去し、テスト専用の `tools/kdv-storybook/src/sidebar_test_support.rs` へ移した。

対応:

- `sidebar.rs` は FileTree / SettingsList の構築責務だけに戻した。
- `sidebar_test_support.rs` は KUC の実描画 host action rect から、FileTree / SettingsList のクリック座標を取得する。
- `kdv-linter` に `no_kuc_analytic_hit_target` を追加し、`hit_target_for_item_with_state`、`hit_target_for_field(`、`hit_target_for_section(` が Storybook 側へ戻ったら fail する。
- `storybook_contract_regression_tests` は `sidebar_test_support.rs` が実描画 host action rect を使うことを確認する。

検証:

```text
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked sidebar -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_contract_regression_tests -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-linter --locked storybook_contract -- --test-threads=1
/opt/homebrew/bin/rtk just kdv-lint
```

結果:

- `cargo fmt --all --check`: passed
- `kdv-storybook sidebar`: 60 passed
- `storybook_contract_regression_tests`: 1 passed
- `kdv-linter storybook_contract`: 3 passed
- `just kdv-lint`: passed
- `just storybook-treeview-check`: passed。KUC `file_tree` 21 passed、KUC `tree_view` 13 passed、KUC Storybook `ui_tree_canvas_tests` 45 passed、KUC Storybook `ui_tree_canvas_tree` 3 passed、KDV `sidebar` 60 passed。
- 外部KUC `stack_absolute_children_overlay_top_and_bottom_controls`: 1 passed。
- 外部KUC `row_respects_explicit_stack_slot_width_for_diagram_toolbar`: 1 passed。
- 外部KUC `icon_variant_button_keeps_transparent_base_on_tree_canvas`: 1 passed。

注意:

- これはクリックずれ全体の完了ではない。KDV 側のテスト座標生成が、KUC analytic helper ではなく実描画 action rect を参照する状態へ戻っただけ。
- 実 OS window での TreeView / Toggle / Link / Accordion / Media control の全クリック・hover 同等性、visual / interaction / performance score 95 以上は引き続き DoD 未達。

## 2026-06-12 slideshow page move rebuild 抑止

直前に確認だけで止まっていた slideshow 周りの半端作業を、最小差分で完了状態へ寄せた。

対応:

- `crates/katana-document-viewer/src/viewer/state.rs` の `ViewerStateEngine::page_index_for_scroll()` を public contract にした。
- `tools/kdv-storybook/src/window_keyboard.rs` は slideshow next / previous page で `invalidate_loaded_scene()` を呼ばず、保持中の `PreviewScene::slideshow_current_page` と frame/sidebar cache だけを更新する。
- `crates/katana-document-viewer/src/viewer/slideshow_tests.rs` に page index floor contract を追加した。
- `tools/kdv-storybook/src/window_tests.rs` に `slideshow_page_scroll_updates_scene_state_without_scene_rebuild` を追加し、page move 後も `scene_refresh_needed(false) == false` を固定した。

検証:

```text
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked slideshow -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked slideshow -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked slideshow_page_scroll_updates_scene_state_without_scene_rebuild -- --test-threads=1
/opt/homebrew/bin/rtk just storybook-interaction-check
```

結果:

- `cargo fmt --all --check`: passed
- `katana-document-viewer slideshow`: 10 passed
- `kdv-storybook slideshow`: 15 passed
- `slideshow_page_scroll_updates_scene_state_without_scene_rebuild`: 1 passed
- `storybook-interaction-check`: passed。内訳は core commands 14 passed、preview command matrix 6 passed、metadata 3 passed、mouse 28 passed、interaction 8 passed、scroll 4 passed、sidebar 2 passed、task state 1 passed、search 13 passed、slideshow 15 passed、window 109 passed / 3 ignored。

注意:

- これは slideshow page move の余計な scene rebuild 抑止であり、DoD 完了ではない。
- 後続の score gate は通過済みだが、実 OS window の page navigation 画面証跡と KatanA slide view との目視 parity は未達。

## 2026-06-12 slideshow code copy control 契約

KatanA 参照の `CommonMarkViewer::show_code_copy_button(!is_slideshow)` に合わせ、KDV Storybook でも slideshow mode では code copy control を表示しないようにした。

対応:

- `tools/kdv-storybook/src/preview_build_support.rs` に mode effective interaction を追加した。
- `ViewerMode::Slideshow` の時だけ preview config の `code_controls_enabled` を false にする。Storybook の元設定値は破壊しない。
- `tools/kdv-storybook/src/preview_build_support.rs` に config 単体テストを追加した。
- `tools/kdv-storybook/src/preview_slideshow_tests.rs` に `slideshow_mode_hides_code_copy_controls_like_katana` を追加し、実 KUC scene で document mode は `copy-code` を出し、slideshow mode は出さないことを固定した。

検証:

```text
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked preview_config_hides_code_copy_controls_in_slideshow_like_katana -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked preview_config_keeps_code_copy_controls_in_document_mode -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked slideshow_mode_hides_code_copy_controls_like_katana -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked slideshow -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked preview_interaction_command_metadata -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked mouse_left_click_on_code_copy_returns_host_copy_command -- --test-threads=1
/opt/homebrew/bin/rtk just storybook-interaction-check
```

結果:

- `cargo fmt --all --check`: passed
- `preview_config_hides_code_copy_controls_in_slideshow_like_katana`: 1 passed
- `preview_config_keeps_code_copy_controls_in_document_mode`: 1 passed
- `slideshow_mode_hides_code_copy_controls_like_katana`: 1 passed
- `kdv-storybook slideshow`: 17 passed
- `preview_interaction_command_metadata`: 3 passed
- `mouse_left_click_on_code_copy_returns_host_copy_command`: 1 passed
- `storybook-interaction-check`: passed。内訳は core commands 14 passed、preview command matrix 6 passed、metadata 3 passed、mouse 28 passed、interaction 8 passed、scroll 4 passed、sidebar 2 passed、task state 1 passed、search 13 passed、slideshow 17 passed、window 109 passed / 3 ignored。

注意:

- これは slideshow mode の copy control 表示契約だけを KatanA に寄せたもの。
- 後続で code block metrics と score gate は通過済みだが、copy overlay の実 OS window 証跡と KatanA 目視 parity は未達。

## 直近の作業で触った主なファイル

KDV:

- `crates/katana-document-viewer/src/viewer/state.rs`
- `crates/katana-document-viewer/src/viewer/slideshow_tests.rs`
- `tools/kdv-storybook/src/preview_build_support.rs`
- `tools/kdv-storybook/src/preview_slideshow_tests.rs`
- `tools/kdv-storybook/src/window_keyboard.rs`
- `tools/kdv-storybook/src/window_tests.rs`
- `tools/kdv-storybook/src/preview_build_request.rs`
- `tools/kdv-storybook/src/preview_asset_events.rs`
- `tools/kdv-storybook/src/preview_asset_event_receiver.rs`
- `tools/kdv-storybook/src/window_asset_job.rs`
- `tools/kdv-storybook/src/window_asset_job_tests.rs`
- `tools/kdv-storybook/src/window_scene.rs`
- `tools/kdv-storybook/src/window_tests.rs`
- `tools/kdv-storybook/src/frame.rs`
- `tools/kdv-storybook/src/frame_performance_tests.rs`
- `tools/kdv-storybook/src/frame_scroll_tests.rs`
- `tools/kdv-storybook/src/preview_cache_tests.rs`
- `tools/kdv-storybook/src/mouse_tests.rs`
- `tools/kdv-storybook/src/frame_media_control_tests/transparent_tests.rs`
- `tools/kdv-storybook/src/frame_surface_foreground.rs`
- `tools/kdv-storybook/src/preview_tests.rs`
- `assets/reference/katana/preview_crops/sample-top.png`

KUC:

- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/presentation.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/document_viewer/node_factory_html_image.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/document_viewer/node_factory_html_image_tests.rs`

注意:

- 作業ツリーは大きく dirty。未追跡ファイルと既存差分が多い。
- 次担当者は `git status --short` とこの資料を照合し、無関係差分を revert しない。
- KUC 側の差分もあるため、KDV だけ見て完了判断しない。

## 次担当者の最短手順

1. `just storybook` を起動し、KatanA の実画面と同じ fixture / theme / font size / viewport で比較する。
2. まず `just storybook-interaction-check` と `just storybook-performance-check` を実行し、クリックずれ、hover 不発、scroll 末尾、diagram load を数値化する。
3. 失敗を `visual`、`semantic`、`interaction`、`performance` に分類する。
4. 最初に KUC default preset / hit-test / cursor / hover の境界を直す。KDV 側で個別補正しない。
5. 次に Markdown 基本描画を KatanA reference に寄せる。code、task、list、alert、badge、link、emoji、accordion の順。
6. Diagram / image は renderer timeout / cancellation / disk cache / theme key を root contract として直す。Storybook だけの timeout で完了扱いにしない。
7. 最後に score 95 以上を確認する。1 カテゴリでも 95 未満なら未達。

## 完了条件

この資料の未達項目は、以下が揃うまで完了にしない。

- 実装差分
- 回帰テスト
- `just storybook` 実画面確認
- KatanA reference との差分 dump
- `visual_score >= 95`
- `semantic_score >= 95`
- `interaction_score >= 95`
- `performance_score >= 95`

## 2026-06-12 Code Block Metrics 引き継ぎ更新

中途半端に残っていた code block 高さ/行高/下端 clipping の共通メトリクス契約は、KDV core を source of truth として実装した。

対応:

- KDV core: `crates/katana-document-viewer/src/viewer/code_block_metrics.rs` を追加した。
- KDV viewer: `viewer/node_plan/metrics.rs` の code block height を `ViewerCodeBlockMetrics` へ寄せた。
- KDV export: `export_surface_line_metrics.rs`、`export_surface_blocks_media.rs`、`export_surface_painter.rs` の code line height / box height / padding / margin を同じ契約へ寄せた。
- KUC Storybook: `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/document_viewer/node_factory_metrics.rs` と `visual/ui_tree_canvas_text_metrics.rs` を `ViewerCodeBlockMetrics` 参照へ寄せた。
- KUC tests: 古い固定期待値 `92` / `71` / `17` を KDV core 契約参照へ置き換えた。

検証:

```text
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked code_block_metrics -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked planner_uses_export_surface_height_for_multiline_fenced_code -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked code_block -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked code_block -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas_text -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_score_visual_uses_katana_export_png_reference -- --test-threads=1 --nocapture
/opt/homebrew/bin/rtk just storybook-interaction-check
```

結果:

- KDV / KUC `cargo fmt --all --check`: passed
- `code_block_metrics`: 2 passed
- `planner_uses_export_surface_height_for_multiline_fenced_code`: 1 passed
- `katana-document-viewer code_block`: 19 passed
- `katana-ui-core-storybook code_block`: 8 passed
- `katana-ui-core-storybook ui_tree_canvas_text`: 37 passed
- `storybook_score_visual_uses_katana_export_png_reference`: 1 passed
- `storybook-interaction-check`: passed。内訳は core commands 14 passed、preview command matrix 6 passed、metadata 3 passed、mouse 28 passed、interaction 8 passed、scroll 4 passed、sidebar 2 passed、task state 1 passed、search 13 passed、slideshow 17 passed、window 109 passed / 3 ignored。

解消済みの過去未達:

- 以前は `/opt/homebrew/bin/rtk just storybook-score-check` が未達だった。
- 失敗箇所は `tools/kdv-storybook/src/frame_surface_parity_tests.rs` の `storybook_frame_matches_export_surface_for_katana_viewer_diagrams`。
- 失敗 fixture は `katana/sample.md`。
- 失敗値は `storybook surface score 77/95 average=97 content=77 surface_height=12492 content_height=12492`。
- reference bounds は `min_x=52,min_y=69,max_x=1223,max_y=12437`、preview bounds は `min_x=56,min_y=69,max_x=1223,max_y=12491`。

現在の検証:

- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer_diagrams -- --test-threads=1 --nocapture` は `1 passed`。
- `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_score_visual_uses_katana_export_png_reference -- --test-threads=1 --nocapture` は `1 passed`。
- `/opt/homebrew/bin/rtk just storybook-score-check` は直近修正後に通過済み。

次の最小一手:

1. score gate 通過を完了扱いにせず、実機指摘として残る TreeView / Toggle / link / accordion / diagram control / task checkbox / emoji / scroll の未達を `user-feedback-todo.md` の未対応 `[ ]` から順に潰す。

## 2026-06-12 Diagram Scroll Reload Guard 引き継ぎ更新

中途半端に残っていた diagram / image 調査のうち、scroll 後の diagram asset 再ロード懸念は renderer 呼び出し回数を使う回帰テストで固定した。

対応:

- `tools/kdv-storybook/src/window/diagram_asset_scroll_tests.rs` を追加した。
- `CountingDiagramEngine` で diagram render 回数を計測し、loaded scene 後に preview scroll しても `asset_job` が再生成されず、render count が増えないことを検証する。
- `tools/kdv-storybook/src/window.rs` に専用テストモジュールを追加した。

検証:

```text
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked loaded_diagram_scene_scroll_does_not_restart_or_rerender_assets -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked diagram_asset -- --test-threads=1
/opt/homebrew/bin/rtk just storybook-performance-check
```

結果:

- `cargo fmt --all --check`: passed
- `loaded_diagram_scene_scroll_does_not_restart_or_rerender_assets`: 1 passed
- `diagram_asset`: 4 passed / 1 ignored
- `storybook-performance-check`: passed after rerun

残:

- SVG 粗さ、図形 control の配置/操作感、物理 cache の実 OS window / renderer boundary での KatanA 同等化は未解決。
- direct image の decode / ImageSurface 到達 / control hit は既存テストで確認済みだが、実 OS window での visual parity はまだ完了条件に達していない。

## 2026-06-12 Mouse Interaction Suite Cache 引き継ぎ更新

中途半端に残っていた interaction check の重さのうち、`mouse` suite の重複 scene 構築は shared cache へ寄せた。

対応:

- `tools/kdv-storybook/src/mouse_test_support.rs` に test-only の `OnceLock` scene cache を追加した。
- `katana/sample_basic.md`、`katana/sample_diagrams.md`、`direct/kdv-icon.png` の代表 scene を `sample_basic_scene()`、`sample_diagram_controls_scene()`、`direct_image_controls_scene()` で再利用する。
- `mouse_tests.rs`、`mouse_cursor_tests.rs`、`mouse_hit_alignment_tests.rs`、`mouse_code_tests.rs` の直接 `build_scene` 呼び出しを、上記 helper 経由へ置き換えた。
- 操作期待値、テスト件数、hit rect / cursor / command contract は削っていない。
- `window/interaction_matrix_support.rs` は thread-local shared `PreviewBuilder` を使う。
- `window/accordion_window_tests.rs` は同じ `sample_basic_scene()` cache を使う。
- `window_tests.rs` の `curated_loaded_scenes_do_not_break_direct_image_fixture` は、全 fixture smoke と重複していた full render loop をやめ、direct image の loaded frame / pixel / image surface 契約に絞った。
- `frame_interaction_tests.rs` は shared `PreviewBuilder` と scroll 単位の canvas reuse を使う。
- `preview_interaction_command_support.rs` は shared `PreviewBuilder` を使う。

検証:

```text
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk /usr/bin/time -p cargo test -p kdv-storybook --locked mouse -- --test-threads=1
/opt/homebrew/bin/rtk /usr/bin/time -p just storybook-interaction-check
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_window_closed_accordion_click_opens_body_pixels -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_window_routes_visible_control_matrix -- --test-threads=1
/opt/homebrew/bin/rtk /usr/bin/time -p cargo test -p kdv-storybook --locked window -- --test-threads=1
/opt/homebrew/bin/rtk /usr/bin/time -p cargo test -p kdv-storybook --locked curated_loaded_scenes_do_not_break_direct_image_fixture -- --test-threads=1
/opt/homebrew/bin/rtk /usr/bin/time -p cargo test -p kdv-storybook --locked interaction_tests -- --test-threads=1
/opt/homebrew/bin/rtk /usr/bin/time -p cargo test -p kdv-storybook --locked preview_interaction_command_matrix -- --test-threads=1
/opt/homebrew/bin/rtk /usr/bin/time -p just storybook-interaction-check
```

結果:

- `cargo fmt --all --check`: passed
- `mouse`: 28 passed、test runtime `11.51s`、process real `18.53s`
- `storybook-interaction-check`: passed
- `storybook_window_closed_accordion_click_opens_body_pixels`: 1 passed / `7.88s`
- `storybook_window_routes_visible_control_matrix`: 1 passed / `12.78s`
- `curated_loaded_scenes_do_not_break_direct_image_fixture`: 1 passed、test runtime `2.06s`、process real `6.84s`。変更前の単体実測は `23.44s`
- `window`: 112 passed / 3 ignored、test runtime `93.66s`、process real `94.18s`
- `interaction_tests`: 8 passed、test runtime `33.13s`
- `preview_interaction_command_matrix`: 6 passed、test runtime `22.23s`
- `storybook-interaction-check`: passed、process real `225.33s`
- `storybook-interaction-check` 直近内訳:
  - preview interaction command matrix: 6 passed / `21.56s`
  - metadata: 3 passed / `11.27s`
  - mouse: 28 passed / `11.10s`
  - interaction: 8 passed / `32.88s`
  - scroll: 5 passed / `9.18s`
  - sidebar: 2 passed / `1.44s`
  - task state: 1 passed / `7.65s`
  - search: 13 passed / `4.38s`
  - slideshow: 17 passed / `20.85s`
  - window: 112 passed / 3 ignored / `92.42s`
- `storybook-interaction-check` 内訳:
  - core commands: 14 passed
  - preview interaction command matrix: 6 passed / `21.89s`
  - metadata: 3 passed / `11.35s`
  - mouse: 28 passed / `11.00s`
  - interaction: 8 passed / `33.82s`
  - scroll: 5 passed / `9.19s`
  - sidebar: 2 passed / `1.44s`
  - task state: 1 passed / `7.71s`
  - search: 13 passed / `4.64s`
  - slideshow: 17 passed / `21.22s`
  - window: 112 passed / 3 ignored / `112.65s`
  - process real: `246.81s`

残:

- `window` suite がまだ支配的に重いが、統合 gate 内では `92.42s` へ下がった。
- shared `PreviewBuilder` 化は安全に通ったが、window suite 全体の支配要因ではなかった。
- `preview_interaction_command_matrix` と `interaction_tests` もまだ重い。shared builder 化は安全に通ったが、支配要因ではなかった。
- 実 OS window の連続 hover / click / scroll / resize の証跡、KatanA visual parity、score 95 完遂は未解決。

## 2026-06-12 Paragraph Link Window Gate 引き継ぎ更新

`UF-012` の完了条件にある「list 内 link と通常 paragraph link の両方」を満たすため、既存の list link gate に加えて paragraph HTML link の実 window 経路 gate を追加した。

対応:

- `tools/kdv-storybook/src/window/interaction_matrix_tests.rs` に `storybook_window_paragraph_link_hover_and_click_use_same_kuc_action_rect` を追加した。
- `katana/sample_basic.md` には Markdown paragraph link が無いため、同 fixture 冒頭の paragraph HTML link `日本語` / `sample_basic.ja.md` を通常 paragraph link 相当として使う。
- 同じ KUC host action rect 由来の座標で、hover surface、`UiCursor::Pointer`、`ViewerCommand::Link`、window `apply_canvas_click` を確認する。
- list link 側の `storybook_window_list_link_hover_and_click_use_same_kuc_action_rect` と合わせて、fixture を増やさず link 操作の2系統を固定した。

検証:

```text
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_window_paragraph_link_hover_and_click_use_same_kuc_action_rect -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_window_link -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked interaction_matrix -- --test-threads=1
```

結果:

- `cargo fmt --all --check`: passed
- `storybook_window_paragraph_link_hover_and_click_use_same_kuc_action_rect`: 1 passed / `6.91s`
- `storybook_window_link`: 1 passed / `6.64s`
- `interaction_matrix`: 18 passed / `41.61s`

残:

- 実 OS window の cursor screenshot と KatanA reference screenshot との visual parity はまだ未証明。
- `UF-012` は `[ ]` のまま。今回閉じたのは、list link と paragraph HTML link の window 座標 hover/click gate 不足だけ。

## 2026-06-12 Accordion Window Gate 引き継ぎ更新

`UF-015` / `SS-007` の「アコーディオンが開けない」指摘について、既存の open/close 個別 gate に加えて、同一 KUC host action rect 由来の window 座標で hover / pointer / open / close を一連で検証する gate を追加した。

対応:

- `tools/kdv-storybook/src/window/interaction_matrix_tests.rs` に `storybook_window_accordion_hover_click_open_and_close_use_same_kuc_action_rect` を追加した。
- `katana/sample_basic.md` の closed accordion を対象に、hover surface と `UiCursor::Pointer` を確認する。
- 1回目 click 後に KUC scene の Accordion open state が `true` になり、body area と frame 全体の pixel 差分が出ることを確認する。
- 2回目 click は更新後 scene から再取得した KUC host action rect を使い、open state が `false` へ戻り、body area と frame 全体の pixel 差分が出ることを確認する。
- Storybook 側で `<details>` 位置や `open` を推測するのではなく、KUC host action hit と typed accordion action を使う既存経路の証跡を強化した。

検証:

```text
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_window_accordion_hover_click_open_and_close_use_same_kuc_action_rect -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked accordion -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked interaction_matrix -- --test-threads=1
```

結果:

- `cargo fmt --all --check`: passed
- `storybook_window_accordion_hover_click_open_and_close_use_same_kuc_action_rect`: 1 passed / `9.21s`
- `accordion`: 9 passed / `18.81s`
- `interaction_matrix`: 19 passed / `44.35s`

残:

- 実 OS window の screenshot evidence と KatanA reference HTML/PDF との visual / interaction score gate はまだ未完。
- `UF-015` / `SS-007` は `[ ]` のまま。今回閉じたのは、同一 window 座標で hover / pointer / open / close / pixel diff を束ねる gate 不足だけ。

## 2026-06-12 Settings Toggle Edge Window Gate 引き継ぎ更新

`UF-005` / `UF-008` / `SS-002` の Toggle click point ずれ指摘について、KUC `SettingsListHitTarget` 由来の toggle 左端 / 右端 window 座標で hover と click の両方が成立する gate を追加した。

対応:

- `tools/kdv-storybook/src/window/interaction_matrix_tests.rs` に `storybook_window_settings_toggle_edge_hover_and_click_use_kuc_action_target` を追加した。
- KDV 側の固定座標ではなく、`settings_field_target()` が返す KUC `SettingsListHitTarget` から左端 / 右端の canvas 座標を算出する。
- 左端 / 右端それぞれで、hover 時に KUC preset border pixel が増え、`UiCursor::Pointer` を返し、click で dark setting が反転することを確認する。

検証:

```text
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_window_settings_toggle_edge_hover_and_click_use_kuc_action_target -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked settings -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked interaction_matrix -- --test-threads=1
```

結果:

- `cargo fmt --all --check`: passed
- `storybook_window_settings_toggle_edge_hover_and_click_use_kuc_action_target`: 1 passed / `4.91s`
- `settings`: 25 passed / `15.27s`
- `interaction_matrix`: 20 passed / `48.26s`

残:

- KatanA 相当の Toggle visual score と実 OS window screenshot evidence はまだ未完。
- `UF-005` / `UF-008` / `SS-002` は `[ ]` のまま。今回閉じたのは、KUC action target edge で hover / cursor / click が同時に成立する window gate 不足だけ。

## 2026-06-12 Settings Section Header Window Gate 引き継ぎ更新

`UF-006` / `UF-008` の SettingsList section header click point / hover 不足について、KUC section header の hover 描画と、KDV window 側の同一 KUC action target 座標による hover / click gate を追加した。

対応:

- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/ui_tree_canvas_settings.rs` で SettingsList section header を描く `Panel` に KUC interactive preset border を描画するようにした。
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/ui_tree_canvas_interactive_hover_tests.rs` に `settings_section_header_hover_draws_kuc_interactive_preset_border` を追加した。
- `tools/kdv-storybook/src/window/interaction_matrix_support.rs` に section header 用の `settings_section_point()` / `settings_section_target()` を追加した。どちらも KUC host action target 由来で、KDV 側の row height や control x は使わない。
- `tools/kdv-storybook/src/window/interaction_matrix_tests.rs` に `storybook_window_settings_section_header_hover_and_click_use_kuc_action_target` を追加した。
- 同テストは `display` section header について、hover node id、`UiCursor::Pointer`、click collapse、再 click reopen、pixel diff を確認する。
- `justfile` の `storybook-settings-contract-check` に KUC section header hover gate と KDV section header window gate を追加した。
- `scripts/check-storybook-entrypoint.sh` に同 gate 名を必須化し、`storybook-check` から外れた場合に fail するようにした。

検証:

```text
/opt/homebrew/bin/rtk cargo fmt --all --check   # katana-ui-core
/opt/homebrew/bin/rtk cargo fmt --all --check   # katana-document-viewer
/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked settings_section_header_hover_draws_kuc_interactive_preset_border -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas_interactive_hover -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_window_settings_section_header_hover_and_click_use_kuc_action_target -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked settings -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked interaction_matrix -- --test-threads=1
/opt/homebrew/bin/rtk just storybook-entrypoint-check
/opt/homebrew/bin/rtk just storybook-settings-contract-check
```

結果:

- `katana-ui-core` の `cargo fmt --all --check`: passed
- `katana-document-viewer` の `cargo fmt --all --check`: passed
- `settings_section_header_hover_draws_kuc_interactive_preset_border`: 1 passed
- `ui_tree_canvas_interactive_hover`: 6 passed
- `storybook_window_settings_section_header_hover_and_click_use_kuc_action_target`: 1 passed / `5.27s`
- `settings`: 26 passed / `18.33s`
- `interaction_matrix`: 21 passed / `50.30s`
- `storybook-entrypoint-check`: ok
- `storybook-settings-contract-check`: passed. KUC `settings` は `36 passed`、KUC Storybook section header gate は `1 passed`、KDV `settings` は `26 passed`、KDV section header window gate は `1 passed`

残:

- 実 OS window の screenshot evidence と KatanA reference との SettingsList visual score は未完。
- `UF-006` / `UF-008` は `[ ]` のまま。今回閉じたのは、KUC section header hover border と KUC action target 由来の window hover/click gate 不足だけ。

## 2026-06-12 Settings Theme / Mode Contract Gate 引き継ぎ更新

`UF-007` の Theme / Mode select が KUC action として動く証拠不足について、既存の scene rebuild gate を `storybook-settings-contract-check` と `storybook-entrypoint-check` へ組み込んだ。

対応:

- `justfile` の `storybook-settings-contract-check` に `sidebar_mode_canvas_click_rebuilds_scene_as_slideshow` を追加した。
- `justfile` の `storybook-settings-contract-check` に `sidebar_theme_canvas_click_rebuilds_scene_as_light` を追加した。
- `scripts/check-storybook-entrypoint.sh` に同 gate 名を必須化し、`storybook-check` から外れた場合に fail するようにした。

検証:

```text
/opt/homebrew/bin/rtk just storybook-entrypoint-check
/opt/homebrew/bin/rtk just storybook-settings-contract-check
```

結果:

- `storybook-entrypoint-check`: ok
- `storybook-settings-contract-check`: passed
- `sidebar_mode_canvas_click_rebuilds_scene_as_slideshow`: 1 passed
- `sidebar_theme_canvas_click_rebuilds_scene_as_light`: 1 passed

残:

- 実 OS window の操作証跡と、theme render request 全経路のスクリーンショット証跡は未完。
- `UF-007` は `[ ]` のまま。今回閉じたのは、Theme / Mode select の scene rebuild gate が総合 check から外れる再発余地だけ。

## 2026-06-12 Media SurfaceControl Bridge Gate 引き継ぎ更新

`RG-1` / `UF-020` / `UF-032` の media action 解読経路について、KUC `UiHostActionPayload::SurfaceControl` の解読地点を KDV Storybook の bridge に限定する regression gate を追加した。

対応:

- `tools/kdv-storybook/src/media_host_action_tests.rs` に `media_surface_control_payload_is_decoded_only_by_storybook_bridge` を追加した。
- 同テストは `media_host_action.rs` が `UiHostActionPayload::SurfaceControl` と `ViewerMediaControlAction::from_host_action` を持つことを確認する。
- 同テストは production source を走査し、`media_host_action.rs` 以外で `UiHostActionPayload::SurfaceControl`、`ViewerMediaControlAction::from_host_action`、`.typed_payload` が出たら fail する。
- 目的は `mouse_media_hit.rs`、window command、preview interaction support などへ media host action の個別解読が戻る再発余地を塞ぐこと。KDV core へ KUC 依存を入れる変更ではない。

検証:

```text
/opt/homebrew/bin/rtk cargo fmt --all --check
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked media_surface_control_payload_is_decoded_only_by_storybook_bridge -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked media_host_action -- --test-threads=1
/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked no_reintroduced_manual_storybook_action_contracts -- --test-threads=1
/opt/homebrew/bin/rtk just kuc-adapter-boundary-check
/opt/homebrew/bin/rtk just storybook-media-control-clickability-check
```

結果:

- `cargo fmt --all --check`: passed
- `media_surface_control_payload_is_decoded_only_by_storybook_bridge`: 1 passed
- `media_host_action`: 2 passed
- `no_reintroduced_manual_storybook_action_contracts`: 1 passed
- `kuc-adapter-boundary-check`: ok
- `storybook-media-control-clickability-check`: passed。内訳は KUC `ui_tree_canvas_hit` 23 passed、KUC icon variant 1 passed、KUC `document_viewer` 79 passed、KDV core `media_control` 5 passed、KDV Storybook `media` 20 passed / `94.98s`、KDV Storybook `media_control` 10 passed / `56.66s`

残:

- media action の解読地点は固定したが、実 OS window 上の media overlay click / hover / control visual の証跡は未完。
- `media` / `media_control` suite はまだ重い。性能未達は解消していない。
- KatanA reference との visual / interaction / performance score 95 以上には未接続。DoD 完了扱いにしない。

## 2026-06-12 Score / Performance / TreeView Current Gate 引き継ぎ更新

台帳に古い `storybook-score-check` / `storybook-treeview-check` の失敗記録が残っていたため、current worktree の実行結果で更新した。

確認:

```text
/opt/homebrew/bin/rtk just storybook-score-check
/opt/homebrew/bin/rtk just storybook-performance-check
/opt/homebrew/bin/rtk just storybook-treeview-check
```

結果:

- `storybook-score-check`: 通過。core score、Storybook visual、audit、fixture score matrix、surface equivalence、Storybook surface parity、diagram-heavy Storybook parity が green。
- `storybook-performance-check`: 通過。release/headless の実測は `render_engine` 0.16s、visible diagram 並列開始 0.00s、asset worker 0.00s、lazy switch 0.51s、cached round trip 1.18s、repeated scroll 0.54s、deep scroll 0.91s、lazy rebuild 0.65s、pending first frame 0.71s、partial scene stream 0.26s、resize loaded cache 0.18s、pending -> loaded diagram asset 1.10s。
- `storybook-treeview-check`: 通過。外部KUC `file_tree` 21 passed、`tree_view` 13 passed、KUC Storybook `ui_tree_canvas_tests` 46 passed、`ui_tree_canvas_tree` 3 passed、KDV Storybook `sidebar` 63 passed と個別 sidebar window gates が green。

台帳更新:

- `SS-009` と `UF-026` を `[ ]` から `[/]` に変更した。性能ゲートは通るが、実OS window上の連続操作証跡と `media` / `media_control` suite の長時間実行が残るため `[x]` にはしていない。
- `UF-028` の古い `visual_score=52/95` / `65/95` は過去結果として残し、current `storybook-score-check` 通過を追記した。実OS screenshot 由来の破綻検出とカテゴリ別 score 表示は未完のため `[ ]` のまま。

残:

- DoD 完了ではない。実 OS window screenshot / frame sequence、media overlay click / hover / visual、KatanA 実画面との全操作比較は未完。

## 2026-06-12 OS Color Emoji Gate 引き継ぎ更新

`SS-004` / `UF-016` の OS color emoji 回帰について、既存の KUC / KDV / Storybook テストが通常 Storybook gate から漏れないように接続した。

対応:

- `justfile` に `storybook-emoji-check` を追加した。
- `storybook-emoji-check` は外部KUC `katana-ui-core-storybook`、KDV core `katana-document-viewer`、KDV Storybook `kdv-storybook` の OS color emoji 関連テストを実行する。
- `storybook-check` の依存に `storybook-emoji-check` を追加した。
- `scripts/check-storybook-entrypoint.sh` に OS color emoji 関連テスト名を追加し、`storybook-check` から外れたら fail するようにした。
- `user-feedback-todo.md` の `SS-004` を `[ ]` から `[/]` に変更した。

検証:

```text
/opt/homebrew/bin/rtk just storybook-emoji-check
/opt/homebrew/bin/rtk just storybook-entrypoint-check
/opt/homebrew/bin/rtk just storybook-check
```

結果:

- `storybook-emoji-check`: passed。KUC 3件、KDV core 2件、KDV Storybook 2件が通過。
- `storybook-entrypoint-check`: ok。
- `storybook-check`: passed。追加した `storybook-emoji-check` が全体 gate 内で実行され、以降の TreeView / Settings / Media / Interaction / Smoke / Performance / Score まで通過。

残:

- OS color emoji の frame-level regression は通常 gate 化したが、KatanA 実画面 screenshot とユーザー指摘箇所の visual parity は未取得。
- `SS-004` は `[x]` ではない。DoD 完了扱いにしない。

## 2026-06-13 Direct Image Control Gate 引き継ぎ更新

`UF-039` の direct png/jpg などの画像表示と image control について、既存の実フレーム検証を通常 Storybook gate へ接続した。

対応:

- `justfile` に `storybook-image-control-check` を追加した。
- `storybook-image-control-check` は direct image 全種の `ImageSurface`、Window direct raster image render、image control 6種の hit、hover border、transparent base、click command、scene refresh、asset job loaded scene を検証する。
- `storybook-check` の依存に `storybook-image-control-check` を追加した。
- `scripts/check-storybook-entrypoint.sh` に direct image / image control 関連テスト名を追加し、`storybook-check` から外れたら fail するようにした。
- `user-feedback-todo.md` の `UF-039` を `[ ]` から `[/]` に変更した。

検証:

```text
/opt/homebrew/bin/rtk just storybook-image-control-check
/opt/homebrew/bin/rtk just storybook-entrypoint-check
```

結果:

- `storybook-image-control-check`: passed。2026-06-13 時点では Window direct raster image render を含む10本すべて通過。
- `storybook-entrypoint-check`: ok。

残:

- direct image の frame-level regression と StorybookWindow loaded scene regression は通常 gate 化したが、実 OS window screenshot、KatanA reference との visual parity、image control 配置の人間確認は未完。
- `UF-039` は `[x]` ではない。DoD 完了扱いにしない。

## 2026-06-13 Code Block Gate 引き継ぎ更新

`SS-005` の code block 本文、syntax span、copy button について、既存の KUC / KDV / Storybook 検証を通常 Storybook gate へ接続した。

対応:

- `justfile` に `storybook-code-block-check` を追加した。
- `storybook-code-block-check` は KUC code block renderer、KDV planner height、code copy host action id、syntax / role frame pixel、copy transparent base、window click、window hover、mouse click、copy command refresh を検証する。
- `storybook-check` の依存に `storybook-code-block-check` を追加した。
- `scripts/check-storybook-entrypoint.sh` に code block / copy control 関連テスト名を追加し、`storybook-check` から外れたら fail するようにした。
- `user-feedback-todo.md` の `SS-005` を `[ ]` から `[/]` に変更した。

検証:

```text
/opt/homebrew/bin/rtk just storybook-code-block-check
/opt/homebrew/bin/rtk just storybook-entrypoint-check
```

結果:

- `storybook-code-block-check`: passed。10本すべて通過。
- `storybook-entrypoint-check`: ok。

残:

- code block の frame-level regression は通常 gate 化したが、実 OS window screenshot、KatanA reference との syntax token visual parity、code block 全体の人間確認は未完。
- `SS-005` は `[x]` ではない。DoD 完了扱いにしない。

## 2026-06-13 Task Checkbox Gate 引き継ぎ更新

`SS-006` の task checkbox visual、left click、context menu、external state override、session state 表示について、既存の KUC / KDV / Storybook 検証を通常 Storybook gate へ接続した。

対応:

- `justfile` に `storybook-task-checkbox-check` を追加した。
- `storybook-task-checkbox-check` は KUC checkbox visual、KUC ContextMenu / checkbox hit、KDV window click、context menu、external state override、session state、mouse 経路を検証する。
- `storybook-check` の依存に `storybook-task-checkbox-check` を追加した。
- `scripts/check-storybook-entrypoint.sh` に task checkbox 関連テスト名を追加し、`storybook-check` から外れたら fail するようにした。
- `user-feedback-todo.md` の `SS-006` を `[ ]` から `[/]` に変更した。

検証:

```text
/opt/homebrew/bin/rtk just storybook-task-checkbox-check
/opt/homebrew/bin/rtk just storybook-entrypoint-check
```

結果:

- `storybook-task-checkbox-check`: passed。20本すべて通過。
- `storybook-entrypoint-check`: ok。

残:

- task checkbox の KUC / KDV / Storybook regression は通常 gate 化したが、実 OS window screenshot、KatanA reference との visual parity、task list 全体の人間確認は未完。
- `SS-006` は `[x]` ではない。DoD 完了扱いにしない。

## 2026-06-13 Accordion Gate 引き継ぎ更新

`SS-007` の accordion parsing、KUC typed action、hover、click、open / close、body visibility について、既存の KUC / KDV / Storybook 検証を通常 Storybook gate へ接続した。

対応:

- `justfile` に `storybook-accordion-check` を追加した。
- `storybook-accordion-check` は KDV core の details -> accordion node、KUC typed host action、KUC Storybook accordion visual/action、KDV window hover/click/open-close/body pixel 差分、manual action contract regression を検証する。
- `storybook-check` の依存に `storybook-accordion-check` を追加した。
- `scripts/check-storybook-entrypoint.sh` に accordion 関連テスト名を追加し、`storybook-check` から外れたら fail するようにした。
- `user-feedback-todo.md` の `SS-007` を `[ ]` から `[/]` に変更した。

検証:

```text
/opt/homebrew/bin/rtk just storybook-accordion-check
/opt/homebrew/bin/rtk just storybook-entrypoint-check
```

結果:

- `storybook-accordion-check`: passed。17本すべて通過。
- `storybook-entrypoint-check`: ok。

残:

- accordion の KUC / KDV / Storybook regression は通常 gate 化したが、実 OS window screenshot、KatanA reference HTML/PDF との visual parity、accordion section 全体の人間確認は未完。
- `SS-007` は `[x]` ではない。DoD 完了扱いにしない。

## 2026-06-13 Diagram Load Gate 引き継ぎ更新

`SS-008` の diagram lazy load、control、math control 除外、cache、theme、scroll stability について、既存の KDV core / KDV Storybook 検証を通常 Storybook gate へ接続した。

対応:

- `justfile` に `storybook-diagram-load-check` を追加した。
- `storybook-diagram-load-check` は KDV core の diagram fixture / asset loader / memory cache / disk cache / parallel start、KDV Storybook の math control 除外、direct diagram surface、control layout / hover / click、asset job completion、spinner animation、dark/light cache、physical SVG cache、scroll scope、pending -> loaded release gate を検証する。
- `storybook-check` の依存に `storybook-diagram-load-check` を追加した。
- `scripts/check-storybook-entrypoint.sh` に diagram load / control 関連テスト名を追加し、`storybook-check` から外れたら fail するようにした。
- `user-feedback-todo.md` の `SS-008` を `[ ]` から `[/]` に変更した。

検証:

```text
/opt/homebrew/bin/rtk just storybook-diagram-load-check
/opt/homebrew/bin/rtk just storybook-entrypoint-check
```

結果:

- `storybook-diagram-load-check`: passed。30本すべて通過。
- `storybook-entrypoint-check`: ok。

残:

- diagram lazy load / controls / cache / scroll scope regression は通常 gate 化したが、実 OS window screenshot、renderer が生成した SVG 内の dark theme 差分、KatanA reference との図形 control visual parity は未完。
- `SS-008` は `[x]` ではない。DoD 完了扱いにしない。

## 2026-06-13 Slideshow Gate 引き継ぎ更新

KatanA slideshow view の page index、key navigation、mode switch、scene reuse、code copy control 非表示について、既存の KDV core / KDV Storybook 検証を通常 Storybook gate へ接続した。

対応:

- `justfile` に `storybook-slideshow-check` を追加した。
- `storybook-slideshow-check` は KDV core の slideshow state / page index / command / render engine、KDV Storybook の mode切替、key navigation、page scroll、scene rebuild抑止、SettingsList state 表示、slideshow mode の code copy control 非表示を検証する。
- `storybook-check` の依存に `storybook-slideshow-check` を追加した。
- `scripts/check-storybook-entrypoint.sh` に slideshow 関連テスト名を追加し、`storybook-check` から外れたら fail するようにした。
- `user-feedback-todo.md` の slideshow page move / slideshow code copy control 項目へ追記した。

検証:

```text
/opt/homebrew/bin/rtk just storybook-slideshow-check
/opt/homebrew/bin/rtk just storybook-entrypoint-check
```

結果:

- `storybook-slideshow-check`: passed。26本すべて通過。
- `storybook-entrypoint-check`: ok。

残:

- slideshow state / navigation / mode / scene reuse / control visibility regression は通常 gate 化したが、実 OS window の page navigation 画面証跡と KatanA slide view との visual parity は未完。
- DoD 完了扱いにしない。

## 2026-06-13 Coordinate Contract Gate 引き継ぎ更新

`SS-002` / `UF-008` の click / hover / action 座標契約について、散っていた KUC / KDV / Storybook の代表検証を通常 Storybook gate へ接続した。

対応:

- `justfile` に `storybook-coordinate-contract-check` を追加した。
- `storybook-coordinate-contract-check` は KUC core の window coordinate normalizer、KUC Storybook の rendered hit rect / row layout、KDV Storybook の window coordinate、FileTree row、SettingsList toggle / section header、Link、Accordion、Task、MediaControl の hover / cursor / click representative を検証する。
- `storybook-check` の依存に `storybook-coordinate-contract-check` を追加した。
- `scripts/check-storybook-entrypoint.sh` に coordinate contract 関連テスト名を追加し、`storybook-check` から外れたら fail するようにした。
- `user-feedback-todo.md` の `SS-002` / `UF-008` に進捗として追記した。ただし両方とも `[ ]` のままにしている。

検証:

```text
/opt/homebrew/bin/rtk just storybook-coordinate-contract-check
/opt/homebrew/bin/rtk just storybook-entrypoint-check
```

結果:

- `storybook-coordinate-contract-check`: passed。KUC core normalizer 1件、KUC Storybook hit 23件、row layout 1件、KDV window coordinate 3件、normalizer source guard 1件、hit rect center 5件、sidebar/settings/link/accordion/task/media/window regression 10件、manual action contract guard 1件が通過。
- `storybook-entrypoint-check`: ok。

残:

- coordinate contract は通常 gate 化したが、実 OS window screenshot、全 node の同一 layout result 共有、KatanA viewer / slideshow との visual / interaction score parity は未完。
- `SS-002` / `UF-008` は `[x]` ではない。DoD 完了扱いにしない。
