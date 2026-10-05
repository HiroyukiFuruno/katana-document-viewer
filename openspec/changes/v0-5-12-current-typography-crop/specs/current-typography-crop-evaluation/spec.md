## ADDED Requirements

### Requirement: Explicit fixture-bound current crop entry

The native harness SHALL provide separate current Typography (katana/sample.md) and Diagrams (katana/sample_diagrams.md) visual entrypoints. Each entry SHALL require explicit immutable manifest/crop/full/geometry inputs and bind the declared fixture to the render fixture before scoring.

#### Scenario: Current Typography evaluation
- **WHEN** the Typography entry receives a valid sample.md declaration and decodable PNG inputs
- **THEN** it SHALL evaluate sample.md using the existing 95 visual threshold without changing reference or geometry

#### Scenario: Fixture mismatch rejection
- **WHEN** either entry receives the other fixture declaration
- **THEN** it SHALL reject the mismatch before scoring and preserve all four input files

### Requirement: Keep independent evidence boundaries

The harness MUST decode both PNG inputs and preserve existing corruption rejection. A visual result MUST NOT promote semantic/interaction/performance or producer identity to completed acceptance.

#### Scenario: Rehashed corrupt PNG
- **WHEN** hashes match a header-only, CRC-corrupt or invalid-zlib input
- **THEN** native decoding SHALL reject it before scoring

#### Scenario: Visual-only acceptance
- **WHEN** the current visual score passes
- **THEN** missing nonvisual evidence SHALL remain unevaluated and Issue #58/#59 SHALL NOT be closed solely on that result
