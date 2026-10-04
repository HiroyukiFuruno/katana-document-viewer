<!-- subagent-spark-harness-strict-start -->

## 1. 公開境界と依存

- [x] 1.1 office2pdf0.8.1のGitHub Release・registry実取得・PR1965の修正差分を照合する。証跡: file: `evidence/dependency-update-2026-10-04.md`、cargo info exit0/実crate SHAとregistry VCS一致。delegation-exception: `直列のクリティカルパス`。
- [x] 1.2 exact registry依存・lockfile・自然次版0.5.11を更新し、全依存の最新互換/major候補と契約を監査する。証跡: file: `evidence/dependency-update-2026-10-04.md`、全workspace dry-run更新候補なし/新graph静的contract exit0。採用候補の全gateは3.1で未完。delegation-exception: `直列のクリティカルパス`。

## 2. 回帰と元原本

- [x] 2.1 公開synthetic fixtureのautofitなし26pt/中央アンカー・明示autofit保持をKDV実worker/PDF合成変換でRED→GREENにする。証跡: file: `evidence/dependency-update-2026-10-04.md`、test: `office_wrapped_title_contract`の実PDF glyph/全文/複数行/anchor3件PASS。公開0.5.10実workerでは暗黙autofitの1件FAILを確認。通常profile/full gateは3.1で別検証。原本はGitHubへ出さない。delegation-exception: `直列のクリティカルパス`。
- [x] 2.2 新registry graphの元原本PPTX変換を同SHA/同制限/同fontで再受入し、全slideの文字/geometryと未評価の区別を記録する。証跡: file: `evidence/original-slide-verification-2026-10-04.md`、3PPTX全46pageのcollector/別verifier exit0。自然PDF診断であり、配布受入・geometry tolerance・95点は未評価。delegation-exception: `直列のクリティカルパス`。
- [x] 2.3 外部crop/full両PNGの実デコードを既存native scorerで必須化し、hash更新済みheader-only/IDAT CRC/zlib破損の回帰をRED→GREENにする。証跡: file: `evidence/current-crop-png-integrity-2026-10-04.md`、実native4test PASS/6破損入力拒否・入力bytes非変更。score/geometry/referenceは変更しない。delegation-exception: `直列のクリティカルパス`。
- [x] 2.4 push governanceの内部workspace誤拒否を実Git回帰で直し、外部source/Issue証拠の必須条件を維持する。証跡: file: `evidence/governance-workspace-2026-10-04.md`、旧10300の内部接続assertion RED→17438 GREEN、最終release-governance-check37231 exit0/内部4条件・外部9拒否・相対/wildcard exclude・commit/working-tree境界・caller保全PASS。通常再pushの終端は3.2で確認。delegation-exception: `直列のクリティカルパス`。

## 3. 品質と公開

- [ ] 3.0 Linux大型sidebarの実NotFoundを診断し、原因修正と同サイズ受入を完遂する。証跡: file: `evidence/linux-sidebar-diagnostics-2026-10-04.md`、通常FAILを保持した失敗時限定trace採取を準備、Linux原因は未確定。delegation-exception: `直列のクリティカルパス`。
- [ ] 3.0.1 Windows wrapped-title ITのP1を実AppContainer起動で修正し、同じPDF3件と直接起動拒否のnative回帰を通す。証跡: file: `evidence/windows-worker-regression-2026-10-04.md`、Mac focused3件/AST/strict Clippy PASS、Windows nativeは未実行。製品API/安全検査/制限は非変更。delegation-exception: `直列のクリティカルパス`。
- [ ] 3.0.2 追加Windows環境P1で必須変数の明示補完を修正し、全OS builder回帰とWindows native起動を検証する。証跡: file: `evidence/windows-worker-regression-2026-10-04.md`、同unitのRED→GREEN/実PDF3を含むMac4PASS、Windows nativeは未確認。delegation-exception: `直列のクリティカルパス`。
- [ ] 3.0.3 b391/bf0のWindows実PDF3件のCreateProcessW失敗を診断し、製品と同じ親環境契約で再検証する。証跡: file: `evidence/windows-worker-regression-2026-10-04.md`、file: `.github/workflows/test-and-build.yml`、bf0 native8件は5PASS/実PDF3FAIL・原始HRESULT0x800700CB。本番Nullの3並行はPASS、helper全親環境保持/TEMP・TMP置換の候補回帰を追加中。Windows実PDF再成功は未完。delegation-exception: `直列のクリティカルパス`。

- [x] 3.1 fmt/AST/strict Clippy/全test/coverage100%/fidelity/score/semver/strict release-check/V8 singleton/consumer linkを新graphで通す。証跡: file: `evidence/quality-gate-2026-10-04.md`、標準release-check73836 exit0、functions3742/3742・lines30730/30730各100/未cover0、package/publish dry-run PASS、実native V8 link PASS。公開後fresh consumerは3.3で別検証。delegation-exception: `直列のクリティカルパス`。
- [ ] 3.2 Draft PR作成/current-HEAD review/P0P1修正/個別reply resolve/fresh全thread/Ready/required finalHEAD全成功/通常merge/自動公開を完了する。PR63 HEADbf0はDraft/通常push済み、cloud review重大指摘なし。CI37173239313はWindows FAIL/Mac・Ubuntu SUCCESS、preflight37173239317 SUCCESS。Windows起動P1 thread PRRT_kwDOSTfBFs6otIi4と環境P1 PRRT_kwDOSTfBFs6otZjKは実PDF成功待ちで未resolve。新環境候補は3.0.3で検証中、旧HEADの成功を次HEADへ転用しない。delegation-exception: `直列のクリティカルパス`。
- [ ] 3.3 GitHub/crates.io/checksum/VCS/公開workerとfresh registry consumerを検証して、局所Issue証跡更新・merged local branch cleanupを完了する。delegation-exception: `直列のクリティカルパス`。

## 4. 元DoDの保持

- [ ] 4.1 旧Issue58/59のHTML元正常close・全packaged/clean-machine・四分類独立95を満たすかownerへ根拠付き差し戻しを確定してからClose/全cleanupする。局所公開だけで完了にしない。delegation-exception: `直列のクリティカルパス`。

## User Review Phase

- [/] 「二重計上直すで良いです」: 既存0.5.10で公開済み、今回の修正理由へすり替えない。証跡: file: `openspec/changes/v0-5-10-office-monitor-stop-latency/handoff.md`、PR62/公開0.5.10は旧公開台帳を参照。delegation-exception: `直列のクリティカルパス`。
- [ ] 「残2件を対応し必要な公開まで」: 局所改善と元受入を区別し、未検証のCloseをしない。delegation-exception: `直列のクリティカルパス`。

禁止: 新stash/autostash/worktree/clone/master tracked編集/sibling編集/remote削除/no-verify/admin/path/git override/原本公開/閾値・coverage・reference・geometry・deadline緩和。性能競合を避け、完了済み旧gateを新graph成功へ転用しない。
