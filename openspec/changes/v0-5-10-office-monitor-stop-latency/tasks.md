<!-- subagent-spark-harness-strict-start -->

## 1. 原因と実装

- [x] 1.1 公開0.5.9の原本6件・代表Officeで段階とproducer/hash/非競合を固定する。元6件と代表XLSX、代表DOCX/PPTX uncontendedが各exit0。最初の代表runは途中競合のため性能比較から除外し保全。file: `Justfile`。delegation-exception: `直列のクリティカルパス`。
- [x] 1.2 監視停止のpark前/待機開始側の通知保持回帰を追加し旧実装REDを確認する。証跡: agent: `01a10050-fbd8-7eb2-837c-5738d3bed5a1` / model: `gpt-6-luna` / reasoning: `medium` / file: `crates/katana-document-viewer/src/multi_format/office_worker_monitor_tests.rs` / file: `openspec/changes/v0-5-10-office-monitor-stop-latency/evidence/monitor-ab.md` / command: `multi_agent_v1.spawn_agent` / verify: `rtk bash -c 'rtk proxy env RUSTFLAGS="-D warnings" CARGO_TARGET_DIR=tmp/issue59-rss.oBZVZI/cargo-target cargo test --locked -j2 -p katana-document-viewer --lib office_worker_monitor -- --nocapture'` / close: `multi_agent_v1.close_agent`。mainの旧source standalone最終回帰session70783は3FAIL/2PASS、現製品Cargoは5PASS。mainが未compileの参照名誤り/unused importを修正し実RED/GREENを照合。
- [x] 1.3 100ms周期・RSS上限・kill・joinを保ったpark/unpark修正とGREENを確認する。現workspace focused test session28161は5PASS/0FAIL、readonly reviewerHookeのP0/P1なしをmain照合。file: `crates/katana-document-viewer/src/multi_format/office_worker_monitor.rs`。delegation-exception: `直列のクリティカルパス`。
- [x] 1.4 同graph/入力の監視終了待ちとmetadata/frame/資源をA/Bし、残る変換/XLSX/配布/HTML未完と区別する。session17788全16run exit0、77file bytes一致/全8資源0、局所改善のみ。file: `evidence/monitor-ab.md`。delegation-exception: `直列のクリティカルパス`。
- [x] 1.5 DOCX/PPTX/XLSXの支配的段階を細分計測し、XLSX parse約2.7秒と親preflight残差を追跡する。候補release build session29009 exit0、細分session22024全8case exit0。大きいXLSXはfilter_catalog2503ms/zip_integrity515ms、後方filter/CRC/不正XML拒否を省く最適化は不採用。確認済み監視だけ品質・安全契約を維持して改善しA/B実証、元配布/HTMLの完了にはしない。file: `design.md`。delegation-exception: `直列のクリティカルパス`。

## 2. 品質と公開

- [x] 2.0 PR62のP2二重計上を既存計測契約内で修正する。証跡: native回帰2件は旧sourceで各exit101（44377/10154）、候補18184は両PASS。外側office.preflightを維持、内側office.package_inspectionはdepth0のみ。agent: `01a100ba-119a-7703-b670-86e0079bdabd` / model: `gpt-6-luna` / reasoning: `medium` / file: `scripts/feasibility/verify-office-profiling-stages.py` / command: `multi_agent_v1.spawn_agent` / verify: `rtk just office-profiling-stage-check` / close: `multi_agent_v1.close_agent`。mainがRED/全mutation/GREENと実traceを照合、file: `openspec/changes/v0-5-10-office-monitor-stop-latency/evidence/preflight-trace-review.md`。人間「二重計上直すで良いです！」で明示承認、API/表示/95/DoD変更なし。thread reply/resolve・再レビューは2.3で追跡する。

- [x] 2.1 公開依存互換/major監査・必要更新とlock/registry/V8 singletonを検証する。証跡: direct outdated全3package deps空、互換transitive cc/font-types/uuid更新、KMM/KAL既存解決版へ下限整合。lock SHA52443bbe…、registry KUC0.4.1/KRR0.4.22/office2pdf0.8.0、V8 singleton152.2.0。公開KRR修正版は未公開なので最終公開前に再確認する。file: `Cargo.toml` / file: `Cargo.lock` / file: `openspec/changes/v0-5-10-office-monitor-stop-latency/evidence/validation-progress.md`。delegation-exception: `直列のクリティカルパス`。
- [x] 2.2 fmt/AST/strict Clippy/全test/coverage/semver/Office fidelity/独立visual/strict release-checkを既存基準で通す。証跡: 2.0修正後のfull release-check82524 exit0、functions3742/3742・lines30730/30730、semver196/196、core1960PASS、通常Storybook640+isolated1/追加score23/ignored parity2/外部crop1PASS、代表Office fidelity48909とverify-record exit0。package915file/内側stage修正含有/private scratchなし、publish dry-run/unpublished確認PASS。旧23701のrawは保持し新sourceへ流用しない。file: `openspec/changes/v0-5-10-office-monitor-stop-latency/evidence/preflight-trace-review.md`。delegation-exception: `直列のクリティカルパス`。
- [ ] 2.3 関心別commit・通常push・Draft current-HEADレビュー・P0/P1修正・個別reply/resolve・fresh全thread取得を実施する。
- [ ] 2.4 Ready/3OS/preflight成功後通常mergeし、自動Release・tag/GitHub Release/crates.ioを確認する。
- [ ] 2.5 fresh registry consumer・checksum・V8 singleton・Office操作/closeを確認し局所Issue証跡更新とbranch cleanupを行う。Issue #58/#59は元DoDが全て満たされるまでCloseしない。

## 固定境界

公開0.5.9/reference/原本は非変更。HTML正常closeはrootが公開KRR修正後に担当する。Issue #58非visual採点方式の未回答を承認と扱わない。新stash/autostash/worktree/clone、master tracked編集、sibling編集、remote削除、no-verify/admin、品質/coverage/安全上限/geometry/deadline緩和は禁止。
