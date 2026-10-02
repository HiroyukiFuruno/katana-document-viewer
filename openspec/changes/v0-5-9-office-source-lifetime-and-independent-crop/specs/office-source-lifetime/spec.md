## ADDED Requirements

### Requirement: Converted Office source bytes are released before PDF parsing

The system SHALL release the consumed Office source bytes after successful conversion and before PDF parsing, without changing source metadata, conversion identity, diagnostics, capabilities, page count, limits, or rendered frames.

#### Scenario: Successful conversion
- **WHEN** the worker has returned the converted PDF and diagnostics
- **THEN** original Office bytes are no longer owned when PDF parsing begins, while the final artifact retains its existing metadata and page items

### Requirement: Causal RSS attribution uses actual unchanged acceptance

The system SHALL distinguish original-byte lifetime, live allocation, released resident memory, worker termination, and backend allocation using the same real inputs and unchanged acceptance limits.

#### Scenario: Backend memory remains resident
- **WHEN** all KDV counters and live malloc return to cold but RSS remains high
- **THEN** the evidence reports the residual rather than declaring the whole original acceptance passed

### Requirement: Unified Office raster releases child-owned transient memory

The system SHALL rasterize uncached Office frames requested through DocumentSession in the existing constrained worker, retaining parent-owned PDF metadata and bounded frame cache and preserving rendered bytes, request validation, sandbox, deadlines, memory and output limits, and typed errors. It SHALL NOT add an in-process fallback or change direct PDF and low-level Office public rendering APIs.

#### Scenario: Uncached frame followed by cache reuse
- **WHEN** an Office frame is requested at a new scale and then requested at the same scale
- **THEN** the first frame is returned from a completed constrained worker and the second reuses the bounded parent cache with identical frame metadata and pixels

#### Scenario: Raster worker fails
- **WHEN** the child cannot start, exceeds a limit or deadline, crashes, or returns invalid bounded output
- **THEN** the existing typed error reports the failure without rasterizing the same request in the parent

#### Scenario: Consumer uses a small converted PDF output limit
- **WHEN** converted PDF fits OfficeWorkerConfig.max_output_bytes but its rendered RGBA exceeds that PDF byte limit while remaining within existing PDF render resource limits
- **THEN** unified Office rendering succeeds with the same pixels as the direct API, keeping PDF input and RGBA output limits distinct
