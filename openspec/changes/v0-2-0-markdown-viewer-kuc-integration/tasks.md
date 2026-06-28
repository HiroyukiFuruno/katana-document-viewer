# Tasks: katana-document-viewer v0.2.0 markdown viewer KUC integration

## 現在の完了判定

- `tasks.md` はOpenSpec artifactとしての完了済み作業一覧を保持する。
- 現時点の計画書は `kdv-v0.2.0-viewer-recovery-plan.md` とする。
- 現時点の引き継ぎ入口は `handoff-current-2026-06-13.md` とする。
- 現時点の残Task正本は `remaining-plan.md` とする。
- ユーザー実機指摘の未対応台帳は `user-feedback-todo.md` とする。
- 詳細な未達履歴は `handoff-unresolved-2026-06-12.md` を参照する。
- 既存の `handoff.md` は 2026-06-03 時点の履歴資料であり、現在の完了判定正本として扱わない。
- `remaining-plan.md` の未完了項目が残っている間は、v0.2.0 viewer parity完了とは扱わない。
- `user-feedback-todo.md` に未対応 `[ ]` が残っている間も、v0.2.0 viewer parity完了とは扱わない。
- `just storybook-score-check` や個別 gate が通っていても、KatanA viewer / slideshow との visual / semantic / interaction / performance 95点以上と、ユーザー実機指摘の消し込みが揃うまでは完了扱いにしない。

## Current Done Items

- `rtk ./scripts/openspec validate v0-2-0-markdown-viewer-kuc-integration --strict --no-interactive` が通る。
- KDV core の解析、render、viewer node 生成は UI vendor 非依存を維持する。
- KDV UI表示の検証入口はKUCをベースにする。
- vendor-free Storybookの実描画はローカルKUCの `UiTreeCanvasRenderer` を使う。KDV側の旧独自canvas renderer群は `tmp/archive/kdv-storybook-old-renderer-2026-06-02/` へ退避済み。
- `just storybook` はテストだけではなく、vendor-free KUC preview windowをinteractive起動する。
- `scripts/check-storybook-entrypoint.sh` は `just storybook` がtest-only / smoke-onlyへ戻る退化をfail fastする。
- `storybook-content-check` はKUC Storybook feature matrix、fixture score matrix、direct source matrix、export quality、surface equivalenceを実行する。
- `storybook-check` はentrypoint、Storybook contract、content gate、window smoke、KUC smoke、performance checkを含む。
- `frame_tests.rs` はKUC feature matrixがStorybook preview領域の実frame pixelまたはImageSurface数へ届くことを検証する。
- `frame_interaction_tests.rs` はsearch highlight、Slideshow mode、media control toggleがStorybook preview領域の実frame pixelへ届くことを検証する。
- `window_tests.rs` はwindow resize検出だけでStorybook scene viewport更新対象になることを検証する。
- `ViewerCommandFactory` はlink clickとtask checkbox/context menuをKMM metadata付きviewer commandへ正規化する。
- `kuc-adapter-boundary-check` はKDV core / preview / KUC viewer / vendor-free Storybook / local KUC coreのvendor runtime依存混入を検出する。
- `kuc-adapter-boundary-check` はvendor-free Storybookへ旧KDV独自renderer、独自Tree構築、vendor runtime参照が戻る退化も検出する。
- `just check` と `just storybook-check` は `kuc-adapter-boundary-check` を含む。
- `rtk cargo test -p kdv-storybook --locked frame -- --test-threads=1` が通る。
- `rtk cargo test -p kdv-storybook --locked window -- --test-threads=1` が通る。
- `rtk cargo test -p katana-document-viewer --locked commands -- --test-threads=1` が通る。
- `rtk cargo test -p katana-document-viewer --locked viewer -- --test-threads=1` が通る。
- `rtk just kuc-adapter-boundary-check` が通る。
- `rtk just storybook-content-check` が通る。
- `rtk just storybook-check` が通る。
- `rtk cargo test -p katana-ui-core-storybook --locked ui_tree_canvas -- --test-threads=1` が通る。
- `rtk just ast-lint` が通る。

<!-- subagent-spark-harness-strict-start -->

- [x] direct visual score gateの監査と修正は分離作業で実施済み。証跡: agent: `019e8377-49be-7e20-b0b3-8bfef655c636` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `crates/katana-document-viewer/src/export_quality/html_score_direct_visual.rs` / file: `crates/katana-document-viewer/src/export_quality/html_score_direct_visual_helper_tests.rs` / file: `crates/katana-document-viewer/src/export_quality/html_score_direct_visual_tests.rs` / file: `crates/katana-document-viewer/src/export_quality/html_score.rs` / file: `crates/katana-document-viewer/src/export_quality/html_score_tests.rs` / command: `multi_agent_v1.spawn_agent` / close: `multi_agent_v1.close_agent` / verify: `rtk cargo test -p katana-document-viewer --release --locked whitespace -- --test-threads=1`
- [x] 分離作業ハーネスを `just check` とCIへ接続し、関連ファイルの証跡漏れをfail fastする。証跡: agent: `019e7fb2-ddac-70d0-b777-3b700c7330e1` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `.codex/workflows/subagent-spark-policy.md` / file: `.github/workflows/test-and-build.yml` / file: `Justfile` / file: `scripts/check-subagent-spark-harness.sh` / file: `scripts/check-subagent-spark-harness-tests.sh` / file: `scripts/check-subagent-spark-harness-edge-tests.sh` / file: `scripts/check-subagent-spark-harness-policy-tests.sh` / file: `scripts/check-subagent-spark-harness-change-tests.sh` / file: `scripts/check-subagent-spark-harness-verify-tests.sh` / file: `scripts/check-subagent-spark-harness-coverage-tests.sh` / file: `scripts/check-subagent-spark-harness-ci-tests.sh` / file: `scripts/check-subagent-spark-harness-diff-tests.sh` / file: `scripts/subagent-spark-harness-lib.sh` / file: `scripts/subagent-spark-harness-ci.sh` / file: `scripts/subagent-spark-harness-diff.sh` / file: `scripts/subagent-spark-harness-contracts.sh` / file: `scripts/subagent-spark-harness-terms.sh` / file: `scripts/subagent-spark-harness-change.sh` / file: `scripts/subagent-spark-harness-evidence.sh` / file: `scripts/subagent-spark-harness-verify.sh` / command: `multi_agent_v1.spawn_agent` / close: `multi_agent_v1.close_agent` / verify: `rtk just check-subagent-harness`
- `storybook-content-check` を復元し、`storybook-check` の必須gateへ組み込んだ。KUC Storybook feature matrix、fixture score matrix、direct source matrix、export quality、surface equivalenceがStorybook検証から外れても通る偽陽性を塞いだ。
