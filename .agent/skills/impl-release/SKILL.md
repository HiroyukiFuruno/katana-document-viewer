---
name: impl-release
description: katana-document-viewer で指定バージョンの OpenSpec 実装、品質確認、リリース PR、自動リリース確認までを進めるときに使う。/impl-release vX.Y.Z と同等のリリース実装ワークフロー。
---

# impl-release

このスキルは、`/impl-release vX.Y.Z` として扱うリリース実装ワークフローの入口です。

## 実行ルール

1. ユーザー指定のバージョン（例: `v0.1.0`）を対象にする。
2. 作業開始前に `git status --short --branch` と `git fetch origin --prune --tags` を実行する。
3. `just VERSION=vX.Y.Z release-target-check` で、公開済み release line から見て自然な次版であることを確認する。
4. 作業ブランチは `release/vX.Y.Z` に統一する。
5. 直接 `cargo publish` や tag 作成で迂回しない。公開は `master` merge 後の GitHub Actions に任せる。
6. 詳細手順は `.codex/workflows/impl-release.md` を正として読む。

## 必須検査

- `Test and Build (macos-latest)`
- `Test and Build (ubuntu-latest)`
- `Test and Build (windows-latest)`
- `preflight`
