# Tasks: katana-document-viewer v0.3.0 browser session adapter

## Release Tasks

- [x] v0.3.0 adapter release contract を、legacy Storybook UI acceptance と分離する。証跡: `rtk just VERSION=0.3.0 release-verify`。
  legacy artifact の freshness/source-integrity check は維持し、adapter release は
  registry lock、ownership contract、adapter integration contract、strict quality gate
  を必須条件とする。
- [x] KRR `0.4.0` の crates.io 公開後、temporary local patch を除外し registry lockfile を確定する。証跡: `rtk cargo generate-lockfile` により `katana-render-runtime 0.4.0` を crates.io checksum 付きで解決。
- [x] 同一 version の未実装 PDF pagination plan を後続 version へ繰り延べ、`v0.3.0` release target と衝突しないことを機械検証する。証跡: `rtk just VERSION=0.3.0 release-target-check`
- [x] Git pre-push hook が継承する Git environment を harness とその fixture temporary workspace に漏らさず、非 Git workspace を誤判定しない。証跡: `rtk just check`
- [ ] KDV の release preflight、GitHub Release、crates.io `0.3.0` 公開を確認する。

<!-- subagent-spark-harness-strict-start -->
- [x] KDV browser-session adapter の ownership を独立 review し、KRR への raw source/input/navigation 中継以外の HTML semantics を持たないことを確認する。証跡: agent: `019f75e7-c5e2-7293-b738-cfcc0290f921` / model: `gpt-5.3-codex-spark` / reasoning: `medium` / file: `crates/katana-document-viewer/src/browser_session.rs` / file: `crates/katana-document-viewer/src/browser_session_worker.rs` / file: `crates/katana-document-viewer/tests/browser_session_adapter_contract.rs` / command: `multi_agent_v1.spawn_agent` / verify: `rtk cargo test -p katana-document-viewer --test browser_session_adapter_contract --locked -- --test-threads=1` / close: `multi_agent_v1.close_agent`
