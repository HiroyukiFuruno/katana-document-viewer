<!-- subagent-spark-harness-strict-start -->

## 1. Candidate decision record

- [x] 1.1 Implement a deterministic monitor that reads the exact official dependency and lock source, compares crates.io and GitHub release metadata, and emits a machine-readable decision record. delegation-exception: `直列のクリティカルパス`。証跡: command: `rtk proxy python3 scripts/release/monitor-office2pdf-upstream.py --output /tmp/kdv-office2pdf-monitor.json`.
- [x] 1.2 Add isolated self-tests for current, eligible, yanked, incompatible, metadata-disagreement, and unavailable outcomes. delegation-exception: `直列のクリティカルパス`。証跡: command: `rtk proxy python3 scripts/release/monitor-office2pdf-upstream.py --self-test`.
- [x] 1.3 Reject git/path overrides and unrelated dependency changes during candidate preparation. delegation-exception: `直列のクリティカルパス`。証跡: file: `scripts/release/monitor-office2pdf-upstream.py`.

## 2. Quality-gated automation

- [x] 2.1 Add a daily scheduled and manually dispatchable workflow that uploads the decision record for every run. delegation-exception: `直列のクリティカルパス`。証跡: file: `.github/workflows/office2pdf-upstream-monitor.yml`.
- [x] 2.2 Prepare a candidate branch only after updating the exact manifest/lockfile and running unchanged complete quality, Office fixture, fidelity, resource, package, and release-contract gates. delegation-exception: `直列のクリティカルパス`。証跡: command: `rtk proxy just office2pdf-upstream-monitor-check`.
- [x] 2.3 Create a draft Issue #45 dependency-update PR only after every candidate gate passes; retain rejection diagnostics otherwise. delegation-exception: `直列のクリティカルパス`。証跡: command: `rtk proxy python3 scripts/release/verify-office2pdf-monitor-contract.py`.

## 3. Verification and issue closure evidence

- [x] 3.1 Add a Just entrypoint and workflow-contract tests for the monitor and PR gating. delegation-exception: `直列のクリティカルパス`。証跡: command: `rtk proxy just office2pdf-upstream-monitor-check`.
- [x] 3.2 Run formatter, lint, AST lint, monitor self-tests, OpenSpec validation, and the applicable quality checks without changing thresholds or references. delegation-exception: `直列のクリティカルパス`。証跡: command: `rtk proxy just JOBS=2 check`; command: `rtk proxy just office2pdf-upstream-monitor-check`; command: `rtk proxy scripts/openspec validate close-issue-45-office2pdf-monitor --strict`.
- [ ] 3.3 Record the first current-version monitor result and update Issue #45 with workflow and evidence links after the workflow is merged.
