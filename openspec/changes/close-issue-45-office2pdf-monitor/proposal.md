## Why

KDV now consumes the official, exact registry `office2pdf =0.7.0`, but there
is no repeatable way to notice a newer upstream release and prove that adopting
it preserves KDV's security, fidelity, performance, and downstream-release
contracts. A scheduled monitor keeps this maintenance decision evidence-based
without reverting to a fork or silently changing a dependency.

## What Changes

- Add a daily and manually dispatchable GitHub Actions workflow that compares
  KDV's exact `office2pdf` dependency with the latest unyanked crates.io
  release and upstream GitHub release/tag metadata.
- Add a deterministic monitor script with offline self-tests and a
  machine-readable result artifact describing current, candidate, deferred, or
  rejected status.
- For a newer compatible candidate, create an isolated dependency-update PR
  only after the unchanged Office, fidelity, resource-release, and release
  validation gates pass.
- Reject path/git dependency overrides and retain actionable upstream evidence
  when the candidate cannot pass or cannot yet be safely adopted.

## Capabilities

### New Capabilities

- `office2pdf-upstream-monitor`: Detect official office2pdf releases, retain
  machine-readable comparison evidence, and conditionally prepare a
  quality-gated registry-only update PR.

### Modified Capabilities

- None.

## Impact

This adds a release-maintenance script, its tests, a scheduled GitHub Actions
workflow, and an OpenSpec contract. It does not change KDV's public API or
current `office2pdf =0.7.0` dependency until a candidate passes every required
gate.
