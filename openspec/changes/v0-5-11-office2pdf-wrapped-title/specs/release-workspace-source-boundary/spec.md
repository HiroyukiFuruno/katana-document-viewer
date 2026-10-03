## ADDED Requirements

### Requirement: Governance distinguishes committed workspace members from external sources

The push checker SHALL allow a relative internal dependency only when the destination is an explicit non-excluded workspace member and its package identity exists in the same pushed Git commit. It SHALL NOT derive membership from the working tree or allow absolute or repository-external paths.

#### Scenario: Storybook connects the local implementation
- **WHEN** a normal, development, build, or workspace dependency names the matching package in a committed workspace member
- **THEN** the checker accepts this internal connection without changing published registry dependency requirements

#### Scenario: The working tree differs from the pushed commit
- **WHEN** the pushed commit excludes a member but the working tree removes the exclusion
- **THEN** the checker still rejects the dependency using the committed state

#### Scenario: Only workspace membership or a dependency destination changes
- **WHEN** a push changes only the root member list or exclusions, or renames a dependency destination package without editing the dependent manifest
- **THEN** the checker inspects all committed manifests and rejects the now-invalid existing path connection

#### Scenario: Root metadata changes without invalidating dependencies
- **WHEN** the root manifest changes metadata while existing internal connections remain valid
- **THEN** the checker accepts the push after inspecting the existing dependent manifests

### Requirement: External source and Issue guards remain mandatory

The checker SHALL continue rejecting git, absolute or external paths, unregistered or excluded members, mismatched package identities, nested source overrides, and source overrides in patch or replace tables. It SHALL preserve Open Issue URL and upstream registry, migration, manifest, lockfile, and verification evidence requirements.

#### Scenario: A path is not a committed internal connection
- **WHEN** a dependency fails any internal membership or identity requirement or uses a prohibited override table
- **THEN** the push is rejected rather than disabling the source guard

#### Scenario: The release commit lacks an Issue URL
- **WHEN** the pushed commit range has no full URL for an Open Issue in the repository
- **THEN** the push is rejected even when all workspace dependencies are valid
