# ADR: document_viewer harness ownership

Date: 2026-06-24

## Status

Accepted for v0.2.0 recovery planning.

## Context

KDV currently uses KUC as the real UI component/action layer for the KatanA
derived Markdown viewer. The KatanA viewer-specific adapter code lives under the
KUC Storybook source tree as `src/document_viewer.rs` and
`src/document_viewer/*`, but that code imports `katana-document-viewer`.

KUC is consumed by KDV through Cargo dependencies, not through a sibling source
checkout. KDV pins the reviewed KUC package revision in `Cargo.toml` /
`Cargo.lock` and treats the sibling KUC workspace as upstream development
evidence only.

The KUC `v0.1.0` tag does not contain the typed context-menu host action API
required by KDV. Using that tag would force KDV back to forbidden downstream
item-id / string / `state_id` action recovery. KDV therefore pins
`katana-ui-core` and `katana-ui-core-storybook` to KUC `v0.1.1`.

In the packaged KUC boundary, `katana-ui-core-storybook` does not own the
KatanA viewer-specific `document_viewer` harness. Therefore:

- `cargo test -p katana-ui-core-storybook --locked document_viewer -- --test-threads=1`
  returns `0 passed`.
- That command is not release proof for the KDV KatanA viewer path.
- Re-adding a KDV dependency and root export to KUC Storybook would make KUC own
  KDV-specific harness behavior and contradict the packaged KUC boundary.

## Decision

KDV owns the KatanA document_viewer harness.

KUC remains the generic UI component/action layer. KUC should own generic
TreeView, SettingsList, Toggle, Button, text span, context menu, and media
control contracts. KUC should not own KDV-specific viewer fixtures, KMM/KDV node
planning, or KatanA Markdown viewer regression tests.

KDV keeps the KatanA viewer harness under
`tools/kdv-storybook/src/document_viewer.rs` and
`tools/kdv-storybook/src/document_viewer/*`. KDV must not include that harness
from `../katana-ui-core` with `#[path = ...]`.

The KUC dependency proof is:

```text
katana-ui-core = { git = "https://github.com/HiroyukiFuruno/katana-ui-core.git", tag = "v0.1.1" }
katana-ui-core-storybook = { git = "https://github.com/HiroyukiFuruno/katana-ui-core.git", tag = "v0.1.1" }
```

`Cargo.lock` must pin both packages to
`50eef732edaf2b8b29f55e74827a4f3ac8693243`.

The KDV release proof for the viewer-specific `document_viewer` harness is:

```text
cargo test -p kdv-storybook --locked document_viewer -- --test-threads=1
```

The KUC command below is kept only as historical evidence when it appears in old
notes. It must not be listed as a current required release command:

```text
cargo test -p katana-ui-core-storybook --locked document_viewer -- --test-threads=1
```

## Consequences

- KDV consumes KUC through Cargo package dependencies and KUC render model APIs.
  KDV release readiness is proven by `kdv-storybook` document_viewer
  regressions.
- KUC remains self-contained and does not need a dependency on
  `katana-document-viewer`.
- KUC issue #8 remains the upstream blocker for generic interaction targets.
  This ADR does not close TreeView / FileTree / SettingsList / Toggle / Button /
  link-like span / media control typed target work.
- Handoff and release instructions must require the KDV document_viewer
  regression command, not the KUC 0-test filter command.

## Verification

- KUC current evidence:
  `/opt/homebrew/bin/rtk cargo test -p katana-ui-core-storybook --locked document_viewer -- --test-threads=1`
  reports `0 passed`.
- KDV current evidence:
  `/opt/homebrew/bin/rtk cargo test -p kdv-storybook --locked document_viewer -- --test-threads=1`
  reports viewer-specific document_viewer tests and is the KDV release proof.
- KDV release DoD checks this ADR, the handoff command set, and the KDV
  replacement command so the stale KUC command cannot silently become current
  release proof again.
- KDV release DoD checks `Cargo.toml`, `Cargo.lock`, and
  `tools/kdv-storybook/src/main.rs` so sibling KUC path dependencies or
  sibling `#[path]` source includes cannot silently return.
