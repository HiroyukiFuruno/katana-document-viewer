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

- [ ] Ubuntuで不足したOffice cache state/artifact・PDF outlineのlibrary単体回帰を追加し、100% coverageをCIで再確認する。delegation-exception: `直列のクリティカルパス`。

- [ ] PR68 P2: 同directoryを小さい総容量で開き直したloadをCapacityで拒否し、ちょうど上限・clear後復旧を実filesystem回帰で確認する。delegation-exception: `直列のクリティカルパス`。
