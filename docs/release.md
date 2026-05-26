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
- `kdv` の梱包（package）と公開の事前実行（publish dry-run）

crates.io 公開対象は `kdv` だけにする。
`katana-document-preview`、`katana-document-preview-egui`、`katana-document-viewer`、`katana-document-viewer-cli` は crates.io 公開対象にしない。

## 公開順序

取り込み（merge）後の `Release` ワークフロー（workflow）は次の順で動く。

1. `just release-verify`
2. リリースタグ（release tag）作成
3. GitHub リリース（GitHub Release）作成
4. `kdv` を crates.io に公開

## crates.io の取り消し

誤って crates.io に公開した版は、GitHub Actions ではなくローカルの `cargo yank` で取り消す。
この操作は一度きりの管理作業であり、workflow 化しない。

```bash
cd /Users/hiroyuki_furuno/works/private/katana-document-viewer
cargo yank katana-document-preview --version 0.1.1 --registry crates-io
cargo yank katana-document-preview-egui --version 0.1.1 --registry crates-io
cargo yank katana-document-viewer --version 0.1.0 --registry crates-io
```

`katana-document-viewer-cli` は未公開なら取り消し対象にしない。
