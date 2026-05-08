---
description: 指定バージョンの OpenSpec 実装、品質確認、リリース準備、PR 作成、自動リリース確認までを進めるワークフロー。
---

# /impl-release vX.Y.Z

指定バージョンの実装から、`release/vX.Y.Z` 取り込み依頼（Pull Request）、`master` 取り込み後の自動リリース確認までを進める。

## 前提

- 作業対象 repository は `katana-document-viewer`
- default branch は `master`
- release branch は `release/vX.Y.Z`
- release は `master` へ merge された `release/vX.Y.Z` Pull Request を起点に GitHub Actions が実行する
- 公開に必要な秘匿値（secret）は `CARGO_REGISTRY_TOKEN`
- 手動の `cargo publish` や tag 作成で自動リリースを迂回しない

## Phase 0: 状態確認

```bash
git status --short --branch
git fetch origin --prune --tags
just VERSION=vX.Y.Z release-target-check
```

既存差分がある場合は、release 作業へ混ぜる前に関心事を分ける。
版番号が公開済み release line から見て不自然な場合は、ユーザーへ確認して止める。

## Phase 1: 実装

```bash
git switch master
git pull --ff-only origin master
git switch -c release/vX.Y.Z
```

対象 OpenSpec change がある場合は、`proposal.md`、`design.md`、`tasks.md`、`specs/**/spec.md` を読んでから実装する。
完了した task だけ `tasks.md` の checkbox を更新する。

## Phase 2: 品質確認

```bash
just check
just VERSION=vX.Y.Z release-check
git diff --check
```

失敗した場合は、allow や除外設定で逃げず、設計またはテストを直して同じ入口を再実行する。

## Phase 3: PR 作成

```bash
git status --short --branch
git add <release に必要な files>
git commit -m "release: vX.Y.Z リリース準備"
git push -u origin release/vX.Y.Z
gh pr create --base master --head release/vX.Y.Z --title "Prepare vX.Y.Z release" --body-file <pr-body-file>
```

Pull Request 本文には、少なくとも `just VERSION=vX.Y.Z release-check` の結果を書く。

## Phase 4: PR gate

必須検査（required check）は次を確認する。

- `Test and Build (macos-latest)`
- `Test and Build (ubuntu-latest)`
- `Test and Build (windows-latest)`
- `preflight`

```bash
gh pr checks <PR番号またはURL> --watch
```

`--admin` での強制 merge は使わない。

## Phase 5: merge と自動リリース

merge はユーザー承認後だけ実行する。

```bash
gh pr merge --merge --delete-branch <PR番号またはURL>
git switch master
git pull --ff-only origin master
gh run list --repo HiroyukiFuruno/katana-document-viewer --workflow Release --limit 5
```

Release workflow が成功したら、GitHub Release と crates.io 公開状態を確認する。
