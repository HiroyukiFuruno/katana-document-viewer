# Issue #43 Release Governance Handoff

## Execution boundary

- This change updates the pre-push dispatcher, its installation path, release
  workflow ordering, cleanup verification, and their contract tests as one
  ordered control plane. Each implementation step changes the executable
  boundary used to validate the next step.

## Delegation record

- The implementation and verification remained in the main task.
  delegation-exception: `直列のクリティカルパス` / file:
  `openspec/changes/close-issue-43-release-governance/tasks.md`.

## Verified handoff

- `rtk proxy just release-governance-check` validates issue/registry evidence,
  override rejection, delegated-hook failure propagation, and safe cleanup.
- `rtk proxy python3 scripts/release/verify-release-contract.py --target-version
  v0.5.6` verifies the cleanup order after GitHub Release creation and before
  registry publication.
