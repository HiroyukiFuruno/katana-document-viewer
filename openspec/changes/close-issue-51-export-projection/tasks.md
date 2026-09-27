<!-- subagent-spark-harness-strict-start -->

## 1. Evaluated projection contract

- [x] 1.1 Trace evaluated table and media payloads through the viewer-plan and Storybook boundary, recording missing typed attributes. delegation-exception: `直列のクリティカルパス`。証跡: file: `design.md`; file: `types_table.rs`; file: `node_factory_media_impl.rs`.
- [x] 1.2 Extend the KDV-owned typed projection payload for table rows, cells, spans, alignment, wrapped height, and media block margins without adding a field to public `ViewerNode`. delegation-exception: `直列のクリティカルパス`。
  証跡: `types_table.rs`; `builder_node_resolve.rs`; `just semver-check` 196/196 PASS.
- [x] 1.3 Add core structural tests that reject a flattened table projection. delegation-exception: `直列のクリティカルパス`。
  証跡: `types_table_tests.rs`; `builder_html_height_tests.rs`.

## 2. Storybook adapter

- [x] 2.1 Construct table KUC grids from the typed payload, preserving cell geometry, spans, alignment, and accessibility text. delegation-exception: `直列のクリティカルパス`。
  証跡: `node_factory_table.rs`; `node_factory_table_tests.rs`.
- [x] 2.2 Reserve diagram export-block geometry for pending and loaded media assets, with images inside authored margins. delegation-exception: `直列のクリティカルパス`。
  証跡: `node_factory_media_impl.rs`; `node_factory_media_controls.rs`.
- [x] 2.3 Add focused adapter geometry tests for table grids and diagram wrapper/image rectangles. delegation-exception: `直列のクリティカルパス`。
  証跡: `node_factory_table_tests.rs`; `node_factory_media_tests.rs`.

## 3. Parity and release evidence

- [x] 3.1 Replace offset-tolerant surface-parity expectations with direct table/diagram geometry assertions. delegation-exception: `直列のクリティカルパス`。
  証跡: `frame_surface_parity_tests.rs` direct table/diagram assertions.
- [x] 3.2 Run focused core and Storybook tests without threshold, reference, scale, or dependency-override changes. delegation-exception: `直列のクリティカルパス`。証跡: command: `rtk proxy cargo test -p katana-document-viewer planner_preserves_typed_table_cells_and_alignment_without_raw_text_parsing --locked`; command: `rtk proxy cargo test -p kdv-storybook table_node_preserves_rows_cells_alignment_wrapping_and_geometry --locked`.
- [ ] 3.3 Run strict lint, AST lint, the applicable Storybook parity gate, and record current-HEAD evidence before closing Issue #51. delegation-exception: `直列のクリティカルパス`。
  - [/] 2026-09-23再検証: core library `1918 pass / 0 fail / 1 ignored`、strict Clippy、AST lint、fmt、OpenSpec strict、semver `196/196`、table/diagram focused回帰、fast/diagram-heavy export-surface parity両方がPASS。Storybook全件は`658 pass / 1 fail / 21 ignored`で唯一の失敗がKUC 0.3.12のdeep-scroll viewport hit。KUC #52にAPI間差異と実window画素変化を引き継いだ。候補は未commit/未push、current-HEAD review/full release gate未完了のため本項は閉じない。delegation-exception: `直列のクリティカルパス`。証跡: command: `rtk proxy cargo test -p katana-document-viewer --lib --locked -- --test-threads=2`; command: `rtk proxy cargo test -p kdv-storybook storybook_frame_matches_export_surface_for_katana_viewer --locked -- --ignored --test-threads=1 --nocapture`; issue: <https://github.com/HiroyukiFuruno/katana-ui-core/issues/52#issuecomment-5794768341>。
  - [x] 2026-09-23 strict coverage再実行: 型付き投影がないTableの高さfallbackへ回帰テストを追加し、core `1919 pass / 0 fail / 1 ignored`、functions `3674/3674`・lines `30127/30127`（100%）で`just coverage`がexit 0。delegation-exception: `直列のクリティカルパス`。証跡: command: `rtk proxy cargo test -p katana-document-viewer --lib table_without_typed_projection_uses_wrapped_body_height --locked`; command: `rtk proxy just coverage`。
  - [/] 2026-09-23 candidate: `just fmt-check`, `just ast-lint`, `just JOBS=2 lint`, strict OpenSpec validation, direct/sample.md、全fast/diagram-heavy export-surface parity が通過。KUC ExportMediaFrame のrenderer-owned 18px余白とclipを二重適用しない回帰テストも通過。current-HEAD commit/push、KUC公開patchでの全Storybook gate、reviewは未完了。delegation-exception: `直列のクリティカルパス`。証跡: file: `handoff.md`; Issue comment: <https://github.com/HiroyukiFuruno/katana-document-viewer/issues/51#issuecomment-5786694936>。
  - [/] 自己レビューで公開`ViewerNode`へのfield追加が`constructible_struct_adds_field`と判明。公開型を復元し、typed snapshotから`node_id`で投影する内部adapterへ修正。focused core/Storybook tests、strict Clippy、`just semver-check` 196/196、fast/diagram-heavy parityは通過。公開KUC patchでのfull gateとcurrent-HEAD reviewは未完了。delegation-exception: `直列のクリティカルパス`。証跡: command: `rtk proxy just semver-check`; command: `rtk proxy cargo test -p kdv-storybook storybook_frame_matches_export_surface_for_katana_viewer --locked -- --ignored`。

## Focused evidence

- `cargo test -p katana-document-viewer planner_preserves_typed_table_cells_and_alignment_without_raw_text_parsing --locked`
- `cargo test -p katana-document-viewer table_projection_preserves_rows_alignment_and_export_geometry --locked`
- `cargo test -p katana-document-viewer empty_table_projection_has_no_geometry --locked`
- `cargo test -p kdv-storybook table_node_preserves_rows_cells_alignment_wrapping_and_geometry --locked`
- `cargo test -p kdv-storybook pending_export_diagram_reserves_the_planned_block_height --locked`
- `cargo test -p kdv-storybook export_surface_diagram_frame_uses_one_renderer_owned_vertical_margin --locked`
- `cargo test -p kdv-storybook katana_sample_export_surface_tree_places_table_at_target_plus_padding --locked`
- `cargo test -p kdv-storybook katana_sample_export_surface_tree_preserves_explicit_diagram_wrapper_geometry --locked`
