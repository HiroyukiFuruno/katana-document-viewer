# KDV v0.2.0 Viewer Recovery Work Instruction

この文章を次セッションの最初の依頼として使うこと。

## 作業依頼

`/Users/hiroyuki_furuno/works/private/katana-document-viewer` で KDV v0.2.0 viewer recovery を継続してください。

最初に必ず以下を読んで、現状の未達と禁止事項を把握してください。

- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/handoff-kdv-v0.2.0-viewer-recovery-2026-06-13.md`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/openspec/changes/v0-2-0-markdown-viewer-kuc-integration/user-feedback-todo.md`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/openspec/changes/v0-2-0-markdown-viewer-kuc-integration/design.md`
- `/Users/hiroyuki_furuno/works/private/katana-document-viewer/openspec/changes/v0-2-0-markdown-viewer-kuc-integration/specs/markdown-viewer-kuc-integration/spec.md`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core`
- `/Users/hiroyuki_furuno/works/private/katana`

## 最終目標

`just storybook` で KUC 実部品を使った Katana 由来 viewer を interactive 起動し、KatanA viewer / export HTML / export PDF と同等の表示・操作・性能を満たしてください。

完了条件は以下です。

- KDV core は vendor 非依存。
- KUC が共通 UI 契約の唯一の実部品層。
- Storybook は KUC 部品を host するだけ。
- visual / semantic / interaction / performance score が全カテゴリ 95 点以上。
- score は KDV 自己比較ではなく KatanA reference artifact と比較する。
- `just storybook` は smoke test ではなく interactive viewer を起動する。

## 絶対にやらないこと

- KDV Storybook 側で TreeView / SettingsList / Toggle / Button / Media control の座標判定を再実装しない。
- KDV Storybook 側で `state_id` / `style_class` / 文字列 parse から action を復元しない。
- KDV 側で KUC の見た目だけを真似た独自 widget を追加しない。
- fallback で別 renderer に逃がさない。
- egui / gpui / floem adapter を先に戻さない。
- score gate を自己比較や甘い閾値で通さない。
- dirty worktree を reset / checkout / clean で戻さない。

## 直近で最優先に直すこと

1. `asset job result send failed: sending on a closed channel` を直す。
   - cancel 済み asset job の channel close は正常破棄としてログを出さない。
   - 未 cancel の channel close は異常としてログを残す。
   - 対象: `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/window_asset_job.rs`

2. code block copy を直す。
   - `HostCommand::CopyText(String)` は text だけなので不十分。
   - copy source と target を持つ typed command にする。
   - Storybook host は clipboard に書き込み、code copy 後に該当 button を check mark 表示にする。
   - 対象:
     - `/Users/hiroyuki_furuno/works/private/katana-document-viewer/crates/katana-document-viewer/src/viewer/commands.rs`
     - `/Users/hiroyuki_furuno/works/private/katana-document-viewer/crates/katana-document-viewer/src/viewer/commands_factory.rs`
     - `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/window_command.rs`
     - `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core-storybook/src/document_viewer/node_factory_code.rs`

3. toggle / SettingsList の反応遅延と hover 領域を直す。
   - hover border は toggle 単体ではなく label + control を含む設定行全体。
   - KUC SettingsList の row interaction contract で表現する。
   - KDV 側で座標補正や action 合成をしない。
   - toggle / hover / selection の変更で不要な asset job を再起動しない。
   - 対象:
     - `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/src/molecule/app_primitives/settings/hit_test.rs`
     - `/Users/hiroyuki_furuno/works/private/katana-ui-core/crates/katana-ui-core/src/molecule/app_primitives/settings/render_tests.rs`
     - `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/settings_action.rs`
     - `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/window_scene.rs`
     - `/Users/hiroyuki_furuno/works/private/katana-document-viewer/tools/kdv-storybook/src/preview_cache.rs`

4. SVG / diagram 表示を KatanA と同等にする。
   - KatanA 側の SVG/RGBA/Retina/cache 実装を先に読む。
   - scroll で同一図形を再ロードしない。
   - 一度生成した SVG / surface は source hash + path + AST位置 + theme + DPI + renderer options で cache する。

5. Markdown viewer 本体の未達を潰す。
   - task checkbox は KUC checkbox を使い、click で session state を変える。
   - link hover / click / cursor を KUC text span action で動かす。
   - accordion を KUC action 経由で開閉する。
   - OS emoji / special characters は KatanA/egui の白黒化や共通化を踏襲せず、KUC/KDV 独自仕様として OS 依存 glyph をそのまま表示する。
   - alert / list / code block / horizontal rule / table / footnote を KatanA reference と比較して直す。
   - scroll 末尾が途切れないよう bottom spacer を入れる。

## 作業手順

1. まず `just storybook` を起動して、現状の不具合を再確認する。
2. 直近最優先の 1 から順に、失敗するテストを追加してから実装修正する。
3. 変更ごとに OpenSpec change 配下の `user-feedback-todo.md` の該当項目へ対応状況を書く。
4. KUC 側を直すべきものは KUC 側で直す。KDV 側で部品挙動を再実装しない。
5. 1つ直したら必ず関連 gate を通してから次へ進む。

## 検証コマンド

コマンド実行は必ず `/opt/homebrew/bin/rtk` を先頭に付ける。

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

## 注意

現在の KDV repo は dirty / untracked が多い。`git reset --hard`、`git checkout .`、`git clean -fd` は禁止。

subagent を使う場合は、完了後すぐ `close_agent` する。放置しない。
