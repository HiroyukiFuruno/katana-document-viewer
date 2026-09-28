# ADR: KUC installed boundary and ContextMenu typed actions

Date: 2026-06-23

## Status

Accepted for v0.2.0 recovery planning. KUC issue
[#7](https://github.com/HiroyukiFuruno/katana-ui-core/issues/7) has a local
typed-action implementation in this recovery worktree; release readiness still
requires verification and user acceptance.

## Context

KDV v0.2.0 must use KUC as the only real UI component/action layer. KUC v0.1.0
is moving toward a cargo-install / self-contained crate boundary, so KDV cannot
keep treating the sibling KUC working tree as a private patch surface.

The immediate failure was task-list secondary-click behavior. KDV opened a
KUC-rendered context menu, then mapped the selected item id back into task state
with `viewer_task_state_from_context_menu_item_id()`. That meant the downstream
host reconstructed behavior from item ids instead of receiving a typed action
from KUC. It also left the clicked row identity split between the original
right-click hit and the later menu item selection.

This violates the recovery rule that KDV Storybook must not rebuild behavior
from `state_id`, `style_class`, labels, item ids, strings, or local coordinate
patches.

## Decision

KDV will treat KUC context-menu typed item actions as an upstream dependency.
KDV will not fix task context menu target drift by adding item-id parsing,
state-id parsing, row-coordinate expansion, or KDV-local context-menu behavior.

KUC issue #7 is the required upstream contract:

- `ContextMenuItem` / `UiContextMenuItem` accepts an optional typed action.
- Host-action collection returns that typed item action when present.
- KUC Storybook/canvas exposes selected menu item action, not only item id.
- Task context-menu items carry target identity and requested marker/state.
- Untyped menu items keep the current id/path behavior for compatibility.

KDV now consumes KUC `context_menu_host_action_at(...)` and
`task_control_state_action()` for task context-menu selection. The task marker,
target node id, row index, and state id travel as typed action payload rather
than being recovered from the context-menu item id.

## Consequences

- KDV can continue using KUC through the consumer-side host boundary, but KDV
  must not add behavior that should belong to KUC.
- KDV release readiness is still blocked by user acceptance and by any
  user-visible context-menu target drift that reappears during interactive
  review.
- KDV removed the item-id bridge from the context-menu selection path and added
  a regression for multiple tasks in one list where only the right-clicked row
  changes.
- KUC changes must be additive and backward compatible for v0.1.0.

## Verification Required After KUC #7

- KUC core test: typed context-menu item action is preserved in
  `UiHostActionPlan`.
- KUC Storybook/canvas test: `context_menu_host_action_at(...)` returns the
  selected item typed action.
- KDV regression: right-click a non-first task row, choose another marker, and
  assert only the right-clicked row changes.
- KDV regression guard: context-menu selection path contains no item-id, marker
  string, `state_id`, or style-class action reconstruction.
