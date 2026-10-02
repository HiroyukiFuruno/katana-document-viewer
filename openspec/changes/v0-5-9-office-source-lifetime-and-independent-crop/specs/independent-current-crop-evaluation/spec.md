## ADDED Requirements

### Requirement: Current external artifacts are explicit immutable inputs

The evaluator SHALL accept the external KatanA artifact and provenance explicitly, validate SHA, dimensions and frame conditions, and preserve existing tracked references unchanged.

#### Scenario: Candidate provenance mismatch
- **WHEN** an input SHA, dimension or frame condition differs from its declared provenance
- **THEN** evaluation fails without rewriting a reference or substituting a KDV self-render

### Requirement: Every score category is independently evidenced

The evaluator SHALL require independently evidenced visual, semantic, interaction and performance scores of at least 95 each; one category SHALL NOT stand in for another.

#### Scenario: Pixels exist but behavior evidence is missing
- **WHEN** a candidate crop exists without independent interaction or performance evidence
- **THEN** those categories remain unmet instead of inheriting the visual score
