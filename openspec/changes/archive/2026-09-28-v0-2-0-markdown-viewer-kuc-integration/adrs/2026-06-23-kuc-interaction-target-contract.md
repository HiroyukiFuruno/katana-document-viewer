# ADR: KUC interaction target contract

Date: 2026-06-23

## Status

Accepted for v0.2.0 recovery planning. Implementation is blocked on KUC issue
[#8](https://github.com/HiroyukiFuruno/katana-ui-core/issues/8).

## Context

KDV Storybook currently depends on KUC-rendered UI for file navigation,
settings, text spans, and media controls. User-visible regressions have
repeatedly appeared as hover area drift, cursor/action mismatch, row hit target
drift, and control click-point drift.

Fixing these in KDV by rebuilding TreeView, FileTree, SettingsList, Toggle,
Button, link-like span, or media control geometry would violate the recovery
rule that KUC is the only real UI component/action layer.

The component behavior must follow the same idea as HTML/React event handlers:
default components may do nothing, but interactive components need a typed
action/event inlet that host applications can override without parsing labels,
style classes, state ids, or coordinates.

## Decision

KDV will treat KUC unified interaction targets as an upstream dependency. KDV
will not patch hover, cursor, row action, or button action drift by adding
coordinate constants, control-lane reconstruction, style-class parsing, or
string-based action recovery.

KUC issue #8 is the required upstream contract:

- TreeView and FileTree rows expose a typed interaction target for hover,
  cursor, selection, and row action.
- SettingsList rows expose row-level hover and action targets that include both
  label and control area.
- Toggle and Button expose typed action injection through component builder APIs,
  not only by direct `UiCommonProps` manipulation.
- link-like span targets expose hover, cursor, and typed click action.
- media control targets expose typed action, hover visual id, and cursor.
- The contract supports typed action injection by defaulting to no-op behavior
  while allowing consumers to supply actions explicitly.

Until #8 is implemented and released/consumable by KDV, KDV must keep the
interaction-target drift risk open and must not mark release ready solely from
Storybook-side coordinate fixes.

## Consequences

- KDV can host KUC output, but KDV must not become the owner of KUC component
  behavior.
- KDV release readiness remains blocked by any user-visible hover/click/cursor
  drift not covered by KUC typed interaction targets.
- After KUC #8 lands, KDV Storybook must consume KUC interaction targets and
  remove local geometry/action reconstruction from sidebar, settings, link, and
  media control paths.
- KUC changes must be generic and additive for v0.1.0; Katana/KDV-specific
  enum names do not belong in KUC core.

## Verification Required After KUC #8

- KUC core tests: every interactive component exposes a typed target/action
  contract while preserving no-op defaults.
- KUC Storybook/canvas tests: hit queries return the same target identity used
  for hover, cursor, and action dispatch.
- KDV regression: sidebar FileTree, SettingsList row toggle, link-like spans,
  and media controls operate through KUC interaction targets only.
- KDV regression guard: no TreeView/FileTree/SettingsList/Toggle/Button/link-like
  span/media control action path reconstructs behavior from coordinates, labels,
  style classes, or state ids.
