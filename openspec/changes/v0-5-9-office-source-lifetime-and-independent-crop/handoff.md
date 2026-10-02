# v0.5.9 継続台帳

## 固定した指示と担当境界

Git復旧は `core.bare=false` の設定だけで完了し、`git st` exit0を確認済み。
新しいstash/autostash、新worktree、clone、並行作業branchは禁止。
既存cwdの `release/v0.5.9` で #58/#59 の原因解明・必要修正・公開・後処理まで続ける。
KDV外はreadonly調査のみ。rootのGUI受入をKDV単体診断で代用しない。

## 実測済みと未検証

旧locked graphで実5OfficeのRSS増分262144→136256KiB、全close8counter0、metadata/frame一致。
原本dropだけではRSS改善が再現しないため、Officeの親raster実行を既存制限付きchildへ隔離した。
current KatanA cropはprovenance回帰27件と独立visual100/95が成功。
いずれも新依存graph・元GUI受入・四項目各95点・公開の完了を意味しない。

KRR0.4.22/office2pdf0.8.0/KUC0.4.1はregistry候補として取り込み済み。
KUC0.4.0候補では新graphの5Office cold139520KiB・最大PPTX11反復cold98240/warm4576KiB、Office fidelity、semver196件がPASS。ただし最新reader/JSON変更とKUC0.4.1を含む最終graphでの再測定が必要。
新private raster modeの実異常系はCPU hard-limit拒否と実引数不正→exit64を含む5件が成功。実際の読込失敗/length不一致/OS resource拒否を追加し、JSONは書込中も64KiB上限を守るwriterに変更した。閾値・除外は変更していない。
最新KUC0.4.1 graphで通常check/strict Clippy/AST/fmt、semver196pass/58skip、release-contract-check、V8 singleton152.2.0、Office fidelityがexit0。厳格coverageはfunctions3739/3739・lines30683/30683で100%。rawはissue59-rss.oBZVZI内のkuc041-finalログ。完全release-check・公開・元GUI受入は未完了。台帳のstrict markerと実ID/critical-path例外を補完した単独harnessはexit0。
最新graphの実5Office cold141248KiB、最大PPTX11反復cold98496/warm4336KiBで元上限を維持し、全close8counter0、直前Office0.8.0候補とのmetadata/frame10行一致。worker/rlib/lock SHAはevidence/product-ab-2026-10-02.md。元GUI受入の代用にはしない。
最新scorerの同immutable外部crop visualも100/95（average99、exit0）。完全release-checkは`issue59-rss.oBZVZI/release-check-v059-kuc041-final.log`でexit0を確認。package/publish dry-run成功、0.5.9は未公開。debug linkの`__eh_frame`警告は依存台帳に明記し、警告なしとは報告しない。
互換dependency dry-runは更新0件・lock SHA不変。mainが全workspace/root-depsのmajor監査を追加実行中。git st復旧とは別に、削除済み旧pathを指していたcore.hooksPathを既存install-governance-hookで現cwdへ戻した。既存lefthook delegateは保持し、通常push検査を迂回しない。

## 最終差分の限定レビュー

James（01a0fbcf-1b37-7be0-8dc5-49a48d40b0a4、gpt-6-luna/medium）はprivate raster reader/response/parent/protocol/entrypointと異常系テストをreadonlyで確認し、再現可能なP0/P1なしと報告。build/testは実行しておらず、mainの実ゲートを代替しない。終了済み。mainは既存cache/public API/Windows command/sandbox設定の差分を別途読み合わせた。

HTML closeはrootの修正後close_all_documentsでも元5秒基準に未達との独立報告がある。主スレッドの既存sampleは資源確認ループ、workerはKRRのfallback I/O/layoutであり、同期joinが主原因だったとは確定していない。rootの新しい補足でKRR #95はreopen済み。Office隔離やcounter0からHTML解消を推定しない。

## 次のクリティカルパス

### 最新の差分と未再検証（2026-10-02）

完全release-check session97206 exit0。分割後の通常check/semver196/strict coverage100%に加え、package検証とpublish dry-run、未公開確認まで成功。生成crateには新raster source/実ITが存在し、private scratch/binary/AGENTSの混入patternは0件。macOS debug linkの既存`__eh_frame`警告は保持し、警告ゼロとは報告しない。self-reviewを候補commit/Draft PR準備PASSとした。source修正は凍結後のtest分割のみで、製品処理・閾値・原本は変えていない。次は関心別commit→通常push hook→Draft/レビュー→全thread再取得→Ready/3OS/preflight→merge/公開→fresh consumer/下流/Close/cleanup。private raw/原本/ignored AGENTSはstageしない。delegation-exception: `直列のクリティカルパス` / file: `Justfile`

session97206のsemver196pass/58skip、通常check（core1957pass/1ignored、通常Storybook640pass/0fail/22ignoredと隔離mouseclick1pass）、全harness、provenance27件、V8 singletonが成功した。続くstrict coverageもfunctions3740/3740・lines30702/30702・未カバー0で成功し、package検証ビルドへ進行中。raw coverage全summaryはRTK teeから`issue59-rss.oBZVZI/coverage-v059-dimension-split-final-summary.log`へ保持し、mainがTOTALを照合した。完全release-checkの最終終了コードは未確認。標準入口のcargo cleanは再生成可能なprivate target34.8GiBだけを整理し、source/凍結artifact/既存stashを保持した。delegation-exception: `直列のクリティカルパス` / file: `Justfile`

公開依存の再照合でもKUC0.4.1/KRR0.4.22/office2pdf0.8.0が最新公開版、KDV最新は0.5.8で、新たな依存更新は不要。mainが3repoのGitHub releases/latestと3crateのcargo searchを別途ライブ照合した。証跡: agent: `01a0fca4-4040-75d3-8d2f-300f4e0ef887` / model: `gpt-6-luna` / reasoning: `low` / file: `Cargo.lock` / command: `multi_agent_v1.spawn_agent` / verify: `rtk bash -c 'rtk proxy cargo search katana-ui-core --limit 1'` / close: `multi_agent_v1.close_agent`。Maxwellのregistry非yanked確認はCargo/sparse index経由で、直接APIは403だったことを区別する。現候補manifest/lockは変更しない。

session55336はsemver196pass/58skipを確認したが、追加回帰によりparent testsが216行となってAST file-length検査でexit101。寸法検証3testを新しい`pdf_raster_worker_dimension_tests.rs`へそのまま移し、parent196行/tests155行/dimensions66行、fmt/AST session26648 exit0を確認した。製品処理・期待値・test件数・閾値・除外は変更しない。完全release-checkをsession97206・raw `release-check-v059-dimension-split-final.log`で再実行中。前gateの部分成功を完全成功と扱わない。delegation-exception: `直列のクリティカルパス` / file: `Justfile`

Faradayはproduct-ab証跡だけの件数訂正を担当し、mainがrawのone10/two15行と3箇所の10行表記を照合した。証跡: agent: `01a0fc99-f07d-7dc2-8b00-707ac01fbcbf` / model: `gpt-6-luna` / reasoning: `low` / file: `openspec/changes/v0-5-9-office-source-lifetime-and-independent-crop/evidence/product-ab-2026-10-02.md` / command: `multi_agent_v1.spawn_agent` / verify: `rtk bash -c 'rtk proxy rg -c "^cycle=" issue59-rss.oBZVZI/unified-product-v059-kuc041-final-five.log issue59-rss.oBZVZI/unified-product-v059-rgba-limit-final-five-one.log issue59-rss.oBZVZI/unified-product-v059-rgba-limit-final-five.log'` / close: `multi_agent_v1.close_agent`。現在の継続DoDは公開・fresh consumer・元GUI/独立四項目・Issue closure/既存stash整理までで、rootからの定期照合でもOffice cold RSSとHTML normal-close受入未達の条件は維持されている。

全registry consumer/明示cropのsession46100はexit0、core1957pass/1ignored、Storybook664pass/0fail/22ignored、crop1pass/0fail・visual100/95。完全release-check初回session26605はsemverの入力発見でexit101。前回Cargo packageの自動生成展開物がprivate cargo-target配下で元crateと重複していたため、その生成物1件だけを`tmp/trash/2026-10-02-122300/katana-document-viewer-0.5.9`へ回収可能な移動を行った。Git ignore一致を確認し、ソース/通常cache/凍結worker/rlib/stashを変更していない。元のrelease-check入口・ゲート条件を変更せずsession55336で別raw `release-check-v059-rgba-limit-final-rerun.log`へ再実行中。Issue59の現候補実測更新はissuecomment-5952206399。生成物の入力衝突を解消しただけで、semver/完全gateの成功確認はまだない。delegation-exception: `直列のクリティカルパス` / file: `Justfile`

coverage再実行はexit0。functions3740/3740・lines30702/30702・未カバー0で、既存Justfileの必須条件を全て満たした。証跡: agent: `01a0fc78-2cd4-75a0-bde8-04d44e61f356` / model: `gpt-6-luna` / reasoning: `low` / file: `Justfile` / command: `multi_agent_v1.spawn_agent` / verify: `rtk bash -c 'rtk proxy tail -n 5 issue59-rss.oBZVZI/coverage-v059-rgba-limit-final-rerun.log'`（mainのraw照合） / close: `multi_agent_v1.close_agent`。Lockeの「regions98.80%なのでDoD未達」という解釈は採用しない。regions100%は既存gateにも今回の依頼にもなく、functions/lines100%・uncovered0を別の条件に置換しない。regionsは495未カバーのまま明記する。

release build session61430はexit0、製品worker/rlibを凍結し新driverのcompile session86850もexit0。worker SHA ba7d19d1、rlib94fece11、driver37102567、lock3b28f987。RSSは両build終了後、session57760で同5Officeの再測定を開始。Office fidelity session83791はexit0、verification.tolerances_met=true/失敗0、DOCX2ページMAE0.02525295/RMSE0.10328595、XLSX各不一致/欠落0。raw `office-fidelity-v059-rgba-limit-final.json`。完全release-check/semver/95点の現候補再実行と公開工程は未完了。delegation-exception: `直列のクリティカルパス` / file: `Justfile`

最終coverageは未カバー1行でexit1。functions3740/3740、lines30701/30702で、表示の100.00%を成功とは扱わない。`pdf_raster_worker_parent.rs:90`の寸法エラー伝播を直接確認するunit回帰をmainが追加した。製品コード/安全上限/閾値/除外は変更していない。証跡: agent: `01a0fc70-029e-7592-a2de-105de76412bd` / model: `gpt-6-luna` / reasoning: `low` / file: `Justfile` / command: `multi_agent_v1.spawn_agent` / verify: `rtk bash -c 'rtk proxy tail -n 12 issue59-rss.oBZVZI/coverage-v059-rgba-limit-final.log'`（mainの結果照合） / close: `multi_agent_v1.close_agent`。実行は`rtk proxy env`経由の`just JOBS=1 coverage`で、両targetは絶対パス。raw `coverage-v059-rgba-limit-final.log`。Lockeによる再実行は別ログ`coverage-v059-rgba-limit-final-rerun.log`で進行中。release buildとは別target・各1jobで並行し、RSS測定は両build完了後に行う。

現候補の通常全件checkはsession82463 exit0。本体1956pass/0fail/1ignored、workspaceの実IT、Storybook通常経路640+1pass/0fail、provenance27件、strict Clippy/AST/fmt/境界/harness/V8 singletonが成功した。raw `just-check-v059-rgba-limit-final-rerun.log`。完全release-check/coverage/semver/最新RSS/独立fidelityと公開工程は未完了。互換dependency dry-run session95279は更新0、lock不変。製品の再測定用buildは正しいbin名`kdv-office-worker`と`RUSTFLAGS=-D warnings`を明示したsession61430で進行中（先行2commandは存在しないbin名によりexit101、検証成功に数えない）。delegation-exception: `直列のクリティカルパス` / file: `Justfile`

Hegelのreadonly cross-platform process契約確認ではP0/P1なし。証跡: agent: `01a0fc67-82d8-7be3-8be7-0592aae3dbd9` / model: `gpt-6-luna` / reasoning: `medium` / file: `crates/katana-document-viewer/src/multi_format/office_worker_process_windows.rs` / command: `multi_agent_v1.spawn_agent` / verify: `rtk cargo test --workspace --all-targets --all-features --locked --exclude kdv-storybook`（mainの通常check内で実行） / close: `multi_agent_v1.close_agent`。mainは親frame/RGBA/reader/public APIを別途確認。Windows実行・元GUI受入をこの静的確認で代替しない。既存process契約に独立cancellation tokenはなく、確認した終了保証はtimeout/typed errorである。

最新再検証: session56190の通常checkはAST lintで`complete_response`の35行制限違反を検出しexit101。応答identity/status検証をfocused helperへ分割し、再実行session82463ではfmt/strict Clippy/AST/entrypoint/document boundaryがPASS、全件testへ進行中。raw `just-check-v059-rgba-limit-final-rerun.log`。検査除外/安全上限/期待値の変更はない。delegation-exception: `直列のクリティカルパス` / file: `Justfile`

Issue58の独立証跡をLinnaeusがreadonly照合した。証跡: agent: `01a0fc2e-b232-7261-b761-5beee3a06ec3` / model: `gpt-6-luna` / reasoning: `medium` / file: `openspec/changes/v0-5-9-office-source-lifetime-and-independent-crop/evidence/current-crop-input-verification.md` / command: `multi_agent_v1.spawn_agent` / verify: `rtk bash -c 'rtk proxy git -C /Users/hiroyuki_furuno/works/private/katana-post-v0.22.41-document-fidelity rev-parse HEAD'` とKatanA canonical evidenceのreadonly照合 / close: `multi_agent_v1.close_agent`。producer現HEADはb82c047、cropは旧7b5224d由来で、独立semantic/interaction/performance各95点の実数と採用判断はない。mainはIssue本文/specの四項目条件を再確認した。公開だけでIssue58をCloseせず、画像条件counterから他分類の点数を推定しない。

最新追記: Office custom-limit/frame/cache ITはsession37356 exit0・1pass/0fail、実worker異常系はsession61000 exit0・5pass/0fail。128bytes受理/129bytes拒否/CPU拒否等を維持した。通常全件checkはsession56190、raw `just-check-v059-rgba-limit-final.log` で進行中。以下の37356「再build中」はこの追記前の履歴。coverage/semver/RSS/fidelity/完全release-checkと公開以降はまだ未完了。delegation-exception: `直列のクリティカルパス` / file: `Justfile`

完全release-check/coverage/RSSの上記PASSは、後述の互換修正前snapshot。現在のHEADは6ae11c2、release/v0.5.9は未commit/未push、PRなし。current codeのPASSと混同しない。

- Git fatal復旧はbare=falseのみでgit st成功。削除済みhook pathも現cwdへ復旧し、既存Lefthook委譲を保全。delegation-exception: `直列のクリティカルパス` / file: `.githooks/pre-push`
- Issue #60 https://github.com/HiroyukiFuruno/katana-document-viewer/issues/60 を作成。native local Git環境をself-test範囲だけ隔離し、実fixtureのcaller metadata非変更と例外復元回帰がGREEN。実repo config/index SHA前後一致。過去のbare=true原因自体は未確定。
- remote自動削除はユーザー指示と矛盾したため、Release workflowを公開後auditへ変更。自動--applyを拒否するcontract RED→GREEN、実workflow/既存governance入口PASS。
- Darwinのreadonly契約レビューで小さいPDF出力上限がRGBAを拒否する互換回帰を確認。実package rlib+凍結workerのconsumerはPDF55,832bytes成功後にRGBA2,001,580bytesを1MiBで拒否、exit101。private requestへpixel由来RGBA上限を分離する候補修正済み。128bytesの実request受理も異常系へ追加。証跡: agent: `01a0fbf9-5bf3-7ef2-be35-954e27a9b24d` / model: `gpt-6-luna` / reasoning: `medium` / file: `crates/katana-document-viewer/src/multi_format/pdf_raster_worker_parent.rs` / command: `multi_agent_v1.spawn_agent` / verify: `rtk env CARGO_BUILD_JOBS=1 cargo test --locked -j1 -p katana-document-viewer --test office_raster_worker_contract` / close: `multi_agent_v1.close_agent`。その実Cargo検証は進行中で、GREENを確認した記録ではない。
- 現在、単一Cargo実行 `CARGO_BUILD_JOBS=1 RUSTFLAGS=-D warnings CARGO_TARGET_DIR=issue59-rss.oBZVZI/cargo-target cargo test --locked -j1 -p katana-document-viewer --test office_raster_worker_contract -- --nocapture` が依存を再build中。unified exec session37356。最初のflagなし同検証session81404は自己所有jobを中断済み（exit130）。凍結artifactのREDを確定したので、37356は修正後GREEN検証。delegation-exception: `直列のクリティカルパス` / file: `crates/katana-document-viewer/tests/office_raster_worker_contract.rs`
- 次はそのGREEN、実worker failure5件、lib/全workspace check、strict coverage、semver、Office/RSS/fidelity、完全release-checkを現候補で再実行する。前結果を流用しない。delegation-exception: `直列のクリティカルパス` / file: `Justfile`
- readonly reviewer Epicurus（01a0fc00-0761-7640-b2d0-3ed43bf6251b、Luna/medium）はGit fixture範囲P0/P1なし、Ampere（01a0fc10-58cf-7c31-9f69-454e29867e16、Luna/medium）はRGBA分離範囲P0/P1なし。Darwin/Epicurus/Ampereは全てclose済み。mainが実ゲートを別途実行する。
- 最終互換dry-runは更新0、major/root-deps auditはworkspace3crate候補0でexit0、lock SHA不変。旧「監査中」は完了した履歴に更新。新しいソース修正ではmanifest/lockを変えていない。
- 新stash/autostash、製品worktree/clone/sibling編集/remote削除は行わない。旧5stashは同OIDのまま保全。private scratch/原本/凍結binaryをcommit/packageへ含めない。
- Issue #58の独立semantic/interaction/performance・現行artifact採用、元GUI/公開後consumer、Issue Closure/cleanupは未完了。

- mainが新graphの完全gateと実測を統合し、Draft current-HEAD review、P0/P1 reply/resolve、Ready、3OS/preflight、merge、公開、fresh registry consumer、元下流受入、Issue closureを順に確認する。delegation-exception: `直列のクリティカルパス` / file: `openspec/changes/v0-5-9-office-source-lifetime-and-independent-crop/tasks.md`
- mainが既存stash5件の意味的採否を判断し、必要差分の統合・検証後にのみ不要物を整理する。delegation-exception: `直列のクリティカルパス` / file: `openspec/changes/v0-5-9-office-source-lifetime-and-independent-crop/tasks.md`

独立作業の補助担当は最新ユーザーModel Routing Policyに従い、Nietzsche（01a0fb7f-93f4-7ab2-a225-2207853e8254、gpt-6-luna/low）が依存台帳1ファイル、Heisenberg（01a0fb7f-952b-7823-8c28-a38b03fd78bb、gpt-6-luna/medium）が最古stashをreadonly監査。両者は終了済み。mainは実manifest/lock、archive、GitHub Releaseと照合して結果を統合する。

一次情報は同階層のtasks.mdとevidence。未対応項目を完了と読み替えない。

追加readonly担当: Volta（01a0fba6-c195-7303-ab98-327a61775f1f、gpt-6-luna/low）がKUC0.4.1公開・参照を監査、Noether（01a0fbae-200f-7ed2-b222-f52e694a94e8、gpt-6-luna/medium）が過去依存stash2件を意味的監査。終了済み。mainは公開版/cargo/lock checksumと現行sourceを再照合し、監査者の「stash OIDが一覧にない」という誤りは採用しない（mainのGitで既存5OID不変）。
