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

master 99371a97からrelease/v0.5.13を作成。stash 0、worktreeは現checkoutのみ。KRRの既存担当はHTML画像99の修正と品質検査を継続中。永続cache API・実独立process回帰・PDF段階traceを追加。release-target-check、AST、semver-check（196 PASS）、公開APIエラー経路を含む限定回帰はPASS。coverageは全体実行に追加integration profileを合算してfunctions/lines 100%。現在の公開KRR 0.4.23を使うrelease-checkは終了コード0でPASS（最新sourceの3776関数/31090行100%、package/dry-run含む）。Draftレビューへ進む。これは最終KRR取り込み・公開完了の証跡とは扱わず、新版公開後に同じ最終source/graphでrelease-checkとcurrent HEAD reviewを再確認する。

私有DOCX/PDF6件の取得からRGBA artifactまでをcold/restart/warmで実測し、全12process成功、restartの変換/raster省略を確認。DOCX cold最大3.753秒は目安未達として記録。#67の歴史的エラーはPNG内の過去表示と判明し、原本は未入手。合法descriptor回帰と現在の原本PPTX3件×2回は成功、未再現の原本を修正済みとは扱わない。KDV #67とKatanA owner #345へ根拠を投稿済み。
