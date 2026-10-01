## ADDED Requirements

### Requirement: Grid border entries are available for a single-pass host projection

`DocumentSurfaceFrame` MUST expose a public, read-only batch accessor that returns every
stored `(DocumentGridCoordinate, DocumentGridCellBorders)` entry without copying,
recalculating, or performing a coordinate lookup per returned entry. The returned entries
MUST preserve grid input order.

#### Scenario: Host projects visible grid borders in one traversal
- **WHEN** a host receives a grid `DocumentSurfaceFrame` and iterates its batch border entries once
- **THEN** it can obtain every coordinate and all four projected border sides in time proportional
  to the number of returned entries

#### Scenario: Non-grid frame has no border entries
- **WHEN** a host receives a page `DocumentSurfaceFrame`
- **THEN** the batch accessor returns an empty read-only collection

### Requirement: Batch projection preserves existing border semantics

The batch accessor MUST return the same border value as `grid_cell_borders` for every
returned coordinate, including style, color, `none`/missing sides, and merged-cell coordinates.
It MUST NOT remove entries because all or some sides are absent.

#### Scenario: Mixed border states remain observable
- **WHEN** a spreadsheet frame contains visible, `none`, missing, and merged-cell border states
- **THEN** the batch collection contains their original coordinates and values without filtering

### Requirement: Existing single-cell lookup remains compatible

`DocumentSurfaceFrame::grid_cell_borders` MUST remain public and preserve its existing return
semantics while the batch accessor is added.

#### Scenario: Existing consumer uses single-cell lookup
- **WHEN** an existing consumer requests one coordinate through `grid_cell_borders`
- **THEN** it receives the same value it received before the batch accessor was introduced
