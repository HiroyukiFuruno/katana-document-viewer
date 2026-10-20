# 依存確認 2026-10-06

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
