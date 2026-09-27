## Context

KDV currently uses the official crates.io package `office2pdf =0.7.0` as an
exact, registry-only dependency. The historical fork migration is complete,
but current maintenance has no durable scheduled comparison with upstream.
The monitor must handle external API failure, a newer incompatible candidate,
and a candidate that fails KDV's complete quality contract without modifying
the release branch or weakening any gate.

## Goals / Non-Goals

**Goals:**

- Compare the current exact manifest/lock resolution with the latest unyanked
  crates.io version and matching upstream GitHub release/tag metadata.
- Emit a machine-readable decision record for every scheduled or manual run.
- Prepare a registry-only dependency-update branch and PR only after unchanged
  KDV quality, Office fixture, fidelity, resource-release, and package gates
  pass.
- Preserve evidence for deferred and rejected candidates, including upstream
  issue/PR links supplied by the workflow dispatch input.

**Non-Goals:**

- Reintroducing `office2pdf-katana`, a git/path dependency, or an in-KDV
  conversion/layout workaround.
- Auto-merging, tagging, publishing KDV, or publishing a candidate crate.
- Treating a skipped external query, an API error, or a locally passing focused
  test as a compatibility pass.

## Decisions

### D1: Repository-owned Python decision script

The monitor is a standard-library Python script that parses `Cargo.toml`,
checks the lockfile source, retrieves crates.io and GitHub metadata, and writes
a versioned JSON result. Its self-test injects payloads rather than calling the
network. This keeps the candidate decision reproducible and reviewable.

Using only Dependabot was rejected: it cannot require KDV's Office-specific
fidelity/resource gates or retain a structured upstream decision record.

### D2: Exact registry candidate only

A candidate is eligible only when it is newer than the exact manifest version,
comes from the unyanked crates.io registry, has a matching non-draft upstream
release/tag, and is semver-compatible under the current major line. Candidate
preparation updates the exact manifest and lockfile together; the script
rejects any git/path override before or after preparation.

Blind `cargo update` was rejected because it can change unrelated transitive
dependencies and cannot prove the requested package version or source.

### D3: Two-phase workflow

The scheduled/manual workflow first writes and uploads a monitor result. Only
an eligible candidate enters a clean, isolated branch where it runs the same
strict checks used for release preparation: complete lint/test/coverage,
Office data-descriptor and bounded-preflight contracts, Office fidelity and
first-frame/resource checks, package dry-run, and registry/dependency release
contract checks. A PR is created only after all commands succeed.

Running the heavy suite before discovery was rejected because no-candidate runs
must be inexpensive, while creating a PR before the suite would misrepresent
an unverified dependency update.

### D4: Failure is evidence, not fallback

Workflow failures upload the JSON decision record and diagnostics. A failed or
incompatible candidate leaves the default branch unchanged and records the
candidate version, upstream release URL/tag, gate outcome, and optional
upstream issue/PR references. The existing official version stays pinned.

## Risks / Trade-offs

- [Risk] Upstream API or GitHub metadata can be unavailable. → The run records
  `unavailable`, uploads its result, creates no PR, and exits distinctly.
- [Risk] A new crate version resolves but regresses document behavior. → The
  full existing gates remain mandatory and a failed candidate cannot create a
  PR or change the checked-out branch.
- [Risk] A release tag and crates.io package disagree. → Treat it as deferred
  and preserve both values in the result rather than selecting either source.
- [Risk] Scheduled credentials could overreach. → The workflow has the minimum
  contents/pull-request write permissions required only for a passing update;
  it never merges or publishes.

## Migration Plan

1. Add the script, its deterministic self-tests, and a Just recipe.
2. Add the scheduled/manual workflow with artifact upload and isolated PR
   preparation.
3. Verify the current `0.7.0` no-update result and the injected eligible,
   incompatible, unavailable, and gate-failure paths.
4. Enable the workflow after the current release branch is merged; no existing
   dependency is changed by this rollout.

## Open Questions

- None. A future major `office2pdf` release remains a recorded deferred
  candidate until a separately reviewed migration change defines its API and
  compatibility work.
