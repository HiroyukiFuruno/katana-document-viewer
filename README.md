<h1 align="center">katana-markdown-preview</h1>

<p align="center">
  Vendor-neutral Markdown preview library for
  <a href="https://github.com/HiroyukiFuruno/KatanA">KatanA</a>.
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <img src="https://img.shields.io/badge/status-scaffolding-orange" alt="Status: scaffolding">
</p>

---

## Design

Two-crate structure separates interface from implementation:

```
katana-markdown-preview        ← neutral trait + DTO (no egui, no framework)
katana-markdown-preview-egui   ← egui MVP implementation
```

KatanA depends on both crates. When the custom UI replaces egui in the future,
only the `-egui` crate is swapped; KatanA's dependency on the neutral interface
crate remains unchanged.

## Status

Scaffolding. Full implementation migrated from KatanA in the `v0.26.0` change
(`openspec/changes/v0-1-0-markdown-preview-extraction`).

## License

MIT
