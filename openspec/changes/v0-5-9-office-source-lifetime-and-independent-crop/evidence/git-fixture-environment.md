# Issue #60: fixtureのGit環境隔離

現在のfatal復旧は `core.bare=false` のみで、`git st` exit0を確認済み。

old HEAD 6ae11c2のgovernance `_self_test_new_branch_range` を独立した実Git metadataへ `GIT_DIR` / `GIT_WORK_TREE` を継承して実行すると、caller configが変更され、fixtureのgit addが失敗した。これは実際のmetadata破損のREDであり、過去の `core.bare=true` 発生経路そのものを確定した証拠ではない。

`git_fixture_environment.isolated_git_fixture` はself-testだけnative `git rev-parse --local-env-vars`を除外し、finallyで復元する。通常のpush検証/cleanup処理は従来どおりの環境を使う。fixtureへコピーするvalidatorとhelperの組を維持した。

`rtk proxy just release-governance-check` exit0。実Gitでcaller GIT_DIRのみ/full local環境の各2scriptを実行し、metadataの全file byte snapshot、caller作業領域の非変更を確認した。別unit回帰で例外時の環境復元と非Git markerの保持を確認した。

実KDV repoの実行前後SHAは一致:

- config `f7a0f765a73ea575775fd6c6b9d8936b8a49ae3422c95a55cad93bd40e59fee0`
- index `607eaee8139197e50b75633d4c5b402f06ed5a973c9efffc9947917fca335df9`

既存5stashは保持、新stash/autostash・製品worktree/clone・sibling編集は実施しない。新しく初期化したGit repositoryは既存self-testに必要な一時fixtureのみ。

公開、current-HEAD review/CI、公開後Issue Closeは未完了。

既存Release workflowのremote `--apply` は、最新ユーザー指示「remoteの明示依頼なしでは削除しない」と矛盾していたため、公開後の非破壊監査へ変更した。release contractは自動削除設定を拒否するRED→GREENを確認し、公開→registry→監査の順序を維持する。これをbranch削除完了とは扱わない。

独立readonlyレビュー: agent `01a0fc00-0761-7640-b2d0-3ed43bf6251b` / `gpt-6-luna` / `medium`、指定Git fixture/helper/copy validator差分にP0/P1なし。mainが実Git回帰とSHAを別途確認した。
