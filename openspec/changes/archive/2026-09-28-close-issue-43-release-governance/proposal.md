## Why

KDV の変更が Issue、依存更新の証跡、公開後の安全な cleanup と結び付いていないと、下流への未公開依存や未統合 branch を残したまま release を進められてしまう。Issue #43 の契約を repository 内で検証可能にし、release 完了後だけ安全な整理を実行する。

## What Changes

- 非 default branch の commit で対象 repository の Open Issue 参照を必須にする pre-commit/pre-push dispatcher を追加する。
- 下流依存を更新する変更に、上流公開版、migration note、manifest、lockfile、検証証跡を要求する検証を追加する。
- repository 固有の既存 hook を dispatcher から先に委譲し、失敗を隠さない。
- 公開済み GitHub Release を確認した場合だけ実行できる release cleanup を追加し、clean・merged・未使用の local branch/worktree だけを整理対象にする。
- release workflow から publish 成功後の cleanup を呼び出し、cleanup 不能な対象は削除せず明示的に失敗させる。

## Capabilities

### New Capabilities

- `issue-linked-change-governance`: 非 default branch の変更と下流依存更新を Issue と検証証跡に結び付ける。
- `post-release-safe-cleanup`: 公開済み release 後にのみ安全条件を満たす Git branch と worktree を整理する。

### Modified Capabilities

- なし。

## Impact

- `.githooks/`、release scripts、GitHub Actions release workflow、Justfile の release 導線、および hook/cleanup の自動テスト。
- 対象 repository 内の Git metadata と GitHub Release API を読み取るが、未使用かつ安全条件を満たす対象以外は削除しない。
