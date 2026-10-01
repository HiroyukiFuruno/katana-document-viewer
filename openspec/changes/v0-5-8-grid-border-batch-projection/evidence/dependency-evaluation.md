# v0.5.8 dependency evaluation

Evaluated on 2026-10-01 against published registry artifacts only. No `path` or `git`
override was introduced.

## KatanA sibling dependencies

- `katana-ui-core` `0.4.0` was verified through `cargo info katana-ui-core@0.4.0` and the
  public [v0.4.0 release](https://github.com/HiroyukiFuruno/katana-ui-core/releases/tag/v0.4.0).
  Its release adds a V2 consumer artifact-plan API while retaining the V1 consumer contract.
  KDV does not consume the new V2 API. The exact KDV pin is updated from `=0.3.17` to `=0.4.0`.
- `katana-render-runtime` was refreshed from `0.4.20` to the compatible published `0.4.21` by
  `cargo update`.
- The resolved graph contains only `v8 152.2.0`; the final release gate must re-run the duplicate
  tree and fresh-consumer link checks after all source changes are complete.

## General dependency refresh

`just outdated` identified KUC `0.4.0` as the only direct normal dependency upgrade. `cargo
update` refreshed all compatible lockfile packages available to the current manifest, including
the `wasm-bindgen` family, `js-sys`, `web-sys`, `yoke-derive`, and `tokio`.

The candidate remains subject to the unchanged strict quality, semver, dependency, V8 singleton,
and fresh registry-consumer gates. No fidelity reference, geometry compensation, or acceptance
threshold changes are part of this evaluation.
