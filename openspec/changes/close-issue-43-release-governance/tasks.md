<!-- subagent-spark-harness-strict-start -->

## 1. Issue-linked push governance

- [x] 1.1 Add a repository-owned pre-push dispatcher that delegates to a configured existing hook before KDV checks. delegation-exception: `直列のクリティカルパス`。証跡: `rtk proxy bash -n .githooks/pre-push scripts/release/install-governance-hook.sh`。
- [x] 1.2 Implement the same-repository Open Issue reference validator for non-default branch push ranges. delegation-exception: `直列のクリティカルパス`。証跡: `rtk proxy python3 scripts/release/verify-issue-governance.py --self-test`。
- [x] 1.3 Detect dependency manifest/lockfile updates and require public registry version, migration, manifest, lockfile, and validation evidence; reject path/git overrides. delegation-exception: `直列のクリティカルパス`。証跡: `rtk proxy python3 scripts/release/verify-issue-governance.py --self-test`。
- [x] 1.4 Add isolated automated coverage for successful governance, missing Issue, missing dependency evidence, override rejection, and delegate failure. delegation-exception: `直列のクリティカルパス`。証跡: `rtk proxy python3 scripts/release/verify-issue-governance.py --self-test`。

## 2. Published-release cleanup

- [x] 2.1 Implement a dry-run-first cleanup command that verifies the target GitHub Release before any mutation. delegation-exception: `直列のクリティカルパス`。証跡: `rtk proxy python3 scripts/release/post-release-cleanup.py --self-test`。
- [x] 2.2 Implement local cleanup with default switch/fast-forward and clean/merged/unused retain checks for branches and worktrees. delegation-exception: `直列のクリティカルパス`。証跡: `rtk proxy python3 scripts/release/post-release-cleanup.py --self-test`。
- [x] 2.3 Implement remote release-branch cleanup without force deletion and only after merge proof. delegation-exception: `直列のクリティカルパス`。証跡: `rtk proxy python3 scripts/release/post-release-cleanup.py --self-test`。
- [x] 2.4 Add isolated automated coverage for absent release, safe deletion, dirty/checked-out retention, and unmerged remote retention. delegation-exception: `直列のクリティカルパス`。証跡: `rtk proxy python3 scripts/release/post-release-cleanup.py --self-test`。

## 3. Release integration and verification

- [x] 3.1 Add explicit hook setup/documentation without replacing an existing hook delegate. delegation-exception: `直列のクリティカルパス`。証跡: `rtk proxy bash -n .githooks/pre-push scripts/release/install-governance-hook.sh`。
- [x] 3.2 Invoke remote-scope cleanup only after the GitHub Release workflow step succeeds. delegation-exception: `直列のクリティカルパス`。証跡: `rtk proxy python3 scripts/release/verify-release-contract.py --target-version v0.5.6`。
- [x] 3.3 Run formatter, strict lint, AST lint, governance/cleanup tests, and release contract checks; record results in this change. delegation-exception: `直列のクリティカルパス`。証跡: `rtk proxy just fmt-check && just lint && just ast-lint && just release-governance-check && python3 scripts/release/verify-release-contract.py --target-version v0.5.6`。
