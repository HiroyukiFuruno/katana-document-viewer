## ADDED Requirements

### Requirement: Scheduled upstream comparison
The repository SHALL provide a daily scheduled and manually dispatchable
monitor that reads KDV's exact `office2pdf` manifest and lockfile resolution,
compares it with the latest unyanked crates.io package and upstream GitHub
release/tag, and writes a machine-readable decision record.

#### Scenario: Current official version is already latest
- **WHEN** crates.io and the upstream release resolve to the current exact
  registry version
- **THEN** the monitor SHALL record `current`, upload the record, and SHALL NOT
  change dependencies or create a pull request

#### Scenario: Upstream metadata is unavailable or disagrees
- **WHEN** either external source cannot be retrieved or the crate version and
  upstream release/tag disagree
- **THEN** the monitor SHALL record `unavailable` or `deferred` with both
  observed values and SHALL NOT create a pull request

### Requirement: Registry-only candidate eligibility
The monitor SHALL consider a candidate eligible only when it is newer than the
exact current version, unyanked on crates.io, semver-compatible within the
current major line, represented by a non-draft upstream release/tag, and
resolvable without a git/path dependency override.

#### Scenario: Compatible official candidate
- **WHEN** a newer candidate satisfies every eligibility condition
- **THEN** the workflow SHALL prepare an isolated branch with only the exact
  manifest and lockfile update plus generated evidence

#### Scenario: Major, yanked, or overridden candidate
- **WHEN** a candidate changes the major line, is yanked, or requires a
  git/path override
- **THEN** the monitor SHALL record `deferred` or `rejected`, retain the
  current dependency, and SHALL NOT create a pull request

### Requirement: Quality-gated update pull request
The workflow SHALL create a dependency-update pull request only after the
candidate branch passes unchanged complete lint/test/coverage, Office
data-descriptor and bounded-preflight, DOCX/XLSX/PPTX fidelity,
first-frame/resource-release, package, and release-contract checks.

#### Scenario: Candidate passes every required gate
- **WHEN** every required candidate command succeeds on the isolated branch
- **THEN** the workflow SHALL create a draft pull request that references Issue
  #45 and includes registry version, migration, manifest, lockfile, validation,
  and upstream evidence

#### Scenario: Candidate fails a required gate
- **WHEN** any required candidate command fails
- **THEN** the workflow SHALL upload diagnostics and a `rejected` result, SHALL
  NOT create a pull request, and SHALL NOT relax a threshold, fixture,
  reference, or resource limit

### Requirement: Downstream release boundary
The monitor SHALL NOT merge, tag, publish KDV, or claim downstream KatanA
adoption. A passing update PR SHALL leave those release actions to the normal
review and release workflow.

#### Scenario: Passing monitor run
- **WHEN** the monitor creates a candidate pull request
- **THEN** it SHALL state that KDV publication and KatanA adoption remain
  required post-merge release steps
