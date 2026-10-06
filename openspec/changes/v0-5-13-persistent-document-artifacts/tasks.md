<!-- subagent-spark-harness-strict-start -->

# 主担当の統合・検証タスク

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

- [/] ユーザー指摘: ビルド生成物は毎回削除せず、約2回分の40GiBを超えたときだけ整理する。通常のjust導線へguardを組み込み、容量境界・所有範囲・使用中保護・実Cargo整理・排他保持を検証した。証跡: `just build-cache-script-test` 23 PASS、file: `docs/build-cache-maintenance.md`。delegation-exception: `直列のクリティカルパス`。

- [ ] ユーザー指摘: 実行可能な修正・検査・結果回収を残したまま進捗報告で停止しない。CI開始や自動化登録は完了ではない。KDV内の残作業を解消し、KRR公開以外に進められる作業がないことを証跡で確認してから待機する。delegation-exception: `直列のクリティカルパス`。

アンチパターン: push/CIを開始して終了し、ユーザーの再指示まで失敗回収・修正を進めない。正: 最新HEADの検査を回収し、失敗修正・review reply/resolve・再検査を継続する。担当が止まったら残DoDと停止理由を確認し、続行指示、担当変更または主担当引取を行い、証跡前に完了扱いしない。同じ規則を主担当自身にも適用し、チャット終了・heartbeat登録を作業完了の代わりにしない。ユーザーの再催促を待たず、既存範囲の修正・検査回収・証跡整備を完了する。検索語: 無意味な停止禁止 / actionable work before wait / subagent completion evidence。
