## ADDED Requirements

### Requirement: Preview renders KME document models

The preview system SHALL render KME document models through the Floem preview implementation.

#### Scenario: Render canonical KME fixture

- **WHEN** kdp receives a KME model for `sample.md`
- **THEN** kdp renders supported nodes in Floem preview
- **THEN** rendered nodes expose public hit-test metadata

### Requirement: Preview adoption depends on P0, P1, and P2 contracts

The preview system SHALL treat KME preview adoption as downstream work after shared AST lint, KME model, and shared widget boundaries are available.

#### Scenario: Start KME preview implementation

- **WHEN** kdp starts KME preview implementation
- **THEN** P0 `katana-ast-lint` governance is available
- **THEN** P1 KME public DTOs are available
- **THEN** P2 widget boundaries for metadata UI are considered

### Requirement: Preview exposes unresolved metadata

The preview system SHALL expose unresolved KME metadata targets to the user.

#### Scenario: Show unresolved target

- **WHEN** KME or editor metadata sync returns an unresolved target
- **THEN** preview can display that unresolved state
- **THEN** the target is not silently hidden

### Requirement: Preview avoids parser internals

The preview system MUST NOT depend on parser-private AST types for KME rendering.

#### Scenario: KME parser changes

- **WHEN** KME changes its internal parser implementation
- **THEN** kdp continues consuming KME-owned public DTOs
