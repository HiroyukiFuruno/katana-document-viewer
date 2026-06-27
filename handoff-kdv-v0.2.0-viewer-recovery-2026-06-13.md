# KDV v0.2.0 Viewer Recovery Handoff

作成日: 2026-06-13

## 結論

KDV v0.2.0 はまだ DoD 未達。次セッションは実装を続ける前に、この文書を作業台帳として扱い、未達を消し込むこと。

現状の主問題は「見た目の微修正」ではなく、KDV / KUC / Storybook の責務境界が崩れていること。KDV が座標や action を復元する設計に戻ると同じ不具合を繰り返すため、KUC の実部品契約を使って state / event / action / hover / hit target を通すこと。

## 最終目標

`just storybook` で KUC 実部品を使った Katana 由来 viewer を interactive 起動し、KatanA viewer / export HTML / export PDF と同等の表示・操作・性能を満たす。

必須条件:

- KDV core は vendor 非依存。
- KUC は共通 UI 契約の唯一の実部品層。
- Storybook は KUC 部品を host するだけ。
- egui / gpui / floem adapter は KatanA 由来 viewer が確立するまで復帰しない。
- visual / semantic / interaction / performance score は全カテゴリ 95 点以上。
- score は KDV 自己比較で通してはいけない。KatanA reference artifact と比較する。

## 参照すべき仕様

- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/openspec/changes/v0-2-0-markdown-viewer-kuc-integration/proposal.md`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/openspec/changes/v0-2-0-markdown-viewer-kuc-integration/design.md`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/openspec/changes/v0-2-0-markdown-viewer-kuc-integration/specs/markdown-viewer-kuc-integration/spec.md`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/openspec/changes/v0-2-0-markdown-viewer-kuc-integration/tasks.md`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/openspec/changes/v0-2-0-markdown-viewer-kuc-integration/user-feedback-todo.md`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core`
- `/Users/hiroyuki_furuno/works/private/katana`

注意: `openspec tasks` は完了扱いでも、ユーザー指摘台帳は未完了。現状判断は OpenSpec change 配下の `user-feedback-todo.md` と実画面を優先する。

## 禁止事項

- KDV Storybook 側で TreeView / SettingsList / Toggle / Button / Media control の座標判定を再実装しない。
- KDV Storybook 側で `state_id` / `style_class` / 文字列 parse から action を復元しない。
- KDV 側で KUC の見た目だけを真似た独自 widget を追加しない。
- fallback で別 renderer に逃がさない。失敗時は log + diagnostic node + raw string node。
- `just storybook` を smoke test だけにしない。interactive viewer を起動すること。
- score gate を自己比較や甘い閾値で通さない。
- dirty worktree を reset / checkout / clean で戻さない。

## 直近の未達と原因候補

### 1. code block copy が動かない

現状:

- copy button を押してもクリップボード更新の保証がない。
- 押下後に button が check mark にならない。
- copy button の配置も KatanA と乖離している。

確認済み原因:

- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/crates/katana-document-viewer/src/viewer/commands.rs`
  - `HostCommand::CopyText(String)` が text だけを持つ。
  - どの node / control から copy したか Storybook host が判断できない。
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/crates/katana-document-viewer/src/viewer/commands_factory.rs`
  - `copy-source` / `copy-code` が raw text だけを `CopyText` に詰める。
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/window_command.rs`
  - `apply_host_command` が `invalidate_lazy_scene(false)` するだけ。
  - 実クリップボード書き込みも copied state 更新もない。
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/document_viewer/node_factory_code.rs`
  - KUC 側に copied code node state がない。

次作業:

- `HostCommand::CopyText(String)` を typed command に変更する。
  - 例: `CopyTextCommand { source: CopyTextSource, target: ViewerTarget, text: String }`
  - `CopyTextSource` は最低 `Code` と `DiagramSource` を分ける。
- Storybook host で `CopyTextSource::Code` を受けたら:
  - macOS では `/usr/bin/pbcopy` へ書き込む。
  - copied code node id を session state に保存する。
  - lazy scene を再構築して copy button を check mark 表示へ変える。
- KUC `KucViewerConfig` / `KucNodeFactory` に copied code node ids を渡す。
- KUC code copy button は copied state 時に同じ action を維持しつつ label を check mark にする。
- copy button の top/right margin と padding は KatanA 表示に合わせて contract test で固定する。

必要テスト:

- KDV command factory: code copy / diagram source copy が `target` と `source` を保持する。
- Storybook window command: `CopyTextSource::Code` で copied state が更新される。
- KUC node factory: copied code node の copy button が check mark 表示になる。
- Interaction E2E: code copy click 後に button 表示が変わる。

### 2. asset job の不穏ログ

ログ:

```text
[kdv-storybook] asset job result send failed: sending on a closed channel
```

確認済み原因:

- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/window_asset_job.rs`
  - worker が最後の結果を送信する際、receiver 側が閉じていても常に `eprintln!` している。
  - job cancel による正常破棄と、本当に異常な channel close を区別していない。

次作業:

- worker closure 内で `worker_cancel_token` を clone して保持する。
- `sender.send(...)` が失敗した場合:
  - `cancel_token == true` ならログを出さない。
  - `cancel_token == false` なら現行ログを残す。
- cancel 済み job の channel close を正常系として扱うテストを追加する。

### 3. toggle の反応が遅い

現状:

- 人間の体感でも toggle 反応が遅い。
- toggle クリックごとに preview scene / asset job が過剰に再構築されている疑い。
- hover border が toggle 単体に寄っており、KatanA 仕様の「label + control を含む行全体」になっていない。

確認すべき箇所:

- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/settings_action.rs`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/window_scene.rs`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/window_asset_job.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/src/molecule/app_primitives/settings/hit_test.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/src/molecule/app_primitives/settings/render_tests.rs`

次作業:

- KUC SettingsList 側で interactive row contract を定義する。
  - hover border は field row 全体。
  - cursor / action / hit target は同じ row contract から返す。
  - label 側を押しても toggle / select field の action が返るべきか、KatanA 仕様に合わせて固定する。
- KDV 側は KUC の `interaction_for_hit` / `action_for_hit` / `hit_target_for_hit` を使うだけにする。
- toggle で不要な asset worker を再起動しない。
  - hover / selection / code-controls / image-controls / diagram-controls の UI 表示変更は asset surface の再生成ではない。
  - asset job key から asset surface に無関係な interaction flags を外す。
  - dark theme / source / diagram source / DPI / renderer options など asset 実体に影響するものだけ cache key に残す。
- scene は即時更新、asset は必要な時だけ background 更新にする。

必要テスト:

- KUC SettingsList: field row 全体が hover / border / cursor / action の target になる。
- KDV Storybook: toggle click で 1 frame 以内に state が変わる。
- KDV Storybook: hover / selection toggle では asset job が再起動しない。
- KDV Storybook: dark theme toggle では theme key を変えて asset render する。

### 4. SVG が荒い

現状:

- drawio / svg の線が粗い。
- KatanA 側の SVG/RGBA/Retina 補正が KDV に反映されていない疑い。
- 一度ロード済み図形を scroll で再ロードしている。

次作業:

- KatanA 側の SVG 補正・RGBA・scale/DPI・cache 実装を先に読む。
  - `/Users/hiroyuki_furuno/works/private/katana`
  - `rg -n "rgba|premult|resvg|tiny_skia|svg|Pixmap|scale|dpi|retina|cache" crates -g '*.rs'`
- KDV asset loader / image surface / KUC surface metrics へ反映する。
- 図形 cache は「ファイルパス + AST位置 + source hash + theme + DPI + renderer options」で物理ファイルとして保持する。
- scroll で同一図形を再ロードしない。

必要テスト:

- 同一 diagram を scroll しても asset job が再投入されない。
- DPI / scale factor 変更時だけ SVG surface が再生成される。
- KatanA reference screenshot と線幅/解像度比較を score に入れる。

### 5. TreeView / FileTree

現状:

- TreeView のクリックポイントがズレる再発あり。
- FileTree の見た目が KatanA と乖離。
- KDV 側で wrapper / row 計算を持つと再発する。

次作業:

- FileTree が必要か再判断する。
  - 基本は KUC `TreeView` を上手く使えばよい。
  - FileTree が wrapper なら hit target / selection / toggle を内部 contract として持つこと。
- KDV は fixture id を座標から計算しない。
- KUC の TreeView row contract を唯一の source of truth にする。

必要テスト:

- KUC TreeView: row hit target が描画順 / scroll offset / Retina scale と一致。
- KDV Storybook: KUC action 経由で fixture が切り替わる。
- KDV source linter: manual tree hit-test / action synthesis が戻ったら fail。

### 6. Markdown viewer 本体

未達:

- task list が KUC checkbox として見えない、クリックして session state が変わらない。
- link hover / click / cursor が不完全。
- accordion が開かない。
- OS依存 emoji が正しく表示されない。emoji / special characters は KatanA/egui の白黒化や共通化を踏襲せず、KUC/KDV 独自の OS glyph 表示・選択・clipboard 契約として扱う。
- strikethrough / underline の位置が不正。
- list marker の位置や Retina 対応が不正。
- alert block が KatanA / GitHub style と乖離。
- code block の padding / copy button / syntax / clip が不正。
- scroll 末尾が途切れる。
- scroll 時に図形が再ロードされ、重い。

次作業:

- task checkbox:
  - KUC checkbox を使う。
  - click は markdown 編集まで行わず、KDV session state に変更を持つ。
  - context menu で `[ ]`, `[x]`, `[/]`, `[-]` を変更できるようにする。
- link:
  - link span は KUC text span action と cursor preset で処理する。
  - KDV 側で文字矩形を再計算しない。
- accordion:
  - KUC action 経由で open state を KDV session state に保存する。
- emoji / special characters:
  - egui 由来表示に戻さない。
  - KatanA/egui の白黒化や共通化を踏襲しない。
  - KUC/KDV 独自仕様として OS 依存の文字・絵文字をそのまま表示し、selection / clipboard copy は可視テキスト範囲と一致させる。
- alert:
  - GitHub GFM alert と KatanA 表示を reference として固定する。
  - 左線、icon、title、body spacing、色 token を KUC contract 化する。
- scroll:
  - viewer は 1枚の縦長 document として扱う。
  - bottom spacer を入れて toc/search jump が末尾でも機能する。
  - scroll で asset reload しない。

## score gate の再整備

現状の score は信頼してはいけない。目視で明らかに 95 点ではない状態が通っていた。

次作業:

- `just storybook-score-check` を実 score gate として再定義する。
- `final_score = min(visual_score, semantic_score, interaction_score, performance_score)`。
- KDV self comparison は fail。
- KatanA export HTML / PDF / screenshot reference を `assets/reference/katana` に固定する。
- score failure は画像 dump / semantic diff / interaction failure reason を残す。

## 推奨作業順

1. `just storybook` が interactive viewer を起動することを確認する。
2. `asset job result send failed` を cancel-aware に直す。
3. `HostCommand::CopyText` を typed command にし、code copy の clipboard + check mark を直す。
4. KUC SettingsList row interaction contract を直し、toggle の hover/クリック範囲を label + control 全体にする。
5. toggle / hover / selection で asset job が走らないよう cache key と invalidation を分離する。
6. TreeView / FileTree を KUC contract に戻し、manual hit-test を禁止する。
7. SVG/RGBA/Retina/cache を KatanA 参照で実装する。
8. task checkbox / link / accordion / emoji / alert / list / code block を KatanA と比較しながら直す。
9. scroll bottom spacer と scroll時 asset reload を直す。
10. score gate を KatanA reference で作り直し、全カテゴリ 95 点以上にする。

## 検証コマンド

最低限:

```text
just storybook
just storybook-window-smoke
just storybook-interaction-check
just storybook-media-control-clickability-check
just storybook-score-check
```

KDV:

```text
cargo test -p katana-document-viewer --locked
cargo test -p kdv-storybook --locked document_viewer -- --test-threads=1
cargo test -p kdv-storybook --locked -- --test-threads=1
```

KUC:

```text
cd /Users/hiroyuki_furuno/works/private/katana-ui-core
cargo test -p katana-ui-core --locked settings -- --test-threads=1
cargo test -p katana-ui-core --locked file_tree -- --test-threads=1
```

KUC Storybook の `document_viewer` filter は現行KUC self-contained boundaryでは `0 passed` になるため、v0.2.0 release proof にはしない。KatanA document_viewer harness の正本検証は KDV `cargo test -p kdv-storybook --locked document_viewer -- --test-threads=1` とする。

実行時はこの環境の規約に従い、shell では `/opt/homebrew/bin/rtk` を先頭に付ける。

## 現在の作業ツリー注意

- 2026-06-13 時点で KDV repo の `git status --short` は 275 entries。
- 既存の dirty / untracked はユーザーまたは前作業由来の可能性がある。
- `git reset --hard` / `git checkout .` / `git clean -fd` は禁止。
- 関連ファイルだけ diff を確認して、必要な変更を分離して進める。

## subagent 注意

- 画面上に大量の subagent 履歴が残っていた。
- この handoff 作成時点で、こちらが ID を把握していた agent は閉じた。
- 公開ツールには全 agent 一覧 API が見えていなかったため、画面履歴の 391 件を一括列挙して閉じる手段は確認できていない。
- 次セッションで subagent を使う場合は、完了後すぐ `close_agent` する。

## 直近で触るべきファイル候補

KDV:

- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/crates/katana-document-viewer/src/viewer/commands.rs`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/crates/katana-document-viewer/src/viewer/commands_factory.rs`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/window_command.rs`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/window_asset_job.rs`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/window_scene.rs`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/settings_action.rs`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/preview_cache.rs`

KUC:

- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/src/molecule/app_primitives/settings/hit_test.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/src/molecule/app_primitives/settings/render_tests.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/src/molecule/structured/tree_view_hit_test.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/src/molecule/structured/file_tree.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/document_viewer/config.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/document_viewer/node_factory.rs`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/document_viewer/node_factory_code.rs`

KatanA reference:

- `/Users/hiroyuki_furuno/works/private/katana/crates/katana-core/src/preview/section/mod.rs`
- `/Users/hiroyuki_furuno/works/private/katana/crates/katana-core/src/preview/types.rs`
- `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/section_show/markdown/mod.rs`
- `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/core_render.rs`
- `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/preview_pane/background.rs`
- `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/diagram_controller.rs`
- `/Users/hiroyuki_furuno/works/private/katana/crates/katana-ui/src/widgets/markdown_hooks/`

## 完了条件

- `just storybook` が interactive viewer を起動する。
- 左上は KUC TreeView/FileTree 契約で fixture tree を表示し、クリックできる。
- 左下は KUC SettingsList 契約で設定を表示し、行全体 hover/クリックが効く。
- 右側 viewer は vertical scroll / resize / bottom spacer / dark passthrough が動く。
- code copy は clipboard に入り、button が check mark へ変わる。
- task checkbox は表示と session state が変わる。
- link / accordion / search / diagram controls / image controls が実操作できる。
- diagram/image/PDF は lazy + parallel + cache + cancellation-safe。
- SVG は KatanA と同等の解像度。
- OS emoji は KUC/KatanA 由来で表示される。
- visual / semantic / interaction / performance score が全カテゴリ 95 点以上。
