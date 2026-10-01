## 1. Public batch contract

- [x] 1.1 Add a read-only `DocumentSurfaceFrame` batch accessor for coordinate/border entries without changing the existing single-cell lookup.
- [x] 1.2 Keep every stored coordinate and four-side border value in grid input order, including missing, `none`, and merged-cell states.

## 2. Regression coverage

- [x] 2.1 Add a mixed-style grid regression proving the batch accessor returns each entry once and agrees with single-cell lookup.
- [x] 2.2 Add a large-grid regression whose host-style projection consumes the batch collection once without repeated frame lookup.

## 3. Verification and release

- [x] 3.1 Run focused document-surface, public semver, format, strict Clippy, AST, and OpenSpec checks.
- [x] 3.2 Evaluate KUC 0.4.0 and all other compatible published dependencies; record adoption or rejection evidence without lowering gates.
- [ ] 3.3 Complete the Draft PR review, required checks, merge, v0.5.8 registry publication, fresh registry consumer check, Issue #56 closure, and scoped cleanup.
