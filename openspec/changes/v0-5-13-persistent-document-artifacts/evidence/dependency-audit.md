# 依存確認 2026-10-06

## 2026-10-22 IronCalcの公式ZIP修正版

IronCalc0.9.1は2026-10-21T13:39:32Zに公開済み。checksumは`aabf0987c9f8e941476a7bfd2564496ca9f97bbf25a7165f7d499895b35fff0c`で、公式ReleaseがPR1565を含む。manifest/lock/release契約を0.9.1へ変更し、通常のprecise updateは終了0。IronCalcのZIP0.6/thiserror1のedgeがZIP9.0.5/thiserror2へ変わり、itertools0.12.1は0.15.0へ移行した。同時に既存derive_arbitrary1.5.0のsyn依存edgeが2.0.119から3.0.9へ解決された。Python公開契約とself-testは成功したが、追加sourceのRust全ゲートは再検証中。

office2pdf0.9.1とowner fork0.1.0には旧ZIPが残り、公式修正版は未公開。KRRも公開0.4.23のまま。従ってIronCalc更新だけで#69解消やKRRだけの待機とは扱わない。監査証跡: `tmp/kdv-upstream-publication-20261022.json`。

先行標準checkはexit101。ironcalcのみのprecise updateでは互換要件`0.9`内の既存ironcalc_base0.9.0が保持され、0.9.1で削除されたWorkbook/Worksheet.viewsにより上流crate自身がコンパイル失敗した。公式公開済みironcalc_base0.9.1へ更新し、必要なstatrs0.19.1も採用した。公開契約へbase版を追加し、古いbaseを拒否するself-testを追加した。Python契約/self-testは成功、修正graphの全検査を再実行する。証跡: `tmp/kdv-ironcalc091-anchor-check.log`、`tmp/kdv-ironcalc-base091-update.log`。未公開patchやregistry sourceの直接変更は行っていない。

上流の依存下限について[IronCalc #1578](https://github.com/ironcalc/IronCalc/issues/1578)へ実際のコンパイルエラーと組合せ、下限0.9.1への修正案を報告した。KDV側では公開済みbaseへの更新で解消できるため、このIssueの解決待ちをKDVの完了条件へ追加しない。

- 開始時: KDV 0.5.12、KRR 0.4.23、office2pdf 0.8.1、zip直依存8.6.0。workspace全dependencyとCargo.lockを確認。
- `cargo outdated --workspace --depth 1 --exit-code 0`: All dependencies are up to date。現在の互換要件内でdirect dependency更新候補なし。major候補の評価と全推移lock確認は継続する。
- 永続artifact内容hashに、既に推移graphへ存在するsha2 0.11をdirect dependencyとして追加。ゲートは変更しない。
- KRRの既存チャットはHTML Chrome比較99以上とMermaid修正の対応中。現時点の最新公開は0.4.23のため、完了した新版の公開確認を待ってから依存下限・lock・公開契約へ反映する。
- 公開前: registry source/checksum、inverse tree/V8 singleton、全品質/semver/score/coverageを再検証する。
- `cargo outdated --workspace --aggressive --depth 1 --exit-code 0` もAll dependencies are up to date。現在の公開direct major候補はなく、推移graphはworkspace updateで更新候補0件。最終KRR版公開後にlock graphを再評価する。

PATH worker解決には公開registryのwhich8.0.6を直接採用（MSRV1.70はworkspace1.95.0以下）。既存V8 buildのwhich6.0.3はそのままで、lockの追加はwhich8.0.6の1packageだけ。他dependencyの更新なし。bare executableだけ実体へ解決し、同じ絶対pathをhashとspawnへ束縛する。直接file hashの既存契約を保持する。最終graphの全品質ゲートで採用可否を判定する。

## 2026-10-15 zip 9と公開前先行検査

ユーザー実行の依存更新でzip9.0.2と推移25件を採用。ZipFile::nameのResult<Cow<str>>を既存のarchive/engineエラーへ伝播し、production2箇所・fixture5箇所を移行した。安全制約・公開API・期待値・ゲート変更なし。更新graphでcheckとrelease-checkがともにEXIT0。3805関数/31335行100%、semver196 PASS、package951files/publish dry-run/未公開確認成功。macOS package linkerにcompact unwindのサイズ警告あり。公開KRRはRelease/registryとも0.4.23で、最終リリースは修正版公開後に取込・再検査する。

## root競合対策のcapability依存

公開registry cap-std4.0.3を追加し、cap-primitives/fs-set-times/io-extras/io-lifetimes/ipnet/maybe-owned/rustix-linux-procfs/winx/ambient-authorityの追加lock11件を確認。既存依存の無関係更新は行わず、Dir相対I/OとWindows no-delete-shareを使用する。Windows実体比較は既にgraph内のwindows-sys0.61.2を直接指定し128bit FILE_ID_INFOを使用。通常check・semver196・3823関数31449行100%・package/publishdryrunで採用を確認。公開KRRは再確認でも0.4.23、修正版待ちは維持。

## 2026-10-20 公開済み0.9系列への先行移行

IronCalcを0.9.0、office2pdfを0.9.1、直接ZIPを9.0.5へ更新した。すべて公開registry依存で、最終path/git依存は追加しない。IronCalcの型変更だけでなく、不正・単一セル・重複結合範囲を省略する入力処理の差異を評価した。既存worksheet XML読み取り内でmodel経路だけ結合範囲を検証・保持し、従来の不正範囲エラーとgeometryを維持する。streamingは従来経路を維持する。

上流office2pdf#2325と派生docx#13/umya#19はマージ済み・CI成功だが、修正版registryは未公開。IronCalc#1565は2026-10-20T08:13:24Zに承認後マージ済みで、exact-head CI6件成功。公式公開版はIronCalc0.9.0/office2pdf0.9.1/派生2package0.1.0で、推移ZIP0.6/2.4/8.6が残る。今回の移行だけでZIP問題解消とは扱わない。公開順序は派生package→office2pdf、別途IronCalcとし、公式版取得後にno-ignoreの最終graphを再検証する。

KRRはGitHub Release/registryとも0.4.23で、新版待ちは維持する。全5PRのレビュー・CI・公式packageを再確認した証跡は`tmp/upstream-20261020-final-publication.json`。新しい未対応レビュー指摘はない。現在のKDV全品質検査・coverage・公開前検査は別途最終結果を回収し、旧HEADの成功を代用しない。


2026-10-20T04:59:23Zに5上流PRのレビュー・CI・mergeと公式registry/Releaseを再確認した。office2pdf#2325、owner docx#13、owner umya#19は統合済みで新たな未対応指摘なし。IronCalc#1565は6checks成功・レビュー/merge待ち。元umya#384はownerによるworkflow承認待ち。公式IronCalc0.9.0/office2pdf0.9.1/owner forks0.1.0には旧ZIPが残り、修正版公式packageの公開は未完。KRRも0.4.23のままであり、KRRだけを待てば最終公開できる状態とは報告しない。詳細のローカル監査証跡は `tmp/upstream-20261020-current-publication.json`。上流への公開順序依頼はoffice2pdf Issue2323 comment6281792811で追跡する。


2026-10-20T09:24:21Zの認証済み全ページ監査で、IronCalc#1565の保守者nhatcherによる承認（08:13:13Z）とマージ（08:13:24Z）を確認した。全5PRの未解決thread/新しいactionable findingは0。IronCalcの公式registryは依然0.9.0（zip ^0.6.6）であり、残る条件は修正版公式公開。office2pdf/owner forksとKRRの公開状況は未変更。元umya#384のworkflow承認待ちは残るが、owner forkの公式公開graphへの移行を妨げる条件には追加しない。証跡: `tmp/upstream-20261020-latest-final-audit.json`。

## 2026-10-29 公式office2pdf0.10.0の採用

公式Release v0.10.0とregistry公開を確認した。archive SHA256は`bb210d293ca16b7ca9874a82390c9e1857558e2a97137d003a2b382de64a7a2e`、VCSは`e8d7378f49823d03208f52eb86ab6f0a12b71d8e`。PR2325 merge `aa226d197b202160fd2e9d9bf73d47fe1d733316`から87 ahead/0 behindで修正を包含する。派生2package0.2.0は前日の検証と同一checksum。

ユーザーの既存依存更新を保持してexact office2pdf0.10.0へ更新し、Cargo updateは終了0。実lock graphはoffice2pdf0.10.0、派生2package0.2.0、ZIP9.1.2の一版のみ。旧ZIP0.6.6/2.5.0/8.6.0は除去された。release契約の選択engineはmanifestのexact値から導出される。公開後registry consumerにもoffice2pdf0.10.0/派生0.2.0の期待版と旧ZIP・旧派生混在の拒否を追加し、自己検査で正常graph・旧版混在・office欠落を検証した。契約self-testと静的release契約は成功、通常全検査と実consumer契約は実行中。まだ全検査成功やKDV公開完了とは扱わない。KRRはregistry/Release0.4.23のままで、修正版公開後の採用と再検査が必要。

証跡: `tmp/kdv-daily-publication-20261029.json`、`tmp/kdv-office010-check.log`、`tmp/kdv-office010-contract.log`。

通常check18048はexit101でepaint_default_fonts0.37の既存定数4箇所がfeature未指定と判明した。公式0.37のCargo.toml/src定義を確認し`monochrome_emoji_fonts`を明示する。ユーザーの0.37更新と既存fallbackを維持し、ゲート緩和なし。重複起動して待機後にbuildへ入った実consumer契約54461は、同じ既知の不備と競合buildを避けるため自分のcargo PID77260へTERMで中断した（exit241、成功に数えない）。修正後は通常check、公開前検査を直列で回収する。

通常全品質check99637はactual EXIT0（`tmp/kdv-office010-check-feature-fix.log`）。fmt/strict Clippy/AST、Rust test結果の集計3003件（Storybook646件+分離1件を含む）、容量guard75回帰（macOS専用条件による6skip）、既存harnessが成功した。新consumer契約self-testもEXIT0。3OS CI・coverage/semver/package/dry-run・current HEAD reviewは、この新graphでは未回収として継続する。
