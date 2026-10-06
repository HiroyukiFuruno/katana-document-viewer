# KDV v0.5.13 引継ぎ

最新指示とDoDは ../../issue-release-tasks.md と tasks.md を読み直す。Issue #65/#66/#67のKDV内対応後、対応中KRR新版の公開を待ってregistry依存を更新し、最終graphのゲートとreview後に公開する。旧KRR 0.4.23では公開しない。

限定調査・回帰はLuna担当へ分離し、主担当が統合と自己レビューを実施した。以下は依存順に進める主担当の引継ぎである。

- [x] 公開cache契約・実process回帰・合法PPTX診断を統合した。delegation-exception: `直列のクリティカルパス`。
- [ ] 現検証graphのDraft PRを作成してreviewを取得する。delegation-exception: `直列のクリティカルパス`。
- [ ] KRR対応の公開後にmanifest/lock/release契約/registry consumer期待版を更新し、最終source/graphでrelease-checkを通す。delegation-exception: `直列のクリティカルパス`。
- [ ] current HEAD review、指摘reply/resolve、fresh P0P1ゼロ、Ready、required CI、merge、Actions公開、fresh registry consumer、Issue Close、ローカル整理。delegation-exception: `直列のクリティカルパス`。

現sourceのrelease-checkはPASS（関数3776/行31090とも100%、package/dry-run）。commit f0f7d41aの通常pushは実装検査を通過後、開始済みchangeのこの引継ぎ文書不足で拒否された。文書追加後にharnessを再確認し、通常pushを再実行する。新worktree/stash/master編集/他repo編集/私有資料公開/閾値変更/remote branch削除は禁止。
