# v0.3.0 Browser Session Adapter Handoff

## Ownership

- KRR owns the in-process Rust/V8 HTML runtime, DOM, CSS layout/paint,
  JavaScript dispatch, hit-test, frame rasterization, and navigation intent.
- KDV owns only the session adapter. It forwards raw `HtmlBrowserSource`,
  viewport, input, resize, refresh, navigation, updates, errors, and close.
- KatanA owns main-document acquisition and native frame/input presentation.

## Verified State

- `rtk cargo test -p katana-document-viewer --test browser_session_adapter_contract --locked -- --test-threads=1`
  passed 2 tests, including source-boundary and public command forwarding.
- `rtk just coverage` passed. The adapter production sources have 100% line,
  function, and region coverage.
- `rtk just storybook-release-acceptance-artifacts` regenerated the existing
  KDV native Storybook evidence. Its freshness/source-integrity gate remains
  required for the legacy UI release contract.
- `rtk just VERSION=0.3.0 release-verify` passed: the adapter release contract,
  strict checks, coverage, package validation, and crates.io publish dry-run
  completed with KRR `0.4.0` resolved from the registry.
- The release evidence harness clears hook-inherited Git environment before it
  checks a temporary workspace. Its regression test reproduces a pre-push
  `GIT_DIR` and proves the workspace remains non-Git.

## Independent Review

- [x] Browser-session adapter ownership was independently reviewed. No HTML parser, CSS cascade/layout, JavaScript/V8 runtime, browser hit-test, WebView, Chromium, or external browser process is implemented in the adapter sources. 証跡: agent: `019f75e7-c5e2-7293-b738-cfcc0290f921` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `crates/katana-document-viewer/src/browser_session.rs` / file: `crates/katana-document-viewer/src/browser_session_state.rs` / file: `crates/katana-document-viewer/src/browser_session_types.rs` / file: `crates/katana-document-viewer/src/browser_session_worker.rs` / file: `crates/katana-document-viewer/tests/browser_session_adapter_contract.rs` / command: `multi_agent_v1.spawn_agent` / verify: `rtk cargo test -p katana-document-viewer --test browser_session_adapter_contract --locked -- --test-threads=1` / close: `multi_agent_v1.close_agent`

## Remaining Release Preconditions

1. Publish KDV `0.3.0` after the release preflight passes, then prove its
   public artifact.
2. Let KatanA resolve
   only registry KDV `0.3.0` before its v0.22.33 native acceptance rerun.
