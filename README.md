<h1 align="center">katana-document-viewer</h1>

<p align="center">
  Vendor-neutral Markdown viewer and export library for
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
katana-document-viewer         ← neutral trait + DTO (no egui, no framework)
katana-document-viewer-floem   ← Floem viewer/export implementation
```

KatanA depends on the neutral interface and the Floem implementation. KDV does
not own editor-viewer synchronization control; KatanA commands viewer or editor.

HTML/PDF/PNG/JPG export belongs to KDV so viewer display and export share the
same render pipeline. Diagram and math rendering is delegated to KCF.

## Status

Scaffolding. This repository is being renamed from `katana-document-preview`
to `katana-document-viewer` before its first release.

## License

MIT
