# リリース手順

## 方針

`release/vX.Y.Z` ブランチから `master` へ取り込み依頼（Pull Request）を作る。
その取り込み依頼（Pull Request）では通常の品質ゲート（quality gate）とリリース前検査を必須にする。
取り込み（merge）後は自動実行基盤（GitHub Actions）がタグ（tag）、GitHub リリース（GitHub Release）、crates.io 公開を実行する。

## 必須検査

GitHub のブランチ保護（branch protection）では、少なくとも次を必須検査（required check）にする。

- `Test and Build (macos-latest)`
- `Test and Build (ubuntu-latest)`
- `Test and Build (windows-latest)`
- `preflight`

## リリース前検査

`preflight` は `release/v...` ブランチの取り込み依頼（Pull Request）で `just release-check` を実行する。
内容は次の通り。

- 整形確認（format）、静的検査（lint）、単体テスト（unit test）、`kal check` による抽象構文木検査（AST lint）
- カバレッジ（coverage）。現状の下限は行カバレッジ（line coverage）100%
- `Cargo.toml` の版番号（version）とブランチ版番号（branch version）の一致
- 作業領域（workspace）内部依存の版番号（version）一致
- 対象版番号（version）が crates.io に未公開であること
- crates.io 公開対象が無効化されていること

`katana-document-preview`、`katana-document-preview-egui`、`katana-document-viewer`、`katana-document-viewer-cli` は crates.io 公開対象にしない。
crate 名の整理が完了するまで、自動リリースはGitHub Releaseまでで止める。

## 公開順序

取り込み（merge）後の `Release` ワークフロー（workflow）は次の順で動く。

1. `just release-verify`
2. リリースタグ（release tag）作成
3. GitHub リリース（GitHub Release）作成
4. crates.io 公開は実行しない

## 必要な秘匿値

自動実行基盤（GitHub Actions）には次の秘匿値（secret）が必要。
値は crates.io の API トークン（API token）を使う。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-document-viewer
gh secret set CARGO_REGISTRY_TOKEN
```

トークン（token）は秘匿値として扱い、リポジトリ（repository）に保存しない。
