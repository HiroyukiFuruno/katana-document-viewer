## Purpose

公開済みreleaseだけを対象に、localとremoteのrelease作業状態を安全条件付きで整理する。

## Requirements

### Requirement: Cleanup requires a published GitHub Release
The cleanup command SHALL first verify that the target version has a published GitHub Release in the target repository. It MUST report and exit without modifying Git branches or worktrees when the release does not exist.

#### Scenario: Target release is absent
- **WHEN** cleanup is requested for a version without a GitHub Release
- **THEN** the command SHALL retain all local and remote Git state and report the missing release

#### Scenario: Target release is published
- **WHEN** cleanup is requested for a version with a published GitHub Release
- **THEN** the command SHALL continue to evaluate candidates under its selected scope

### Requirement: Local cleanup only removes independently safe candidates
For local cleanup, the command SHALL switch to the default branch and fast-forward from origin before it evaluates candidates. It MUST remove a local branch or worktree only when it is clean, merged into the refreshed default branch, non-default, and unused by any worktree. It MUST retain and report every target that fails any condition.

#### Scenario: Clean merged unused release branch
- **WHEN** a non-default local release branch is clean, merged, and unused after default branch refresh
- **THEN** local cleanup SHALL remove that branch

#### Scenario: Dirty or checked-out branch
- **WHEN** a candidate branch is dirty or checked out by a worktree
- **THEN** local cleanup SHALL retain it and report the failed condition

### Requirement: Release workflow cleanup is limited to safe remote release branches
After GitHub Release creation succeeds, the release workflow SHALL run the cleanup command in remote scope. Remote cleanup MUST delete only the merged, non-default branch associated with the released pull request and MUST not force-delete a branch.
The cleanup step MUST authenticate its GitHub Release lookup independently of earlier workflow steps.

#### Scenario: Merged release branch is safe to delete remotely
- **WHEN** the GitHub Release has been created and the released pull request branch is merged and non-default
- **THEN** the workflow cleanup SHALL delete that remote branch without force

#### Scenario: Cleanup has an authenticated GitHub Release lookup
- **WHEN** the workflow invokes remote cleanup after publishing a GitHub Release
- **THEN** the cleanup step SHALL receive the GitHub Actions token required to read that Release

#### Scenario: Remote branch is not proven merged
- **WHEN** the release workflow cannot prove that the candidate remote branch is merged
- **THEN** remote cleanup SHALL retain the branch and report the condition
