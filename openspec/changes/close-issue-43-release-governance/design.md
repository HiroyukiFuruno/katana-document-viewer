## Context

Issue #43 requires that KDV release work remain traceable to an Open Issue, that a downstream dependency change carries its public-version and validation evidence, and that release cleanup never destroys active work. The repository has release scripts and a GitHub release workflow, but it has no repository-owned hook dispatcher or publish-gated cleanup contract.

## Goals / Non-Goals

**Goals:**

- Enforce a machine-readable Issue reference before a non-default branch can be pushed.
- Require evidence when a downstream dependency manifest or lockfile changes.
- Preserve any pre-existing repository-specific hook before KDV policy runs.
- Provide a deterministic, dry-run-first cleanup tool that proves a GitHub Release exists before changing branches, worktrees, or branches.
- Run the release workflow's remote-branch cleanup only after the GitHub Release has been created.

**Non-Goals:**

- Do not delete a dirty, unmerged, checked-out, default, or otherwise ambiguous local branch/worktree.
- Do not make a CI checkout attempt to clean a developer's local worktrees.
- Do not infer upstream publication from a path/git override, a local checkout, or a commit SHA.
- Do not change release acceptance thresholds or bypass existing hooks.

## Decisions

### Repository-owned dispatcher delegates before policy

`.githooks/pre-push` SHALL invoke an existing configured hook delegate before KDV checks. This keeps existing repository policy authoritative and avoids silently replacing a user or organization hook. A portable shell dispatcher and Python validators are chosen because the existing release tooling is shell/Python and the checks need deterministic Git/GitHub process boundaries.

Alternatives considered:

- Replacing `core.hooksPath`: rejected because it can discard an existing hook.
- A commit-message-only convention: rejected because it cannot validate downstream dependency evidence at push time.

### Evidence lives in the referenced Open Issue

The validator extracts one repository Issue URL from the branch or commit range and queries the matching Open Issue. When a dependency manifest or lockfile is changed, the Issue body or comments MUST contain the upstream registry version, migration note, manifest and lockfile acknowledgement, and named verification evidence. This makes the release record externally auditable instead of relying on a local-only file.

Alternatives considered:

- A free-form PR description: rejected because pushes can precede a PR and the Issue is the stated unit of work.
- Automatically accepting any version string: rejected because registry-only provenance must be explicit.

### Cleanup separates CI remote cleanup from developer local cleanup

`scripts/release/post-release-cleanup.py` has an audit mode and explicit scopes. In GitHub Actions it can safely remove only a merged, non-default remote release branch after confirming the created GitHub Release. A developer-run local scope can switch to default, fast-forward it, then remove only local branches/worktrees independently proven clean, merged, and unused. Every candidate is reported before mutation; a retain condition is a non-destructive failure/report rather than a delete.

Alternatives considered:

- A single `git branch -D` sweep: rejected because it ignores worktree checkout, dirty state, and merge ancestry.
- CI cleanup of all local worktrees: rejected because CI cannot observe a developer machine.

### Tests use isolated temporary repositories and mocked GitHub command boundaries

Validator and cleanup tests SHALL create temporary Git repositories and provide fake `gh` responses. They verify positive paths and retain paths without contacting a real release or deleting a caller's worktree.

## Risks / Trade-offs

- [Existing custom hook is not discoverable] → The dispatcher exposes an explicit delegate path and fails if its configured delegate fails; installation does not overwrite an existing file.
- [Issue API unavailable during push] → The push is rejected with the failed evidence query rather than proceeding with unknown governance state.
- [Release created but local cleanup cannot fast-forward] → The cleanup reports failure and retains every candidate for manual resolution.
- [Remote branch has active worktree or lacks merge proof] → Remote cleanup is skipped and reported; no force deletion is used.

## Migration Plan

1. Add dispatcher, validator, cleanup tool, and isolated tests.
2. Install the repository hooks through an explicit setup recipe without replacing an existing delegate.
3. Add a post-GitHub-Release workflow step for the narrowly scoped remote audit/cleanup.
4. Run the normal release checks and hook/cleanup contract tests.
5. Roll back by removing the workflow call and hook installation configuration; retained Git state requires no rollback because unsafe targets are never deleted.

## Open Questions

- None. The default branch, repository Issue URLs, and public GitHub Release are discoverable at runtime and SHALL be validated rather than hard-coded.
