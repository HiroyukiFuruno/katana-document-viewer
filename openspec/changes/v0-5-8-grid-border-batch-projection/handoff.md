# v0.5.8 Grid Border Batch Projection Handoff

## Ownership

- KDV owns the public, read-only `DocumentSurfaceFrame` projection of neutral
  grid coordinates and four-side borders.
- KUC remains an internal rendering dependency; this change must not expose
  its types through KDV's public API.
- KatanA owns host-side batch consumption after KDV's published registry
  release and must not duplicate KDV's document-frame projection.

## Verified Work

- `grid_cell_border_entries` returns the stored coordinate and border pairs in
  input order without allocation, while `grid_cell_borders` remains available
  for compatibility.
- Unit regressions cover mixed border styles, absent and `none` sides,
  materialized spreadsheet grids, and a large one-pass host-style projection.
- KUC is pinned to the published `0.4.0` registry artifact; the dependency
  boundary, semver, V8 singleton, and release contract checks have evidence in
  `evidence/dependency-evaluation.md`.

## Execution

- The remaining Draft PR review, required checks, publication, and fresh
  registry-consumer verification are a serial critical path in the main task.
  delegation-exception: `直列のクリティカルパス` / file:
  `openspec/changes/v0-5-8-grid-border-batch-projection/tasks.md`

## Release Work

1. Re-run the full gate on the committed head, then create the Draft PR.
2. Resolve required review findings before marking the PR ready and merging.
3. Verify the GitHub Release, crates.io artifact, and a fresh registry-only
   consumer before closing Issue #56 and handing the published API to KatanA.
