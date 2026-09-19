# KUC v0.3.11 handoff integration plan

## Fixed upstream boundary

- KUC PR: <https://github.com/HiroyukiFuruno/katana-ui-core/pull/58>
- Reviewed candidate: `ec0a4b04555ddd1baa45cb5dc919fa10394bae1b`
- Normal merge commit: `742ebd6cb877667422c1723cb56b298c294272cf`
- Candidate and merge tree: `0ba2dc54eeed9a9c78058530f6ec65f98f5633c6`
- Merge time: `2026-09-13T18:42:20Z`
- Release run: <https://github.com/HiroyukiFuruno/katana-ui-core/actions/runs/34775455520>

At this audit the Release run is still in progress and crates.io still reports
`katana-ui-core 0.3.10`. Therefore the owner worktree remains on exact registry
`=0.3.10`; no path/git/patch override or unpublished `0.3.11` dependency is
introduced. The final dependency update starts only after the KUC owner supplies
the public tag, GitHub Release, crates.io package checksum, and source SHA proof.

The KDV owner worktree is
`/Users/hiroyuki_furuno/works/private/kdv-post-v0.5.5-document-fidelity`, branch
`release/v0.5.6`, base HEAD
`3b4ecb13396c8f067e838cacb2a13087dd88309d`. Its existing dirty release diff is
authoritative and must not be replaced by the diagnostic consumer probe.

## Handoff provenance

The read-only KUC consumer probe contains 46 Rust source/test paths: 45 modified
and one new. Manifests, local `patch.crates-io` configuration, generated assets,
logs, and `target/` are not admissible KDV release inputs.

- `consumer-complete-handoff.md`: SHA-256
  `08dec8a636ec2cede5f7cbd4830492ef7e7ec68b0bb2e5bc5abf93ff4cc5b34c`
- `consumer-complete-source-hashes.json`: SHA-256
  `b290019ee9f8b95061fe5dd1085b4a39f7d49c1f828fef12e7b6fa4489e1dc49`
- `consumer-final-3-result.md`: SHA-256
  `adc8b8cf15d3e0fc937c3c04891319f50a3ac89dda510b9ac721abf58f80c936`

Source paths:

- `/Users/hiroyuki_furuno/works/private/katana-ui-core/tmp/v0311-release/consumer-complete-handoff.md`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/tmp/v0311-release/consumer-complete-source-hashes.json`
- `/Users/hiroyuki_furuno/works/private/katana-ui-core/tmp/v0311-release/consumer-final-3-result.md`

The final unchanged-source probe passed all-target Clippy, KDV library
`1907 passed / 0 failed / 1 ignored`, and Storybook
`645 passed / 0 failed / 21 ignored`. This is diagnostic evidence only;
registry-only owner verification remains mandatory.

After PR #58 merged, `shasum -a 256 -c` was rerun against every entry in
`consumer-complete-source-hashes.json`; all 46 frozen probe paths returned
`OK`.

## 46-path classification

### Baseline-owner paths: 30

For these paths the owner matched the recorded probe baseline during the audit.
After publication, apply only the recorded responsibility hunks and confirm the
pre-apply hash again; do not copy the probe tree wholesale.

- `crates/katana-document-viewer/src/dependency_tests.rs`
- `crates/katana-document-viewer/src/export_surface/export_surface_text.rs`
- `crates/katana-document-viewer/src/export_surface/export_surface_text_html_tests.rs`
- `crates/katana-document-viewer/src/html_sanitizer.rs`
- `crates/katana-document-viewer/src/lib.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/builder_html_heading_tests.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/builder_html_height_tests.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/builder_media_data_image.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/builder_media_height.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/builder_media_text_height.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/builder_media_text_height_tests.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/builder_soft_merge.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/builder_span_line_counter.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/builder_surface_height_tests.rs`
- `crates/katana-document-viewer/src/viewer/node_plan/builder_tests.rs`
- `tools/kdv-storybook/src/document_viewer/node_factory_hover.rs`
- `tools/kdv-storybook/src/document_viewer/node_factory_html_badge.rs`
- `tools/kdv-storybook/src/document_viewer/node_factory_html_badge_parser.rs`
- `tools/kdv-storybook/src/document_viewer/node_factory_html_image.rs`
- `tools/kdv-storybook/src/document_viewer/node_factory_html_image_tests.rs`
- `tools/kdv-storybook/src/document_viewer/node_factory_html_media_tests.rs`
- `tools/kdv-storybook/src/document_viewer/node_factory_text.rs`
- `tools/kdv-storybook/src/document_viewer/node_factory_text_role_tests.rs`
- `tools/kdv-storybook/src/document_viewer/node_labels.rs`
- `tools/kdv-storybook/src/frame_interaction_tests.rs`
- `tools/kdv-storybook/src/frame_media_control_tests/hover_tests.rs`
- `tools/kdv-storybook/src/frame_media_control_tests/icon_tests.rs`
- `tools/kdv-storybook/src/frame_media_control_tests/layout_tests.rs`
- `tools/kdv-storybook/src/frame_score_preview_crop_tests.rs`
- `tools/kdv-storybook/src/window/interaction_matrix_tests.rs`

### Owner-diverged paths: 13

These paths require manual hunk integration. The decision below is binding for
the v0.5.6 candidate.

| Path | Hunk decision |
| --- | --- |
| `crates/katana-document-viewer/src/preview_runtime/storybook_score_gate_tests.rs` | Preserve owner score/reference changes. After publication set both exact KUC expectations to `0.3.11`; reject the stale `0.3.7` metadata expectation. |
| `crates/katana-document-viewer/src/viewer/node_plan/builder_node_push.rs` | Adopt native anchor, HTML allocation, and soft-row merge behavior hunk by hunk; preserve unrelated owner work and verify planner regressions. |
| `crates/katana-document-viewer/src/viewer/node_plan/builder_spacing_tests.rs` | Replace obsolete numeric expectations with the final native contract; retain non-conflicting owner coverage only when it passes the unchanged full suite. |
| `tools/kdv-storybook/src/document_viewer/adapter.rs` | Adopt zero horizontal/top interactive padding while retaining owner light/dark assertions and still-valid export top/right padding assertions. |
| `tools/kdv-storybook/src/document_viewer/adapter_layout.rs` | Adopt zero interactive horizontal/bottom padding; retain export padding `56`. |
| `tools/kdv-storybook/src/document_viewer/node_factory.rs` | Integrate centered link row, natural interactive HTML height, `1px` interactive rule versus `2px` export rule, and malformed recovery; preserve KDV semantic IDs and export behavior. |
| `tools/kdv-storybook/src/document_viewer/node_factory_list_tests.rs` | Retain valid owner natural-height and export fixed-height coverage; add final KUC row-height expectations. |
| `tools/kdv-storybook/src/document_viewer/node_factory_tests.rs` | Retain owner export fixed-height/width coverage and integrate the richer natural hover geometry contract. |
| `tools/kdv-storybook/src/document_viewer/node_labels_tests.rs` | Keep the owner's more precise renamed test; do not revert behavior or naming. |
| `tools/kdv-storybook/src/document_viewer_tests.rs` | Update content height and last anchor for zero preview padding while preserving the owner long-document intent. |
| `tools/kdv-storybook/src/preview_scene_from_output.rs` | Adopt scroll extent from the last host target plus viewport. Exclude the diagnostic `KUC_PROBE_NODE_GEOMETRY` env/`eprintln!` hunk. |
| `tools/kdv-storybook/src/preview_tests.rs` | Adopt malformed raw-tail/native viewport expectations and exact description geometry; preserve unrelated owner tests. |
| `tools/kdv-storybook/src/preview_theme_bridge.rs` | Adopt the 0.3.11 H1-H6/native raster and theme-token baseline; preserve KDV-owned color semantics and non-superseded export overrides. |

### Already-probe paths: 2

These paths already matched the frozen probe during the audit. Preserve them and
recheck their hashes after all surrounding hunk integration.

- `tools/kdv-storybook/src/frame_hover/matrix_tests.rs`
- `tools/kdv-storybook/src/mouse_hit_alignment_tests.rs`

### New path: 1

- `crates/katana-document-viewer/src/markdown_line_break.rs`: add the semantic
  soft-line predicate, wire it through `lib.rs`, and retain hard Markdown breaks
  as separate rows.

## Required functional decisions

- `builder_media_height.rs` gains `html_data_image_height()` so a valid centered
  encoded SVG uses its measured `162px` height before native text allocation,
  without double decoding. Malformed input remains on the `21px` native text
  fallback and plain text remains `36px` where specified.
- Raw malformed HTML remains visible text; comments, scripts, non-image tags,
  and ordinary quoted attributes do not become image surfaces.
- Native preview uses viewport `1187x2225`, preview padding `0`, export padding
  `56`, natural H1-H6/text/list heights, and the `Row(28px, Center)` badge row.
- Badge and media-control behavior retains exact segment-width, clipping, hover,
  and hit-rectangle contracts from the handoff.
- Scroll/hover rendering uses the actual KUC host and semantic target; changed
  pixels remain inside the resolved hit rectangle.

## Explicit exclusions

- Do not apply the stale `0.3.7` expectation in
  `storybook_score_gate_tests.rs`.
- Do not ship `KUC_PROBE_NODE_GEOMETRY` diagnostics.
- Do not carry probe manifests, path/git/patch overrides, logs, generated
  artifacts, or `target/` into the KDV release.
- Do not add consumer compensation for KUC P2 issues #59, #60, #61, or #62.
- Do not change timeout, score, coverage, lint, archive safety, or acceptance
  thresholds to make integration pass.

## Integration batches and status

1. **Pending public proof — registry contracts:** verify KUC public
   tag/Release/checksum/source proof, set
   both KDV manifests and source guards to exact `=0.3.11`, update the lockfile
   precisely, and prove registry-only resolution while retaining the currently
   published KRR `0.4.19`. Complete KUC #52 acceptance first. KRR `0.4.20` is a
   later KDV release gate and is applied only after its independent public proof;
   it is not a prerequisite for KUC #52 acceptance.
2. **Implemented — KDV core:** semantic soft breaks, planner merge, media/image
   height, malformed HTML fallback, viewer/export anchors, and regressions. The
   final owner library run passed `1907 / 0 / 1`.
3. **Implemented, runtime gate pending KUC — Storybook native surface:** adapter
   geometry, node factory, natural typography, theme bridge, labels, and export
   preservation. The final source requires KUC 0.3.11 H4-H6 typography APIs.
4. **Implemented, runtime gate pending KUC — media surface:** badges, image
   handling, media controls, and crop contracts.
5. **Implemented, runtime gate pending KUC — interaction surface:** hover, host
   targeting, scrolling, hit alignment, and canonical score/export contracts.
   Probe-only `KUC_PROBE_*` diagnostics were removed after hunk integration.
6. **Partially verified — release gates:** format, AST lint, diff check, task
   harness, strict library Clippy, and the full library suite pass. An
   independent read-only self-review found no P0/P1 and no new P2. Workspace
   all-target Clippy, full Storybook,
   `cargo tree -d`, V8 singleton `152.2.0`, `just check`, release-check,
   package/publish dry-run, score/export, and fresh registry consumer link remain
   after the public KUC/KRR dependency update.

Each batch requires a scoped diff review before the next batch. A changed KUC
or KRR publication SHA invalidates this integration boundary and requires a new
handoff/hash comparison.

## Remaining release work

- After KUC v0.3.11 public proof, apply the two KUC exact-version source-guard
  hunks and manifest/lockfile updates while retaining KRR `0.4.19`; complete the
  canonical sample/diagrams `>=95`, export, hover, and full Storybook KUC #52
  acceptance without P2 compensation.
- After the independent KRR v0.4.20 public proof, adopt its registry artifact
  and run the KDV-wide V8/link and release gates.
- Commit and push the single `release/v0.5.6` branch, obtain a fresh-head review,
  reply/resolve every actual thread, make the Draft PR Ready only with P0/P1
  zero, pass all required OS/preflight checks, and merge normally.
- Verify KDV GitHub Release and crates.io publication, run the fresh registry
  consumer and KatanA packaged adoption, close scoped issues/OpenSpec work, and
  perform branch/worktree cleanup.
