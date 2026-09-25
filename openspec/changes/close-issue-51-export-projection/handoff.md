# Issue #51 Export Projection Handoff

## Execution boundary

- The projection spans the viewer plan, height calculation, Storybook adapter,
  and pixel-parity assertions. Each step exposes the typed payload consumed by
  the next, so the implementation and diagnosis remained one ordered path.

## Delegation record

- The implementation and focused verification remained in the main task.
  delegation-exception: `直列のクリティカルパス` / file:
  `openspec/changes/close-issue-51-export-projection/tasks.md`.

## Current release boundary

- Focused core and adapter regressions pass without threshold, baseline, or
  dependency-override changes.
- With registry-only KUC `0.3.12` and the refreshed compatible lockfile,
  `just fmt-check`, `just ast-lint`, `just JOBS=2 lint`, and strict OpenSpec
  validation pass. The focused table-grid test, seven export-diagram tests,
  and four sample export-tree geometry tests also pass.
- The direct diagram fixture initially scored 90/95. KUC `0.3.12`'s
  `ExportMediaFrame` renderer itself places the image 18 px below the frame
  origin, while KDV's adapter also applied an explicit 18 px child margin.
  Removing that duplicate margin raised the score to 93/95; KUC's child-height
  clip then still removed the bottom 18 px. Setting the child clip to the full
  already-reserved wrapper height retained one renderer-owned top margin and
  the bottom margin without changing the authored block rectangle.
- With that owner-layer correction, `direct/sample.md`, all ten diagram-heavy
  surface fixtures, and all five fast surface fixtures pass the unchanged
  95-point threshold. The focused no-double-margin and wrapper geometry tests
  pass. No threshold, reference, fixture, or dependency override was changed.
- The current committed HEAD and public KUC viewport-hit repair are still
  required before Issue #51 is closed or the release advances. The KDV-wide
  Storybook registry consumer gate still fails on KUC `0.3.12`'s separate
  accordion viewport-hit regression.
- A subsequent patch-API self-review exposed a `ViewerNode` public-field
  addition. The candidate now keeps the v0.5.5 struct shape and derives a
  typed snapshot projection map by `node_id` for the internal Storybook
  adapter. `just semver-check` passes 196/196 checks; the new typed-map
  regression and strict Clippy/AST lint pass. Fast five-fixture and diagram-heavy
  ten-fixture parity also pass on this updated candidate at the unchanged 95
  threshold. Issue progress is recorded at
  <https://github.com/HiroyukiFuruno/katana-document-viewer/issues/51#issuecomment-5790303848>.
- On 2026-09-23, a command quoting error unintentionally ran `cargo clean` in
  this worktree, removing 26.5 GiB of regenerable `target` outputs. Git source
  status was verified unchanged; binaries/artifacts must be rebuilt for any
  later gate, and no pre-clean run substitutes for final current-HEAD evidence.
