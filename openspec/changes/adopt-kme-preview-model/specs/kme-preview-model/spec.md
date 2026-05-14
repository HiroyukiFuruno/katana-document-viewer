## ADDED Requirements

### Requirement: Viewer renders KMM document models

The viewer system SHALL render KMM document models through the Floem viewer implementation.

#### Scenario: Render canonical KMM fixture

- **WHEN** KDV receives a KMM model for `sample.md`
- **THEN** KDV renders supported nodes in the Floem viewer
- **THEN** rendered nodes expose public hit-test metadata

### Requirement: Viewer adoption depends on P0, P1, and P2 contracts

The viewer system SHALL treat KMM viewer adoption as downstream work after shared AST lint, KMM model, and shared widget boundaries are available.

#### Scenario: Start KMM viewer implementation

- **WHEN** KDV starts KMM viewer implementation
- **THEN** P0 `katana-ast-lint` governance is available
- **THEN** P1 KMM public DTOs are available
- **THEN** P2 widget boundaries for metadata UI are considered

### Requirement: Viewer exposes unresolved metadata

The viewer system SHALL expose unresolved KMM metadata targets to the user.

#### Scenario: Show unresolved target

- **WHEN** KMM or editor metadata sync returns an unresolved target
- **THEN** the viewer can display that unresolved state
- **THEN** the target is not silently hidden

### Requirement: Viewer avoids parser internals

The viewer system MUST NOT depend on parser-private AST types for KMM rendering.

#### Scenario: KMM parser changes

- **WHEN** KMM changes its internal parser implementation
- **THEN** KDV continues consuming KMM-owned public DTOs

### Requirement: KDV does not own editor-viewer synchronization control

KDV MUST expose viewer commands and hit-test metadata without coordinating editor state.

#### Scenario: KatanA synchronizes editor and viewer

- **WHEN** KatanA decides that viewer or editor state should change
- **THEN** KatanA sends scroll, selection, or highlight commands to KDV or KLE
- **THEN** KDV does not call KLE and does not coordinate synchronization itself
