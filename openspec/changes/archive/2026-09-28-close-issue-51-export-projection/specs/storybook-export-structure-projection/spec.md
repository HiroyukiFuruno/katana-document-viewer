## ADDED Requirements

### Requirement: Storybook projects evaluated tables as structured KUC grids

The Storybook adapter SHALL construct a typed KUC grid from each evaluated
table payload and SHALL preserve row order, cell order, spans, alignment, and
wrapped-row height without parsing flattened table text.

#### Scenario: Table structure reaches the KUC tree
- **WHEN** an evaluated viewer plan contains a table with merged and wrapped
  cells
- **THEN** the KUC tree contains a grid with the same row count, cell spans,
  alignment, and cell rectangles as the evaluated table payload

#### Scenario: Flat table text cannot satisfy the projection
- **WHEN** a table node has no typed structural payload
- **THEN** the adapter structural test fails rather than accepting a generic
  text node as the table rendering path

### Requirement: Storybook diagram geometry matches the export block contract

The Storybook adapter SHALL reserve the evaluated export-block rectangle for a
diagram wrapper and SHALL position the image inside its authored vertical
margins for both pending and loaded assets.

#### Scenario: Loaded diagram preserves wrapper margins
- **WHEN** an evaluated diagram block has non-zero top and bottom margins and
  its image asset is loaded
- **THEN** the wrapper and image rectangles match the export block geometry and
  the margins remain reserved

#### Scenario: Parity rejects compensating offsets
- **WHEN** a table or diagram projection changes a block rectangle while an
  overall image offset could make the crop appear closer
- **THEN** the focused geometry and parity tests fail without changing score
  thresholds or reference artifacts
