## Why

Issue #51 found that the Storybook projection discards table structure and can
give diagram media a wrapper geometry different from KDV's export surface. This
permits surface-parity drift that image offset tolerance cannot correctly
diagnose, so the release must preserve the evaluated document geometry as typed
KUC nodes.

## What Changes

- Project evaluated table rows, cells, spans, alignment, and wrapped-row height
  into the Storybook KUC tree without flattening them to a text node.
- Align diagram media wrapper and image geometry with the export block contract,
  including reserved vertical margins.
- Replace offset-tolerant parity expectations with structural and geometry
  assertions for tables and diagrams.
- Keep visual-score thresholds, reference assets, scale compensation, and
  dependency overrides unchanged.

## Capabilities

### New Capabilities

- `storybook-export-structure-projection`: Typed KUC projection of evaluated
  table and diagram geometry that matches the export surface contract.

### Modified Capabilities

- None.

## Impact

- Affects the KDV viewer projection model and `tools/kdv-storybook` node
  factory, parity tests, and focused geometry regressions.
- Does not change public KUC APIs, visual thresholds, reference artifacts, or
  sibling repositories.
