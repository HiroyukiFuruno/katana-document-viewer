## ADDED Requirements

### Requirement: Non-default branch pushes require an Open Issue reference
The repository SHALL reject a push from a non-default branch unless the pushed commits contain a repository Issue URL that resolves to an Open Issue in the same repository. The dispatcher MUST run a configured existing hook delegate before this validation and MUST propagate a delegate failure.

#### Scenario: Push has a same-repository Open Issue reference
- **WHEN** a non-default branch push contains an Issue URL for an Open Issue in the repository
- **THEN** the governance validator SHALL allow the push after the existing delegate succeeds

#### Scenario: Push lacks an Issue reference
- **WHEN** a non-default branch push has no same-repository Open Issue URL
- **THEN** the governance validator SHALL reject the push without invoking a release mutation

#### Scenario: Existing hook fails
- **WHEN** the configured existing hook delegate exits nonzero
- **THEN** the dispatcher SHALL reject the push and SHALL NOT run the KDV governance validator

### Requirement: Downstream dependency updates carry release evidence
When a non-default branch changes a dependency manifest or lockfile, the referenced Open Issue SHALL contain the upstream public registry version, an API migration note, manifest and lockfile acknowledgement, and named verification evidence. The validator MUST reject path or git overrides as registry adoption evidence.

#### Scenario: Registry dependency update has complete evidence
- **WHEN** the changed dependency uses a published registry version and the referenced Issue contains every required evidence field
- **THEN** the governance validator SHALL allow the push

#### Scenario: Dependency update lacks lockfile evidence
- **WHEN** a dependency manifest or lockfile changes but the referenced Issue omits lockfile acknowledgement
- **THEN** the governance validator SHALL reject the push with the missing field

#### Scenario: Dependency update uses a path override
- **WHEN** a changed dependency uses a path or git override as adoption evidence
- **THEN** the governance validator SHALL reject the push
