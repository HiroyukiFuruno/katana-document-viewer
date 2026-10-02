<!-- subagent-spark-harness-strict-start -->

## 1. Git復旧と固定条件

- [x] 1.1 `core.bare=false`のみで`git st` exit0を確認する。証跡: `git config --local --get core.bare` false、`git rev-parse --is-inside-work-tree` true、`git st` exit0。delegation-exception: `直列のクリティカルパス`。
- [x] 1.2 旧indexと既存commitの一致、untracked衝突なしを確認し、stashなしで既存cwdとmasterを最新履歴へfast-forward、単一release/v0.5.9を準備する。証跡: `git rev-list --left-right --count master...origin/master` 0/0、`git worktree list --porcelain` existing rootのrelease/v0.5.9のみ、`issue59-rss.oBZVZI/cleanup-preflight.md`。delegation-exception: `直列のクリティカルパス`。
- [x] 1.3 DoD/禁止事項/未検証を物理台帳に固定する。証跡: 本tasks/design/specs、`rtk proxy scripts/openspec validate v0-5-9-office-source-lifetime-and-independent-crop --strict` PASS。delegation-exception: `直列のクリティカルパス`。

- [x] 1.4 Issue #60でself-test fixtureのGit環境継承を実GitでRED→GREENにする。証跡: old HEADのgovernance fixtureでcaller config変更を再現、native `--local-env-vars`隔離後のcaller-only/full environment各2scriptと例外復元回帰、`just release-governance-check` exit0、実repo config/index SHA前後不変。delegation-exception: `直列のクリティカルパス`。過去の`core.bare=true`発生経路自体は未確定で、fixture metadata破損の再現と区別する。公開/Closeは4.2–4.3。

## 2. Issue #59

- [x] 2.1 原本寿命の最小修正とmetadata/frame保持回帰を実施する。証跡: `rtk proxy cargo test --locked --release -j1 --target-dir issue59-rss.oBZVZI/cargo-target -p katana-document-viewer --lib office_static_adapter::tests` 5pass、`--test office_source_lifetime` 2pass、baseline同回帰RED→候補GREEN。file: `evidence/product-ab-2026-10-02.md`。delegation-exception: `直列のクリティカルパス`。
- [x] 2.2 同実入力・worker・SHA・倍率で製品A/Bし、原本drop、親子終了、live malloc、residentを測る。証跡: 同locked graph/worker SHA203810e8、実5Officeのbaseline261312→candidate139552KiB、metadata/frame10行diff exit0、全close8counter0、両親PIDのcold/final vmmap、両run exit0。寿命traceの旧RED→GREENは2.1。file: `evidence/product-ab-2026-10-02.md`。delegation-exception: `直列のクリティカルパス`。以前の13行表記はrawの件数10行へ訂正した。latest graph/元GUIの受入は4.3、mixed/全残差判断は2.3に残す。
- [ ] 2.3 残差を原因別に確定し、KDV修正かowner Issueへの差し戻しかを証拠で判断する。delegation-exception: `直列のクリティカルパス`。
- [x] 2.4 DocumentSessionのOffice rasterを既存sandbox/deadline/制限/型付きerror/cacheを保ったchildへ隔離し、実製品A/Bで検証する。証跡: agent: `01a0fb56-9ed1-7a51-b53e-704e150550cd` / agent: `01a0fb56-9f94-74b3-9d2e-c73f2d6c3da6` / model: `gpt-6-luna` / reasoning: `medium` / file: `crates/katana-document-viewer/src/multi_format/pdf_raster_worker_render.rs` / file: `crates/katana-document-viewer/src/multi_format/office_worker_process_command_config.rs` / command: `multi_agent_v1.spawn_agent` / verify: `rtk cargo test --locked -p katana-document-viewer --target-dir issue59-rss.oBZVZI/cargo-target --lib` / close: `multi_agent_v1.close_agent`。Confuciusはchild責務分割、Socratesはprocess設定分割。mainがmodule参照・cfg importを修正してCargo検証を再実行。実5OfficeのRSS増分262144→136256KiB、metadata/frame一致、最大PPTX11反復cold100320/warm6352KiB、全close8counter0、旧graph library1950pass/0fail/1ignored、実frame/cache契約1pass・寿命2pass。詳細はevidence/product-ab-2026-10-02.md。公開後の元GUI受入は4.3に残す。

- [x] 2.5 カスタムPDF容量上限をRGBAへ誤適用する互換性回帰を修正し、実workerでRED→GREENと完全gateを再検証する。証跡: agent: `01a0fbf9-5bf3-7ef2-be35-954e27a9b24d` / model: `gpt-6-luna` / reasoning: `medium` / file: `crates/katana-document-viewer/src/multi_format/pdf_raster_worker_parent.rs` / command: `multi_agent_v1.spawn_agent` / verify: `rtk cargo test --locked -j1 -p katana-document-viewer --test office_raster_worker_contract` / close: `multi_agent_v1.close_agent`。Darwinはreadonly契約レビュー、mainが実回帰/修正/検証を担当。PDF input容量とRGBA描画pixel制限を分離し、安全上限は緩和しない。分割後の完全release-check session97206 exit0、詳細はevidence/final-rgba-limit-validation.md。

## 3. Issue #58

- [x] 3.1 現行KatanA候補の明示入力・SHA/寸法/frame provenance検証と不一致回帰を実装する。証跡: agent: `01a0fb60-fd83-7031-9410-2075d1a61ae6` / model: `gpt-6-luna` / reasoning: `low` / file: `scripts/feasibility/verify-current-preview-crop.py` / command: `multi_agent_v1.spawn_agent` / verify: `rtk just current-preview-crop-verifier-test` / close: `multi_agent_v1.close_agent`。Jasonは独立入力/既存受入境界のreadonly照合、mainが実producer JSONとSHAを照合。27/27、実artifact/frame1223のprovenance_verified=true、PNG全体decode PASS、visual100/95。他の独立三項目は3.2に残す。初期実装/検証はSartre/Aquinasとmainによるもので、この記録をJasonによる実装と扱わない。evidence/current-crop-input-verification.md参照。
- [ ] 3.2 既存reference非上書き・四項目各95点の独立評価/採用を完了する。
- [x] 3.1a 四つの明示artifact入力を受けるcrop採点ターゲットを追加し、visual100/95を確認する。証跡: `issue59-rss.oBZVZI/current-crop-visual-score.log`と最新KUC0.4.1での別再実行`current-crop-visual-score-kuc041-final.log`、file: `Justfile`。delegation-exception: `直列のクリティカルパス`。root新HEADの採用と他の三項目は3.2に残す。

## 4. 公開・後処理

- [x] 4.1 最新公開依存の調査/更新/再検証、fmt/AST/strict Clippy/全test/coverage100%/semver/95点/release-checkを実行する。証跡: final-rgba-limit-validation.md、session97206完全release-check exit0、fixed Office oracle/scorecardと明示crop visual100/95。独立四項目の採用は3.2、公開後consumer/元GUIは4.3に残す。delegation-exception: `直列のクリティカルパス` / file: `Justfile`
- [x] 4.1a KUC 0.4.1のGitHub Release/registry公開を照合し、exact registry採用後に全依存ゲートを再検証する。証跡: registry checksum f8fd7856、lock SHA3b28f987、通常check/strict coverage/semver/Office fidelity/visual/release-check exit0、file: `evidence/dependency-update-2026-10-02.md`。delegation-exception: `直列のクリティカルパス`。旧0.4.0結果を新graphへ流用せず別実行。root独立四項目・公開後fresh consumerは未完了。
- [x] 4.1b 最新KUC0.4.1 graphの通常check・厳格coverage・semver・Office fidelity・明示crop visualを再実行する。証跡: `just-check-v059-kuc041.log`、`coverage-v059-kuc041-final-gate.log` functions3739/3739・lines30683/30683、`semver-v059-kuc041.log`196pass/58skip、`office-fidelity-v059-kuc041-final.json`、`current-crop-visual-score-kuc041-final.log` visual100/95。file: `evidence/product-ab-2026-10-02.md`。delegation-exception: `直列のクリティカルパス`。完全release-check・四項目採用・公開/下流は未完了。
- [x] 4.1c 完全release-checkと最終互換/major依存監査を実行する。証跡: `release-check-v059-kuc041-final.log` exit0（package/publish dry-run含む）、`dependency-compatible-final-dry-run.log`更新0件、`dependency-major-final-audit.json` workspace3crate/root-deps候補0件、lock SHA不変。file: `evidence/dependency-update-2026-10-02.md`。delegation-exception: `直列のクリティカルパス`。公開/下流/Issue58の独立三項目は未完了。
- [ ] 4.2 Draft current-HEAD review、指摘reply/resolve、P0/P1 0の再取得、Ready、3OS/preflight、merge、自動公開を完了する。
- [x] 4.2a Draft PR61のUbuntu CIで検出された寿命回帰testのC ABI不一致を修正する。`write`/`_write`のbufferを`*const c_void`へ合わせ、固定byte列をcastして渡す。製品code・期待値・閾値・lint条件は不変。証跡: Ubuntu run37015396565 job110864901393のstrict lint RED、修正後macOSの実Office寿命test2pass/0fail、fmt/AST/strict Clippy exit0。Ubuntu/Windows実行は次HEADのCIで別検証する。delegation-exception: `直列のクリティカルパス` / file: `crates/katana-document-viewer/tests/support/office_source_lifetime_allocator.rs`
- [ ] 4.3 fresh registry consumer/V8 singleton/checksum/元下流受入を確認し、対象Issueを根拠付きCloseする。
- [ ] 4.4 過去stash5件の意味的採否・必要差分の正式統合と検証、不要branch/worktree整理を完了する。最終採否/削除はmainの直列判断。delegation-exception: `直列のクリティカルパス`。Deweyの初期監査をmainがZIP回帰等で訂正済み。現在の再監査も個別履歴・source・実回帰で再照合し、棚卸しだけを整理完了としない。
- [x] 4.5 最新モデル指示と物理証跡の矛盾を検出する回帰を追加し、旧Spark medium制約と必須実行ID/検証/終了記録を維持した明示Luna規則を実装する。証跡: `rtk proxy bash scripts/check-subagent-spark-harness-tests.sh` RED→GREEN、明示Luna none/low/medium成功、未許可model・矛盾reasoning・ID/verify/close欠落拒否。delegation-exception: `直列のクリティカルパス` / file: `scripts/subagent-spark-harness-evidence.sh` / file: `scripts/check-subagent-spark-harness-tests.sh` / file: `.codex/workflows/subagent-spark-policy.md`

- [x] 4.1d 2.5の互換修正と1.4のGit/release監査修正後に、通常check・strict coverage100%・semver・fidelity・RSS・完全release-checkを再実行する。4.1a–cのPASSは本修正前snapshotの証跡で、現候補へ流用しない。分割後の完全release-check session97206 exit0、strict coverage functions3740/3740・lines30702/30702、Office fidelity、実5Office cold135664KiB・最大PPTX11反復cold96608/warm2512KiBを確認。公開・下流・独立採用・Close/cleanupは未完了。証跡: file: `evidence/final-rgba-limit-validation.md`。delegation-exception: `直列のクリティカルパス`。

## User Review Phase

- [/] 「先にgit stを直す」: 設定だけで復旧、exit0確認。同期は復旧そのものではなく後続Issue対応の準備として区別した。証跡: `git st` exit0、`issue59-rss.oBZVZI/README.md`。delegation-exception: `直列のクリティカルパス`。
- [/] 「以後stash禁止」: 新stash/autostashを全面禁止。AGENTS.mdへ明文化、既存5件は採否検証前に消していない。証跡: file: `AGENTS.md`、`rtk proxy git stash list --format='%gd %H %s'` 既存5件のOID不変。delegation-exception: `直列のクリティカルパス`。
- [ ] 「原因確定まで続行、KDVなら公開/Close/掃除、KDV除外なら差し戻してClose」: 未検証を完了扱いしない。

検証導線: file: `Justfile`。検索語: root cause first / no stash / no autostash / current crop independent 95 / registry release and cleanup。
