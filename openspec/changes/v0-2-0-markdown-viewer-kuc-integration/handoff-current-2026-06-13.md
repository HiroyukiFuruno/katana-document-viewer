# KDV v0.2.0 現状引き継ぎ入口

最終更新: 2026-06-13

## 結論

KDV v0.2.0 viewer parity は未完了。KatanA の viewer / slideshow を完全踏襲したとは言えない。

次担当者は、まずこのファイルを読み、その後に正本資料を辿ること。過去に `just storybook-score-check` や個別 gate が通った記録はあるが、ユーザー実機指摘の破綻を全て fail できる状態ではないため、DoD 達成の根拠にはしない。

## 正本資料

- 計画書: `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/kdv-v0.2.0-viewer-recovery-plan.md`
- 残作業正本: `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/remaining-plan.md`
- ユーザー実機指摘台帳: `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/user-feedback-todo.md`
- 詳細引き継ぎ履歴: `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/handoff-unresolved-2026-06-12.md`
- 設計方針: `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/design.md`

## 現時点の未達サマリ

### 1. KDV / KUC 境界

未達:

- KDV Storybook 側に window / frame / mouse / host action の調整実装がまだ多い。
- KUC 側の interactive preset、hit-test、cursor、hover border、click callback の責務が利用側へ漏れている。
- KDV 側で座標復元、style class parse、state id parse、action 合成が戻る余地を完全には潰し切れていない。
- `MediaControl` は KDV domain 固有 widget として扱うべきだが、KUC 汎用部品との境界がまだ曖昧。

次に必要な状態:

- Toggle / Button / Linkable / Tree row / Settings field は、利用側が個別補正しなくても hover border、cursor、click action を返す。
- KDV は KUC action を viewer state / command に反映するだけにする。
- KDV 側へ低レベル hit-test や座標式が戻ったら lint / test で fail する。

### 2. TreeView / FileTree / SettingsList

未達:

- TreeView の体感が重い。
- FileTree の見た目が KatanA の file explorer と大きく違う。
- TreeView / Toggle の代表 click point は gate 化済みだが、実 OS window の全操作点証跡は未達。
- Toggle hover border が標準挙動として安定している証拠が不足している。
- SettingsList が KUC default preset だけで成立しているとは言い切れない。

次に必要な状態:

- KUC Storybook の TreeView / FileTree / Toggle と KDV Storybook の visual / hit-test contract を同一条件で比較する。
- 実ウィンドウで TreeView row center、toggle track edge、settings row center のクリック E2E を追加する。
- TreeView の重さを frame time、hit-test cache、KUC tree rebuild count で数値化する。

### 3. Markdown 基本描画

未達:

- code block の見た目、配置、copy button、syntax、clipping が KatanA と一致していない。
- task checkbox は state 反映の gate はあるが、見た目、右クリック context menu、KatanA parity が未達。
- list marker / bullet / indent / baseline は実 OS window Retina 表示と KatanA screenshot parity が未達。
- badge、link underline、strikethrough、alert block は一部 gate 済みだが、KatanA reference section visual parity が未達。
- accordion は window gate があるが、実 OS window screenshot と KatanA reference parity が未達。
- horizontal rule / table / HTML alignment の総合 visual parity が不十分。
- OS color emoji が KatanA / OS 依存表示として正しく出ている証拠が不足している。
- preview font size が大きすぎる可能性があり、外部設定と Storybook inspector の確認が必要。

次に必要な状態:

- `katana/sample_basic.md` を KatanA と KDV で同じ viewport / theme / font size にして feature 単位で比較する。
- code、task、list、alert、badge、link、emoji、accordion を KatanA screenshot、KDV screenshot、export HTML/PDF screenshot の比較対象にする。
- KUC text renderer の emoji / inline code / list marker / bullet shape を KUC 側の contract test に寄せる。

### 4. Diagram / Image / Math / Asset

未達:

- diagram が永遠にロード中に見えるケースが残る。
- lazy / parallel / cancellation / disk cache の contract が KatanA と同等である証拠が不足している。
- scroll / resize 時に図形が再ロードされるように見える。
- dark theme が renderer request、cache key、rendered surface に正しく反映されているか未証明。
- diagram/image control の配置、透明 base、hover highlight、click action が KatanA / GitHub style と一致していない。
- png / jpg など direct image の表示・操作の実 OS window 証跡が不足している。
- SVG が粗い。KUC presentation downscale だけでは不十分で、rasterize 解像度、RGBA、renderer boundary を見る必要がある。
- math は media control 除外 gate はあるが、math 自体の visual parity は未達。

次に必要な状態:

- KatanA の `core_render.rs`、`background.rs`、diagram worker / polling / cancellation を再読解する。
- renderer boundary に cancellable request、timeout、RGBA、高解像度 SVG rasterize、物理 cache parity を持たせる。
- image / diagram controls は KDV domain widget として、KUC の汎用 Button preset を使いながら KatanA layout へ寄せる。

### 5. Scroll / Resize / Viewport

未達:

- bottom spacer、bottom scroll、resize bottom anchor は headless / window state regression があるが、実 OS window screenshot と KatanA visual score が未達。
- scroll が重い。
- scroll 時に text / diagram が消える、または再ロードに見える問題は、実 OS window の連続操作証跡が不足している。
- viewer は 1 枚の document page として全 node を持ち、viewport は表示窓として扱う必要がある。

次に必要な状態:

- `content_height`、bottom spacer、viewport height、max scroll、last target rect を KatanA と比較する。
- scroll offset を scene build に混ぜず、render area / hit-test のみに渡ることを全経路で gate 化する。
- scroll 1 step あたりの scene rebuild count、asset job restart count、frame render time を数値化する。

### 6. Score / Gate

未達:

- `just storybook-score-check` が通っても、実画面のクリックずれ、hover 不発、accordion 操作、diagram control 操作、OS emoji、scroll 末尾 clipping を十分に捕捉できない。
- score が KDV self parity や空画面比較で通る状態は禁止だが、現状の gate はまだ DoD の証明として弱い。
- visual / semantic / interaction / performance のカテゴリ別 95 点以上が、実機指摘と完全には接続されていない。
- 追補: Storybook surface parity は row alignment 偽陽性を落とすようになった。`direct/html-alignment.htm` は HTML `<h1>` の export surface 分類と KUC HTML role origin offset を修正し、95 点以上へ復旧済み。`katana/sample.md` は空コードブロック高さのKUC側再計算を撤去し、surface parity gate が通過済み。`katana/sample_basic.md` も KUC text raster の font family contract を KDV export surface と揃え、surface parity gate が通過済み。threshold を下げず、検出された surface parity 未達を直した。

次に必要な状態:

- KatanA reference screenshot / export HTML / export PDF を正本にして、KDV preview crop と比較する。
- broken UI が通る gate は gate 自体を未対応に戻す。
- score は平均ではなく `min(visual, semantic, interaction, performance)` で評価する。

### 7. Fixture / Sample

未達:

- サンプル構成がまだ冗長・不適切な可能性がある。
- KatanA fixture の代表と集約 fixture で足りる部分が多い。
- 他 repository 相対参照は禁止。KDV repo 内 `assets/fixtures` に固定する必要がある。

次に必要な状態:

- `assets/fixtures/katana` を正本として、同じ確認観点の重複 fixture を削る。
- `direct` fixture は source kind の最小検証に限定する。
- TreeView 上はカテゴリ別に見通しよく表示する。

## 直近でやりきった中途半端な作業

以下は、途中で止めずに gate 接続と資料反映まで行った。ただし DoD 完了ではない。

- Hover contract gate:
  - `storybook-hover-contract-check` を追加し、`storybook-check` と entrypoint check へ接続した。
  - `user-feedback-todo.md` の UF-009 / UF-010 に反映済み。
  - 検証: `/opt/homebrew/bin/rtk just storybook-hover-contract-check` は通過。

- Direct image Window gate:
  - `tools/kdv-storybook/src/window/image_fixture_window_tests.rs` を追加した。
  - `storybook-image-control-check` を追加し、`storybook-check` と entrypoint check へ接続した。
  - direct raster image が loaded scene 経路で `ImageSurface` と preview pixel に到達することを固定した。
  - 検証: `/opt/homebrew/bin/rtk just storybook-image-control-check` は通過。

- Scroll / resize contract gate:
  - `storybook-scroll-resize-contract-check` を追加し、`storybook-check` と entrypoint check へ接続した。
  - bottom spacer、last target top alignment、window bottom scroll、resize bottom anchor、diagram scroll/resize cache reuse の代表テストを通常 gate 化した。
  - 検証: `/opt/homebrew/bin/rtk just storybook-scroll-resize-contract-check` は通過。

- Score visual entrypoint split:
  - `storybook-score-check` の broad filter `storybook_score_visual` を、preview crop、scaled canvas、export PNG の3つの明示 gate に分解した。
  - `scripts/check-storybook-entrypoint.sh` でも同3 gate を必須化し、score recipe の dry-run から必須比較対象が読めるようにした。
  - 検証: `/opt/homebrew/bin/rtk just storybook-score-check` は通過。

- Sidebar KUC FileTree / SettingsList contract audit:
  - KDV Storybook 左ペインは、現時点で `StorybookSidebar::render` から KUC `FileTree::render_with_state_and_offset` と KUC `SettingsList` を組み合わせる経路になっていることを確認した。
  - click / hover は KDV の row height 推測ではなく、KUC render tree を `UiTreeInteractionSurface` に渡し、`FileTree::action_from_host_plan` と `SettingsList::action_from_host_plan` で action を復元する経路になっていることを確認した。
  - スクロール後の FileTree について、複数の可視 file row を KUC host action 座標で hover -> click し、同じ fixture が選択される matrix を追加した。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked sidebar_hit -- --test-threads=1` は `22 passed`。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked sidebar_ -- --test-threads=1` は `63 passed`。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked sidebar_frame_file_tree_draws_kuc_icons_and_indent_guides -- --test-threads=1` は `1 passed`。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked scrolled_file_tree_hover_then_click_matrix_selects_visible_files -- --test-threads=1` は `1 passed`。
  - 検証: `/opt/homebrew/bin/rtk just storybook-treeview-check` は通過。
  - 未達: 実 OS window の全操作点、TreeView 体感性能、KatanA Explorer visual parity、Toggle hover border / cursor の全点証跡は未達。DoD 完了扱いにしない。

- SettingsList section header matrix:
  - KUC `SettingsList` の `display` / `interaction` / `state` section header について、KUC action target 座標で hover、pointer cursor、collapse、reopen が通る matrix を追加した。
  - 同じ section header について、KUC action target の左右端でも pointer cursor、hover state、collapse、reopen が通る matrix を追加した。KDV 側で row height や section label 幅を復元せず、KUC host action rect の `left` / `right` だけを使う。
  - 複数 section matrix は preview scene を作らず最小 catalog で SettingsList 契約だけを検証するようにし、edge matrix は左右端を交互に collapse/reopen へ割り当てて重複 click を減らした。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_window_settings_section_header_e2e_toggle_matrix -- --test-threads=1` は `1 passed`。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_window_settings_section_header_edge_click_matrix -- --test-threads=1` は `1 passed`。
  - 検証: `/opt/homebrew/bin/rtk just storybook-entrypoint-check` は通過。
  - 検証: `/opt/homebrew/bin/rtk just storybook-settings-contract-check` は通過。
  - 実測: `storybook-settings-contract-check` 内の section header e2e は `5.35s`、edge click は `4.71s`。`window::interaction_matrix_tests` は `23 passed`、`59.71s`。
  - 未達: 実 OS window の全域 click 証跡、KatanA visual parity、score 95 は未達。DoD 完了扱いにしない。

- `katana/sample_basic.md` surface parity の途中改善:
  - KDV/KUC の inline code 背景を `inline-code-background` theme token へ分離し、light theme では export surface と同じ `#eff2f6` を使うようにした。
  - KUC blockquote の背景全面塗りをやめ、KatanA/export と同じ quote bar + child content 表現へ寄せた。
  - KUC blockquote 内 quoted code は通常 code block 高さではなく compact quoted code metrics を使い、`let quoted_code = true;` が clip されないようにした。
  - KUC alert は stripe の上下 padding、icon y、compact alert line height を export surface 寄りに調整した。
  - KUC alert icon を export surface と同じ Note / Tip / Important / Warning / Caution の outline shape へ寄せ、alert title 1 行目を bold span として raster するようにした。
  - KUC text raster の macOS 固定 font family をやめ、proportional は `Family::SansSerif`、monospace は `Family::Monospace` を使うようにした。KDV export surface と異なる glyph metrics で全体の row 差分が増える問題を解消した。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --manifest-path /Users/hiroyuki_furuno/works/private/katana-ui-core/Cargo.toml --locked blockquote -- --test-threads=1` は `3 passed`。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --manifest-path /Users/hiroyuki_furuno/works/private/katana-ui-core/Cargo.toml --locked alert -- --test-threads=1` は `11 passed`。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --manifest-path /Users/hiroyuki_furuno/works/private/katana-ui-core/Cargo.toml --locked text_raster -- --test-threads=1` は `5 passed`。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked bridge_passes_kdv_ -- --test-threads=1` は `2 passed`。
  - 検証: `KDV_STORYBOOK_SURFACE_PARITY_FIXTURE=katana/sample_basic.md KDV_STORYBOOK_SURFACE_DUMP_DIR=/tmp/kdv-surface-katana-sample-basic-after-font-family /opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer_diagrams -- --test-threads=1` は `1 passed`。
  - 未達: `katana/sample_basic.md` surface parity 単体は通過したが、v0.2.0 DoD 全体の visual / semantic / interaction / performance 95 点以上、実機操作、KatanA screenshot / export HTML / export PDF 正本比較は未達。DoD 完了扱いにしない。

- Alert block Storybook scene contract:
  - `tools/kdv-storybook/src/frame_alert_contract_tests.rs` を追加した。
  - `katana/sample_basic.md` の Note / Tip / Important / Warning / Caution が、raw `[!TYPE]` や `TYPE: body` ではなく、title/body 分離、alert role、Wrap、tone、左罫線 token として KDV -> KUC Storybook scene に届くことを固定した。
  - `storybook-content-check` と `scripts/check-storybook-entrypoint.sh` へ接続した。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked katana_alert_scene_keeps_title_body_and_kind_contract -- --test-threads=1` は `1 passed`。
  - 検証: `/opt/homebrew/bin/rtk just storybook-entrypoint-check` は通過。
  - 検証: `/opt/homebrew/bin/rtk just storybook-content-check` は通過。
  - 未達: KatanA screenshot / export HTML / export PDF との alert section visual parity、最終 pixel 同等性、score 95 は未達。DoD 完了扱いにしない。

- Alert block planner/export height contract:
  - `crates/katana-document-viewer/src/viewer/node_plan/metrics.rs` に alert 専用高さ式を追加した。
  - `LABEL: body` 形式の alert を、通常本文 1 行ではなく `title 1行 + body wrapped line + 32px vertical padding` として扱う。
  - `crates/katana-document-viewer/src/viewer/node_plan/builder_media_text_height.rs` の interactive preview span 計測経路でも alert は同じ metrics 経路へ戻し、planner 上で `46px` に潰れる再発を防いだ。
  - `crates/katana-document-viewer/src/viewer/node_plan/types_tests.rs` に `alert_height_keeps_export_surface_vertical_padding` を追加した。
  - `crates/katana-document-viewer/src/viewer/node_plan/builder_rich_height_tests.rs` に `planner_keeps_export_surface_alert_vertical_padding` を追加した。
  - 実装前の赤確認: `/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked alert_height_keeps_export_surface_vertical_padding -- --test-threads=1` は `124px` 期待に対して `92px` で失敗。`/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked alert_vertical_padding -- --test-threads=1` は planner 経由で `124px` 期待に対して `46px` で失敗。
  - 修正後の検証: `/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked alert_height_keeps_export_surface_vertical_padding -- --test-threads=1` は `1 passed`。
  - 修正後の検証: `/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked alert_vertical_padding -- --test-threads=1` は `1 passed`。
  - 修正後の検証: `/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked viewer::node_plan::builder::rich_height_tests -- --test-threads=1` は `3 passed`。
  - 修正後の検証: `/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked viewer::node_plan::builder::surface_height_tests -- --test-threads=1` は `3 passed`。
  - 修正後の検証: `/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked viewer::node_plan::types::tests -- --test-threads=1` は `7 passed`。
  - 未達: alert 色、icon、左罫線、本文位置、KatanA screenshot / export HTML / export PDF との final pixel parity、score 95 は未達。DoD 完了扱いにしない。

- Direct HTML surface parity:
  - `crates/katana-document-viewer/src/export_surface/export_surface_block_factory/html.rs` で HTML `<h1>`〜`<h6>` を body line ではなく heading line として扱うようにした。
  - `crates/katana-document-viewer/src/export_surface/export_surface_line_impl.rs` に centered / right aligned heading line constructor を追加した。
  - `crates/katana-document-viewer/src/export_surface/export_surface_block_factory/html_tests.rs` に `html_heading_is_rendered_as_heading_line` を追加した。
  - 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/ui_tree_canvas_text_metrics.rs` で HTML text role だけ export surface origin offset を持つようにした。通常 Markdown body の top margin は 0 のまま。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked html_heading_is_rendered_as_heading_line -- --test-threads=1` は `1 passed`。
  - 検証: 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked compact_html_ -- --test-threads=1` は `2 passed`。
  - 検証: `KDV_STORYBOOK_SURFACE_PARITY_FIXTURE=direct/html-alignment.htm KDV_STORYBOOK_SURFACE_DUMP_DIR=/tmp/kdv-surface-html-align-after-kuc-offset /opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer -- --test-threads=1` は `2 passed`。
  - 修正済み: `katana/sample.md` の後続 25px 前倒しは解消済み。`katana/sample_basic.md` も KUC text raster の font family contract を export surface と揃え、surface parity gate が通過済み。

- Katana sample surface parity current:
  - `tools/kdv-storybook/src/frame_surface_row_alignment.rs` に row alignment report と上位 loss band 診断を追加した。
  - `tools/kdv-storybook/src/frame_surface_similarity.rs` は `SurfaceParityReport` に `row_score` / 双方向 row score / row loss band を含める。
  - `tools/kdv-storybook/src/frame_surface_parity_tests.rs` は失敗時に row loss band を `TargetSample` へ結び付け、該当 `line` / `raw` / `artifact_id` を出す。
  - `crates/katana-document-viewer/src/export_surface/export_surface_line_metrics.rs` は compact body line height を KUC compact body と同じ `23px` に寄せた。
  - 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/ui_tree_canvas_text_table_wrap.rs` は table row layout の最小行高を KDV export surface と同じ `52px` にした。このファイルは外部KUC側で git 管理外の新規ファイル扱いだが、既存 `ui_tree_canvas_text_table.rs` の `#[path = "ui_tree_canvas_text_table_wrap.rs"]` が参照している。
  - 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/ui_tree_canvas_text_wrap.rs` は explicit width と compact document 用 wrap metrics を使い、長い inline code が export surface と同じ 3 行へ折り返されるようにした。
  - 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/document_viewer/node_factory.rs` と `node_factory_hover.rs` は viewer node に explicit width を渡す。
  - 外部KUC `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/visual/ui_tree_canvas_text_table_layout.rs` は table layout でも explicit width を優先する。
  - 検証: `/opt/homebrew/bin/rtk cargo fmt -p katana-document-viewer -- -q --check` は通過。
  - 検証: `/opt/homebrew/bin/rtk cargo fmt -p kdv-storybook -- -q --check` は通過。
  - 検証: 外部KUC `/opt/homebrew/bin/rtk cargo fmt -p katana-ui-core-storybook -- -q --check` は通過。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked document_typography_scales_body_heading_and_code_lines -- --test-threads=1` は `1 passed`。
  - 検証: 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked table_row_layout_keeps_kdv_export_surface_min_height -- --test-threads=1` は `1 passed`。
  - 検証: 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked compact_inline_code_wraps_like_viewer_surface -- --test-threads=1` は `1 passed`。
  - 検証: 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked explicit_width_controls_document_text_wrap_area -- --test-threads=1` は `1 passed`。
  - 検証: 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked table_layout_uses_explicit_viewer_width -- --test-threads=1` は `1 passed`。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked frame_surface_similarity -- --test-threads=1` は `7 passed`。
  - 検証: `KDV_STORYBOOK_SURFACE_PARITY_FIXTURE=direct/html-alignment.htm /opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer -- --test-threads=1` は `2 passed`。
  - 修正済み: `assets/fixtures/katana/sample.md` line 637〜647 付近の table が 25px 早く出る原因は、空コードブロックの高さを KDV plan/export は `84px` としていた一方、KUC code node が本文行数から `59px` へ再計算していたこと。KUC `node_factory_code.rs` は viewer rect height を source of truth に戻した。
  - 追加gate: KDV `builder_surface_height_tests` に `katana_sample_empty_code_block_uses_export_surface_height`、`katana_sample_consecutive_code_block_uses_export_surface_height`、`katana_sample_consecutive_list_to_table_gap_is_planned` を追加した。KDV Storybook には `katana_sample_export_surface_tree_places_table_at_target_plus_padding` を追加し、table y が export target + viewer padding と一致することを固定した。KUC Storybook には `code_block_height_comes_from_viewer_rect_not_kuc_text_metrics` と `scroll_area_padded_viewer_blocks_keep_table_y_equal_to_normal_layout` を追加した。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p katana-document-viewer --locked katana_sample_ -- --test-threads=1` は `10 passed`。
  - 検証: 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --manifest-path /Users/hiroyuki_furuno/works/private/katana-ui-core/Cargo.toml --locked code -- --test-threads=1` は `30 passed`。
  - 検証: 外部KUC `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --manifest-path /Users/hiroyuki_furuno/works/private/katana-ui-core/Cargo.toml --locked scroll_area_padded_viewer_blocks_keep_table_y_equal_to_normal_layout -- --test-threads=1` は `1 passed`。
  - 検証: `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked katana_sample_export_surface_tree_places_table_at_target_plus_padding -- --test-threads=1` は `1 passed`。
  - 検証: `KDV_STORYBOOK_SURFACE_PARITY_FIXTURE=katana/sample.md KDV_STORYBOOK_SURFACE_DUMP_DIR=/tmp/kdv-surface-katana-sample-final /opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer_diagrams -- --test-threads=1` は `1 passed`。
  - 修正済みゲート: `KDV_STORYBOOK_SURFACE_PARITY_FIXTURE=katana/sample_basic.md KDV_STORYBOOK_SURFACE_DUMP_DIR=/tmp/kdv-surface-katana-sample-basic-after-font-family /opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked storybook_frame_matches_export_surface_for_katana_viewer_diagrams -- --test-threads=1` は `1 passed`。
  - 診断: 直前の `93/95` / `94/95` 失敗は、空コードブロック高さ、alert surface 差分に加えて、KUC text raster が macOS 固定 font family を使い、KDV export surface の `Family::SansSerif` / `Family::Monospace` と glyph metrics がずれていたことが主因だった。threshold を下げず、font family contract を KUC 側で揃えて解消した。
  - 次: `sample_basic.md` surface parity 単体は通過済み。以後は DoD 全体の score gate、実機操作、KatanA screenshot / export HTML / export PDF 正本比較へ戻す。`sample.md` の 25px 前倒しへ戻る修正は禁止する。

- 整合性確認:
  - `/opt/homebrew/bin/rtk just storybook-entrypoint-check` は通過。
  - `/opt/homebrew/bin/rtk cargo fmt --all --check` は通過。
  - `STORYBOOK_FRAMES=1 /opt/homebrew/bin/rtk just storybook` は interactive binary の 1 frame 起動経路通過。

## 次担当者の最短手順

1. `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/user-feedback-todo.md` の `[ ]` と `[/]` を確認する。
2. `just storybook` を起動し、KatanA 実画面と同じ fixture / theme / font size / viewport で比較する。
3. `just storybook-interaction-check`、`just storybook-performance-check`、`just storybook-score-check` を実行し、実画面指摘との接続不足を洗い出す。
4. 最初に KUC default preset / hit-test / cursor / hover の境界を直す。KDV 側で個別補正しない。
5. 次に Markdown 基本描画を KatanA reference に寄せる。code、task、list、alert、badge、link、emoji、accordion の順。
6. Diagram / image は renderer timeout / cancellation / disk cache / theme key / RGBA / high-resolution rasterize を root contract として直す。
7. 最後に score 95 以上を確認する。1 カテゴリでも 95 未満なら未達。

## 完了条件

以下が揃うまで、v0.2.0 viewer parity を完了にしない。

- `user-feedback-todo.md` に未対応 `[ ]` が残っていない。
- `remaining-plan.md` に未完了項目が残っていない。
- `just storybook` が interactive viewer を起動し、KatanA 実画面との主要 fixture 比較に耐える。
- `just storybook-interaction-check` が、クリックずれ、hover、link、accordion、task、media control、search、slideshow を実操作として検証する。
- `just storybook-performance-check` が、file switch first frame、pending display、scroll / resize、diagram lazy completion を数値で検証する。
- `just storybook-score-check` が visual / semantic / interaction / performance の全カテゴリを KatanA reference に対して 95 点以上で検証する。
- score は自己比較、空画面比較、平均点で通らない。
- KDV core に vendor crate 依存が戻っていない。
- KDV Storybook に独自 TreeView、独自 toggle、独自 settings UI、独自 media hit-test、独自 action 合成が戻っていない。

## 現在の未コミット注意

この引き継ぎ作成時点で、以下のような未コミットまたは未追跡差分が存在した。次担当者は `git status --short -- <対象>` で対象を絞って確認すること。

- `Justfile`
- `scripts/check-storybook-entrypoint.sh`
- `tools/kdv-storybook/src/window.rs`
- `tools/kdv-storybook/src/window/image_fixture_window_tests.rs`
- `tools/kdv-storybook/src/frame_alert_contract_tests.rs`
- `tools/kdv-storybook/src/main.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/metrics.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/builder_media_text_height.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/types_tests.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/builder_rich_height_tests.rs`
- `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/user-feedback-todo.md`
- `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/handoff-unresolved-2026-06-12.md`
- `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/kdv-v0.2.0-viewer-recovery-plan.md`
- `openspec/changes/v0-2-0-markdown-viewer-kuc-integration/handoff-current-2026-06-13.md`
