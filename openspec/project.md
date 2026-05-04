# katana-markdown-preview OpenSpec

## Project

`katana-markdown-preview`（kmp）は、Markdown preview の neutral interface と egui MVP 実装を提供する library。KatanA はこれを git dependency として consume する。

## Design Principles

- `katana-markdown-preview` crate（neutral interface）は `egui` に依存しない。
- `katana-markdown-preview-egui` crate が egui 実装を持つ。将来の独自 UI への差し替えはこの crate のみ変更すれば良い。
- KatanA は neutral interface と egui 実装の両方を dependency に取るが、interface 経由でしか呼ばない。

## Versioning

- `v0.1.x`: KatanA v0.26.0 で分離する preview 実装の移管。egui MVP 確立。
- `v0.2.x`: 独自 UI 実装への差し替え（egui 脱却）

## Consumers

- [KatanA](https://github.com/HiroyukiFuruno/KatanA) — git tag pinned（v0.26.0 で取り込み）
