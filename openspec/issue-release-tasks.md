# Issue #65–#67 / KRR更新取り込みリリース

最新指示: 2026-10-06。Issue対応後、進行中のKRR修正版が公開されてから取り込み、KDVをリリースする。KDVが先に終わってもKRRを待つ。

## 対象と完了条件

- [ ] #65: cold/warm/独立プロセス再起動で取得・変換・decode・raster・初回frameを計測し、所有する遅延を改善。匿名入力で再現入口を残す。
- [ ] #66: host所有の保存場所・clear方針、内容/版/config依存の無効化、安全制約、破損/欠損/原子書込、独立プロセスcache hitと変更時missの公開契約・回帰。
- [ ] #67: 合法PPTX ZIPのlocal-header/data-descriptor/central-directory経路を再現し、安全制約維持で修正または原因根拠を確定。原本未入手の検証範囲を明記。
- [ ] 直接・推移依存とlockfileを調査し、既存ゲートを維持する更新を採用。
- [ ] KRR「Fix Mermaid classDiagram parsing」の対応・公開完了を確認し、公開registry版を依存に取り込む。現時点公開版は0.4.23で、対応中の新版を待つ。
- [ ] 全既存品質ゲート、Draft PR、review、指摘reply/resolve、fresh P0/P1ゼロ確認、Ready、required CI、merge、自動公開。
- [ ] fresh registry consumer、Issue根拠付きClose、branch-hygieneでローカル整理。

## 制約

KDV-local DoD。下流実アプリ/配布/OS clipboardをKDV完了条件に追加しない。品質閾値/coverage/referenceを下げない。私有原本・画像はGitや公開Issueへ載せない。他repo編集・新worktree・stash作成禁止。master cleanとstash 0を維持。作業はrelease/v0.5.13、公開API追加に伴う版判断はrelease contractで再確認する。

## 現状

2026-10-22更新: IronCalc0.9.1の公式Releaseとregistry公開、ZIP9.0.5依存を確認し、manifest/lock/release契約へ取り込んだ。Python公開契約と自己検査は成功。office2pdfと派生2packageのZIP修正版、KRR新版は引き続き公開待ち。PR68のtarget pathname差し替え指摘は、POSIX directory FD / Windows pinned HANDLEによる内容整理へ変更して対応中。最新sourceの全ゲートとWindows native回帰を回収するまで、この追加修正を完了扱いしない。

2026-10-20更新: 公開IronCalc0.9.0/office2pdf0.9.1へ先行移行し、結合セルの互換性を維持した。標準coverageは3839関数/31593行100%、通常検査も成功。最新の追跡は `changes/v0-5-13-persistent-document-artifacts/tasks.md` の「2026-10-20 公開エンジン0.9系の先行準備」を参照する。ZIP修正PRはIronCalcを含め対象owner側でマージ済みだが、公式修正版のregistry公開は未確認で、KRR新版も未公開。旧ZIPを含む現公開版を最終リリースに使用しない。追加のclean/sweep使用中保護と最新HEADのpush/review/CI/公開前検査を先行して完了する。

master 99371a97からrelease/v0.5.13を作成。stash 0、worktreeは現checkoutのみ。KRRの既存担当はHTML画像99の修正と品質検査を継続中。永続cache API・実独立process回帰・PDF段階traceを追加。release-target-check、AST、semver-check（196 PASS）、公開APIエラー経路を含む限定回帰はPASS。coverageは全体実行に追加integration profileを合算してfunctions/lines 100%。現在の公開KRR 0.4.23を使うrelease-checkは終了コード0でPASS（最新sourceの3776関数/31090行100%、package/dry-run含む）。Draftレビューへ進む。これは最終KRR取り込み・公開完了の証跡とは扱わず、新版公開後に同じ最終source/graphでrelease-checkとcurrent HEAD reviewを再確認する。

私有DOCX/PDF6件の取得からRGBA artifactまでをcold/restart/warmで実測し、全12process成功、restartの変換/raster省略を確認。DOCX cold最大3.753秒は当時のcycle値であり、初回frame単独の値ではない。目安未達として記録する。#67の歴史的エラーはPNG内の過去表示と判明し、原本は未入手。合法descriptor回帰と当時の局所検証PPTX3件×2回は成功したが、未再現の歴史的原本を修正済みとは扱わない。KDV #67とKatanA owner #345へ根拠を投稿済み。

2026-10-29更新: office2pdf0.10.0と派生2package0.2.0が公式公開済みとなり、checksum/VCS/修正PR包含を確認してKDVへ採用した。lockから旧ZIP0.6/2.5/8.6が消え、追加互換更新後はZIP9.1.3のみ。ユーザーの既存依存更新を保持し、epaint_default_fonts0.37でfeature化された既存絵文字fontも明示有効化した。採用後の通常check/全品質・公開前検査・レビューは `changes/v0-5-13-persistent-document-artifacts/tasks.md` 末尾で追跡する。KRRは0.4.23のままで、修正版公開・採用後までKDV公開は待つ。

2026-10-29 HEAD `dc8782d0eaeea19208ef8bbea2c21375fcf4b07c` はoffice2pdf 0.10.0/派生2package 0.2.0/ZIP 9.1.3採用後の独立準備として、通常push session 35287 EXIT0、CI 40852797638（Ubuntu/Windows/macOS全成功）、preflight 40852797894成功を回収した。coverageは3843関数/31575行・未カバー0（regions 98.71%）、semverは202 pass/58 applicability skip、962 package files検証とpublish dry-run成功、Codex reviewはexact dc8782d0で重大指摘なし・全36 thread解決済み。これはdc8782d0時点の実績で、文書反映後のHEAD結果はPR68と最新の物理receiptへ別途記録する。未回収の結果は成功と扱わない。v0.5.13は未公開で、fresh registry consumerも未実施。KRR 0.4.23のまま公開せず、公式修正版の公開・採用を待つ。証跡: `tmp/kdv-dc8782d0-preparation-receipt.json`。
