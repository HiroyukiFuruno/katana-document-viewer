## Context

The export surface already evaluates tables and diagrams with structured block
geometry. The Storybook projection is a separate KUC adapter: table semantics
must survive this boundary, and media wrapper geometry must not replace the
evaluated block rectangle with only an intrinsic image height. Issue #51
requires a measurable contract rather than a compensating image offset.

## Goals / Non-Goals

**Goals:**

- Carry table rows, cells, spans, alignment, and wrapped height from the
  evaluated viewer plan into a KUC grid.
- Use the export block rectangle as the geometry authority for diagram wrapper
  and image placement.
- Test structure and geometry directly, then retain the existing parity gate
  without offset tolerance.

**Non-Goals:**

- Change visual score thresholds, reference images, viewport scaling, or KUC
  APIs.
- Reinterpret Markdown or introduce a KatanA-specific layout correction.

## Decisions

1. KDV's evaluated viewer plan remains the sole source of table/media data;
   the adapter consumes typed KMM table projections keyed by the plan's node
   IDs instead of parsing `ViewerNode.text`. `ViewerNode` itself retains its
   v0.5.5 public struct shape for patch compatibility. This preserves source
   ownership and makes missing table data a typed test failure rather than a
   text-layout approximation.
2. The adapter constructs a KUC grid only from the table payload. Cell span,
   horizontal/vertical alignment, and wrapped-line height are set before the
   node is attached. Flattening the table into a generic text node is rejected
   by structural tests.
3. Diagram wrappers reserve the planned export-block height and place the
   image within its authored margins. Intrinsic image height is content data,
   not a substitute for wrapper geometry.
4. Parity tests assert table cell rectangles and diagram wrapper/image
   rectangles directly. Global x/y offset allowances are removed because they
   can hide accumulated block-height errors.

## Risks / Trade-offs

- [Risk] KUC's public grid cannot represent an evaluated table attribute. →
  Mitigation: prove the missing public representation with a focused adapter
  test and create a KUC Issue before any sibling-repository change.
- [Risk] Existing fixtures expose legacy flat table text. → Mitigation: retain
  text only as accessibility content while treating typed structural payload as
  the rendering authority.
- [Risk] Adding a table field to public `ViewerNode` breaks existing struct
  literals. → Mitigation: derive an additive projection map from the snapshot,
  pass it through only the Storybook adapter, and enforce `just semver-check`.
- [Risk] Diagram assets are asynchronous. → Mitigation: geometry derives from
  the planned block before asset completion and is checked for pending and
  loaded states.
