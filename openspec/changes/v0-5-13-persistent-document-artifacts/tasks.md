<!-- subagent-spark-harness-strict-start -->

# 主担当の統合・検証タスク

## 2026-10-22 再検証と独立残作業

- [x] 公式IronCalc0.9.1（ZIP9）をmanifest/lock/release契約へ取り込む。registry checksum、ReleaseのPR1565包含、exact-head CI6件成功を確認した。Python release契約/self-test成功。証跡: file: `tmp/kdv-upstream-publication-20261022.json`。delegation-exception: `直列のクリティカルパス`。
- [x] PR68 target pathname差し替えを、所有directoryを保持した内容整理で防ぐ。root/tagは残し、選択clean/sweepは削除前・target作成前に拒否する。40GiB閾値を維持し、POSIX置換・Windows共有競合・外部リンク・readonlyの実回帰と独立レビューを回収する。delegation-exception: `直列のクリティカルパス`。 追補: HEAD4aeb7479で通常push/3OS CI39664459317/preflight39664459501/coverage3843関数31575行100%/semver202PASS/全thread解決を回収済み。証跡 `tmp/kdv-4aeb7479-preparation-receipt.md`。2026-10-29の新graph検査は末尾の別タスクで追跡し、この過去結果を流用しない。
- [x] POSIX中間ancestor差替えを隔離した実filesystemでRED再現し、`/`からの全component no-follow/dir_fd連鎖で拒否する。最終target差替え・外部保持・選択clean拒否を含む限定8回帰は成功（MacではWindows専用3件skip）。最終全script suite/native CIは上記タスクで回収する。証跡: file: `tmp/pr68-posix-ancestor-red.json`、`tmp/kdv-anchor-ancestor-green.log`。delegation-exception: `直列のクリティカルパス`。
- [x] 明示cleanの指定外target削除を実filesystemでRED再現し、CLI単一target > 環境target > 既定targetへ限定する。manifest/global/重複/guard専用指定は削除前拒否、helpはtarget作成なし。標準71回帰成功（MacではWindows専用等4件skip）、独立レビュー追加指摘0。既存tag競合回帰は未対応manifest指定を外して同じ競合を検証し、busy時status3も維持する。最終push/native CIは上記未完タスクで追跡。証跡: file: `tmp/pr68-explicit-target-red.json`、`tmp/kdv-target-selection-full71.log`。delegation-exception: `直列のクリティカルパス`。
- [x] coverageの既存選択cleanとの矛盾を解消し、coverage専用targetだけを明示全体刷新する。親target/所有タグ保持の実filesystem回帰を含む72件成功、公開契約は現recipe受入・旧選択recipe拒否。関数/行100%条件は維持。最終HEADの公開前検査は下記で回収する。証跡: file: `tmp/kdv-coverage-refresh-script72.log`、`tmp/kdv-coverage-refresh-contract-regression.json`。delegation-exception: `直列のクリティカルパス`。
- [x] fresh Codex reviewのlsof未導入P2へ対応し、Linux CIの共通依存導入とPOSIX初期手順を明示する。ツール不在は導入要求を表示して安全に拒否し、生成物/所有タグを保持する実回帰を含む73件成功。証跡: file: `tmp/kdv-lsof-dependency-script73.log`。delegation-exception: `直列のクリティカルパス`。
- [x] Windows native f496回帰の2件失敗を修正し再回収する。属性のみのpinをLIST_DIRECTORY読取で共有検査へ参加させ、失敗時のテストHANDLE漏れを防ぐ。有効なjunction bufferのoutside正例を先行する。bb78再CIで属性専用handleの更新成功が判明したため、祖先/tag/子のopenと列挙を保持HANDLE相対へ統一し、live reparse更新成功後も外部を削除しない回帰と通常junction整理正例を検査する。CIの容量回帰をharnessより先へ移し、ゲートを省略せず早期検出する。証跡: file: `tmp/pr68-f4960c7-windows-job.log`、`tmp/pr68-bb78ec0f-windows-job.log`、`tmp/kdv-windows-relative-io-script75.log`。delegation-exception: `直列のクリティカルパス`。 追補: HEAD4aeb7479で通常push/3OS CI39664459317/preflight39664459501/coverage3843関数31575行100%/semver202PASS/全thread解決を回収済み。証跡 `tmp/kdv-4aeb7479-preparation-receipt.md`。2026-10-29の新graph検査は末尾の別タスクで追跡し、この過去結果を流用しない。
- [x] 上記最終source/graphで既存全ゲート、coverage100%、semver、package/dry-run、3 OS CI、fresh review/reply/resolveを回収する。途中中断のrelease-contract検査（exit130）は成功に数えない。delegation-exception: `直列のクリティカルパス`。 追補: HEAD4aeb7479で通常push/3OS CI39664459317/preflight39664459501/coverage3843関数31575行100%/semver202PASS/全thread解決を回収済み。証跡 `tmp/kdv-4aeb7479-preparation-receipt.md`。2026-10-29の新graph検査は末尾の別タスクで追跡し、この過去結果を流用しない。
- [ ] office2pdf/派生2packageの公式ZIP修正版とKRR新版が公開された後に取り込み、最終全検査・公開・Issue Close・ローカル整理を行う。delegation-exception: `直列のクリティカルパス`。

独立した限定作業はzip_diagnosis・profiling_inventory・cache_testsへ渡した。利用可能モデルの制約に従いgpt-5.6-luna / mediumを明示した実履歴がある。以下は主担当が依存順に行う統合と公開判定であり、workerの実装完了をそのまま完了証跡にしない。

- [x] 1.1 #65/#66の公開契約・設計と実装を照合し、cold/warm/restart計測・無効化・容量/破損/clear/closeの実回帰を統合する。delegation-exception: `直列のクリティカルパス`。
- [x] 1.2 #67の合法PPTX再現で原因を特定し、安全制約を維持した実worker回帰を統合する。原本未取得を明記。delegation-exception: `直列のクリティカルパス`。
- [ ] 1.3 全直接/推移依存とmajor候補を評価し、KRR修正版の公開registry取得後にmanifest/lock/契約を更新する。delegation-exception: `直列のクリティカルパス`。
- [ ] 2.1 fmt/AST/Clippy/full test/coverage100%/score/semver/strict release-checkを同じsourceとdependency graphで通す。delegation-exception: `直列のクリティカルパス`。
- [ ] 2.2 Draft/current HEAD review、指摘修正・reply/resolve、fresh P0P1ゼロ、Ready、required CI、merge、自動公開を完了する。delegation-exception: `直列のクリティカルパス`。
- [ ] 2.3 GitHub/crates.io/fresh exact registry consumer、対象Issue根拠付きClose、ローカル整理を完了する。delegation-exception: `直列のクリティカルパス`。

## User Review Phase

- [ ] KRR修正版を先に取り込んでからリリースし、先にKDVが終わってもKRRを待つ。delegation-exception: `直列のクリティカルパス`。

禁止: stash/autostash/新worktree/master編集/sibling編集/remote branch削除/no-verify/admin/未公開path-git依存/私有資料公開/ゲート緩和。

## CI回帰対応

- [x] PR68 cffa46d5 P2実装修正: live sessionの既存LRUを先に参照し、disk復元pageも同LRUへ登録する。実filesystem41回帰・全runtime品質工程・3791関数/31226行100%を確認した。thread reply/resolve・最新review/CIは2.2で追跡する。証跡: `just VERSION=0.5.13 JOBS=2 release-check` PASS、file: `evidence/self-review.md`。delegation-exception: `直列のクリティカルパス`。

- [x] PR68 e7f6cd1c P2実装修正: 変換PDF本体も生バイト保存にし、64MiB payloadを128MiB cache内に保持する。共通payload codecの破損/制約回帰と全品質工程を回収した。review reply/resolve・fresh確認は2.2で追跡する。証跡: `just JOBS=2 coverage` PASS、file: `evidence/self-review.md`。delegation-exception: `直列のクリティカルパス`。

- [x] PR68 93bf2452 P2実装修正: 生RGBA保存で有効4096×4096ページを128MiB内に保持し、PDF検証成功後だけ変換artifactをpersistする。実filesystem境界・decode失敗回帰と全品質/100% coverageを確認した。review reply/resolve・fresh確認は2.2で追跡する。証跡: `just VERSION=0.5.13 JOBS=2 release-check` PASS、`evidence/self-review.md`。delegation-exception: `直列のクリティカルパス`。

- [x] PR68 1bdfa63b P2実装修正: saveの既存directory/symlinkをUnsafeDirectoryで拒否し、実filesystem RED/GREENと全品質ゲートを確認した。各thread reply/resolve・fresh確認は2.2で追跡する。delegation-exception: `直列のクリティカルパス`。
- [x] PR68 1bdfa63b P2実装修正: 実host image SHA256をkeyへ束縛し、描画依存のdownstream再buildで無効化する。回帰・実process性能・全品質ゲートを確認した。各thread reply/resolve・fresh確認は2.2で追跡する。delegation-exception: `直列のクリティカルパス`。

- [x] Ubuntuで不足したOffice cache state/artifact・PDF outlineのlibrary単体回帰を追加し、1bdfa63bと93bf2452のUbuntu CI成功で100% coverage復旧を確認した。最終KRR graphは2.1で再検証する。証跡: file: `evidence/self-review.md`、https://github.com/HiroyukiFuruno/katana-document-viewer/actions/runs/37417050580 。delegation-exception: `直列のクリティカルパス`。

- [x] PR68 P2: 同directoryを小さい総容量で開き直したloadをCapacityで拒否し、ちょうど上限・clear後復旧を実filesystem回帰で確認する。9d796fe3修正・reply/resolve・fresh確認済み。delegation-exception: `直列のクリティカルパス`。

## 継続・完了判定の是正

- [/] ユーザー指摘: ビルド生成物は毎回削除せず、約2回分の40GiBを超えたときだけ整理する。通常のjust導線へguardを組み込み、容量境界・所有範囲・使用中保護・実Cargo整理・排他保持を検証した。証跡: `just build-cache-script-test` 35 PASS、file: `docs/build-cache-maintenance.md`。delegation-exception: `直列のクリティカルパス`。

- [ ] ユーザー指摘: 実行可能な修正・検査・結果回収を残したまま進捗報告で停止しない。CI開始や自動化登録は完了ではない。KDV内の残作業を解消し、KRR公開以外に進められる作業がないことを証跡で確認してから待機する。delegation-exception: `直列のクリティカルパス`。

アンチパターン: push/CIを開始して終了し、ユーザーの再指示まで失敗回収・修正を進めない。正: 最新HEADの検査を回収し、失敗修正・review reply/resolve・再検査を継続する。担当が止まったら残DoDと停止理由を確認し、続行指示、担当変更または主担当引取を行い、証跡前に完了扱いしない。同じ規則を主担当自身にも適用し、チャット終了・heartbeat登録を作業完了の代わりにしない。ユーザーの再催促を待たず、既存範囲の修正・検査回収・証跡整備を完了する。検索語: 無意味な停止禁止 / actionable work before wait / subagent completion evidence。

- [x] 2026-10-14 依存更新後もKRRを待たずcoverage・semver・公開前検査を先行実施する。`just VERSION=0.5.13 JOBS=2 release-check` EXIT0、3805関数/31335行100%、semver196 PASS、package/publish dry-run/未公開確認成功。証跡: `tmp/dependency-release-precheck-20261014.log`、`tmp/release-continuation.md`。最終KRR取込後の2.1検査を代用しない。delegation-exception: `直列のクリティカルパス`。

- [x] PR68 P2のroot祖先symlink固定と描画worker指紋結合を実装する。rootはprepare後canonicalize、workerは描画時SHA256をpage keyへ含め前後変更時は保存拒否。実worker更新/rollback2件とcache単体59件成功。全ゲート・reply/resolve・fresh確認は2.2で追跡。delegation-exception: `直列のクリティカルパス`。

- [x] PR68の追加P2: worker更新時にlive LRUを破棄し、memory hit前にも指紋を照合する。実同session更新/rollback2件・cache59件・AST成功。Cargo-C後の相対target/env解釈は42回帰成功。全品質/currentreview/replyresolveは2.2で追跡。delegation-exception: `直列のクリティカルパス`。

- [x] PR68 P2: PDF保存失敗のlive hit化を防ぎ、容量確保後の同session再保存とrestart hitを実回帰する。Office変換完了後もworker依存キーを再確認し、変換中変更/rollback時の誤hitを拒否する。実worker3件・cache60件成功、ASTファイル分割と全ゲート/返信解決は2.2で継続。delegation-exception: `直列のクリティカルパス`。

- [x] PR68 P2: root構築時のdirectory handleを保持し、各操作前とlock取得後に実体を照合する。別directory/symlink置換を拒否し、原本復元後の再利用を実filesystem回帰で確認。cache62件・AST成功、全ゲート/返信解決は2.2で継続。delegation-exception: `直列のクリティカルパス`。

- [x] PR68追加P2: 検証後のroot並行置換にも備え、cap-stdの保持Dir相対へ全cache I/O・temp atomic公開・cleanupを移行。lock取得後の置換を決定的に再現し、元directoryのみを操作する回帰を追加。限定cache62件/AST成功、追加noclobber回帰と全ゲート/返信解決は2.2で継続。証跡: file: `tmp/pr68-anchored-cache-tests-final.log`、`tmp/pr68-anchored-cache-ast.log`。delegation-exception: `直列のクリティカルパス`。

- [x] PR68 P2: public used_bytesも同じ.lockを取得し、保存中temp/hardlinkの二重計測や消失entryの参照を防ぐ。実OSlock保持中のreader待機・temp削除/解放後stable値を回帰し、cache66件/AST成功。証跡: file: `tmp/pr68-used-bytes-lock-tests-final.log`。全ゲート/返信解決は2.2で継続。delegation-exception: `直列のクリティカルパス`。


## 2026-10-20 公開エンジン0.9系の先行準備

- [x] 公開前検査で発見した、自動clean後に補助ツールが非空targetを再生成すると所有タグが失われる経路を修正する。空の所有出力へだけタグを再作成し、非空・未所有directoryの採用拒否は維持する。実tiny Cargoの入れ子targetと再度の閾値cleanを回帰し、通常push・新HEADレビュー・CI・完全release-checkを回収する。delegation-exception: `直列のクリティカルパス`。 追補: HEAD4aeb7479で通常push/3OS CI39664459317/preflight39664459501/coverage3843関数31575行100%/semver202PASS/全thread解決を回収済み。証跡 `tmp/kdv-4aeb7479-preparation-receipt.md`。2026-10-29の新graph検査は末尾の別タスクで追跡し、この過去結果を流用しない。
- [x] 自動clean成功直後に空targetの所有タグを準備し直す。実offline Cargoの入れ子buildで旧実装のタグ欠落・次回拒否を再現し、修正後は2回のbuild/閾値cleanが成功した。標準49回帰終了0、独立レビュー指摘0、OpenSpec strict成功。Windowsのbinary suffixにも対応したが、修正後3 OS/full検査は上記で継続する。証跡: file: `tmp/pr68-23bb-semver-red.log`、file: `tmp/pr68-cache-repair-final-script-check.log`。delegation-exception: `直列のクリティカルパス`。
- [x] HEAD23bb9f65の通常push・最新Codexレビュー・3 OS CIを回収した。通常push55713終了0、Codex新規指摘0・全27thread解決済み、3 OS成功。Linuxは3843関数/31575行100%、未カバー0。公開前検査はsemver202成功後、所有タグの安全検査で停止したため、完全release-check成功とは扱わない。証跡: file: `tmp/pr68-23bb-ci-release-evidence.json`、file: `tmp/pr68-23bb-review-receipt.json`。delegation-exception: `直列のクリティカルパス`。
- [x] Issue参照の不足を重い品質検査より前にも検出する。通常push77570の品質delegate通過後のURL欠如拒否を受け、既存最終Issue検査を維持した予備検査を追加した。実Git fixtureで事前拒否・正常push・stdin引継ぎ・delegate後の再検査・既存delegate失敗伝播を検証。旧hookのRED後、標準release-governance-checkが終了0、独立レビュー指摘0。品質検査の省略や閾値緩和なし。証跡: file: `tmp/kdv-upstream09-normal-push-first-result.json`、file: `tmp/release-governance-check-current.log`。delegation-exception: `直列のクリティカルパス`。
- [x] KatanAの保護方針を参考に、clean/sweepも既存guardと同じlock・所有範囲・使用中検査を通す。sweepの失敗隠蔽を除去し、macOS lsofが検出行と終了1を返す場合も使用中として拒否する。実OS lock/open-file回帰と実Cargo fixtureを含む標準script suite48件成功、独立レビューの失敗伝播指摘を修正。通常の40GiB超過時のみ自動cleanする方針は維持する。証跡: file: `tmp/kdv-maintenance-full-script-check.log`。最新HEADの全品質ゲートは通常push hook/CIで回収する。delegation-exception: `直列のクリティカルパス`。
- [x] 公開IronCalc0.9.0/office2pdf0.9.1とZIP9.0.5へ先行移行し、結合セルの不正範囲拒否・1x1/重複geometry・streaming・metadata整合性を維持する。主担当が委譲した実装/実ZIP回帰と独立レビューを統合した。証跡: file: `evidence/self-review.md`。delegation-exception: `直列のクリティカルパス`。
- [x] 実ZIPの空filterColumn経路を回帰で補い、現行source/graphをRust1.99.1の標準coverageで検証する。70732 EXIT0、3839関数/31593行100%、未カバー0。証跡: file: `tmp/kdv-upstream09-canonical-coverage-complete.log`。delegation-exception: `直列のクリティカルパス`。
- [x] 配布パッケージの先行検査を回収する。98034 EXIT0、962files、配布package build/publish dry-run/0.5.13未公開確認成功。これは最後のtest-only回帰追加前のRust1.97.1互換性証跡であり、最終HEADのCI/package検査は別途回収する。証跡: file: `tmp/kdv-upstream09-publish-precheck-final.log`。delegation-exception: `直列のクリティカルパス`。
- [x] 認証済みAPIで上流5PRの全レビュー/threads/CIと公式registry/Releaseを再確認する。2026-10-20T09:24:21Z、新しいactionable findingなし。IronCalc#1565は承認・マージ済みだが、公式ZIP修正版/KRR新版は未公開。証跡: file: `tmp/upstream-20261020-latest-final-audit.json`。delegation-exception: `直列のクリティカルパス`。
- [x] 現行sourceの全通常検査を回収する。10774のfmt/容量guard42/strict Clippy/AST/境界・入口成功後、容量復旧した6955が終了0。workspace2352、Storybook646と分離1件、Office/V8/harness/crop検査成功。source/graph18件のhash不変。証跡: file: `tmp/kdv-upstream09-normal-check-resumed-final.log`。delegation-exception: `直列のクリティカルパス`。
- [x] 通常commit/push、最新HEAD Codex review/全指摘回収・reply/resolve、3 OS CI/既存release-preflightの全release-checkを完了する。最新HEADのsemver/coverage/package/publish dry-runも既存CIで回収し、最終公開前の2.1/2.2を代用しない。delegation-exception: `直列のクリティカルパス`。 追補: HEAD4aeb7479で通常push/3OS CI39664459317/preflight39664459501/coverage3843関数31575行100%/semver202PASS/全thread解決を回収済み。証跡 `tmp/kdv-4aeb7479-preparation-receipt.md`。2026-10-29の新graph検査は末尾の別タスクで追跡し、この過去結果を流用しない。
- [ ] 公式ZIP修正版が公開され次第registry依存として取り込み、#69の旧ZIP graphが解消したことを確認して全再検査する。KRR修正版の取込/最終公開は1.3/2.1–2.3で継続する。delegation-exception: `直列のクリティカルパス`。

上流のoffice2pdf/owner forksへの編集は、親Issue2323を根拠にした今回限りの明示許可と外部編集許可の範囲に限る。現公開0.9系にも旧ZIP依存が残るため、現状をKRRだけの待機と扱わない。

- [x] PR68 b834のP2: 所有タグはlstatで通常ファイルと確認し、外部署名へのsymlinkを未所有として拒否する。実Cargo maintenance経路の旧削除を再現し、自動閾値/明示cleanの両経路でタグ・外部ファイル・無関係出力を保持する51回帰が成功した。fixtureは一時領域内へ隔離した。証跡: file: `tmp/pr68-symlink-red.log`、file: `tmp/pr68-symlink-standard.log`。通常push・各thread reply/resolve・最新HEAD全ゲートは継続する。delegation-exception: `直列のクリティカルパス`。

- [x] PR68 dbd5のP2: 所有タグのlstat/read競合を、同一descriptorの属性検査と署名読取へ変更する。POSIXはNOFOLLOW/NONBLOCK、WindowsはOPEN_REPARSE_POINTと同HANDLE属性検査を使用し、inode照合だけのfallbackは採用しない。実maintenance競合の旧削除RED、修正後拒否・外部/出力保持、同inodeへのsymlink置換、FIFO置換、NOFOLLOW未提供の拒否を回帰した。標準56件成功（MacではWindows専用1件skip）。Windows nativeと最新HEAD全ゲートは通常push/3 OS CIで継続する。証跡: file: `tmp/pr68-tag-handle-dbd5-red.log`、file: `tmp/pr68-tag-handle-final-standard.log`。delegation-exception: `直列のクリティカルパス`。

## 2026-10-29 office2pdf公式ZIP9修正版の採用

- [x] 公式office2pdf0.10.0のRelease・registry checksum/VCSと派生0.2.0/ZIP9要件を確認し、既存ユーザー依存更新を保持してexact registry依存へ更新する。Cargo update EXIT0、旧ZIP0.6.6/2.5.0/8.6.0が消えZIP9.1.2のみ。証跡: `tmp/kdv-daily-publication-20261029.json`。delegation-exception: `直列のクリティカルパス`。
- [ ] 採用後の全品質・coverage関数/行100%・semver・package/dry-run・3OS CI・current HEAD reviewを回収し、失敗を修正する。通常check session18048開始、ログ `tmp/kdv-office010-check.log`。KRR0.4.23のまま公開しない。delegation-exception: `直列のクリティカルパス`。

- [x] 採用後の通常check99637を終了0で回収し、epaint_default_fonts0.37のfeature不足を修正する。新consumerの期待版/旧ZIP・旧派生混在拒否のself-testも成功。全CI/coverage/公開前検査/reviewは上の未完タスクで継続する。証跡: `tmp/kdv-office010-check-feature-fix.log`。delegation-exception: `直列のクリティカルパス`。

- [x] Ubuntu CIのPPTX行送り検査の原因を、Linux/macOSの実PDF baseline比較で特定する。黒画素中心のfont差による誤判定を、同じ19–24pt幅の実baseline検査へ修正し、実PDF行間圧縮の拒否と関連19回帰、strict Clippy/ASTを確認した。既存pixel検査と品質基準は保持。旧HEAD失敗/停止はPASSに数えず、新HEAD全CI/公開前検査/reviewを上記で回収する。証跡: `tmp/pptx-baseline-contracts.log`、`tmp/pptx-office010-linux-macos-baseline-diagnostic.json`。delegation-exception: `直列のクリティカルパス`。

- [x] 待機中に追加された互換lock更新4件を保持し、ZIP9.1.3/ignore0.4.34/globset0.4.21/unicode-width0.2.3の公式checksumと非yankを確認した。最新graphのstrict Clippy/AST49621と実worker19回帰80084は終了0。全検査・レビューは新HEADで上記未完タスクを継続する。証跡: file: `tmp/kdv-extra-lock-registry-verified-20261029.json`、file: `tmp/kdv-latest-lock-office-contracts.log`。delegation-exception: `直列のクリティカルパス`。

- [x] 公開契約の旧pixel文言依存を実PDF baselineの正例/圧縮拒否・同じ19–24pt幅へ整合させる。静的公開契約を通常check先頭へ追加し、重いbuild前にpre-pushで不整合を検出する。prerequisiteなしRED後、現契約/負例self-test/実行順序とgovernance/harness/AST57032が成功した。既存ゲート/閾値は保持し、新HEADの全CI/preflight/reviewは上記で継続する。証跡: file: `tmp/kdv-contract-source-gate-green.log`、file: `tmp/kdv-contract-source-harness.log`。delegation-exception: `直列のクリティカルパス`。

- [x] PR68のCargo設定出力先の容量監視漏れを、Cargo metadataによる実効target解決で修正する。設定ファイル/別config/inline configの実build3経路でRED→GREEN、従来の安全な整理・使用中保護を含む78回帰と公開契約/AST88995が成功した。新HEAD全CI/preflight/reviewは既存未完タスクで回収する。証跡: file: `tmp/kdv-cargo-config-regression-red.log`、file: `tmp/kdv-cargo-config-final-gates.log`。delegation: `/root/cargo_config_guard_audit`（回帰テスト限定、toolはcanonical名のみ返却）。実装修正と検証統合はmainが実施。delegation-exception: `直列のクリティカルパス`。

- [x] clean時もCargo設定済み出力先だけを選び、無関係な既定targetを残す。実build/cleanのRED→GREENと79容量管理回帰、公開契約/ASTを確認した。設定解決前にも既存のmaintenance selector拒否を適用し、不明な出力先は安全側拒否とする。新HEAD全ゲートは既存未完タスクで回収する。証跡: file: `tmp/kdv-cargo-config-clean-red.log`、file: `tmp/kdv-cargo-config-clean-final.log`。mainの実装修正・統合: delegation-exception: `直列のクリティカルパス`。
